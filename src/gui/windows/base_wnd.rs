use crate::co;
use crate::decl::*;
use crate::gui::{events::*, privs::*, *};
use crate::msg::*;
use crate::prelude::*;

/// Allocated on the heap and passed through `WM_UI_THREAD`.
struct ThreadPack {
	func: Box<dyn FnOnce() -> AnyResult<()>>,
}

/// Base to `RawBase` and `DlgBase`, which means all container windows.
///
/// Stores the message closures which are added internally and by the user.
/// These closures are called by wndproc and dlgproc.
pub(in crate::gui) struct BaseWnd {
	hwnd: HWND,
	wnd_ty: WndTy,
	layout: Layout,
	before_events: WindowEventsAll,
	user_events: WindowEventsAll,
	after_events: WindowEventsAll,
}

impl BaseWnd {
	const WM_UI_THREAD: co::WM = unsafe { co::WM::from_raw(co::WM::APP.raw() + 0x3fff) };

	#[must_use]
	pub(in crate::gui) fn new(wnd_ty: WndTy) -> Self {
		let new_self = Self {
			hwnd: HWND::NULL,
			wnd_ty,
			layout: Layout::new(),
			before_events: WindowEventsAll::new(wnd_ty),
			user_events: WindowEventsAll::new(wnd_ty),
			after_events: WindowEventsAll::new(wnd_ty),
		};
		new_self.default_message_handlers();
		new_self
	}

	#[must_use]
	pub(in crate::gui) const fn wnd_ty(&self) -> WndTy {
		self.wnd_ty
	}
	#[must_use]
	pub(in crate::gui) const fn hwnd(&self) -> &HWND {
		&self.hwnd
	}
	pub(in crate::gui) fn set_hwnd(&mut self, hwnd: HWND) {
		self.hwnd = hwnd
	}

	#[must_use]
	pub(in crate::gui) fn before_on(&self) -> &WindowEventsAll {
		&self.before_events
	}
	#[must_use]
	pub(in crate::gui) fn on(&self) -> &WindowEventsAll {
		if self.hwnd != HWND::NULL {
			panic!("Cannot add event after window creation.");
		}
		&self.user_events // user events can be overriden; only the last one is executed
	}
	#[must_use]
	pub(in crate::gui) fn after_on(&self) -> &WindowEventsAll {
		&self.after_events
	}

	pub(in crate::gui) fn process_before_messages(&self, p: WndMsg) -> AnyResult<bool> {
		self.before_events.process_all_messages(p)
	}
	pub(in crate::gui) fn process_user_message(&self, p: WndMsg) -> Option<AnyResult<isize>> {
		self.user_events.process_last_message(p)
	}
	pub(in crate::gui) fn process_after_messages(&self, p: WndMsg) -> AnyResult<bool> {
		self.after_events.process_all_messages(p)
	}

	pub(in crate::gui) fn clear_messages(&self) {
		self.before_events.clear();
		self.user_events.clear();
		self.after_events.clear();
	}

	pub(in crate::gui) fn add_to_layout(&self, hchild: &HWND, resize_behavior: (Horz, Vert)) {
		self.layout.add_child(&self.hwnd, hchild, resize_behavior);
	}

	pub(in crate::gui) fn spawn_thread<F>(&self, func: F)
	where
		F: FnOnce() -> AnyResult<()> + Send + 'static,
	{
		let hwnd = unsafe { self.hwnd.raw_copy() };
		std::thread::spawn(move || {
			func().unwrap_or_else(|err| {
				// If the user func returned an error, create another function
				// which just returns it, then forward it to WM_UI_THREAD.
				let pack = Box::new(ThreadPack { func: Box::new(|| Err(err)) });
				let ptr_pack = Box::into_raw(pack);
				hwnd.GetAncestor(co::GA::ROOTOWNER).map(|hwnd| unsafe {
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
	where
		F: FnOnce() -> AnyResult<()> + Send + 'static,
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
		self.hwnd.GetAncestor(co::GA::ROOTOWNER).map(|hwnd| unsafe {
			hwnd.SendMessage(WndMsg {
				msg_id: Self::WM_UI_THREAD,
				wparam: Self::WM_UI_THREAD.raw() as _,
				lparam: ptr_pack as _, // send pointer
			});
		});
	}

	pub(in crate::gui) fn default_message_handlers(&self) {
		let layout = self.layout.clone();
		self.before_events.wm_size(move |p| {
			layout.rearrange(p);
			Ok(())
		});

		self.before_events.wm(Self::WM_UI_THREAD, |p| {
			// WPARAM is just an additional safety check.
			if unsafe { co::WM::from_raw(p.wparam as _) } == Self::WM_UI_THREAD {
				let ptr_pack = p.lparam as *mut ThreadPack; // retrieve pointer
				let pack = unsafe { Box::from_raw(ptr_pack) };
				let func = pack.func;
				func().unwrap_or_else(|err| quit_error::post_quit_error(p, err));
			}
			Ok(0) // ignored
		});
	}

	pub(in crate::gui) fn run_main_loop(
		haccel: Option<&HACCEL>,
		process_dlg_msgs: bool,
	) -> AnyResult<i32> {
		let mut msg = MSG::default();

		loop {
			// GetMessage only fails if hWnd is invalid, what should not happen.
			if !GetMessage(&mut msg, None, 0, 0).expect(DONTFAIL) {
				// WM_QUIT was sent, gracefully terminate the program.
				// wParam has the program exit code.
				// https://learn.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues
				// PostQuitMessage() may have been called internally, so check QUIT_ERROR.
				return match {
					let mut msg_error = quit_error::QUIT_ERROR.lock().unwrap();
					msg_error.take()
				} {
					Some(msg_err) => Err(msg_err.into()), // MsgError wrapped into AnyResult
					None => Ok(msg.wParam as _),          // successfull exit with ret code
				};
			}

			// If a child window, will retrieve its top-level parent.
			// If a top-level, use itself.
			let hwnd_top_level = msg
				.hwnd
				.GetAncestor(co::GA::ROOT)
				.unwrap_or(unsafe { msg.hwnd.raw_copy() });

			// If we have an accelerator table, try to translate the message.
			if let Some(haccel) = haccel {
				if hwnd_top_level
					.TranslateAccelerator(haccel, &mut msg)
					.is_ok()
				{
					continue; // message translated
				}
			}

			// Try to process keyboard actions for child controls.
			if process_dlg_msgs && hwnd_top_level.IsDialogMessage(&mut msg) {
				continue;
			}

			TranslateMessage(&msg);
			unsafe {
				DispatchMessage(&msg);
			}
		}
	}

	pub(in crate::gui) fn run_modal_loop(&self, process_dlg_msgs: bool) -> AnyResult<i32> {
		let mut msg = MSG::default();

		loop {
			// GetMessage only fails if hWnd is invalid, what should not happen.
			if !GetMessage(&mut msg, None, 0, 0).expect(DONTFAIL) {
				// WM_QUIT was sent, exit modal loop now and signal parent.
				// wParam has the program exit code.
				// https://devblogs.microsoft.com/oldnewthing/20050222-00/?p=36393
				// https://stackoverflow.com/a/29359913/6923555
				PostQuitMessage(msg.wParam as _);
				return Ok(0); // raw modals will always return 0
			}

			if self.hwnd == HWND::NULL || !self.hwnd.IsWindow() {
				return Ok(0); // our modal was destroyed, terminate loop
			}

			// If a child window, will retrieve its top-level parent.
			// If a top-level, use itself.
			let hwnd_top_level = msg
				.hwnd
				.GetAncestor(co::GA::ROOT)
				.unwrap_or(unsafe { msg.hwnd.raw_copy() });

			// Try to process keyboard actions for child controls.
			if process_dlg_msgs && hwnd_top_level.IsDialogMessage(&mut msg) {
				// Processed all keyboard actions for child controls.
				if self.hwnd == HWND::NULL {
					return Ok(0); // our modal was destroyed, terminate loop
				} else {
					continue;
				}
			}

			TranslateMessage(&msg);
			unsafe {
				DispatchMessage(&msg);
			}

			if self.hwnd == HWND::NULL || !self.hwnd.IsWindow() {
				return Ok(0); // our modal was destroyed, terminate loop
			}
		}
	}
}
