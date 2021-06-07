//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! COM class IDs.

#![allow(non_upper_case_globals)]

use crate::structs::CLSID;

pub_const_guid! { CLSID,
	AviDest, 0xe2510970, 0xf137, 0x11ce, 0x8b67, 0x00aa00a3f1a6,
	EnhancedVideoRenderer, 0xfa10746c, 0x9b63, 0x4b6c, 0xbc49, 0xfc300ea5f256,
	FileWriter, 0x8596e5f0, 0x0da5, 0x11d0, 0xbd21, 0x00a0c911ce86,
	FilterGraph, 0xe436ebb3, 0x524f, 0x11ce, 0x9f53, 0x0020af0ba770,
	NullRenderer, 0xc1f400a4, 0x3f08, 0x11d3, 0x9f0b, 0x006008039e37,
}
