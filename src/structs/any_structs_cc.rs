//! Assorted Win32 structs used by common controls.

#![allow(non_snake_case)]

use crate::co;
use crate::structs::{NMHDR, POINT};

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