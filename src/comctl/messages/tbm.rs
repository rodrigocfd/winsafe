use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::privs::*;

/// [`TBM_CLEARSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-clearsel)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmClearSel {
	pub redraw: bool,
}

impl MsgSend for TbmClearSel {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::CLEARSEL.into(),
			wparam: self.redraw as _,
			lparam: 0,
		}
	}
}

/// [`TBM_CLEARTICS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-cleartics)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmClearTics {
	pub redraw: bool,
}

impl MsgSend for TbmClearTics {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::CLEARTICS.into(),
			wparam: self.redraw as _,
			lparam: 0,
		}
	}
}

/// [`TBM_GETBUDDY`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getbuddy)
/// message parameters.
///
/// Return type: `Option<HWND>`.
pub struct TbmGetBuddy {
	pub left_above: bool,
}

impl MsgSend for TbmGetBuddy {
	type RetType = Option<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETBUDDY.into(),
			wparam: self.left_above as _,
			lparam: 0,
		}
	}
}

/// [`TBM_GETCHANNELRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getchannelrect)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmGetChannelRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for TbmGetChannelRect<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETCHANNELRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TBM_GETLINESIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getlinesize)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TbmGetLineSize {}

impl MsgSend for TbmGetLineSize {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETLINESIZE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETNUMTICS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getnumtics)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TbmGetNumTics {}

impl MsgSend for TbmGetNumTics {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETNUMTICS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETPAGESIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getpagesize)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TbmGetPageSize {}

impl MsgSend for TbmGetPageSize {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETPAGESIZE.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getpos)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TbmGetPos {}

impl MsgSend for TbmGetPos {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETPOS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETRANGEMAX`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getrangemax)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TbmGetRangeMax {}

impl MsgSend for TbmGetRangeMax {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETRANGEMAX.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETRANGEMIN`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getrangemin)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TbmGetRangeMin {}

impl MsgSend for TbmGetRangeMin {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETRANGEMIN.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETSELEND`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getselend)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TbmGetSelEnd {}

impl MsgSend for TbmGetSelEnd {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETSELEND.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETSELSTART`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getselstart)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TbmGetSelStart {}

impl MsgSend for TbmGetSelStart {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETSELSTART.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETTHUMBLENGTH`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getthumblength)
/// message, which has no parameters.
///
/// Return type: `u32`.
pub struct TbmGetThumbLength {}

impl MsgSend for TbmGetThumbLength {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETTHUMBLENGTH.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETTHUMBRECT`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getthumbrect)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmGetThumbRect<'a> {
	pub rect: &'a mut RECT,
}

impl<'a> MsgSend for TbmGetThumbRect<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETTHUMBRECT.into(),
			wparam: 0,
			lparam: self.rect as *mut _ as _,
		}
	}
}

/// [`TBM_GETTIC`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-gettic)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct TbmGetTic {
	pub index: u32,
}

impl MsgSend for TbmGetTic {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETTIC.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TBM_GETTICPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getticpos)
/// message parameters.
///
/// Return type: `SysResult<u32>`.
pub struct TbmGetTicPos {
	pub index: u32,
}

impl MsgSend for TbmGetTicPos {
	type RetType = SysResult<u32>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		minus1_as_badargs(v).map(|v| v as _)
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETTICPOS.into(),
			wparam: self.index as _,
			lparam: 0,
		}
	}
}

/// [`TBM_GETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-gettooltips)
/// message, which has no parameters.
///
/// Return type: `Option<HWND>`.
pub struct TbmGetTooltips {}

impl MsgSend for TbmGetTooltips {
	type RetType = Option<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::GETTOOLTIPS.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_GETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-getunicodeformat)
/// message, which has no parameters.
///
/// Return type: `bool`.
pub struct TbmGetUnicodeFormat {}

impl MsgSend for TbmGetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TBM::GETUNICODEFORMAT.into(),
			wparam: 0,
			lparam: 0,
		}
	}
}

/// [`TBM_SETBUDDY`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setbuddy)
/// message parameters.
///
/// Return type: `Option<HWND>`.
pub struct TbmSetBuddy<'a> {
	pub left_above: bool,
	pub hwnd_buddy: &'a HWND,
}

impl<'a> MsgSend for TbmSetBuddy<'a> {
	type RetType = Option<HWND>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_none(v).map(|p| unsafe { HWND::from_ptr(p as _) })
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETBUDDY.into(),
			wparam: self.left_above as _,
			lparam: self.hwnd_buddy.ptr() as _,
		}
	}
}

/// [`TBM_SETLINESIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setlinesize)
/// message parameters.
///
/// Return type: `u32`.
pub struct TbmSetLineSize {
	pub size: u32,
}

impl MsgSend for TbmSetLineSize {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETLINESIZE.into(),
			wparam: 0,
			lparam: self.size as _,
		}
	}
}

/// [`TBM_SETPAGSIZE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setpagesize)
/// message parameters.
///
/// Return type: `u32`.
pub struct TbmSetPageSize {
	pub page_size: u32,
}

impl MsgSend for TbmSetPageSize {
	type RetType = u32;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v as _
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETPAGESIZE.into(),
			wparam: 0,
			lparam: self.page_size as _,
		}
	}
}

/// [`TBM_SETPOS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setpos)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmSetPos {
	pub redraw: bool,
	pub pos: u32,
}

impl MsgSend for TbmSetPos {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETPOS.into(),
			wparam: self.redraw as _,
			lparam: self.pos as _,
		}
	}
}

/// [`TBM_SETPOSNOTIFY`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setposnotify)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmSetPosNotify {
	pub pos: u32,
}

impl MsgSend for TbmSetPosNotify {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETPOSNOTIFY.into(),
			wparam: 0,
			lparam: self.pos as _,
		}
	}
}

/// [`TBM_SETRANGE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setrange)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmSetRange {
	pub redraw: bool,
	pub min_pos: u16,
	pub max_pos: u16,
}

impl MsgSend for TbmSetRange {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETRANGE.into(),
			wparam: self.redraw as _,
			lparam: MAKEDWORD(self.min_pos, self.max_pos) as _,
		}
	}
}

/// [`TBM_SETRANGEMAX`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setrangemax)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmSetRangeMax {
	pub redraw: bool,
	pub max: u32,
}

impl MsgSend for TbmSetRangeMax {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETRANGEMAX.into(),
			wparam: self.redraw as _,
			lparam: self.max as _,
		}
	}
}

/// [`TBM_SETRANGEMIN`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setrangemin)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmSetRangeMin {
	pub redraw: bool,
	pub min: u32,
}

impl MsgSend for TbmSetRangeMin {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETRANGEMIN.into(),
			wparam: self.redraw as _,
			lparam: self.min as _,
		}
	}
}

/// [`TBM_SETSEL`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setsel)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmSetSel {
	pub redraw: bool,
	pub start_pos: u16,
	pub end_pos: u16,
}

impl MsgSend for TbmSetSel {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETSEL.into(),
			wparam: self.redraw as _,
			lparam: MAKEDWORD(self.start_pos, self.end_pos) as _,
		}
	}
}

/// [`TBM_SETSELEND`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setselend)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmSetSelEnd {
	pub redraw: bool,
	pub end: u32,
}

impl MsgSend for TbmSetSelEnd {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETSELEND.into(),
			wparam: self.redraw as _,
			lparam: self.end as _,
		}
	}
}

/// [`TBM_SETSELSTART`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setselstart)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmSetSelStart {
	pub redraw: bool,
	pub start: u32,
}

impl MsgSend for TbmSetSelStart {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETSELSTART.into(),
			wparam: self.redraw as _,
			lparam: self.start as _,
		}
	}
}

/// [`TBM_SETTHUMBLENGTH`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setthumblength)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmSetThumbLength {
	pub length: u32,
}

impl MsgSend for TbmSetThumbLength {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETTHUMBLENGTH.into(),
			wparam: self.length as _,
			lparam: 0,
		}
	}
}

/// [`TBM_SETTIC`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-settic)
/// message parameters.
///
/// Return type: `SysResult<()>`.
pub struct TbmSetTic {
	pub pos: u32,
}

impl MsgSend for TbmSetTic {
	type RetType = SysResult<()>;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		zero_as_badargs(v).map(|_| ())
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETTIC.into(),
			wparam: 0,
			lparam: self.pos as _,
		}
	}
}

/// [`TBM_SETTICFREQ`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setticfreq)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmSetTicFreq {
	pub freq: u32,
}

impl MsgSend for TbmSetTicFreq {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETTICFREQ.into(),
			wparam: self.freq as _,
			lparam: 0,
		}
	}
}

/// [`TBM_SETTIPSIDE`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-settipside)
/// message parameters.
///
/// Return type: `co::TBTS`.
pub struct TbmSetTipSide {
	pub location: co::TBTS,
}

impl MsgSend for TbmSetTipSide {
	type RetType = co::TBTS;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		unsafe { co::TBTS::from_raw(v as _) }
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETTIPSIDE.into(),
			wparam: self.location.raw() as _,
			lparam: 0,
		}
	}
}

/// [`TBM_SETTOOLTIPS`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-settooltips)
/// message parameters.
///
/// Return type: `()`.
pub struct TbmSetTooltips<'a> {
	pub htooltips: Option<&'a HWND>,
}

impl<'a> MsgSend for TbmSetTooltips<'a> {
	type RetType = ();

	unsafe fn isize_to_ret(&self, _: isize) -> Self::RetType {
		()
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TRBM::SETTOOLTIPS.into(),
			wparam: self.htooltips.map_or(0, |h| h.ptr() as _),
			lparam: 0,
		}
	}
}

/// [`TBM_SETUNICODEFORMAT`](https://learn.microsoft.com/en-us/windows/win32/controls/tbm-setunicodeformat)
/// message parameters.
///
/// Return type: `bool`.
pub struct TbmSetUnicodeFormat {
	pub use_unicode: bool,
}

impl MsgSend for TbmSetUnicodeFormat {
	type RetType = bool;

	unsafe fn isize_to_ret(&self, v: isize) -> Self::RetType {
		v != 0
	}

	fn as_generic_wm(&mut self) -> Wm {
		Wm {
			msg_id: co::TBM::SETUNICODEFORMAT.into(),
			wparam: self.use_unicode as _,
			lparam: 0,
		}
	}
}
