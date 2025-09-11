use std::cell::UnsafeCell;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::msg::*;
use crate::prelude::*;

/// Base to all native child controls.
///
/// Owns the window procedure for all subclassed native child controls.
pub(in crate::gui) struct BaseCtrl {
	ctrl_id: u16,
	hwnd: UnsafeCell<HWND>,
	subclass_events: BaseWndEvents,
}

static mut BASE_SUBCLASS_ID: usize = 0; // incremented each time a new subclass is installed

impl BaseCtrl {
	#[must_use]
	pub(in crate::gui) fn new(ctrl_id: u16) -> Self {
		Self {
			ctrl_id,
			hwnd: UnsafeCell::new(HWND::NULL),
			subclass_events: BaseWndEvents::new(WndTy::Raw),
		}
	}

	#[must_use]
	pub(in crate::gui) const fn ctrl_id(&self) -> u16 {
		self.ctrl_id
	}

	#[must_use]
	pub(in crate::gui) fn hwnd(&self) -> &HWND {
		unsafe { &*self.hwnd.get() }
	}

	#[must_use]
	pub(in crate::gui) fn on_subclass(&self) -> &BaseWndEvents {
		if *self.hwnd() != HWND::NULL {
			panic!("Cannot add subclass events after control creation.");
		}
		&self.subclass_events
	}

	pub(in crate::gui) fn set_hwnd(&self, hctrl: HWND) {
		*unsafe { &mut *self.hwnd.get() } = hctrl;
	}

	pub(in crate::gui) fn create_window(
		&self,
		ex_style: co::WS_EX,
		class_name: &str,
		title: Option<&str>,
		style: co::WS,
		pos: POINT,
		size: SIZE,
		parent: &impl GuiParent,
	) {
		let hparent = parent.as_ref().hwnd();

		if *self.hwnd() != HWND::NULL {
			panic!("Cannot create control twice.");
		} else if *hparent == HWND::NULL {
			panic!("Cannot create control before parent window creation.");
		}

		self.set_hwnd(unsafe {
			HWND::CreateWindowEx(
				ex_style,
				AtomStr::from_str(class_name),
				title,
				style,
				pos,
				size,
				Some(hparent),
				IdMenu::Id(self.ctrl_id),
				&hparent.hinstance(),
				None,
			)
			.expect(DONTFAIL)
		});
		self.install_subclass_if_needed();
	}

	pub(in crate::gui) fn assign_dlg(&self, parent: &impl AsRef<BaseWnd>) {
		let hparent = parent.as_ref().hwnd();

		if !hparent.is_dialog() {
			panic!("Parent window is not a dialog, cannot instantiate control.");
		} else if *self.hwnd() != HWND::NULL {
			panic!("Control HWND is already assigned, cannot create it twice.");
		} else if *hparent == HWND::NULL {
			panic!("Cannot create control before parent window creation.");
		}

		self.set_hwnd(hparent.GetDlgItem(self.ctrl_id).expect(DONTFAIL));
		self.install_subclass_if_needed();
	}

	fn install_subclass_if_needed(&self) {
		if self.subclass_events.has_message() {
			let subclass_id = unsafe {
				BASE_SUBCLASS_ID += 1;
				BASE_SUBCLASS_ID
			};

			unsafe {
				self.hwnd().SetWindowSubclass(
					Self::subclass_proc,
					subclass_id,
					self as *const _ as _, // pass pointer to self
				)
			}
			.expect(DONTFAIL);
		}
	}

	extern "system" fn subclass_proc(
		hwnd: HWND,
		msg: co::WM,
		wparam: usize,
		lparam: isize,
		subclass_id: usize,
		ref_data: usize,
	) -> isize {
		let wm_any = WndMsg::new(msg, wparam, lparam);
		Self::subclass_proc_proc(hwnd, wm_any, subclass_id, ref_data).unwrap_or_else(|err| {
			quit_error::post_quit_error(wm_any, err);
			0
		})
	}

	fn subclass_proc_proc(
		hwnd: HWND,
		p: WndMsg,
		subclass_id: usize,
		ref_data: usize,
	) -> AnyResult<isize> {
		let ptr_self = ref_data as *const Self; // retrieve
		let mut user_ret = Option::<isize>::None;

		if !ptr_self.is_null() {
			let ref_self = unsafe { &*ptr_self };
			if *ref_self.hwnd() != HWND::NULL {
				user_ret = ref_self
					.subclass_events
					.process_last_message(p)
					.transpose()?;
			}
		}

		if p.msg_id == co::WM::NCDESTROY {
			// Always check.
			// https://devblogs.microsoft.com/oldnewthing/20031111-00/?p=41883
			hwnd.RemoveWindowSubclass(Self::subclass_proc, subclass_id)?;
			if !ptr_self.is_null() {
				let ref_self = unsafe { &*ptr_self };
				ref_self.subclass_events.clear(); // prevents circular references
			}
		}

		Ok(match user_ret {
			Some(user_ret) => user_ret,
			None => unsafe { hwnd.DefSubclassProc(p) },
		})
	}
}
