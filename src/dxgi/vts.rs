#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::ole::vts::*;

#[repr(C)]
pub(crate) struct IDXGIAdapterVT {
	pub IDXGIObjectVT: IDXGIObjectVT,
	pub EnumOutputs: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub GetDesc: fn(COMPTR, PVOID) -> HRES,
	pub CheckInterfaceSupport: fn(COMPTR, PCVOID, *mut i64) -> HRES,
}

#[repr(C)]
pub(crate) struct IDXGIAdapter1VT {
	pub IDXGIAdapterVT: IDXGIAdapterVT,
	pub GetDesc1: fn(COMPTR, PVOID) -> HRES,
}

#[repr(C)]
pub(crate) struct IDXGIAdapter2VT {
	pub IDXGIAdapter1VT: IDXGIAdapter1VT,
	pub GetDesc2: fn(COMPTR, PVOID) -> HRES,
}

#[repr(C)]
pub(crate) struct IDXGIDeviceVT {
	pub IDXGIObjectVT: IDXGIObjectVT,
	pub GetAdapter: fn(COMPTR, *mut COMPTR) -> HRES,
	pub CreateSurface: fn(COMPTR, PCVOID, u32, u32, PCVOID, *mut COMPTR) -> HRES,
	pub QueryResourceResidency: fn(COMPTR, COMPTR, *mut u32, u32) -> HRES,
	pub SetGPUThreadPriority: fn(COMPTR, i32) -> HRES,
	pub GetGPUThreadPriority: fn(COMPTR, *mut i32) -> HRES,
}

#[repr(C)]
pub(crate) struct IDXGIDeviceSubObjectVT {
	pub IDXGIObjectVT: IDXGIObjectVT,
	pub GetDevice: fn(COMPTR, PCVOID, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub(crate) struct IDXGIFactoryVT {
	pub IDXGIObjectVT: IDXGIObjectVT,
	pub EnumAdapters: fn(COMPTR, u32, *const COMPTR) -> HRES,
	pub MakeWindowAssociation: fn(COMPTR, HANDLE, u32) -> HRES,
	pub GetWindowAssociation: fn(COMPTR, *mut HANDLE) -> HRES,
	pub CreateSwapChain: fn(COMPTR, COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub CreateSoftwareAdapter: fn(COMPTR, HANDLE, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub(crate) struct IDXGIFactory1VT {
	pub IDXGIFactoryVT: IDXGIFactoryVT,
	pub EnumAdapters1: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub IsCurrent: fn(COMPTR) -> BOOL,
}

#[repr(C)]
pub(crate) struct IDXGIKeyedMutexVT {
	pub IDXGIDeviceSubObjectVT: IDXGIDeviceSubObjectVT,
	pub AcquireSync: fn(COMPTR, u64, u32) -> HRES,
	pub ReleaseSync: fn(COMPTR, u64) -> HRES,
}

#[repr(C)]
pub(crate) struct IDXGIObjectVT {
	pub IUnknownVT: IUnknownVT,
	pub SetPrivateData: fn(COMPTR, PCVOID, u32, PCVOID) -> HRES,
	pub SetPrivateDataInterface: fn(COMPTR, PCVOID, COMPTR) -> HRES,
	pub GetPrivateData: fn(COMPTR, PCVOID, *mut u32, PVOID) -> HRES,
	pub GetParent: fn(COMPTR, PCVOID, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub(crate) struct IDXGIOutputVT {
	pub IDXGIObjectVT: IDXGIObjectVT,
	pub GetDesc: fn(COMPTR, PVOID) -> HRES,
	pub GetDisplayModeList: fn(COMPTR, u32, u32, *mut u32, PVOID) -> HRES,
	pub FindClosestMatchingMode: fn(COMPTR, PCVOID, PVOID, COMPTR) -> HRES,
	pub WaitForVBlank: fn(COMPTR) -> HRES,
	pub TakeOwnership: fn(COMPTR, COMPTR, BOOL) -> HRES,
	pub ReleaseOwnership: fn(COMPTR),
	pub GetGammaControlCapabilities: fn(COMPTR, PVOID) -> HRES,
	pub SetGammaControl: fn(COMPTR, PCVOID) -> HRES,
	pub GetGammaControl: fn(COMPTR, PVOID) -> HRES,
	pub SetDisplaySurface: fn(COMPTR, COMPTR) -> HRES,
	pub GetDisplaySurfaceData: fn(COMPTR, COMPTR) -> HRES,
	pub GetFrameStatistics: fn(COMPTR, PVOID) -> HRES,
}

#[repr(C)]
pub(crate) struct IDXGIResourceVT {
	pub IDXGIDeviceSubObjectVT: IDXGIDeviceSubObjectVT,
	pub GetSharedHandle: fn(COMPTR, *mut HANDLE) -> HRES,
	pub GetUsage: fn(COMPTR, *mut u32) -> HRES,
	pub SetEvictionPriority: fn(COMPTR, u32) -> HRES,
	pub GetEvictionPriority: fn(COMPTR, *mut u32) -> HRES,
}

#[repr(C)]
pub(crate) struct IDXGISurfaceVT {
	pub IDXGIDeviceSubObjectVT: IDXGIDeviceSubObjectVT,
	pub GetDesc: fn(COMPTR, PVOID) -> HRES,
	pub Map: fn(COMPTR, PVOID, u32) -> HRES,
	pub Unmap: fn(COMPTR) -> HRES,
}

#[repr(C)]
pub(crate) struct IDXGISwapChainVT {
	pub IDXGIDeviceSubObjectVT: IDXGIDeviceSubObjectVT,
	pub Present: fn(COMPTR, u32, u32) -> HRES,
	pub GetBuffer: fn(COMPTR, u32, PCVOID, *mut COMPTR) -> HRES,
	pub SetFullscreenState: fn(COMPTR, BOOL, COMPTR) -> HRES,
	pub GetFullscreenState: fn(COMPTR, *mut BOOL, *mut COMPTR) -> HRES,
	pub GetDesc: fn(COMPTR, PVOID) -> HRES,
	pub ResizeBuffers: fn(COMPTR, u32, u32, u32, u32, u32) -> HRES,
	pub ResizeTarget: fn(COMPTR, PCVOID) -> HRES,
	pub GetContainingOutput: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetFrameStatistics: fn(COMPTR, PVOID) -> HRES,
	pub GetLastPresentCount: fn(COMPTR, *mut u32) -> HRES,
}
