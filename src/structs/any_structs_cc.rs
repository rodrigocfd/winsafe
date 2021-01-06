//! Assorted Win32 structs used by common controls.

#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::funcs_priv::L_MAX_URL_LENGTH;
use crate::handles::{HDC, HIMAGELIST, HTREEITEM};
use crate::structs::{COLORREF, NMHDR, POINT, RECT};
use crate::WString;

/// [`IMAGELISTDRAWPARAMS`](https://docs.microsoft.com/en-us/windows/win32/api/commoncontrols/ns-commoncontrols-imagelistdrawparams)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
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

/// [`LVCOLUMN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvcolumnw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
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
	/// Sets the `pszText` and `cchTextMax` fields. The buffer will be resized to
	/// hold at least 64 chars.
	pub fn set_pszText(&mut self, buf: &'a mut WString) {
		if buf.buffer_size() < 64 { buf.realloc_buffer(64); } // arbitrary
		self.pszText = unsafe { buf.as_mut_ptr() };
		self.cchTextMax = buf.buffer_size() as i32;
	}
}

/// [`LVFINDINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvfindinfow)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct LVFINDINFO<'a> {
	pub flags: co::LVFI,
	psz: *const u16,
	pub lParam: isize,
	pub pt: POINT,
	pub vkDirection: co::VK,
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
#[derive(Clone, Eq, PartialEq)]
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
	/// Sets the `pszText` and `cchTextMax` fields. The buffer will be resized to
	/// hold at least 64 chars.
	pub fn set_pszText(&mut self, buf: &'a mut WString) {
		if buf.buffer_size() < 64 { buf.realloc_buffer(64); } // arbitrary
		self.pszText = unsafe { buf.as_mut_ptr() };
		self.cchTextMax = buf.buffer_size() as i32;
	}
}

/// [`NMBCDROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbcdropdown)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMBCDROPDOWN {
	pub hdr: NMHDR,
	pub rcButton: RECT,
}

/// [`NMBCHOTITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbchotitem)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMBCHOTITEM {
	pub hdr: NMHDR,
	pub dwFlags: co::HICF,
}

/// [`NMCHAR`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmchar)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMCHAR {
	pub hdr: NMHDR,
	pub ch: u32,
	pub dwItemPrev: u32,
	pub dwItemNext: u32,
}

/// [`NMCUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmcustomdraw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMCUSTOMDRAW {
	pub hdr: NMHDR,
	pub dwDrawStage: co::CDDS,
	pub hdc: HDC,
	pub rc: RECT,
	pub dwItemSpec: usize,
	pub uItemState: co::CDIS,
	pub lItemlParam: isize,
}

impl_default_zero!(NMCUSTOMDRAW);

/// [`NMITEMACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmitemactivate)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
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

/// [`NMLISTVIEW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlistview)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
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

/// [`NMLVDISPINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvdispinfow)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMLVDISPINFO<'a> {
	pub hdr: NMHDR,
	pub item: LVITEM<'a>,
}

/// [`NMLVEMPTYMARKUP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvemptymarkup)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMLVEMPTYMARKUP {
	pub hdr: NMHDR,
	pub dwFlags: co::EMF,
	pub szMarkup: [u16; L_MAX_URL_LENGTH],
}

impl_default_zero!(NMLVEMPTYMARKUP);

/// [`NMLVFINDITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvfinditemw)
/// struct.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMLVFINDITEM<'a> {
	pub hdr: NMHDR,
	pub iStart: i32,
	pub lvfi: LVFINDINFO<'a>,
}

/// [`NMLVGETINFOTIP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvgetinfotipw)
/// struct.
#[derive(Clone, Eq, PartialEq)]
pub struct NMLVGETINFOTIP<'a> {
	pub hdr: NMHDR,
	pub dwFlags: co::LVGIT,
	pszText: *mut u16,
	cchTextMax: i32,
	pub iItem: i32,
	pub iSubItem: i32,
	pub lParam: isize,
	m_pszText: PhantomData<&'a u16>,
}

impl_default_zero!(NMLVGETINFOTIP, 'a);

impl<'a> NMLVGETINFOTIP<'a> {
	/// Returns the `pszText` field.
	pub fn pszText(&self) -> String {
		WString::from_wchars_nullt(self.pszText).to_string()
	}

	/// Sets the `pszText` and `cchTextMax` fields. The buffer will be resized to
	/// hold at least 64 chars.
	pub fn set_pszText(&mut self, buf: &'a mut WString) {
		if buf.buffer_size() < 64 { buf.realloc_buffer(64); } // arbitrary
		self.pszText = unsafe { buf.as_mut_ptr() };
		self.cchTextMax = buf.buffer_size() as i32;
	}
}

/// [`NMLVSCROLL`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvscroll)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMLVSCROLL {
	pub hdr: NMHDR,
	pub dx: i32,
	pub dy: i32,
}

/// [`NMMOUSE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmmouse)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Eq, PartialEq)]
pub struct NMMOUSE {
	pub hdr: NMHDR,
	pub dwItemSpec: usize,
	pub dwItemData: usize,
	pub pt: POINT,
	pub dwHitInfo: isize,
}

/// [`NMTVASYNCDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvasyncdraw)
/// method.
#[repr(C)]
#[derive(Clone, Eq, PartialEq)]
pub struct NMTVASYNCDRAW<'a> {
	pub hdr: NMHDR,
	pimldp: *const IMAGELISTDRAWPARAMS,
	pub hr: co::ERROR,
	pub hItem: HTREEITEM,
	pub lParam: isize,
	pub dwRetFlags: co::ADRF,
	pub iRetImageIndex: i32,
	m_pimldp: PhantomData<&'a IMAGELISTDRAWPARAMS>,
}

impl_default_zero!(NMTVASYNCDRAW, 'a);

impl<'a> NMTVASYNCDRAW<'a> {
	/// Returns the `pimldp` field.
	pub fn pimldp(&self) -> &IMAGELISTDRAWPARAMS {
		unsafe { &*self.pimldp }
	}
}
