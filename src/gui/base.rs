use std::ptr::NonNull;

use crate::co;
use crate::decl::*;
use crate::gui::{*, events::*, privs::*};
use crate::msg::*;
use crate::prelude::*;

/// Allocated on the heap and passed through `WM_UI_THREAD`.
struct ThreadPack {
	func: Box<dyn FnOnce() -> AnyResult<()>>,
}

//------------------------------------------------------------------------------

/// Base to `RawBase` and `DlgBase`, which means all container windows.
pub(in crate::gui) struct Base {
	hwnd: HWND,
	is_dialog: bool,
	parent_ptr: Option<NonNull<Self>>, // used only during creation stuff
	privileged_events: WindowEventsPriv, // inserted internally to automate tasks: all will be executed
	user_events: WindowEventsAll, // ordinary window events, inserted by user: only last added is executed (overwrite previous)
	privileged_events_post: WindowEventsPriv, // all will be executed after user events
	layout_arranger: LayoutArranger,
}

impl Base {
	const WM_UI_THREAD: co::WM = unsafe { co::WM::from_raw(co::WM::APP.raw() + 0x3fff) };

	pub(in crate::gui) unsafe fn from_guiparent<'a>(
		p: &impl GuiParent,
	) -> &'a Self
	{
		let ptr = p.as_base() as *mut Self;
		ptr.as_ref().unwrap()
	}

	pub(in crate::gui) fn new(
		is_dialog: bool,
		parent: Option<&Base>,
	) -> Self
	{
		let new_self = Self {
			hwnd: HWND::NULL,
			is_dialog,
			parent_ptr: parent.map(|parent| NonNull::from(parent)),
			privileged_events: WindowEventsPriv::new(is_dialog),
			user_events: WindowEventsAll::new(),
			privileged_events_post: WindowEventsPriv::new(is_dialog),
			layout_arranger: LayoutArranger::new(),
		};
		new_self.default_message_handlers();
		new_self
	}

	pub(in crate::gui) const fn hwnd(&self) -> &HWND {
		&self.hwnd
	}

	pub(in crate::gui) fn set_hwnd(&mut self, hwnd: HWND) {
		self.hwnd = hwnd
	}

	pub(in crate::gui) const fn is_dialog(&self) -> bool {
		self.is_dialog
	}

	pub(in crate::gui) const fn parent(&self) -> Option<&Base> {
		match self.parent_ptr {
			Some(parent_ptr) => Some(unsafe { parent_ptr.as_ref() }),
			None => None,
		}
	}

	pub(in crate::gui) fn parent_hinstance(&self) -> SysResult<HINSTANCE> {
		match self.parent() {
			Some(parent) => Ok(parent.hwnd().hinstance()),
			None => HINSTANCE::GetModuleHandle(None),
		}
	}

	/// User events can be overriden; only the last one is executed.
	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		if self.hwnd != HWND::NULL {
			panic!("Cannot add event after window creation.");
		}
		&self.user_events
	}

	/// If the user added a closure to the given message, run it.
	pub(in crate::gui) fn process_user_message(&self,
		wm_any: WndMsg,
	) -> AnyResult<ProcessResult>
	{
		self.user_events.process_one_message(wm_any)
	}

	/// Internal events are always executed.
	pub(in crate::gui) fn privileged_on(&self) -> &WindowEventsPriv {
		if self.hwnd != HWND::NULL {
			panic!("Cannot add privileged event after window creation.");
		}
		&self.privileged_events
	}

	/// Internal post-user events are always executed.
	pub(in crate::gui) fn privileged_post_on(&self) -> &WindowEventsPriv {
		if self.hwnd != HWND::NULL {
			panic!("Cannot add post-user privileged event after window creation.");
		}
		&self.privileged_events_post
	}

	/// Processes all messages added internally by the library.
	///
	/// Returns `true` if at least one message was processed.
	pub(in crate::gui) fn process_privileged_messages(&self,
		hwnd: &HWND,
		wm_any: WndMsg,
	) -> AnyResult<bool>
	{
		self.privileged_events.process_all_messages(hwnd, wm_any)
	}

	/// Processes all post-user messages added internally by the library.
	///
	/// Returns `true` if at least one message was processed.
	pub(in crate::gui) fn process_privileged_post_messages(&self,
		hwnd: &HWND,
		wm_any: WndMsg,
	) -> AnyResult<bool>
	{
		self.privileged_events_post.process_all_messages(hwnd, wm_any)
	}

	/// Removes all user and privileged events.
	pub(in crate::gui) fn clear_events(&self) {
		self.privileged_events.clear_events();
		self.user_events.clear_events();
		self.privileged_events_post.clear_events();
	}

	pub(in crate::gui) fn add_to_layout_arranger(&self,
		hchild: &HWND,
		resize_behavior: (Horz, Vert),
	) -> SysResult<()>
	{
		self.layout_arranger.add_child(&self.hwnd, hchild, resize_behavior)
	}

	pub(in crate::gui) fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static,
	{
		let hwnd = unsafe { self.hwnd.raw_copy() };
		std::thread::spawn(move || {
			func().unwrap_or_else(|err| {
				// If the user func returned an error, create another function
				// which just returns it, then forward it to WM_UI_THREAD.
				let pack = Box::new(ThreadPack { func: Box::new(|| Err(err)) });
				let ptr_pack = Box::into_raw(pack);
				hwnd.GetAncestor(co::GA::ROOTOWNER)
					.map(|hwnd| unsafe {
						hwnd.SendMessage(WndMsg {
							msg_id: Self::WM_UI_THREAD,
							wparam: Self::WM_UI_THREAD.raw() as _,
							lparam: ptr_pack as _, // send pointer
						});
					});
			});
		});
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> AnyResult<()> + Send + 'static,
	{
		// This method is analog to SendMessage (synchronous), but intended to
		// be called from another thread, so a callback function can, tunelled
		// by wndproc, run in the original thread of the window, thus allowing
		// GUI updates. With this, the user doesn't have to deal with a custom
		// WM_ message.

		// https://users.rust-lang.org/t/sending-a-boxed-trait-over-ffi/21708/2
		let pack = Box::new(ThreadPack { func: Box::new(func) });
		let ptr_pack = Box::into_raw(pack);

		// Bypass any modals and send straight to main window. This avoids any
		// blind spots of unhandled messages by a modal being created/destroyed.
		self.hwnd.GetAncestor(co::GA::ROOTOWNER)
			.map(|hwnd| unsafe {
				hwnd.SendMessage(WndMsg {
					msg_id: Self::WM_UI_THREAD,
					wparam: Self::WM_UI_THREAD.raw() as _,
					lparam: ptr_pack as _, // send pointer
				});
			});
	}

	fn default_message_handlers(&self) {
		// We cant pass a pointer to Self because at this moment the parent
		// struct isn't created and pinned yet, so we make LayoutArranger
		// clonable.
		let layout_arranger = self.layout_arranger.clone();
		self.privileged_events.wm_create_or_initdialog(move |hwnd, _| {
			layout_arranger.save_original_client_area(hwnd)?; // must be done before the first WM_SIZE
			Ok(())
		});

		let layout_arranger = self.layout_arranger.clone();
		self.privileged_events.wm(co::WM::SIZE, move |_, p| {
			layout_arranger.rearrange(wm::Size::from_generic_wm(p))?;
			Ok(())
		});

		self.privileged_events.wm(Self::WM_UI_THREAD, |_, p| {
			if unsafe { co::WM::from_raw(p.wparam as _) } == Self::WM_UI_THREAD { // additional safety check
				let ptr_pack = p.lparam as *mut ThreadPack; // retrieve pointer
				let pack = unsafe { Box::from_raw(ptr_pack) };
				let func = pack.func;
				func().unwrap_or_else(|err| post_quit_error(p, err));
			}
			Ok(())
		});
	}

	pub(in crate::gui) fn run_main_loop(
		haccel: Option<&HACCEL>,
	) -> AnyResult<i32>
	{
		let mut msg = MSG::default();

		loop {
			if !GetMessage(&mut msg, None, 0, 0)? {
				// WM_QUIT was sent, gracefully terminate the program.
				// wParam has the program exit code.
				// https://learn.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues
				// PostQuitMessage() may have been called internally, so check QUIT_ERROR.
				return match unsafe { QUIT_ERROR.take() } {
					Some(msg_err) => Err(msg_err.into()), // MsgError wrapped into AnyResult
					None => Ok(msg.wParam as _), // successfull exit with ret code
				};
			}

			// If a child window, will retrieve its top-level parent.
			// If a top-level, use itself.
			let hwnd_top_level = msg.hwnd.GetAncestor(co::GA::ROOT)
					.unwrap_or(unsafe { msg.hwnd.raw_copy() });

			// If we have an accelerator table, try to translate the message.
			if let Some(haccel) = haccel {
				if hwnd_top_level.TranslateAccelerator(haccel, &mut msg).is_ok() {
					continue; // message translated
				}
			}

			// Try to process keyboard actions for child controls.
			if hwnd_top_level.IsDialogMessage(&mut msg) {
				continue;
			}

			TranslateMessage(&msg);
			unsafe { DispatchMessage(&msg); }
		}
	}
}
