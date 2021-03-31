use crate::aliases::WinResult;
use crate::co;
use crate::enums::IdStr;
use crate::funcs::PostQuitMessage;
use crate::gui::base::Base;
use crate::gui::events::ProcessResult;
use crate::gui::privs::ui_font;
use crate::handles::{HFONT, HWND};
use crate::msg::{MsgSendRecv, wm, WndMsg};

/// Base to all dialog windows.
pub(crate) struct DlgBase {
	base: Base,
	dialog_id: i32,
}

impl Drop for DlgBase {
	fn drop(&mut self) {
		if !self.base.hwnd_ref().is_null() {
			self.base.hwnd_ref().SetWindowLongPtr(co::GWLP::DWLP_USER, 0); // clear passed pointer
		}
	}
}

impl DlgBase {
	pub fn new(parent_ref: Option<&Base>, dialog_id: i32) -> DlgBase {
		Self {
			base: Base::new(parent_ref, true),
			dialog_id,
		}
	}

	pub fn base_ref(&self) -> &Base {
		&self.base
	}

	pub fn create_dialog_param(&self) -> WinResult<()> {
		if !self.base.hwnd_ref().is_null() {
			panic!("Cannot create dialog twice.");
		}

		// Our hwnd member is set during WM_INITDIALOG processing, already set
		// when CreateDialogParam returns.
		self.base.parent_hinstance()?.CreateDialogParam(
			IdStr::Id(self.dialog_id),
			self.base.parent_ref().map(|parent| *parent.hwnd_ref()),
			Self::dialog_proc,
			Some(self as *const Self as isize), // pass pointer to self
		).map(|_| ())
	}

	pub fn dialog_box_param(&self) -> WinResult<i32> {
		if !self.base.hwnd_ref().is_null() {
			panic!("Cannot create dialog twice.");
		}

		// Our hwnd member is set during WM_INITDIALOG processing, already set
		// when DialogBoxParam returns.
		self.base.parent_hinstance()?.DialogBoxParam(
			IdStr::Id(self.dialog_id),
			self.base.parent_ref().map(|parent| *parent.hwnd_ref()),
			Self::dialog_proc,
			Some(self as *const Self as isize), // pass pointer to self
		).map(|res| res as i32)
	}

	extern "system" fn dialog_proc(
		hwnd: HWND, msg: co::WM, wparam: usize, lparam: isize) -> isize
	{
		|hwnd: HWND, msg, wparam, lparam| -> WinResult<isize>
		{
			let wm_any = WndMsg { msg_id: msg, wparam, lparam };

			let ptr_self = match msg {
				co::WM::INITDIALOG => { // first message being handled
					let wm_idlg = wm::InitDialog::from_generic_wm(wm_any);
					let ptr_self = wm_idlg.additional_data as *mut Self;
					hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, ptr_self as isize); // store
					let ref_self = unsafe { &mut *ptr_self };
					ref_self.base.set_hwnd(hwnd); // store HWND in struct field
					ptr_self
				},
				_ => hwnd.GetWindowLongPtr(co::GWLP::DWLP_USER) as *mut Self, // retrieve
			};

			// If no pointer stored, then no processing is done.
			// Prevents processing before WM_INITDIALOG and after WM_NCDESTROY.
			if ptr_self.is_null() {
				return Ok(hwnd.DefWindowProc(wm_any));
			}

			// Execute privileged closures.
			let ref_self = unsafe { &mut *ptr_self };
			ref_self.base.process_privileged_messages(wm_any);

			if wm_any.msg_id == co::WM::INITDIALOG {
				// Child controls are created in privileged closures, so we set the
				// system font only now.
				ref_self.set_ui_font_on_children();
			}

			// Execute user closure, if any.
			let maybe_processed = ref_self.base.process_effective_message(wm_any);

			if msg == co::WM::NCDESTROY { // always check
				hwnd.SetWindowLongPtr(co::GWLP::DWLP_USER, 0); // clear passed pointer
				ref_self.base.set_hwnd(unsafe { HWND::null_handle() }); // clear stored HWND
			}

			Ok(match maybe_processed {
				ProcessResult::HandledWithRet(res) => res.into(),
				ProcessResult::HandledWithoutRet => true as isize,
				ProcessResult::NotHandled => false as isize,
			})
		}
		(hwnd, msg, wparam, lparam)
			.unwrap_or_else(|err| { PostQuitMessage(err); true as isize })
	}

	fn set_ui_font_on_children(&self) {
		self.base.hwnd_ref().SendMessage(wm::SetFont { hfont: ui_font(), redraw: false });
		self.base.hwnd_ref().EnumChildWindows(Self::enum_proc, ui_font().ptr as isize);
	}
	extern "system" fn enum_proc(hchild: HWND, lparam: isize) -> i32 {
		let hfont = HFONT { ptr: lparam as *mut _ };
		hchild.SendMessage(wm::SetFont { hfont, redraw: false });
		true as i32
	}
}
