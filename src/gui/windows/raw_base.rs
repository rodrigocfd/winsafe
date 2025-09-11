use crate::co;
use crate::decl::*;
use crate::gui::{privs::*, *};
use crate::msg::*;
use crate::prelude::*;

/// Base to all ordinary windows.
///
/// Owns the window procedure for all ordinary windows.
pub(in crate::gui) struct RawBase {
	base: BaseWnd,
}

impl Drop for RawBase {
	fn drop(&mut self) {
		if *self.base.hwnd() != HWND::NULL {
			unsafe {
				self.base.hwnd().SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
			}
		}
	}
}

impl RawBase {
	#[must_use]
	pub(in crate::gui) fn new() -> Self {
		Self { base: BaseWnd::new(WndTy::Raw) }
	}

	#[must_use]
	pub(in crate::gui) const fn base(&self) -> &BaseWnd {
		&self.base
	}

	pub(in crate::gui) fn register_class(
		&self,
		hinst: &HINSTANCE,
		class_name: &str,
		class_style: co::CS,
		class_icon: &Icon,
		class_bg_brush: &Brush,
		class_cursor: &Cursor,
	) -> ATOM {
		let mut wcx = WNDCLASSEX::default();
		wcx.lpfnWndProc = Some(Self::wnd_proc);
		wcx.hInstance = unsafe { hinst.raw_copy() };
		wcx.style = class_style;
		wcx.hIcon = class_icon.as_hicon(hinst).expect(DONTFAIL);
		wcx.hIconSm = class_icon.as_hicon(hinst).expect(DONTFAIL);
		wcx.hbrBackground = class_bg_brush.as_hbrush();
		wcx.hCursor = class_cursor.as_hcursor(hinst).expect(DONTFAIL);

		let mut wclass_name = if class_name.trim().is_empty() {
			WString::from_str(&format!(
				"WNDCLASS.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}.{:#x}",
				wcx.style,
				wcx.lpfnWndProc.map_or(0, |p| p as usize),
				wcx.cbClsExtra,
				wcx.cbWndExtra,
				wcx.hInstance,
				wcx.hIcon,
				wcx.hCursor,
				wcx.hbrBackground,
				wcx.lpszMenuName(),
				wcx.hIconSm,
			))
		} else {
			WString::from_str(class_name)
		};
		wcx.set_lpszClassName(Some(&mut wclass_name));

		SetLastError(co::ERROR::SUCCESS);
		match unsafe { RegisterClassEx(&wcx) } {
			Ok(atom) => atom,
			Err(err) => match err {
				co::ERROR::CLASS_ALREADY_EXISTS => {
					// https://devblogs.microsoft.com/oldnewthing/20150429-00/?p=44984
					// https://devblogs.microsoft.com/oldnewthing/20041011-00/?p=37603
					// Retrieve ATOM of existing window class.
					let hinst = unsafe { wcx.hInstance.raw_copy() };
					let (atom, _) = hinst
						.GetClassInfoEx(&wcx.lpszClassName().unwrap())
						.expect(DONTFAIL);
					atom
				},
				err => panic!("ERROR: RawBase::register_class: {}", err.to_string()),
			},
		}
	}

	pub(in crate::gui) fn create_window(
		&self,
		ex_style: co::WS_EX,
		class_name: ATOM,
		title: Option<&str>,
		style: co::WS,
		pos: POINT,
		size: SIZE,
		hparent: Option<&HWND>,
		hmenu: IdMenu,
		hinst: &HINSTANCE,
	) {
		if *self.base.hwnd() != HWND::NULL {
			panic!("Cannot create window twice.");
		}

		unsafe {
			// The hwnd member is saved in WM_INITDIALOG processing in wnd_proc.
			HWND::CreateWindowEx(
				ex_style,
				AtomStr::Atom(class_name),
				title,
				style,
				pos,
				size,
				hparent,
				hmenu,
				hinst,
				Some(self as *const _ as _), // pass pointer to object itself
			)
		}
		.expect(DONTFAIL);
	}

	pub(in crate::gui) fn delegate_focus_to_first_child(&self) {
		if let Some(hwnd_cur_focus) = HWND::GetFocus() {
			if *self.base.hwnd() == hwnd_cur_focus {
				// https://stackoverflow.com/a/2835220/6923555
				if let Ok(hchild_first) = self.base.hwnd().GetWindow(co::GW::CHILD) {
					hchild_first.SetFocus(); // if window receives focus, delegate to first child
				}
			}
		}
	}

	extern "system" fn wnd_proc(hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize {
		let wm_any = WndMsg::new(msg, wparam, lparam);
		Self::wnd_proc_proc(hwnd, wm_any).unwrap_or_else(|err| {
			quit_error::post_quit_error(wm_any, err);
			0
		})
	}

	fn wnd_proc_proc(hwnd: HWND, p: WndMsg) -> AnyResult<isize> {
		let ptr_self = match p.msg_id {
			co::WM::NCCREATE => {
				// first message being handled
				let msg = unsafe { wm::NcCreate::from_generic_wm(p) };
				let ptr_self = msg.createstruct.lpCreateParams as *const Self;
				unsafe {
					hwnd.SetWindowLongPtr(co::GWLP::USERDATA, ptr_self as _); // store
				}
				let ref_self = unsafe { &*ptr_self };
				ref_self.base.set_hwnd(unsafe { hwnd.raw_copy() }); // store HWND in struct field
				ptr_self
			},
			_ => hwnd.GetWindowLongPtr(co::GWLP::USERDATA) as *const Self, // retrieve
		};

		// If no pointer stored, then no processing is done.
		// Prevents processing before WM_NCCREATE and after WM_NCDESTROY.
		if ptr_self.is_null() {
			return Ok(unsafe { hwnd.DefWindowProc(p) });
		}
		let ref_self = unsafe { &*ptr_self };

		// Execute before-user closures, keep track if at least one was executed.
		let at_least_one_before = ref_self.base.process_before_messages(p)?;

		// Execute user closure, if any.
		let user_ret = ref_self.base.process_user_message(p).transpose()?;

		// Execute post-user closures, keep track if at least one was executed.
		let at_least_one_after = ref_self.base.process_after_messages(p)?;

		// Always check.
		if p.msg_id == co::WM::NCDESTROY {
			unsafe {
				hwnd.SetWindowLongPtr(co::GWLP::USERDATA, 0); // clear passed pointer
			}
			ref_self.base.set_hwnd(HWND::NULL); // clear stored HWND
			ref_self.base.clear_messages(); // prevents circular references
		}

		if let Some(user_ret) = user_ret {
			Ok(user_ret)
		} else if at_least_one_before || at_least_one_after {
			Ok(0)
		} else {
			Ok(unsafe { hwnd.DefWindowProc(p) })
		}
	}
}
