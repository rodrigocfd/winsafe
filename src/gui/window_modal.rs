use std::sync::Arc;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::{IdIdcStr, IdMenu};
use crate::funcs::{AdjustWindowRectEx, DispatchMessage, GetMessage, PostQuitMessage, TranslateMessage};
use crate::gui::events::MsgEvents;
use crate::gui::immut::Immut;
use crate::gui::privs::multiply_dpi;
use crate::gui::traits::Parent;
use crate::gui::window_base::WindowBase;
use crate::handles::{HBRUSH, HCURSOR, HICON, HINSTANCE, HWND};
use crate::structs::{MSG, POINT, RECT, SIZE, WNDCLASSEX};
use crate::WString;

#[derive(Clone)]
pub struct WindowModal(Arc<Immut<Obj>>);

struct Obj { // actual fields of WindowModal
	base: WindowBase,
	opts: CustomModalOpts,
	hchild_prev_focus_parent: Option<HWND>,
}

impl Parent for WindowModal {
	fn hwnd_ref(&self) -> &HWND {
		self.0.base.hwnd_ref()
	}

	fn user_events_ref(&self) -> &MsgEvents {
		self.0.base.user_events_ref()
	}

	fn privileged_events_ref(&self) -> &MsgEvents {
		self.0.base.privileged_events_ref()
	}
}

impl WindowModal {
	pub fn new(parent: &dyn Parent, opts: CustomModalOpts) -> WindowModal {
		let wnd = Self(
			Arc::new(Immut::new(
				Obj {
					base: WindowBase::new(Some(parent)),
					opts,
					hchild_prev_focus_parent: None,
				},
			)),
		);
		wnd.default_message_handlers();
		wnd
	}

	pub fn show_modal(&self) -> WinResult<i32> {
		let hparent = self.0.base.parent_hwnd().unwrap();
		let opts = &self.0.opts;

		let mut wcx = WNDCLASSEX::default();
		let mut class_name_buf = WString::new();
		opts.generate_wndclassex(
			self.0.base.parent_hinstance()?, &mut wcx, &mut class_name_buf)?;
		self.0.base.register_class(&mut wcx)?;

		self.0.as_mut().hchild_prev_focus_parent = HWND::GetFocus();
		hparent.EnableWindow(false); // https://devblogs.microsoft.com/oldnewthing/20040227-00/?p=40463

		let mut wnd_sz = opts.size;
		multiply_dpi(None, Some(&mut wnd_sz))?;

		let mut wnd_rc = RECT { // client area, will be adjusted to size with title bar and borders
			left: 0,
			top: 0,
			right: wnd_sz.cx,
			bottom: wnd_sz.cy,
		};
		AdjustWindowRectEx(&mut wnd_rc, opts.style, false, opts.ex_style)?;
		wnd_sz.cx = wnd_rc.right - wnd_rc.left;
		wnd_sz.cy = wnd_rc.bottom - wnd_rc.top;

		let rc_parent = hparent.GetWindowRect()?; // relative to screen
		let wnd_pos = POINT {
			x: rc_parent.left + (rc_parent.right - rc_parent.left) / 2 - wnd_sz.cx / 2, // center on parent
			y: rc_parent.top + (rc_parent.bottom - rc_parent.top) / 2 - wnd_sz.cy / 2
		};

		self.0.base.create_window( // may panic
			&class_name_buf.to_string(),
			Some(&opts.title),
			IdMenu::None,
			wnd_pos, wnd_sz,
			opts.ex_style, opts.style,
		)?;

		self.run_modal_loop()
	}

	fn run_modal_loop(&self) -> WinResult<i32> {
		loop {
			let mut msg = MSG::default();
			if !GetMessage(&mut msg, None, 0, 0)? {
				// WM_QUIT was sent, exit modal loop now and signal parent.
				// wParam has the program exit code.
				// https://devblogs.microsoft.com/oldnewthing/20050222-00/?p=36393
				PostQuitMessage(co::ERROR::from(msg.wParam as u32));
				return Ok(msg.wParam as i32);
			}

			// If a child window, will retrieve its top-level parent.
			// If a top-level, use itself.
			let hwnd_top_level = msg.hwnd.GetAncestor(co::GA::ROOT)
				.unwrap_or(msg.hwnd);

			// Try to process keyboard actions for child controls.
			if hwnd_top_level.IsDialogMessage(&mut msg) {
				// Processed all keyboard actions for child controls.
				if self.hwnd_ref().is_null() {
					return Ok(0); // our modal was destroyed, terminate loop
				} else {
					continue;
				}
			}

			TranslateMessage(&msg);
			DispatchMessage(&msg);

			if self.hwnd_ref().is_null() {
				return Ok(0); // our modal was destroyed, terminate loop
			}
		}
	}

	fn default_message_handlers(&self) {
		self.user_events_ref().wm_set_focus({
			let self2 = self.clone();
			move |_| {
				if let Some(hfocus) = HWND::GetFocus() {
					if hfocus == *self2.hwnd_ref() {
						self2.0.base.focus_first_child(); // if window receives focus, delegate to first child
					}
				}
			}
		});

		self.user_events_ref().wm_close({
			let self2 = self.clone();
			move || {
				if let Ok(hparent) = self2.hwnd_ref().GetWindow(co::GW::OWNER) {
					hparent.EnableWindow(true); // re-enable parent
					self2.hwnd_ref().DestroyWindow(); // then destroy modal
					if let Some(hprev) = self2.0.hchild_prev_focus_parent {
						hprev.SetFocus(); // this focus could be set on WM_DESTROY as well
					}
				}
			}
		});
	}
}

//------------------------------------------------------------------------------

/// Options to create a [`CustomModal`](crate::gui::CustomModal) programatically
/// with [`CustomModal::new`](crate::gui::CustomModal::new).
pub struct CustomModalOpts {
	/// Window class name to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to an auto-generated string.
	pub class_name: String,
	/// Window class styles to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::CS::DBLCLKS`.
	pub class_style: co::CS,
	/// Window main icon to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to none.
	pub class_icon: HICON,
	/// Window cursor to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::IDC::ARROW`.
	pub class_cursor: HCURSOR,
	/// Window background brush to be
	/// [registered](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw).
	///
	/// Defaults to `co::COLOR::BTNFACE`.
	pub class_bg_brush: HBRUSH,

	/// Window title to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to empty string.
	pub title: String,
	/// Size of window client area, in pixels, to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	/// Does not include title bar or borders.
	///
	/// Will be adjusted to match current system DPI.
	///
	/// Defaults to 500 x 400.
	pub size: SIZE,
	/// Window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS::CAPTION | WS::SYSMENU | WS::CLIPCHILDREN | WS::BORDER | WS::VISIBLE`.
	///
	/// Suggestions:
	/// * `WS::SIZEBOX` to make the window resizable;
	/// * `WS::MAXIMIZEBOX` to have a maximize button.
	pub style: co::WS,
	/// Extended window styles to be
	/// [created](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw).
	///
	/// Defaults to `WS_EX::LEFT | WS_EX::DLGMODALFRAME`.
	pub ex_style: co::WS_EX,
}

impl Default for CustomModalOpts {
	fn default() -> Self {
		Self {
			class_name: "".to_owned(),
			class_style: co::CS::DBLCLKS,
			class_icon: unsafe { HICON::null_handle() },
			class_cursor: unsafe { HCURSOR::null_handle() },
			class_bg_brush: HBRUSH::from_sys_color(co::COLOR::BTNFACE),
			title: "".to_owned(),
			size: SIZE { cx: 500, cy: 400 },
			style: co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER | co::WS::VISIBLE,
			ex_style: co::WS_EX::LEFT | co::WS_EX::DLGMODALFRAME,
		}
	}
}

impl CustomModalOpts {
	fn generate_wndclassex<'a, 'b>( // https://stackoverflow.com/q/65481548/6923555
		&self,
		hinst: HINSTANCE,
		wcx: &mut WNDCLASSEX<'_, 'a>,
		class_name_buf: &'a mut WString) -> WinResult<()>
	{
		wcx.hInstance = hinst;
		wcx.style = self.class_style;
		wcx.hIcon = self.class_icon;
		wcx.hIconSm = self.class_icon;
		wcx.hbrBackground = self.class_bg_brush;

		wcx.hCursor = match self.class_cursor.as_opt() {
			Some(h) => h,
			None => HINSTANCE::oem().LoadCursor(IdIdcStr::Idc(co::IDC::ARROW))?,
		};

		if wcx.lpszClassName().is_empty() {
			*class_name_buf = WindowBase::generate_wcx_class_name_hash(&wcx);
			wcx.set_lpszClassName(class_name_buf);
		}

		Ok(())
	}
}
