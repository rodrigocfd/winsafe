use crate::co;
use crate::comctl::decl::{HIMAGELIST, TCITEM};
use crate::kernel::decl::SysResult;
use crate::msg::WndMsg;
use crate::prelude::MsgSend;
use crate::user::decl::RECT;
use crate::user::privs::{minus1_as_none, zero_as_err};

/// [`TCM_ADJUSTRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-adjustrect)
/// message parameters.
///
/// Return type: `()`.
pub struct AdjustRect<'a> {
	pub display_rect: bool,
	pub rect: &'a mut RECT,
}

unsafe impl<'a> MsgSend for AdjustRect<'a> {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::ADJUSTRECT.into(),
			wparam: self.display_rect as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TCM_DELETEALLITEMS`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-deleteallitems)
/// message, which has no parameters.
///
/// Return type: `SysResult<()>`.
pub struct DeleteAllItems {}

unsafe impl MsgSend for DeleteAllItems {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::DELETEALLITEMS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_DELETEITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-deleteitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct DeleteItem {
	pub index: u32,
}

unsafe impl MsgSend for DeleteItem {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::DELETEITEM.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TCM_DESELECTALL`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-deselectall)
/// message parameters.
///
/// Return type: `()`.
pub struct DeselectAll {
	pub except_current: bool,
}

unsafe impl MsgSend for DeselectAll {
	type RetType = ();

	fn convert_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::DESELECTALL.into(),
			wparam: self.except_current as _,
			lparam: 0,
		}
	}
}

/// [`TCM_GETCURFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getcurfocus)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct GetCurFocus {}

unsafe impl MsgSend for GetCurFocus {
	type RetType = u32;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETCURFOCUS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getcursel)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct GetCurSel {}

unsafe impl MsgSend for GetCurSel {
	type RetType = Option<u32>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|i| i as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETCURSEL.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getextendedstyle)
/// message, which has no parameters.
///
/// Return type: `co::TCS_EX`.
pub struct GetExtendedStyle {}

unsafe impl MsgSend for GetExtendedStyle {
	type RetType = co::TCS_EX;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		co::TCS_EX(v as _)
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETEXTENDEDSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getimagelist)
/// message, which has no parameters.
///
/// Return type: SysResult<HIMAGELIST>.
pub struct GetImageList {}

unsafe impl MsgSend for GetImageList {
	type RetType = SysResult<HIMAGELIST>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|p| HIMAGELIST(p as _))
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETIMAGELIST.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct GetItem<'a, 'b> {
	pub index: u32,
	pub item: &'b mut TCITEM<'a>,
}

unsafe impl<'a, 'b> MsgSend for GetItem<'a, 'b> {
	type RetType = SysResult<()>;

	fn convert_ret(&self, v: isize) -> Self::RetType {
		zero_as_err(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> WndMsg {
		WndMsg {
			msg_id: co::TCM::GETITEM.into(),
			wparam: self.index as _,
			lparam: self.item as *mut _ as _,
		}
	}
}
