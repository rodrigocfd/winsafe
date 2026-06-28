use crate::co;
use crate::decl::*;
use crate::macros::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`PSM_ADDPAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-addpage)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct PsmAddPage<'a> {
	pub hpspg: &'a HPROPSHEETPAGE,
}

impl<'a> MsgSend for PsmAddPage<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::ADDPAGE.into(),
			wparam: 0,
			lparam: self.hpspg.ptr() as _,
		}
	}
}

/// [`PSM_APPLY`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-apply)
/// message, which has no parameters.
///
/// Return type: `SysResult<()>`.
pub struct PsmApply {}

impl MsgSend for PsmApply {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::APPLY.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { PsmCancelToClose: co::PSM::CANCELTOCLOSE.into();
	/// [`PSM_CANCELTOCLOSE`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-canceltoclose)
}

pub_struct_msg_empty! { PsmChanged: co::PSM::CHANGED.into();
	/// [`PSM_CHANGED`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-changed)
}

/// [`PSM_ENABLEWIZBUTTONS`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-enablewizbuttons)
/// message parameters.
///
/// Return type: `()`.
pub struct PsmEnableWizButtons {
	pub btns_to_check: co::PSWIZB,
	pub btns_to_enable: co::PSWIZB,
}

impl MsgSend for PsmEnableWizButtons {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::ENABLEWIZBUTTONS.into(),
			wparam: self.btns_to_check.raw() as _,
			lparam: self.btns_to_enable.raw() as _,
		}
	}
}

/// [`PSM_GETCURRENTPAGEHWND`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-getcurrentpagehwnd)
/// message, which has no parameters.
///
/// Return type: `HWND`.
pub struct PsmGetCurrentPageHwnd {}

impl MsgSend for PsmGetCurrentPageHwnd {
	type RetType = HWND;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { HWND::from_ptr(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::GETCURRENTPAGEHWND.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PSM_GETRESULT`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-getresult)
/// message, which has no parameters.
///
/// Return type: `SysResult<co::ID_PB>`.
pub struct PsmGetResult {}

impl MsgSend for PsmGetResult {
	type RetType = SysResult<co::ID_PB>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|n| unsafe { co::ID_PB::from_raw(n as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::GETRESULT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PSM_GETTABCONTROL`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-gettabcontrol)
/// message, which has no parameters.
///
/// Return type: `SysResult<HWND>`.
pub struct PsmGetTabControl {}

impl MsgSend for PsmGetTabControl {
	type RetType = HWND;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { HWND::from_ptr(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::GETTABCONTROL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PSM_HWNDTOINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-hwndtoindex)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct PsmHwndToIndex<'a> {
	pub hwnd_page: &'a HWND,
}

impl<'a> MsgSend for PsmHwndToIndex<'a> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|n| n as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::GETTABCONTROL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PSM_IDTOINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-idtoindex)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct PsmIdToIndex {
	pub pg_res_id: u32,
}

impl MsgSend for PsmIdToIndex {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|n| n as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::IDTOINDEX.into(),
			wparam: 0,
			lparam: self.pg_res_id as _,
		}
	}
}

/// [`PSM_INDEXTOHWND`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-indextohwnd)
/// message parameters.
///
/// Return type: `SysResult<HWND>`.
pub struct PsmIndexToHwnd {
	pub pg_index: u32,
}

impl MsgSend for PsmIndexToHwnd {
	type RetType = SysResult<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::INDEXTOHWND.into(),
			wparam: self.pg_index as _,
			lparam: 0,
		}
	}
}

/// [`PSM_INDEXTOID`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-indextoid)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct PsmIndexToId {
	pub pg_index: u32,
}

impl MsgSend for PsmIndexToId {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|n| n as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::INDEXTOID.into(),
			wparam: self.pg_index as _,
			lparam: 0,
		}
	}
}

/// [`PSM_INDEXTOPAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-indextopage)
/// message parameters.
///
/// Return type: `SysResult<HPROPSHEETPAGE>`.
pub struct PsmIndexToPage {
	pub pg_index: u32,
}

impl MsgSend for PsmIndexToPage {
	type RetType = SysResult<HPROPSHEETPAGE>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HPROPSHEETPAGE::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::INDEXTOPAGE.into(),
			wparam: self.pg_index as _,
			lparam: 0,
		}
	}
}

/// [`PSM_INSERTPAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-insertpage)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct PsmInsertPage<'a> {
	pub index: u32,
	pub hpage: &'a HPROPSHEETPAGE,
}

impl<'a> MsgSend for PsmInsertPage<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::INSERTPAGE.into(),
			wparam: self.index as _,
			lparam: self.hpage.ptr() as _,
		}
	}
}

/// [`PSM_ISDIALOGMESSAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-isdialogmessage)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct PsmIsDialogMessage<'a> {
	pub msg: &'a mut MSG,
}

impl<'a> MsgSend for PsmIsDialogMessage<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::ISDIALOGMESSAGE.into(),
			wparam: 0,
			lparam: self.msg as *mut _ as _,
		}
	}
}

/// [`PSM_PAGETOINDEX`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-pagetoindex)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct PsmPageToIndex<'a> {
	pub hpspg: &'a HPROPSHEETPAGE,
}

impl<'a> MsgSend for PsmPageToIndex<'a> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|n| n as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::PAGETOINDEX.into(),
			wparam: 0,
			lparam: self.hpspg.ptr() as _,
		}
	}
}

/// [`PSM_PRESSBUTTON`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-pressbutton)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct PsmPressButton {
	pub index: co::PSBTN,
}

impl MsgSend for PsmPressButton {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::PRESSBUTTON.into(),
			wparam: self.index.raw() as _,
			lparam: 0,
		}
	}
}

/// [`PSM_QUERYSIBLINGS`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-querysiblings)
/// message parameters.
///
/// Return type: `isize`.
pub struct PsmQuerySiblings {
	pub param1: isize,
	pub param2: isize,
}

impl MsgSend for PsmQuerySiblings {
	type RetType = isize;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::QUERYSIBLINGS.into(),
			wparam: self.param1 as _,
			lparam: self.param2,
		}
	}
}

pub_struct_msg_empty! { PsmRebootSystem: co::PSM::REBOOTSYSTEM.into();
	/// [`PSM_REBOOTSYSTEM`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-rebootsystem)
}

/// [`PSM_RECALCPAGESIZES`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-recalcpagesizes)
/// message, which has no parameters.
///
/// Return type: `SysResult<()>`.
pub struct PsmRecalcPageSizes {}

impl MsgSend for PsmRecalcPageSizes {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::RECALCPAGESIZES.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`PSM_REMOVEPAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-removepage)
/// message parameters.
///
/// Return type: `()`.
pub struct PsmRemovePage<'a> {
	pub index: Option<u32>,
	pub hpspg: Option<&'a HPROPSHEETPAGE>,
}

impl<'a> MsgSend for PsmRemovePage<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::REMOVEPAGE.into(),
			wparam: self.index.unwrap_or_default() as _,
			lparam: self.hpspg.map_or(std::ptr::null_mut(), |h| h.ptr()) as _,
		}
	}
}

pub_struct_msg_empty! { PsmRestartWindows: co::PSM::RESTARTWINDOWS.into();
	/// [`PSM_RESTARTWINDOWS`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-restartwindows)
}

/// [`PSM_SETBUTTONTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-setbuttontext)
/// message parameters.
///
/// Return type: `()`.
pub struct PsmSetButtonText {
	pub btn: co::PSWIZB,
	pub text: WString,
}

impl MsgSend for PsmSetButtonText {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::SETBUTTONTEXT.into(),
			wparam: self.btn.raw() as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`PSM_SETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-setcursel)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct PsmSetCurSel<'a> {
	pub index: Option<u32>,
	pub hpspg: Option<&'a HPROPSHEETPAGE>,
}

impl<'a> MsgSend for PsmSetCurSel<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::SETCURSEL.into(),
			wparam: self.index.unwrap_or_default() as _,
			lparam: self.hpspg.map_or(std::ptr::null_mut(), |h| h.ptr()) as _,
		}
	}
}

/// [`PSM_SETCURSELID`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-setcurselid)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct PsmSetCurSelId {
	pub id: u16,
}

impl MsgSend for PsmSetCurSelId {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::SETCURSELID.into(),
			wparam: self.id as _,
			lparam: 0,
		}
	}
}

/// [`PSM_SETFINISHTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-setfinishtext)
/// message parameters.
///
/// Return type: `()`.
pub struct PsmSetFinishText {
	pub text: WString,
}

impl MsgSend for PsmSetFinishText {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::SETFINISHTEXT.into(),
			wparam: 0,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`PSM_SETHEADERSUBTITLE`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-setheadersubtitle)
/// message parameters.
///
/// Return type: `()`.
pub struct PsmSetHeaderSubtitle {
	pub index: u32,
	pub text: WString,
}

impl MsgSend for PsmSetHeaderSubtitle {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::SETHEADERSUBTITLE.into(),
			wparam: self.index as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`PSM_SETHEADERTITLE`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-setheadertitle)
/// message parameters.
///
/// Return type: `()`.
pub struct PsmSetHeaderTitle {
	pub index: u32,
	pub text: WString,
}

impl MsgSend for PsmSetHeaderTitle {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::SETHEADERTITLE.into(),
			wparam: self.index as _,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`PSM_SETNEXTTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-setnexttext)
/// message parameters.
///
/// Return type: `()`.
pub struct PsmSetNextText {
	pub text: WString,
}

impl MsgSend for PsmSetNextText {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::SETNEXTTEXT.into(),
			wparam: 0,
			lparam: self.text.as_ptr() as _,
		}
	}
}

/// [`PSM_SETTITLE`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-settitle)
/// message parameters.
///
/// Return type: `()`.
pub struct PsmSetTitle {
	pub incl_prefix_suffix: co::PSH,
	pub title: IdStr,
}

impl MsgSend for PsmSetTitle {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::SETTITLE.into(),
			wparam: self.incl_prefix_suffix.raw() as _,
			lparam: self.title.as_ptr() as _,
		}
	}
}

/// [`PSM_SETWIZBUTTONS`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-setwizbuttons)
/// message parameters.
///
/// Return type: `()`.
pub struct PsmSetWizButtons {
	pub elev_icon: co::PSWIZBF,
	pub btns: co::PSWIZB,
}

impl MsgSend for PsmSetWizButtons {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::SETWIZBUTTONS.into(),
			wparam: self.elev_icon.raw() as _,
			lparam: self.btns.raw() as _,
		}
	}
}

/// [`PSM_SHOWWIZBUTTONS`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-showwizbuttons)
/// message parameters.
///
/// Return type: `()`.
pub struct PsmShowWizButtons {
	pub btns_to_check: co::PSWIZB,
	pub btns_to_enable: co::PSWIZB,
}

impl MsgSend for PsmShowWizButtons {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::SHOWWIZBUTTONS.into(),
			wparam: self.btns_to_check.raw() as _,
			lparam: self.btns_to_enable.raw() as _,
		}
	}
}

/// [`PSM_UNCHANGED`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-unchanged)
/// message parameters.
///
/// Return type: `()`.
pub struct PsmUnchanged<'a> {
	pub hpspg: &'a HPROPSHEETPAGE,
}

impl<'a> MsgSend for PsmUnchanged<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::PSM::UNCHANGED.into(),
			wparam: self.hpspg.ptr() as _,
			lparam: 0,
		}
	}
}
