#![allow(non_camel_case_types)]

use crate::co;
use crate::decl::*;

/// [`DEV_BROADCAST_DEVICEINTERFACE`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_deviceinterface_w)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DEV_BROADCAST_DEVICEINTERFACE {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbcc_classguid: GUID,
	dbcc_name: [u16; 1],
}

impl DEV_BROADCAST_DEVICEINTERFACE {
	/// Returns the `dbcc_name` field.
	#[must_use]
	pub fn dbcc_name(&self) -> String {
		unsafe { WString::from_wchars_nullt(self.dbcc_name.as_ptr()) }
			.to_string()
	}
}

/// [`DEV_BROADCAST_HANDLE`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_handle)
/// struct.
#[repr(C)]
pub struct DEV_BROADCAST_HANDLE {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbch_handle: usize,
	pub dbch_hdevnotify: usize, // HDEVNOTIFY
	pub dbch_eventguid: GUID,
	pub dbch_nameoffset: i16,
	pub dbch_data: [u8; 1],
}

/// [`DEV_BROADCAST_HDR`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_hdr)
/// struct.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct DEV_BROADCAST_HDR {
	pub dbch_size: u32,
	pub dbch_devicetype: co::DBT_DEVTYP,
	dbch_reserved: u32,
}

/// [`DEV_BROADCAST_OEM`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_oem)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DEV_BROADCAST_OEM {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbco_identifier: u32,
	pub dbco_suppfunc: u32,
}

/// [`DEV_BROADCAST_PORT`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_port_w)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DEV_BROADCAST_PORT {
	pub hdr: DEV_BROADCAST_HDR,
	dbcp_name: [u16; 1],
}

impl DEV_BROADCAST_PORT {
	/// Returns the `dbcp_name` field.
	#[must_use]
	pub fn dbcp_name(&self) -> String {
		unsafe { WString::from_wchars_nullt(self.dbcp_name.as_ptr()) }
			.to_string()
	}
}

/// [`DEV_BROADCAST_VOLUME`](https://learn.microsoft.com/en-us/windows/win32/api/dbt/ns-dbt-dev_broadcast_volume)
/// struct.
#[derive(Default)]
pub struct DEV_BROADCAST_VOLUME {
	pub hdr: DEV_BROADCAST_HDR,
	pub dbcv_unitmask: u32,
	pub dbcv_flags: co::DBTF,
}
