use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`TCM_ADJUSTRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-adjustrect)
/// message parameters.
///
/// Return type: `()`.
pub struct TcmAdjustRect<'a> {
	pub display_rect: bool,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for TcmAdjustRect<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct TcmDeleteAllItems {}

impl MsgSend for TcmDeleteAllItems {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct TcmDeleteItem {
	pub index: u32,
}

impl MsgSend for TcmDeleteItem {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct TcmDeselectAll {
	pub except_current: bool,
}

impl MsgSend for TcmDeselectAll {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::DESELECTALL.into(),
			wparam: self.except_current as _,
			lparam: 0,
		}
	}
}

/// [`TCM_GETCURFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getcurfocus)
/// message, which has no parameters.
///
/// Return type: `Option<u32>`.
pub struct TcmGetCurFocus {}

impl MsgSend for TcmGetCurFocus {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|i| i as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct TcmGetCurSel {}

impl MsgSend for TcmGetCurSel {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|i| i as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct TcmGetExtendedStyle {}

impl MsgSend for TcmGetExtendedStyle {
	type RetType = co::TCS_EX;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TCS_EX::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::GETEXTENDEDSTYLE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getimagelist)
/// message, which has no parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct TcmGetImageList {}

impl MsgSend for TcmGetImageList {
	type RetType = Option<HIMAGELIST>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
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
pub struct TcmGetItem<'a, 'b> {
	pub index: u32,
	pub item: &'b mut TCITEM<'a>,
}

impl<'a, 'b> MsgSend for TcmGetItem<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::GETITEM.into(),
			wparam: self.index as _,
			lparam: self.item as *mut _ as _,
		}
	}
}

/// [`TCM_GETITEMCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getitemcount)
/// message, which has no parameters.
///
/// Return type: `SysResult<u32>`.
pub struct TcmGetItemCount {}

impl MsgSend for TcmGetItemCount {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|c| c as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::GETITEMCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETITEMRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getitemrect)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TcmGetItemRect<'a> {
	pub index: u32,
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for TcmGetItemRect<'a> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::GETITEMRECT.into(),
			wparam: self.index as _,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TCM_GETROWCOUNT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getrowcount)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TcmGetRowCount {}

impl MsgSend for TcmGetRowCount {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::GETROWCOUNT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-gettooltips)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
pub struct TcmGetTooltips {}

impl MsgSend for TcmGetTooltips {
	type RetType = Option<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|v| unsafe { HWND::from_ptr(v as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::GETTOOLTIPS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-getunicodeformat)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct TcmGetUnicodeFormat {}

impl MsgSend for TcmGetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TCM_HIGHLIGHTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-highlightitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TcmHighlightItem {
	pub index: u32,
	pub highlight: bool,
}

impl MsgSend for TcmHighlightItem {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::HIGHLIGHTITEM.into(),
			wparam: self.index as _,
			lparam: MAKEDWORD(self.highlight as _, 0) as _,
		}
	}
}

/// [`TCM_HITTEST`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-hittest)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct TcmHitTest<'a> {
	pub info: &'a mut TCHITTESTINFO,
}

impl<'a> MsgSend for TcmHitTest<'a> {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|n| n as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::HITTEST.into(),
			wparam: 0,
			lparam: self.info as *mut _ as _,
		}
	}
}

/// [`TCM_INSERTITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-insertitem)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct TcmInsertItem<'a, 'b> {
	pub index: u32,
	pub item: &'b TCITEM<'a>,
}

impl<'a, 'b> MsgSend for TcmInsertItem<'a, 'b> {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|i| i as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::INSERTITEM.into(),
			wparam: self.index as _,
			lparam: self.item as *const _ as _,
		}
	}
}

/// [`TCM_REMOVEIMAGE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-removeimage)
/// message parameters.
///
/// Return type: `()`.
pub struct TcmRemoveImage {
	pub index: u32,
}

impl MsgSend for TcmRemoveImage {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::REMOVEIMAGE.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TCM_SETCURFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setcurfocus)
/// message parameters.
///
/// Return type: `()`.
pub struct TcmSetCurFocus {
	pub index: u32,
}

impl MsgSend for TcmSetCurFocus {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::SETCURFOCUS.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TCM_SETCURSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setcursel)
/// message parameters.
///
/// Return type: `Option<u32>`.
pub struct TcmSetCurSel {
	pub index: u32,
}

impl MsgSend for TcmSetCurSel {
	type RetType = Option<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_none(v).map(|i| i as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::SETCURSEL.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TCM_SETEXTENDEDSTYLE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setextendedstyle)
/// message parameters.
///
/// Return type: `co::TCS_EX`.
pub struct TcmSetExtendedStyle {
	pub mask: co::TCS_EX,
	pub style: co::TCS_EX,
}

impl MsgSend for TcmSetExtendedStyle {
	type RetType = co::TCS_EX;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TCS_EX::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::SETEXTENDEDSTYLE.into(),
			wparam: self.mask.raw() as _,
			lparam: self.style.raw() as _,
		}
	}
}

/// [`TCM_SETIMAGELIST`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setimagelist)
/// message parameters.
///
/// Return type: `Option<HIMAGELIST>`.
pub struct TcmSetImageList {
	pub himagelist: Option<HIMAGELIST>,
}

impl MsgSend for TcmSetImageList {
	type RetType = Option<HIMAGELIST>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HIMAGELIST::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::SETIMAGELIST.into(),
			wparam: 0,
			lparam: self.himagelist.as_ref().map_or(0, |h| h.ptr() as _),
		}
	}
}

/// [`TCM_SETITEM`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setitem)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TcmSetItem<'a, 'b> {
	pub index: u32,
	pub item: &'b TCITEM<'a>,
}

impl<'a, 'b> MsgSend for TcmSetItem<'a, 'b> {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::SETITEM.into(),
			wparam: self.index as _,
			lparam: self.item as *const _ as _,
		}
	}
}

/// [`TCM_SETITEMEXTRA`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setitemextra)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TcmSetItemExtra {
	pub num_bytes: u32,
}

impl MsgSend for TcmSetItemExtra {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::SETITEMEXTRA.into(),
			wparam: self.num_bytes as _,
			lparam: 0,
		}
	}
}

/// [`TCM_SETITEMSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setitemsize)
/// message parameters.
///
/// Return type: `(u16, u16)`.
pub struct TcmSetItemSize {
	pub width: u16,
	pub height: u16,
}

impl MsgSend for TcmSetItemSize {
	type RetType = (u16, u16);

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		(LOWORD(v as _), HIWORD(v as _))
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::SETITEMSIZE.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.width, self.height) as _,
		}
	}
}

/// [`TCM_SETMINTABWIDTH`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setmintabwidth)
/// message parameters.
///
/// Return type: `u32`.
pub struct TcmSetMinTabWidth {
	pub min_width: Option<u32>,
}

impl MsgSend for TcmSetMinTabWidth {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::SETMINTABWIDTH.into(),
			wparam: 0,
			lparam: self.min_width.map_or(-1, |w| w as _),
		}
	}
}

/// [`TCM_SETPADDING`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setpadding)
/// message parameters.
///
/// Return type: `()`.
pub struct TcmSetPadding {
	pub horizontal: u16,
	pub vertical: u16,
}

impl MsgSend for TcmSetPadding {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::SETPADDING.into(),
			wparam: 0,
			lparam: MAKEDWORD(self.horizontal, self.vertical) as _,
		}
	}
}

/// [`TCM_SETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-settooltips)
/// message parameters.
///
/// Return type: `()`.
pub struct TcmSetTooltips<'a> {
	pub htooltips: Option<&'a HWND>,
}

impl<'a> MsgSend for TcmSetTooltips<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::SETTOOLTIPS.into(),
			wparam: self.htooltips.map_or(0, |h| h.ptr() as _),
			lparam: 0,
		}
	}
}

/// [`TCM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tcm-setunicodeformat)
/// message parameters.
///
/// Return type: `bool`.
pub struct TcmSetUnicodeFormat {
	pub use_unicode: bool,
}

impl MsgSend for TcmSetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TCM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}
