//! Assorted Win32 structs used by common controls.

#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::enums::{BmpIdbRes, IdStr, IndexStr, TreeitemTvi};
use crate::handles::{HBITMAP, HDC, HIMAGELIST, HINSTANCE, HTREEITEM, HWND};
use crate::handles::prelude::Handle;
use crate::privs::{
	HINST_COMMCTRL,
	IS_INTRESOURCE,
	L_MAX_URL_LENGTH,
	MAX_LINKID_TEXT,
};
use crate::structs::{COLORREF, NMHDR, POINT, RECT, SIZE, SYSTEMTIME, WINDOWPOS};
use crate::various::WString;

/// [`BUTTON_IMAGELIST`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-button_imagelist)
/// struct.
#[repr(C)]
pub struct BUTTON_IMAGELIST {
	pub himl: HIMAGELIST,
	pub margin: RECT,
	pub uAlign: co::BIA,
}

/// [`BUTTON_SPLITINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-button_splitinfo)
/// struct.
#[repr(C)]
pub struct BUTTON_SPLITINFO {
	pub mask: co::BCSIF,
	pub himlGlyph: HIMAGELIST,
	pub uSplitStyle: co::BCSS,
	pub size: SIZE,
}

/// [`DATETIMEPICKERINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-datetimepickerinfo)
/// struct.
#[repr(C)]
pub struct DATETIMEPICKERINFO {
	cbSize: u32,
	pub rcCheck: RECT,
	pub stateCheck: co::STATE_SYSTEM,
	pub rcButton: RECT,
	pub stateButton: co::STATE_SYSTEM,
	pub hwndEdit: HWND,
	pub hwndUD: HWND,
	pub hwndDropDown: HWND,
}

impl_default_with_size!(DATETIMEPICKERINFO, cbSize);

/// [`HDHITTESTINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-hdhittestinfo)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct HDHITTESTINFO {
	pub pt: POINT,
	pub flags: co::HHT,
	pub iItem: i32,
}

/// [`HDITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-hditemw)
/// struct.
#[repr(C)]
pub struct HDITEM<'a> {
	pub mask: co::HDI,
	pub cxy: i32,
	pszText: *mut u16,
	pub hbm: HBITMAP,
	cchTextMax: i32,
	pub fmt: co::HDF,
	pub lParam: isize,
	pub iImage: i32,
	pub iOrder: i32,
	pub typeFilter: co::HDFT,
	pub pvFilter: *mut std::ffi::c_void,
	pub state: co::HDIS,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(HDITEM, 'a);

impl<'a> HDITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`HDLAYOUT`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-hdlayout)
/// struct.
#[repr(C)]
pub struct HDLAYOUT<'a, 'b> {
	prc: *mut RECT,
	pwpos: *mut WINDOWPOS,
	fuuu: i32,

	prc_: PhantomData<&'a mut RECT>,
	pwpos_: PhantomData<&'b mut WINDOWPOS>,
}

impl_default!(HDLAYOUT, 'a, 'b);

impl<'a, 'b> HDLAYOUT<'a, 'b> {
	/// Sets the field.
	pub fn set_prc(&mut self, rc: Option<&'a mut RECT>) {
		self.prc = rc.map(|rc| rc as _).unwrap_or(std::ptr::null_mut());
	}

	/// Sets the field.
	pub fn set_pwpos(&mut self, pos: Option<&'b mut WINDOWPOS>) {
		self.pwpos = pos.map(|pos| pos as _).unwrap_or(std::ptr::null_mut());
	}
}

/// [`IMAGELISTDRAWPARAMS`](https://docs.microsoft.com/en-us/windows/win32/api/commoncontrols/ns-commoncontrols-imagelistdrawparams)
/// struct.
#[repr(C)]
pub struct IMAGELISTDRAWPARAMS {
	cbSize: u32,
	pub himl: HIMAGELIST,
	pub i: i32,
	pub hdcDst: HDC,
	pub x: i32,
	pub y: i32,
	pub cx: i32,
	pub cy: i32,
	pub xBitmap: i32,
	pub yBitmap: i32,
	pub rgbBk: COLORREF,
	pub rgbFg: COLORREF,
	pub fStyle: co::ILD,
	pub dwRop: co::ROP,
	pub fState: co::ILS,
	pub Frame: u32,
	pub crEffect: COLORREF,
}

impl_default_with_size!(IMAGELISTDRAWPARAMS, cbSize);

/// [`LITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-litem)
/// struct.
#[repr(C)]
pub struct LITEM {
	pub mask: co::LIF,
	pub iLink: i32,
	pub state: co::LIS,
	pub stateMask: co::LIS,
	szID: [u16; MAX_LINKID_TEXT],
	szUrl: [u16; L_MAX_URL_LENGTH],
}

impl_default!(LITEM);

impl LITEM {
	pub_fn_string_arr_get_set!(szID, set_szID);
	pub_fn_string_arr_get_set!(szUrl, set_szUrl);
}

/// [`LVCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvcolumnw)
/// struct.
#[repr(C)]
pub struct LVCOLUMN<'a> {
	pub mask: co::LVCF,
	pub fmt: co::LVCFMT_C,
	pub cx: i32,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iSubItem: i32,
	pub iImage: i32,
	pub iOrder: i32,
	pub cxMin: i32,
	pub cxDefault: i32,
	pub cxIdeal: i32,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(LVCOLUMN, 'a);

impl<'a> LVCOLUMN<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`LVFINDINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvfindinfow)
/// struct.
#[repr(C)]
pub struct LVFINDINFO<'a> {
	pub flags: co::LVFI,
	psz: *mut u16,
	pub lParam: isize,
	pub pt: POINT,
	pub vkDirection: co::VK_DIR,

	psz_: PhantomData<&'a mut u16>,
}

impl_default!(LVFINDINFO, 'a);

impl<'a> LVFINDINFO<'a> {
	pub_fn_string_ptr_get_set!('a, psz, set_psz);
}

/// [`LVHITTESTINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvhittestinfo)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct LVHITTESTINFO {
	pub pt: POINT,
	pub flags: co::LVHT,
	pub iItem: i32,
	pub iSubItem: i32,
	pub iGroup: i32,
}

/// [`LVITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvitemw)
/// struct.
#[repr(C)]
pub struct LVITEM<'a> {
	pub mask: co::LVIF,
	pub iItem: i32,
	pub iSubItem: i32,
	pub state: co::LVIS,
	pub stateMask: co::LVIS,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub lParam: isize,
	pub iIndent: i32,
	pub iGroupId: co::LVI_GROUPID,
	pub cColumns: u32,
	pub puColumns: *mut i32,
	pub piColFmt: *mut co::LVCFMT_I,
	pub iGroup: i32,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(LVITEM, 'a);

impl<'a> LVITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`LVITEMINDEX`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvitemindex)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct LVITEMINDEX {
	pub iItem: i32,
	pub iGroup: i32,
}

/// [`NMBCDROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbcdropdown)
/// struct.
#[repr(C)]
pub struct NMBCDROPDOWN {
	pub hdr: NMHDR,
	pub rcButton: RECT,
}

/// [`NMBCHOTITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbchotitem)
/// struct.
#[repr(C)]
pub struct NMBCHOTITEM {
	pub hdr: NMHDR,
	pub dwFlags: co::HICF,
}

/// [`NMCHAR`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmchar)
/// struct.
#[repr(C)]
pub struct NMCHAR {
	pub hdr: NMHDR,
	pub ch: u32,
	pub dwItemPrev: u32,
	pub dwItemNext: u32,
}

/// [`NMCUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmcustomdraw)
/// struct.
#[repr(C)]
pub struct NMCUSTOMDRAW {
	pub hdr: NMHDR,
	pub dwDrawStage: co::CDDS,
	pub hdc: HDC,
	pub rc: RECT,
	pub dwItemSpec: usize,
	pub uItemState: co::CDIS,
	pub lItemlParam: isize,
}

/// [`NMDATETIMECHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimechange)
/// struct.
#[repr(C)]
pub struct NMDATETIMECHANGE {
	pub nmhdr: NMHDR,
	pub dwFlags: co::GDT,
	pub st: SYSTEMTIME,
}

/// [`NMDATETIMEFORMAT`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimeformatw)
/// struct.
#[repr(C)]
pub struct NMDATETIMEFORMAT<'a> {
	pub nmhdr: NMHDR,
	pszFormat: *mut u16,
	pub st: SYSTEMTIME,
	pszDisplay: *mut u16,
	szDisplay: [u16; 64], // used as a buffer to pszDisplay

	pszFormat_: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMEFORMAT, 'a);

impl<'a> NMDATETIMEFORMAT<'a> {
	pub_fn_string_ptr_get_set!('a, pszFormat, set_pszFormat);

	/// Returns the `pszDisplay` field.
	pub fn pszDisplay(&self) -> String {
		WString::from_wchars_nullt(self.pszDisplay).to_string()
	}

	/// Sets the `pszDisplay` field.
	pub fn set_pszDisplay(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.szDisplay);
	}
}

/// [`NMDATETIMEFORMATQUERY`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimeformatqueryw)
/// struct.
#[repr(C)]
pub struct NMDATETIMEFORMATQUERY<'a> {
	pub nmhdr: NMHDR,
	pszFormat: *mut u16,
	pub szMax: SIZE,

	pszFormat_: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMEFORMATQUERY, 'a);

impl<'a> NMDATETIMEFORMATQUERY<'a> {
	pub_fn_string_ptr_get_set!('a, pszFormat, set_pszFormat);
}

/// [`NMDATETIMESTRING`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimestringw)
/// struct.
#[repr(C)]
pub struct NMDATETIMESTRING<'a> {
	pub nmhdr: NMHDR,
	pszUserString: *mut u16,
	pub st: SYSTEMTIME,
	pub dwFlags: co::GDT,

	pszUserString_: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMESTRING, 'a);

impl<'a> NMDATETIMESTRING<'a> {
	pub_fn_string_ptr_get_set!('a, pszUserString, set_pszUserString);
}

/// [`NMDATETIMEWMKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimewmkeydownw)
/// struct.
#[repr(C)]
pub struct NMDATETIMEWMKEYDOWN<'a> {
	pub nmhdr: NMHDR,
	pub nVirtKey: i32,
	pszFormat: *mut u16,
	pub st: SYSTEMTIME,

	pszFormat_: PhantomData<&'a mut u16>,
}

impl_default!(NMDATETIMEWMKEYDOWN, 'a);

impl<'a> NMDATETIMEWMKEYDOWN<'a> {
	pub_fn_string_ptr_get_set!('a, pszFormat, set_pszFormat);
}

/// [`NMDAYSTATE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdaystate)
/// struct.
#[repr(C)]
pub struct NMDAYSTATE<'a> {
	pub nmhdr: NMHDR,
	pub stStart: SYSTEMTIME,
	cDayState: i32,
	prgDayState: *mut u32,

	prgDayState_: PhantomData<&'a mut u32>,
}

impl_default!(NMDAYSTATE, 'a);

impl<'a> NMDAYSTATE<'a> {
	pub_fn_array_buf_get_set!('a, prgDayState, set_prgDayState, cDayState, u32);
}

/// [`NMITEMACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmitemactivate)
/// struct.
#[repr(C)]
pub struct NMITEMACTIVATE {
	pub hdr: NMHDR,
	pub iItem: i32,
	pub iSubItem: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
	pub uChanged: co::LVIF,
	pub ptAction: POINT,
	pub lParam: isize,
	pub uKeyFlags: co::LVKF,
}

/// [`NMIPADDRESS`](https://docs.microsoft.com/en-us/windows/win32/api/Commctrl/ns-commctrl-nmipaddress)
/// struct.
#[repr(C)]
pub struct NMIPADDRESS {
	pub hdr: NMHDR,
	pub iField: i32,
	pub iValue: i32,
}

/// [`NMLINK`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlink)
/// struct.
#[repr(C)]
pub struct NMLINK {
	pub hdr: NMHDR,
	pub item: LITEM,
}

/// [`NMLISTVIEW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlistview)
/// struct.
#[repr(C)]
pub struct NMLISTVIEW {
	pub hdr: NMHDR,
	pub iItem: i32,
	pub iSubItem: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
	pub uChanged: co::LVIF,
	pub ptAction: POINT,
	pub lParam: isize,
}

/// [`NMLVCACHEHINT`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvcachehint)
/// struct.
#[repr(C)]
pub struct NMLVCACHEHINT {
	pub hdr: NMHDR,
	pub iFrom: i32,
	pub iTo: i32,
}

/// [`NMLVCUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvcustomdraw)
/// struct.
#[repr(C)]
pub struct NMLVCUSTOMDRAW {
	pub mcd: NMCUSTOMDRAW,
	pub clrText: COLORREF,
	pub clrTextBk: COLORREF,
	pub iSubItem: i32,
	pub dwItemType: co::LVCDI,
	pub clrFace: COLORREF,
	pub iIconEffect: i32,
	pub iIconPhase: i32,
	pub iPartId: i32,
	pub iStateId: i32,
	pub rcText: RECT,
	pub uAlign: co::LVGA_HEADER,
}

/// [`NMLVDISPINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvdispinfow)
/// struct.
#[repr(C)]
pub struct NMLVDISPINFO<'a> {
	pub hdr: NMHDR,
	pub item: LVITEM<'a>,
}

/// [`NMLVEMPTYMARKUP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvemptymarkup)
/// struct.
#[repr(C)]
pub struct NMLVEMPTYMARKUP {
	pub hdr: NMHDR,
	pub dwFlags: co::EMF,
	szMarkup: [u16; L_MAX_URL_LENGTH],
}

impl_default!(NMLVEMPTYMARKUP);

impl NMLVEMPTYMARKUP {
	pub_fn_string_arr_get_set!(szMarkup, set_szMarkup);
}

/// [`NMLVFINDITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvfinditemw)
/// struct.
#[repr(C)]
pub struct NMLVFINDITEM<'a> {
	pub hdr: NMHDR,
	pub iStart: i32,
	pub lvfi: LVFINDINFO<'a>,
}

/// [`NMLVGETINFOTIP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvgetinfotipw)
/// struct.
#[repr(C)]
pub struct NMLVGETINFOTIP<'a> {
	pub hdr: NMHDR,
	pub dwFlags: co::LVGIT,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iItem: i32,
	pub iSubItem: i32,
	pub lParam: isize,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(NMLVGETINFOTIP, 'a);

impl<'a> NMLVGETINFOTIP<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`NMLVKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvkeydown)
/// struct.
#[repr(C)]
pub struct NMLVKEYDOWN {
	pub hdr: NMHDR,
	pub wVKey: co::VK,
	flags: u32,
}

impl_default!(NMLVKEYDOWN);

/// [`NMLVLINK`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvlink)
/// struct.
#[repr(C)]
pub struct NMLVLINK {
	pub hdr: NMHDR,
	pub link: LITEM,
	pub iItem: i32,
	pub iSubItem: i32,
}

/// [`NMLVODSTATECHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvodstatechange)
/// struct.
#[repr(C)]
pub struct NMLVODSTATECHANGE {
	pub hdr: NMHDR,
	pub iFrom: i32,
	pub iTo: i32,
	pub uNewState: co::LVIS,
	pub uOldState: co::LVIS,
}

/// [`NMLVSCROLL`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvscroll)
/// struct.
#[repr(C)]
pub struct NMLVSCROLL {
	pub hdr: NMHDR,
	pub dx: i32,
	pub dy: i32,
}

/// [`NMMOUSE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmmouse)
/// struct.
#[repr(C)]
pub struct NMMOUSE {
	pub hdr: NMHDR,
	pub dwItemSpec: usize,
	pub dwItemData: usize,
	pub pt: POINT,
	pub dwHitInfo: isize,
}

/// [`NMTRBTHUMBPOSCHANGING`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtrbthumbposchanging)
/// struct.
#[repr(C)]
pub struct NMTRBTHUMBPOSCHANGING {
	pub hdr: NMHDR,
	pub dwPos: u32,
	pub nReason: co::TB,
}

/// [`NMSELCHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmselchange)
/// struct.
#[repr(C)]
pub struct NMSELCHANGE {
	pub nmhdr: NMHDR,
	pub stSelStart: SYSTEMTIME,
	pub stSelEnd: SYSTEMTIME,
}

/// [`NMTREEVIEW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtreevieww)
/// struct.
#[repr(C)]
pub struct NMTREEVIEW<'a, 'b> {
	pub hdr: NMHDR,
	pub action: u32, // actual type varies
	pub itemOld: TVITEM<'a>,
	pub itemNew: TVITEM<'b>,
	pub ptDrag: POINT,
}

/// [`NMTVASYNCDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvasyncdraw)
/// struct.
#[repr(C)]
pub struct NMTVASYNCDRAW<'a> {
	pub hdr: NMHDR,
	pimldp: *mut IMAGELISTDRAWPARAMS,
	pub hr: co::ERROR,
	pub hItem: HTREEITEM,
	pub lParam: isize,
	pub dwRetFlags: co::ADRF,
	pub iRetImageIndex: i32,

	pimldp_: PhantomData<&'a mut IMAGELISTDRAWPARAMS>,
}

impl_default!(NMTVASYNCDRAW, 'a);

impl<'a> NMTVASYNCDRAW<'a> {
	pub_fn_ptr_get_set!('a, pimldp, set_pimldp, IMAGELISTDRAWPARAMS);
}

/// [`NMTVCUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvcustomdraw)
/// stuct.
#[repr(C)]
pub struct NMTVCUSTOMDRAW {
	pub nmcd: NMCUSTOMDRAW,
	pub clrText: COLORREF,
	pub clrTextBk: COLORREF,
	pub iLevel: i32,
}

/// [`NMTVITEMCHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvitemchange)
/// struct.
#[repr(C)]
pub struct NMTVITEMCHANGE {
	pub hdr: NMHDR,
	pub uChanged: co::TVIF,
	pub hItem: HTREEITEM,
	pub uStateNew: co::TVIS,
	pub uStateOld: co::TVIS,
	pub lParam: isize,
}

/// [`NMVIEWCHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmviewchange)
/// struct.
#[repr(C)]
pub struct NMVIEWCHANGE {
	pub nmhdr: NMHDR,
	pub dwOldView: co::MCMV,
	pub dwNewView: co::MCMV,
}

/// [`PBRANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-pbrange)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct PBRANGE {
	pub iLow: i32,
	pub iHigh: i32,
}

/// [`TBADDBITMAP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tbaddbitmap)
/// struct.
#[repr(C)]
#[derive(Clone)]
pub struct TBADDBITMAP {
	hInst: HINSTANCE,
	nID: usize,
}

impl_default!(TBADDBITMAP);

impl TBADDBITMAP {
	/// Returns the `hInst` and `nID` fields.
	pub fn nID(&self) -> BmpIdbRes {
		match self.hInst {
			HINST_COMMCTRL => BmpIdbRes::Idb(co::IDB(self.nID)),
			HINSTANCE::NULL => BmpIdbRes::Bmp(HBITMAP(self.nID as _ )),
			hInst => BmpIdbRes::Res(IdStr::from_ptr(self.nID as _), hInst),
		}
	}

	/// Sets the `hInst` and `nID` fields.
	pub fn set_nID(&mut self, val: &BmpIdbRes) {
		*self = match val {
			BmpIdbRes::Idb(idb) => Self { hInst: HINST_COMMCTRL, nID: idb.0 },
			BmpIdbRes::Bmp(bmp) => Self { hInst: HINSTANCE::NULL, nID: bmp.0 as _ },
			BmpIdbRes::Res(res, hInst) => Self { hInst: *hInst, nID: res.as_ptr() as _ },
		}
	}
}

/// [`TBBUTTON`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tbbutton)
/// struct.
#[repr(C)]
pub struct TBBUTTON<'a> {
	pub iBitmap: i32,
	pub idCommand: i32,
	pub fsState: co::TBSTATE,
	pub fsStyle: co::BTNS,
	bReserved: [u8; 6], // assumes 64-bit architecture
	pub dwData: usize,
	iString: isize,

	iString_: PhantomData<&'a mut u16>,
}

impl_default!(TBBUTTON, 'a);

impl<'a> TBBUTTON<'a> {
	/// Returns the `iString` field.
	pub fn iString(&self) -> IndexStr {
		if IS_INTRESOURCE(self.iString as _) {
			IndexStr::Index(self.iString as _)
		} else {
			IndexStr::Str(WString::from_wchars_nullt(self.iString as _))
		}
	}

	/// Sets the `iString` field.
	pub fn set_iString(&mut self, val: &'a mut IndexStr) {
		self.iString = match val {
			IndexStr::Index(i) => *i as _,
			IndexStr::Str(s) => unsafe { s.as_mut_ptr() as _ },
		};
	}
}

/// [`TVINSERTSTRUCT`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvinsertstructw)
/// struct.
#[repr(C)]
pub struct TVINSERTSTRUCT<'a> {
	pub hParent: HTREEITEM,
	hInsertAfter: isize,
	pub itemex: TVITEMEX<'a>,
}

impl_default!(TVINSERTSTRUCT, 'a);

impl<'a> TVINSERTSTRUCT<'a> {
	/// Returns the `hInsertAfter` field.
	pub fn hInsertAfter(&self) -> TreeitemTvi {
		TreeitemTvi::from_isize(self.hInsertAfter)
	}

	/// Sets the `hInsertAfter` field.
	pub fn set_hInsertAfter(&mut self, val: TreeitemTvi) {
		self.hInsertAfter = val.as_isize();
	}
}

/// [`TVITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvitemw)
/// struct.
#[repr(C)]
pub struct TVITEM<'a> {
	pub mask: co::TVIF,
	pub hItem: HTREEITEM,
	pub state: co::TVIS,
	pub stateMask: co::TVIS,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub iSelectedImage: i32,
	pub cChildren: i32,
	pub lParam: isize,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(TVITEM, 'a);

impl<'a> TVITEM<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}

/// [`TVITEMEX`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvitemexw)
/// struct.
#[repr(C)]
pub struct TVITEMEX<'a> {
	pub mask: co::TVIF,
	pub hItem: HTREEITEM,
	pub state: co::TVIS,
	pub stateMask: co::TVIS,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iImage: i32,
	pub iSelectedImage: i32,
	pub cChildren: i32,
	pub lParam: isize,
	pub iIntegral: i32,
	pub uStateEx: co::TVIS_EX,
	hwnd: HWND,
	pub iExpandedImage: i32,
	iReserved: i32,

	pszText_: PhantomData<&'a mut u16>,
}

impl_default!(TVITEMEX, 'a);

impl<'a> TVITEMEX<'a> {
	pub_fn_string_buf_get_set!('a, pszText, set_pszText, cchTextMax);
}
