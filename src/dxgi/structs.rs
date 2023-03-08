#![allow(non_snake_case)]

use crate::kernel::decl::LUID;
use crate::kernel::ffi_types::BOOL;

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

/// [`DXGI_FRAME_STATISTICS`](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_frame_statistics)
/// struct.
#[repr(C)]
#[derive(Default, PartialEq, Eq)]
pub struct DXGI_FRAME_STATISTICS {
	pub PresentCount: u32,
	pub PresentRefreshCount: u32,
	pub SyncRefreshCount: u32,
	pub SyncQPCTime: i64,
	pub SyncGPUTime: i64,
}

/// [`DXGI_GAMMA_CONTROL`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb173061(v=vs.85))
/// struct.
#[repr(C)]
pub struct DXGI_GAMMA_CONTROL {
	pub Scale: DXGI_RGB,
	pub Offset: DXGI_RGB,
	pub GammaCurve: [DXGI_RGB; 1025],
}

impl_default!(DXGI_GAMMA_CONTROL);

/// [`DXGI_GAMMA_CONTROL_CAPABILITIES`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/dxgitype/ns-dxgitype-dxgi_gamma_control_capabilities)
/// struct.
#[repr(C)]
pub struct DXGI_GAMMA_CONTROL_CAPABILITIES {
	ScaleAndOffsetSupported: BOOL,
	pub MaxConvertedValue: f32,
	pub MinConvertedValue: f32,
	pub NumGammaControlPoints: u32,
	pub ControlPointPositions: [f32; 1025],
}

impl_default!(DXGI_GAMMA_CONTROL_CAPABILITIES);

impl DXGI_GAMMA_CONTROL_CAPABILITIES {
	pub_fn_bool_get_set!(ScaleAndOffsetSupported, set_ScaleAndOffsetSupported);
}

/// [`DXGI_RGB`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb173071(v=vs.85))
/// struct.
#[repr(C)]
#[derive(Default, PartialEq)]
pub struct DXGI_RGB {
	pub Red: f32,
	pub Green: f32,
	pub Blue: f32,
}
