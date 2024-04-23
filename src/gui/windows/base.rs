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
	before_user_events: WindowEventsPriv, // inserted internally to automate tasks: all will be executed before user events
	user_events: WindowEvents, // ordinary window events, inserted by user: only last added is executed (overwrite previous)
	after_user_events: WindowEventsPriv, // all will be executed after user events
	layout_arranger: LayoutArranger,
}

impl AsRef<Base> for Base {
	fn as_ref(&self) -> &Base {
		self
	}
}

impl Base {
	const WM_UI_THREAD: co::WM = unsafe { co::WM::from_raw(co::WM::APP.raw() + 0x3fff) };

	#[must_use]
	pub(in crate::gui) fn new(
		is_dialog: bool,
		parent: Option<&impl AsRef<Base>>,
	) -> Self
	{
		let new_self = Self {
			hwnd: HWND::NULL,
			is_dialog,
			parent_ptr: parent.map(|parent| NonNull::from(parent.as_ref())),
			before_user_events: WindowEventsPriv::new(is_dialog),
			user_events: WindowEvents::new(is_dialog),
			after_user_events: WindowEventsPriv::new(is_dialog),
			layout_arranger: LayoutArranger::new(),
		};
		new_self.default_message_handlers();
		new_self
	}

	#[must_use]
	pub(in crate::gui) const fn hwnd(&self) -> &HWND {
		&self.hwnd
	}

	pub(in crate::gui) fn set_hwnd(&mut self, hwnd: HWND) {
		self.hwnd = hwnd
	}

	#[must_use]
	pub(in crate::gui) const fn is_dialog(&self) -> bool {
		self.is_dialog
	}

	#[must_use]
	pub(in crate::gui) const fn parent(&self) -> Option<&Base> {
		match self.parent_ptr {
			Some(parent_ptr) => Some(unsafe { parent_ptr.as_ref() }),
			None => None,
		}
	}

	#[must_use]
	pub(in crate::gui) fn parent_hinstance(&self) -> SysResult<HINSTANCE> {
		match self.parent() {
			Some(parent) => Ok(parent.hwnd().hinstance()),
			None => HINSTANCE::GetModuleHandle(None),
		}
	}

	/// Internal before-user events are always executed.
	#[must_use]
	pub(in crate::gui) fn before_user_on(&self) -> &WindowEventsPriv {
		if self.hwnd != HWND::NULL {
			panic!("Cannot add before-user event after window creation.");
		}
		&self.before_user_events
	}

	/// User events can be overriden; only the last one is executed.
	#[must_use]
	pub(in crate::gui) fn on(&self) -> &WindowEvents {
		if self.hwnd != HWND::NULL {
			panic!("Cannot add event after window creation.");
		}
		&self.user_events
	}

	/// Internal after-user events are always executed.
	#[must_use]
	pub(in crate::gui) fn after_user_on(&self) -> &WindowEventsPriv {
		if self.hwnd != HWND::NULL {
			panic!("Cannot add after-user event after window creation.");
		}
		&self.after_user_events
	}

	/// Processes all before-user messages added internally by the library.
	///
	/// Returns `true` if at least one message was processed.
	pub(in crate::gui) fn process_before_user_messages(&self,
		wm_any: WndMsg,
	) -> AnyResult<bool>
	{
		self.before_user_events.process_all_messages(self.hwnd(), wm_any)
	}

	/// If the user added a closure to the given message, run it.
	pub(in crate::gui) fn process_user_message(&self,
		wm_any: WndMsg,
	) -> AnyResult<WmRet>
	{
		self.user_events.process_last_message(self.hwnd(), wm_any)
	}

	/// Processes all after-user messages added internally by the library.
	///
	/// Returns `true` if at least one message was processed.
	pub(in crate::gui) fn process_after_user_messages(&self,
		wm_any: WndMsg,
	) -> AnyResult<bool>
	{
		self.after_user_events.process_all_messages(self.hwnd(), wm_any)
	}

	/// Removes all user and before/after events.
	pub(in crate::gui) fn clear_events(&self) {
		self.before_user_events.clear_events();
		self.user_events.clear_events();
		self.after_user_events.clear_events();
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
		self.before_user_events.wm_create_or_initdialog(move |hwnd, _| {
			layout_arranger.save_original_client_area(hwnd)?; // must be done before the first WM_SIZE
			Ok(WmRet::HandledOk)
		});

		let layout_arranger = self.layout_arranger.clone();
		self.before_user_events.wm(co::WM::SIZE, move |_, p| {
			layout_arranger.rearrange(wm::Size::from_generic_wm(p))?;
			Ok(WmRet::HandledOk)
		});

		self.before_user_events.wm(Self::WM_UI_THREAD, |_, p| {
			if unsafe { co::WM::from_raw(p.wparam as _) } == Self::WM_UI_THREAD { // additional safety check
				let ptr_pack = p.lparam as *mut ThreadPack; // retrieve pointer
				let pack = unsafe { Box::from_raw(ptr_pack) };
				let func = pack.func;
				func().unwrap_or_else(|err| post_quit_error(p, err));
			}
			Ok(WmRet::HandledOk)
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
