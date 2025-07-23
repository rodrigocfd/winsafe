use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`PSM_ADDPAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-addpage)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct AddPage<'a> {
	pub hpspg: &'a HPROPSHEETPAGE,
}

impl<'a> MsgSend for AddPage<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct Apply {}

impl MsgSend for Apply {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PSM::APPLY.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

pub_struct_msg_empty! { CancelToClose: co::PSM::CANCELTOCLOSE.into();
	/// [`PSM_CANCELTOCLOSE`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-canceltoclose)
}

pub_struct_msg_empty! { Changed: co::PSM::CHANGED.into();
	/// [`PSM_CHANGED`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-changed)
}

/// [`PSM_ENABLEWIZBUTTONS`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-enablewizbuttons)
/// message parameters.
///
/// Return type: `()`.
pub struct EnableWizButtons {
	pub btns_to_check: co::PSWIZB,
	pub btns_to_enable: co::PSWIZB,
}

impl MsgSend for EnableWizButtons {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct GetCurrentPageHwnd {}

impl MsgSend for GetCurrentPageHwnd {
	type RetType = HWND;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { HWND::from_ptr(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct GetResult {}

impl MsgSend for GetResult {
	type RetType = SysResult<co::ID_PB>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|n| unsafe { co::ID_PB::from_raw(n as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct GetTabControl {}

impl MsgSend for GetTabControl {
	type RetType = HWND;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { HWND::from_ptr(v as _) }
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct HwndToIndex<'a> {
	pub hwnd_page: &'a HWND,
}

impl<'a> MsgSend for HwndToIndex<'a> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|n| n as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct IdToIndex {
	pub pg_res_id: u32,
}

impl MsgSend for IdToIndex {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|n| n as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct IndexToHwnd {
	pub pg_index: u32,
}

impl MsgSend for IndexToHwnd {
	type RetType = SysResult<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct IndexToId {
	pub pg_index: u32,
}

impl MsgSend for IndexToId {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|n| n as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct IndexToPage {
	pub pg_index: u32,
}

impl MsgSend for IndexToPage {
	type RetType = SysResult<HPROPSHEETPAGE>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|p| unsafe { HPROPSHEETPAGE::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct InsertPage<'a> {
	pub index: u32,
	pub hpage: &'a HPROPSHEETPAGE,
}

impl<'a> MsgSend for InsertPage<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct IsDialogMessage<'a> {
	pub msg: &'a mut MSG,
}

impl<'a> MsgSend for IsDialogMessage<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct PageToIndex<'a> {
	pub hpspg: &'a HPROPSHEETPAGE,
}

impl<'a> MsgSend for PageToIndex<'a> {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|n| n as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct PressButton {
	pub index: co::PSBTN,
}

impl MsgSend for PressButton {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct QuerySiblings {
	pub param1: isize,
	pub param2: isize,
}

impl MsgSend for QuerySiblings {
	type RetType = isize;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PSM::QUERYSIBLINGS.into(),
			wparam: self.param1 as _,
			lparam: self.param2,
		}
	}
}

pub_struct_msg_empty! { RebootSystem: co::PSM::REBOOTSYSTEM.into();
	/// [`PSM_REBOOTSYSTEM`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-rebootsystem)
}

/// [`PSM_RECALCPAGESIZES`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-recalcpagesizes)
/// message, which has no parameters.
///
/// Return type: `SysResult<()>`.
pub struct RecalcPageSizes {}

impl MsgSend for RecalcPageSizes {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct RemovePage<'a> {
	pub index: Option<u32>,
	pub hpspg: Option<&'a HPROPSHEETPAGE>,
}

impl<'a> MsgSend for RemovePage<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PSM::REMOVEPAGE.into(),
			wparam: self.index.unwrap_or_default() as _,
			lparam: self.hpspg.map_or(std::ptr::null_mut(), |h| h.ptr()) as _,
		}
	}
}

pub_struct_msg_empty! { RestartWindows: co::PSM::RESTARTWINDOWS.into();
	/// [`PSM_RESTARTWINDOWS`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-restartwindows)
}

/// [`PSM_SETBUTTONTEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/psm-setbuttontext)
/// message parameters.
///
/// Return type: `()`.
pub struct SetButtonText {
	pub btn: co::PSWIZB,
	pub text: WString,
}

impl MsgSend for SetButtonText {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct SetCurSel<'a> {
	pub index: Option<u32>,
	pub hpspg: Option<&'a HPROPSHEETPAGE>,
}

impl<'a> MsgSend for SetCurSel<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct SetCurSelId {
	pub id: u16,
}

impl MsgSend for SetCurSelId {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct SetFinishText {
	pub text: WString,
}

impl MsgSend for SetFinishText {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct SetHeaderSubtitle {
	pub index: u32,
	pub text: WString,
}

impl MsgSend for SetHeaderSubtitle {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct SetHeaderTitle {
	pub index: u32,
	pub text: WString,
}

impl MsgSend for SetHeaderTitle {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct SetNextText {
	pub text: WString,
}

impl MsgSend for SetNextText {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct SetTitle {
	pub incl_prefix_suffix: co::PSH,
	pub title: IdStr,
}

impl MsgSend for SetTitle {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct SetWizButtons {
	pub elev_icon: co::PSWIZBF,
	pub btns: co::PSWIZB,
}

impl MsgSend for SetWizButtons {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct ShowWizButtons {
	pub btns_to_check: co::PSWIZB,
	pub btns_to_enable: co::PSWIZB,
}

impl MsgSend for ShowWizButtons {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
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
pub struct Unchanged<'a> {
	pub hpspg: &'a HPROPSHEETPAGE,
}

impl<'a> MsgSend for Unchanged<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::PSM::UNCHANGED.into(),
			wparam: self.hpspg.ptr() as _,
			lparam: 0,
		}
	}
}
