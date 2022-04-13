use std::ptr::NonNull;

use crate::co;
use crate::gui::events::{ProcessResult, WindowEventsAll};
use crate::gui::layout_arranger::{Horz, LayoutArranger, Vert};
use crate::gui::privs::{post_quit_error, QUIT_ERROR};
use crate::gui::very_unsafe_cell::VeryUnsafeCell;
use crate::kernel::decl::{ErrResult, HINSTANCE};
use crate::msg::WndMsg;
use crate::prelude::{GuiEvents, GuiParent, Handle, KernelHinstance, UserHwnd};
use crate::user::decl::{
	DispatchMessage, GetMessage, HACCEL, HWND, MSG, TranslateMessage,
};

/// Base to `RawBase` and `DlgBase`, which means all container windows.
pub(in crate::gui) struct Base {
	hwnd: HWND,
	is_dialog: bool,
	parent_ptr: Option<NonNull<Self>>,
	user_events: WindowEventsAll, // ordinary window events, inserted by user: only last added is executed (overwrite previous)
	privileged_events: WindowEventsAll, // inserted internally to automate tasks: all will be executed
	layout_arranger: VeryUnsafeCell<LayoutArranger>,
}

impl Base {
	const WM_UI_THREAD: co::WM = co::WM(co::WM::APP.0 + 0x3fff);

	pub(in crate::gui) unsafe fn from_guiparent<'a>(
		p: &impl GuiParent) -> &'a Self
	{
		let ptr = NonNull::new_unchecked(p.as_base() as *mut _);
		ptr.as_ref()
	}

	pub(in crate::gui) fn new(
		is_dialog: bool,
		parent: Option<&Base>) -> Self
	{
		let new_self = Self {
			hwnd: HWND::NULL,
			is_dialog,
			parent_ptr: parent.map(|parent| NonNull::from(parent)),
			user_events: WindowEventsAll::new(),
			privileged_events: WindowEventsAll::new(),
			layout_arranger: VeryUnsafeCell::new(LayoutArranger::new()),
		};
		new_self.default_message_handlers();
		new_self
	}

	pub(in crate::gui) const fn hwnd(&self) -> HWND {
		self.hwnd
	}

	pub(in crate::gui) unsafe fn set_hwnd(&mut self, hwnd: HWND) {
		self.hwnd = hwnd
	}

	pub(in crate::gui) const fn is_dialog(&self) -> bool {
		self.is_dialog
	}

	pub(in crate::gui) const fn creation_msg(&self) -> co::WM {
		if self.is_dialog { co::WM::INITDIALOG } else { co::WM::CREATE }
	}

	pub(in crate::gui) fn parent(&self) -> Option<&Base> {
		self.parent_ptr.map(move |parent| unsafe { parent.as_ref() })
	}

	pub(in crate::gui) fn parent_hinstance(&self) -> HINSTANCE {
		self.parent().map_or_else(
			|| HINSTANCE::GetModuleHandle(None).unwrap(),
			|parent| parent.hwnd().hinstance(),
		)
	}

	/// User events can be overriden; only the last one is executed.
	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		if !self.hwnd.is_null() {
			panic!("Cannot add event after window creation.");
		}
		&self.user_events
	}

	pub(in crate::gui) fn process_user_message(&self,
		wm_any: WndMsg) -> ErrResult<ProcessResult>
	{
		self.user_events.process_one_message(wm_any)
	}

	/// Internal events are always executed.
	pub(in crate::gui) fn privileged_on(&self) -> &WindowEventsAll {
		if !self.hwnd.is_null() {
			panic!("Cannot add privileged event after window creation.");
		}
		&self.privileged_events
	}

	pub(in crate::gui) fn process_privileged_messages(&self,
		wm_any: WndMsg) -> ErrResult<()>
	{
		self.privileged_events.process_all_messages(wm_any)
	}

	pub(in crate::gui) fn add_to_layout_arranger(&self,
		hchild: HWND, horz: Horz, vert: Vert)
	{
		self.layout_arranger.as_mut().add(self.hwnd, hchild, horz, vert)
	}

	pub(in crate::gui) fn spawn_new_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static,
	{
		let hwnd = self.hwnd;
		std::thread::spawn(move || {
			func().unwrap_or_else(|err| {
				let pack: Box<Box<dyn FnOnce() -> ErrResult<()>>> = Box::new(Box::new(|| Err(err)));
				let ptr_pack = Box::into_raw(pack);
				hwnd.GetAncestor(co::GA::ROOTOWNER)
					.map(|hwnd| {
						hwnd.SendMessage(WndMsg {
							msg_id: Self::WM_UI_THREAD,
							wparam: Self::WM_UI_THREAD.0 as _,
							lparam: ptr_pack as _,
						});
					});
			});
		});
	}

	pub(in crate::gui) fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()> + Send + 'static,
	{
		// This method is analog to SendMessage (synchronous), but intended to
		// be called from another thread, so a callback function can, tunelled
		// by wndproc, run in the original thread of the window, thus allowing
		// GUI updates. With this, the user doesn't have to deal with a custom
		// WM_ message.

		// https://users.rust-lang.org/t/sending-a-boxed-trait-over-ffi/21708/2
		let pack: Box<Box<dyn FnOnce() -> ErrResult<()>>> = Box::new(Box::new(func));
		let ptr_pack = Box::into_raw(pack);

		// Bypass any modals and send straight to main window. This avoids any
		// blind spots of unhandled messages by a modal being created/destroyed.
		self.hwnd.GetAncestor(co::GA::ROOTOWNER)
			.map(|hwnd| {
				hwnd.SendMessage(WndMsg {
					msg_id: Self::WM_UI_THREAD,
					wparam: Self::WM_UI_THREAD.0 as _,
					lparam: ptr_pack as _,
				});
			});
	}

	fn default_message_handlers(&self) {
		// We cant pass a pointer to Self because at this moment the parent
		// struct isn't created and pinned yet, so we make LayoutArranger
		// clonable.
		let layout_arranger = self.layout_arranger.clone();
		self.privileged_events.wm_size(move |p| {
			layout_arranger.rearrange(&p);
			Ok(()) // not meaningful
		});

		self.privileged_events.wm(Self::WM_UI_THREAD, |p| {
			if co::WM(p.wparam as _) == Self::WM_UI_THREAD { // additional safety check
				let ptr_pack = p.lparam as *mut Box<dyn FnOnce() -> ErrResult<()>>;
				let pack: Box<Box<dyn FnOnce() -> ErrResult<()>>> = unsafe { Box::from_raw(ptr_pack) };
				pack().unwrap_or_else(|err| post_quit_error(err));
			}
			Ok(None) // not meaningful
		});
	}

	pub(in crate::gui) fn run_main_loop(
		haccel: Option<HACCEL>) -> ErrResult<i32>
	{
		let mut msg = MSG::default();

		loop {
			if !GetMessage(&mut msg, None, 0, 0).unwrap() {
				// WM_QUIT was sent, gracefully terminate the program.
				// wParam has the program exit code.
				// https://docs.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues
				return match unsafe { QUIT_ERROR.take() } {
					Some(err) => Err(err),
					None => Ok(msg.wParam as _),
				};
			}

			// If a child window, will retrieve its top-level parent.
			// If a top-level, use itself.
			let hwnd_top_level = msg.hwnd.GetAncestor(co::GA::ROOT)
				.unwrap_or(msg.hwnd);

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
