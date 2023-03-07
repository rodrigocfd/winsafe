#![allow(non_snake_case)]

use crate::kernel::decl::LUID;

/// [`DXGI_ADAPTER_DESC`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_adapter_desc)
/// struct.
#[repr(C)]
pub struct DXGI_ADAPTER_DESC {
	Description: [u16; 128],
	pub VendorId: u32,
	pub DeviceId: u32,
	pub SubSysId: u32,
	pub Revision: u32,
	pub DedicatedVideoMemory: usize,
	pub DedicatedSystemMemory: usize,
	pub SharedSystemMemory: usize,
	pub AdapterLuid: LUID,
}

impl_default!(DXGI_ADAPTER_DESC);

impl DXGI_ADAPTER_DESC {
	pub_fn_string_arr_get_set!(Description, set_Description);
}
