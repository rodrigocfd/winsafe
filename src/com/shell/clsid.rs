//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM class IDs.

#![allow(non_upper_case_globals)]

use crate::structs::CLSID;

pub_const_guid! { CLSID,
	FileOpenDialog, 0xdc1c5a9c, 0xe88a, 0x4dde, 0xa5a1, 0x60f82a20aef7,
	FileSaveDialog, 0xc0b4e2f3, 0xba21, 0x4773, 0x8dba, 0x335ec946eb8b,
	TaskbarList, 0x56fdf344, 0xfd6d, 0x11d0, 0x958a, 0x006097c9a090,
}
