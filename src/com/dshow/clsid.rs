//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! COM class IDs.

#![allow(non_upper_case_globals)]

use crate::structs::{CLSID, GUID};

pub const EnhancedVideoRenderer: CLSID = CLSID::new(0xfa10746c, 0x9b63, 0x4b6c, 0xbc49, 0xfc300ea5f256);
pub const FilterGraph: CLSID = CLSID::new(0xe436ebb3, 0x524f, 0x11ce, 0x9f53, 0x0020af0ba770);

pub const MR_VIDEO_RENDER_SERVICE: GUID = GUID::new(0x1092a86c, 0xab1a, 0x459a, 0xa336, 0x831fbc4d11ff);
pub const MR_VIDEO_MIXER_SERVICE: GUID = GUID::new(0x073cd2fc, 0x6cf4, 0x40b7, 0x8859, 0xe89552c841f8);
pub const MR_VIDEO_ACCELERATION_SERVICE: GUID = GUID::new(0xefef5175, 0x5c7d, 0x4ce2, 0xbbbd, 0x34ff8bca6554);
pub const MR_BUFFER_SERVICE: GUID = GUID::new(0xa562248c, 0x9ac6, 0x4ffc, 0x9fba, 0x3af8f8ad1a4d);
pub const VIDEO_ZOOM_RECT: GUID = GUID::new(0x7aaa1638, 0x1b7f, 0x4c93, 0xbd89, 0x5b9c9fb6fcf0);
