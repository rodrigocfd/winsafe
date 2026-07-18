#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::macros::*;
use crate::ole::vts::*;

com_vtbl! { IDXGIAdapterVT : IDXGIObjectVT
	EnumOutputs(u32, *mut COMPTR) -> HRES
	GetDesc(PVOID) -> HRES
	CheckInterfaceSupport(PCVOID, *mut i64) -> HRES
}

com_vtbl! { IDXGIAdapter1VT : IDXGIAdapterVT
	GetDesc1(PVOID) -> HRES
}

com_vtbl! { IDXGIAdapter2VT : IDXGIAdapter1VT
	GetDesc2(PVOID) -> HRES
}

com_vtbl! { IDXGIDeviceVT : IDXGIObjectVT
	GetAdapter(*mut COMPTR) -> HRES
	CreateSurface(PCVOID, u32, u32, PCVOID, *mut COMPTR) -> HRES
	QueryResourceResidency(COMPTR, *mut u32, u32) -> HRES
	SetGPUThreadPriority(i32) -> HRES
	GetGPUThreadPriority(*mut i32) -> HRES
}

com_vtbl! { IDXGIDeviceSubObjectVT : IDXGIObjectVT
	GetDevice(PCVOID, *mut COMPTR) -> HRES
}

com_vtbl! { IDXGIFactoryVT : IDXGIObjectVT
	EnumAdapters(u32, *const COMPTR) -> HRES
	MakeWindowAssociation(HANDLE, u32) -> HRES
	GetWindowAssociation(*mut HANDLE) -> HRES
	CreateSwapChain(COMPTR, PCVOID, *mut COMPTR) -> HRES
	CreateSoftwareAdapter(HANDLE, *mut COMPTR) -> HRES
}

com_vtbl! { IDXGIFactory1VT : IDXGIFactoryVT
	EnumAdapters1(u32, *mut COMPTR) -> HRES
	IsCurrent() -> BOOL
}

com_vtbl! { IDXGIFactory2VT : IDXGIFactory1VT
	IsWindowedStereoEnabled() -> BOOL
	CreateSwapChainForHwnd(COMPTR, HANDLE, PCVOID, PCVOID, COMPTR, *mut COMPTR) -> HRES
	CreateSwapChainForCoreWindow(COMPTR, COMPTR, PCVOID, COMPTR, *mut COMPTR) -> HRES
	GetSharedResourceAdapterLuid(HANDLE, PVOID) -> HRES
	RegisterStereoStatusWindow(HANDLE, u32, *mut u32) -> HRES
	RegisterStereoStatusEvent(HANDLE, *mut u32) -> HRES
	UnregisterStereoStatus(u32) -> HRES
	RegisterOcclusionStatusWindow(HANDLE, u32, *mut u32) -> HRES
	RegisterOcclusionStatusEvent(HANDLE, *mut u32) -> HRES
	UnregisterOcclusionStatus(u32) -> HRES
	CreateSwapChainForComposition(COMPTR, PCVOID, COMPTR, *mut COMPTR) -> HRES
}

com_vtbl! { IDXGIKeyedMutexVT : IDXGIDeviceSubObjectVT
	AcquireSync(u64, u32) -> HRES
	ReleaseSync(u64) -> HRES
}

com_vtbl! { IDXGIObjectVT : IUnknownVT
	SetPrivateData(PCVOID, u32, PCVOID) -> HRES
	SetPrivateDataInterface(PCVOID, COMPTR) -> HRES
	GetPrivateData(PCVOID, *mut u32, PVOID) -> HRES
	GetParent(PCVOID, *mut COMPTR) -> HRES
}

com_vtbl! { IDXGIOutputVT : IDXGIObjectVT
	GetDesc(PVOID) -> HRES
	GetDisplayModeList(u32, u32, *mut u32, PVOID) -> HRES
	FindClosestMatchingMode(PCVOID, PVOID, COMPTR) -> HRES
	WaitForVBlank() -> HRES
	TakeOwnership(COMPTR, BOOL) -> HRES
	ReleaseOwnership()
	GetGammaControlCapabilities(PVOID) -> HRES
	SetGammaControl(PCVOID) -> HRES
	GetGammaControl(PVOID) -> HRES
	SetDisplaySurface(COMPTR) -> HRES
	GetDisplaySurfaceData(COMPTR) -> HRES
	GetFrameStatistics(PVOID) -> HRES
}

com_vtbl! { IDXGIResourceVT : IDXGIDeviceSubObjectVT
	GetSharedHandle(*mut HANDLE) -> HRES
	GetUsage(*mut u32) -> HRES
	SetEvictionPriority(u32) -> HRES
	GetEvictionPriority(*mut u32) -> HRES
}

com_vtbl! { IDXGISurfaceVT : IDXGIDeviceSubObjectVT
	GetDesc(PVOID) -> HRES
	Map(PVOID, u32) -> HRES
	Unmap() -> HRES
}

com_vtbl! { IDXGISwapChainVT : IDXGIDeviceSubObjectVT
	Present(u32, u32) -> HRES
	GetBuffer(u32, PCVOID, *mut COMPTR) -> HRES
	SetFullscreenState(BOOL, COMPTR) -> HRES
	GetFullscreenState(*mut BOOL, *mut COMPTR) -> HRES
	GetDesc(PVOID) -> HRES
	ResizeBuffers(u32, u32, u32, u32, u32) -> HRES
	ResizeTarget(PCVOID) -> HRES
	GetContainingOutput(*mut COMPTR) -> HRES
	GetFrameStatistics(PVOID) -> HRES
	GetLastPresentCount(*mut u32) -> HRES
}
