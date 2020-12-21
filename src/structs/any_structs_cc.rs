//! Assorted Win32 structs used by common controls.

#![allow(non_snake_case)]

use crate::co;
use crate::handles::HDC;
use crate::internal_defs::L_MAX_URL_LENGTH;
use crate::structs::{NMHDR, POINT, RECT};

/// [`LVFINDINFO`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvfindinfow)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LVFINDINFO {
	pub flags: co::LVFI,
	pub psz: *const u16,
	pub lParam: isize,
	pub pt: POINT,
	pub vkDirection: co::VK,
}

impl Default for LVFINDINFO {
	fn default() -> Self {
		Self {
			psz: std::ptr::null(),
			..Default::default()
		}
	}
}

/// [`LVITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-lvitemw)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LVITEM {
	pub mask: co::LVIF,
	pub iItem: i32,
	pub iSubItem: i32,
	pub state: co::LVIS,
	pub stateMask: co::LVIS,
	pub pszText: *mut u16,
	pub cchTextMax: i32,
	pub iImage: i32,
	pub lParam: isize,
	pub iIndent: i32,
	pub iGroupId: co::LVI_GROUPID,
	pub cColumns: u32,
	pub puColumns: *mut i32,
	pub piColFmt: *mut co::LVCFMT,
	pub iGroup: i32,
}

impl Default for LVITEM {
	fn default() -> Self {
		Self {
			pszText: std::ptr::null_mut(),
			puColumns: std::ptr::null_mut(),
			piColFmt: std::ptr::null_mut(),
			..Default::default()
		}
	}
}

/// [`NMBCDROPDOWN`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbcdropdown)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct NMBCDROPDOWN {
	pub hdr: NMHDR,
	pub rcButton: RECT,
}

/// [`NMBCHOTITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmbchotitem)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct NMBCHOTITEM{
	pub hdr: NMHDR,
	pub dwFlags: co::HICF,
}

/// [`NMCHAR`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmchar)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct NMCHAR {
	pub hdr: NMHDR,
	pub ch: u32,
	pub dwItemPrev: u32,
	pub dwItemNext: u32,
}

/// [`NMCUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmcustomdraw)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct NMCUSTOMDRAW {
	pub hdr: NMHDR,
	pub dwDrawStage: co::CDDS,
	pub hdc: HDC,
	pub rc: RECT,
	pub dwItemSpec: usize,
	pub uItemState: co::CDIS,
	pub lItemlParam: isize,
}

/// [`NMITEMACTIVATE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmitemactivate)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
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
#[derive(Default, Copy, Clone, Eq, PartialEq)]
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
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct NMLVDISPINFO {
	pub hdr: NMHDR,
	pub item: LVITEM,
}

/// [`NMLVEMPTYMARKUP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvemptymarkup)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NMLVEMPTYMARKUP {
	pub hdr: NMHDR,
	pub dwFlags: co::EMF,
	pub szMarkup: [u16; L_MAX_URL_LENGTH],
}

impl Default for NMLVEMPTYMARKUP {
	fn default() -> Self {
		Self { // https://stackoverflow.com/a/30949671/6923555
			hdr: NMHDR::default(),
			dwFlags: co::EMF::default(),
			szMarkup: [0u16; L_MAX_URL_LENGTH],
		}
	}
}

/// [`NMLVFINDITEM`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvfinditemw)
/// struct.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NMLVFINDITEM {
	pub hdr: NMHDR,
	pub iStart: i32,
	pub lvfi: LVFINDINFO,
}

/// [`NMLVGETINFOTIP`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvgetinfotipw)
/// struct.
pub struct NMLVGETINFOTIP {
	pub hdr: NMHDR,
	pub dwFlags: co::LVGIT,
	pub pszText: *mut u16,
	pub cchTextMax: i32,
	pub iItem: i32,
	pub iSubItem: i32,
	pub lParam: isize,
}

/// [`NMLVSCROLL`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmlvscroll)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct NMLVSCROLL {
	pub hdr: NMHDR,
	pub dx: i32,
	pub dy: i32,
}

/// [`NMMOUSE`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmmouse)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct NMMOUSE {
	pub hdr: NMHDR,
	pub dwItemSpec: usize,
	pub dwItemData: usize,
	pub pt: POINT,
	pub dwHitInfo: isize,
}