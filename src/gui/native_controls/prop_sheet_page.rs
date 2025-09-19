use std::any::Any;
use std::pin::Pin;
use std::sync::Arc;

use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;

struct PropSheetPageObj {
	base: BaseWnd,
	dlg_id: u16,
}

/// A page of a
/// [property sheet](https://learn.microsoft.com/en-us/windows/win32/controls/property-sheets),
/// passed to [`PropSheet`](crate::gui::PropSheet).
#[derive(Clone)]
pub struct PropSheetPage(Pin<Arc<PropSheetPageObj>>);

unsafe impl Send for PropSheetPage {}

impl AsRef<BaseWnd> for PropSheetPage {
	fn as_ref(&self) -> &BaseWnd {
		&self.0.base
	}
}

impl GuiWindow for PropSheetPage {
	fn hwnd(&self) -> &HWND {
		self.as_ref().hwnd()
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

impl GuiParent for PropSheetPage {}

impl PropSheetPage {
	/// Instantiates a new `PropSheetPage` object, to be used with a
	/// [`PropSheet`](crate::gui::PropSheet). Pages are always built upon a
	/// dialog template.
	#[must_use]
	pub fn new_dlg(dlg_id: u16) -> Self {
		let new_self = Self(Arc::pin(PropSheetPageObj { base: BaseWnd::new(WndTy::Dlg), dlg_id }));
		new_self
	}

	#[must_use]
	pub(in crate::gui) fn generate_propsheetpage(&self) -> PROPSHEETPAGE {
		if *self.0.base.hwnd() != HWND::NULL {
			panic!("Cannot create property sheet page dialog twice.");
		}

		let mut ps_page = PROPSHEETPAGE::default(); // to be passed to PropertyPage()
		ps_page.pfnDlgProc = Some(Self::dlg_proc);
		ps_page.pszTemplate_pResource = MAKEINTRESOURCE(self.0.dlg_id as _) as _;
		ps_page.lParam = self as *const _ as _; // pointer to object itself
		ps_page.dwFlags |= co::PSP::USETITLE | co::PSP::PREMATURE;
		ps_page
	}

	/// Exposes methods to handle window messages.
	///
	/// # Panics
	///
	/// Panics if the window is already created. Events must be set before
	/// window creation.
	#[must_use]
	pub fn on(&self) -> &impl GuiEventsPropSheetPage {
		self.as_ref().on()
	}

	extern "system" fn dlg_proc(hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize {
		let wm_any = WndMsg::new(msg, wparam, lparam);
		Self::dlg_proc_proc(hwnd, wm_any).unwrap_or_else(|err| {
			quit_error::post_quit_error(wm_any, err);
			true as _
		})
	}

	fn dlg_proc_proc(hwnd: HWND, p: WndMsg) -> AnyResult<isize> {
		let ptr_self = match p.msg_id {
			co::WM::INITDIALOG => {
				// First message being handled.
				let msg = unsafe { wm::InitDialog::from_generic_wm(p) };

				// For property sheet page dialogs, LPARAM contains PROPSHEETPAGE.
				let ptr_psp = msg.additional_data as *const PROPSHEETPAGE;
				let ref_psp = unsafe { &*ptr_psp };
				let ptr_self = ref_psp.lParam as *const Self;

				unsafe {
					hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, ptr_self as _); // store
				}
				let ref_self = unsafe { &*ptr_self };
				ref_self.0.base.set_hwnd(unsafe { hwnd.raw_copy() }); // store HWND in struct field
				ptr_self
			},
			_ => hwnd.GetWindowLongPtr(co::GWLP::DWLP_USER) as *const Self, // retrieve
		};

		if ptr_self.is_null() {
			// If no pointer stored, then no processing is done.
			// Prevents processing before WM_INITDIALOG and after WM_NCDESTROY.
			Ok(0) // FALSE
		} else {
			let ref_self = unsafe { &*ptr_self };
			DlgBase::dlg_proc_proc2(&ref_self.0.base, hwnd, p)
		}
	}
}
