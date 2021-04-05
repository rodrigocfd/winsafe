//! Assorted Win32 structs used by common controls.

#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::handles::{HDC, HIMAGELIST, HTREEITEM, HWND};
use crate::privs::{L_MAX_URL_LENGTH, MAX_LINKID_TEXT};
use crate::structs::{COLORREF, NMHDR, POINT, RECT, SIZE, SYSTEMTIME};
use crate::WString;

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

impl Default for DATETIMEPICKERINFO {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.cbSize = std::mem::size_of::<Self>() as u32;
		obj
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

impl Default for IMAGELISTDRAWPARAMS {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.cbSize = std::mem::size_of::<Self>() as u32;
		obj
	}
}

/// [`LITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-litem)
/// struct.
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct LITEM {
	pub mask: co::LIF,
	pub iLink: i32,
	pub state: co::LIS,
	pub stateMask: co::LIS,
	szID: [u16; MAX_LINKID_TEXT],
	szUrl: [u16; L_MAX_URL_LENGTH],
}

impl LITEM {
	/// Returns the `szID` field.
	pub fn szID(&self) -> String {
		WString::from_wchars_slice(&self.szID).to_string()
	}

	/// Sets the `szID` field.
	pub fn get_szID(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.szID);
	}

	/// Returns the `szUrl` field.
	pub fn szUrl(&self) -> String {
		WString::from_wchars_slice(&self.szUrl).to_string()
	}

	/// Sets the `szUrl` field.
	pub fn set_szUrl(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.szUrl);
	}
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
	m_pszText: PhantomData<&'a u16>,
}

impl_default_zero!(LVCOLUMN, 'a);

impl<'a> LVCOLUMN<'a> {
	/// Returns the `pszText` field.
	pub fn pszText(&self) -> String {
		WString::from_wchars_nullt(self.pszText).to_string()
	}

	/// Sets the `pszText` field.
	pub fn set_pszText(&mut self, buf: &'a mut WString) {
		self.pszText = unsafe { buf.as_mut_ptr() };
		self.cchTextMax = buf.buffer_size() as i32;
	}
}

/// [`LVFINDINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvfindinfow)
/// struct.
#[repr(C)]
pub struct LVFINDINFO<'a> {
	pub flags: co::LVFI,
	psz: *const u16,
	pub lParam: isize,
	pub pt: POINT,
	pub vkDirection: co::VK_DIR,
	m_psz: PhantomData<&'a u16>,
}

impl_default_zero!(LVFINDINFO, 'a);

impl<'a> LVFINDINFO<'a> {
	/// Returns the `psz` field.
	pub fn psz(&self) -> String {
		WString::from_wchars_nullt(self.psz).to_string()
	}

	/// Sets the `psz` field.
	pub fn set_psz(&mut self, buf: &'a WString) {
		self.psz = unsafe { buf.as_ptr() };
	}
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
	m_pszText: PhantomData<&'a u16>,
}

impl_default_zero!(LVITEM, 'a);

impl<'a> LVITEM<'a> {
	/// Returns the `pszText` field.
	pub fn pszText(&self) -> String {
		WString::from_wchars_nullt(self.pszText).to_string()
	}

	/// Sets the `pszText` field.
	pub fn set_pszText(&mut self, buf: &'a mut WString) {
		self.pszText = unsafe { buf.as_mut_ptr() };
		self.cchTextMax = buf.buffer_size() as i32;
	}
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
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct NMDATETIMEFORMAT {
	pub nmhdr: NMHDR,
	pszFormat: *const u16,
	pub st: SYSTEMTIME,
	pszDisplay: *const u16,
	szDisplay: [u16; 64], // used as a buffer to pszDisplay
}

impl NMDATETIMEFORMAT {
	/// Returns the `pszFormat` field.
	pub fn pszFormat(&self) -> String {
		WString::from_wchars_nullt(self.pszFormat).to_string()
	}

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
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct NMDATETIMEFORMATQUERY {
	pub nmhdr: NMHDR,
	pszFormat: *const u16,
	pub szMax: SIZE,
}

impl NMDATETIMEFORMATQUERY {
	/// Returns the `pszFormat` field.
	pub fn pszFormat(&self) -> String {
		WString::from_wchars_nullt(self.pszFormat).to_string()
	}
}

/// [`NMDATETIMESTRING`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimestringw)
/// struct.
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct NMDATETIMESTRING {
	pub nmhdr: NMHDR,
	pszUserString: *const u16,
	pub st: SYSTEMTIME,
	pub dwFlags: co::GDT,
}

impl NMDATETIMESTRING {
	/// Returns the `pszUserString` field.
	pub fn pszUserString(&self) -> String {
		WString::from_wchars_nullt(self.pszUserString).to_string()
	}
}

/// [`NMDATETIMEWMKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdatetimewmkeydownw)
/// struct.
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct NMDATETIMEWMKEYDOWN {
	pub nmhdr: NMHDR,
	pub nVirtKey: i32,
	pszFormat: *const u16,
	pub st: SYSTEMTIME,
}

impl NMDATETIMEWMKEYDOWN {
	/// Returns the `pszFormat` field.
	pub fn pszFormat(&self) -> String {
		WString::from_wchars_nullt(self.pszFormat).to_string()
	}
}

/// [`NMDAYSTATE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmdaystate)
/// struct.
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct NMDAYSTATE {
	pub nmhdr: NMHDR,
	pub stStart: SYSTEMTIME,
	cDayState: i32,
	prgDayState: *mut u32,
}

impl NMDAYSTATE {
	/// Returns the `prgDayState` field.
	pub fn prgDayState(&mut self) -> &mut [u32] {
		unsafe {
			std::slice::from_raw_parts_mut(
				self.prgDayState,
				self.cDayState as usize,
			)
		}
	}
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

/// [`NMLVDISPINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvdispinfow)
/// struct.
#[repr(C)]
pub struct NMLVDISPINFO<'a> {
	pub hdr: NMHDR,
	pub item: LVITEM<'a>,
}

/// [`NMLVEMPTYMARKUP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvemptymarkup)
/// struct.
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct NMLVEMPTYMARKUP {
	pub hdr: NMHDR,
	pub dwFlags: co::EMF,
	szMarkup: [u16; L_MAX_URL_LENGTH],
}

impl NMLVEMPTYMARKUP {
	/// Returns the `szMarkup` field.
	pub fn szMarkup(&self) -> String {
		WString::from_wchars_slice(&self.szMarkup).to_string()
	}

	/// Sets the `szMarkup` field.
	pub fn get_szID(&mut self, text: &str) {
		WString::from_str(text).copy_to_slice(&mut self.szMarkup);
	}
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
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct NMLVGETINFOTIP {
	pub hdr: NMHDR,
	pub dwFlags: co::LVGIT,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iItem: i32,
	pub iSubItem: i32,
	pub lParam: isize,
}

impl NMLVGETINFOTIP {
	/// Returns the `pszText` field.
	pub fn pszText(&self) -> String {
		WString::from_wchars_nullt(self.pszText).to_string()
	}

	/// Sets the `pszText` field.
	pub fn get_pszText(&mut self, text: &str) {
		WString::from_str(text)
			.copy_to_pointer(self.pszText, self.cchTextMax as usize);
	}
}

/// [`NMLVKEYDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvkeydown)
/// struct.
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct NMLVKEYDOWN {
	pub hdr: NMHDR,
	pub wVKey: co::VK,
	flags: u32,
}

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

/// [`NMSELCHANGE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmselchange)
/// struct.
#[repr(C)]
pub struct NMSELCHANGE {
	pub nmhdr: NMHDR,
	pub stSelStart: SYSTEMTIME,
	pub stSelEnd: SYSTEMTIME,
}

/// [`NMTVASYNCDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvasyncdraw)
/// struct.
///
/// You cannot directly instantiate this object.
#[repr(C)]
pub struct NMTVASYNCDRAW {
	pub hdr: NMHDR,
	pimldp: *const IMAGELISTDRAWPARAMS,
	pub hr: co::ERROR,
	pub hItem: HTREEITEM,
	pub lParam: isize,
	pub dwRetFlags: co::ADRF,
	pub iRetImageIndex: i32,
}

impl NMTVASYNCDRAW {
	/// Returns the `pimldp` field.
	pub fn pimldp(&self) -> &IMAGELISTDRAWPARAMS {
		unsafe { &*self.pimldp }
	}
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
