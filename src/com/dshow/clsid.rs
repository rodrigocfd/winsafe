//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! COM class IDs.

#![allow(non_upper_case_globals)]

use crate::structs::CLSID;

pub_const_guid! { CLSID,
	EnhancedVideoRenderer, 0xfa10746c, 0x9b63, 0x4b6c, 0xbc49, 0xfc300ea5f256,
	FilterGraph, 0xe436ebb3, 0x524f, 0x11ce, 0x9f53, 0x0020af0ba770,
}
