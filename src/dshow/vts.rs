#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::ole::vts::*;
use crate::oleaut::vts::*;

#[repr(C)]
pub struct IBaseFilterVT {
	pub IMediaFilterVT: IMediaFilterVT,
	pub EnumPins: fn(COMPTR, *mut COMPTR) -> HRES,
	pub FindPin: fn(COMPTR, PCSTR, *mut COMPTR) -> HRES,
	pub QueryFilterInfo: fn(COMPTR, PVOID) -> HRES,
	pub JoinFilterGraph: fn(COMPTR, COMPTR, PCSTR) -> HRES,
	pub QueryVendorInfo: fn(COMPTR, *mut PSTR) -> HRES,
}

#[repr(C)]
pub struct IEnumFiltersVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(COMPTR, u32, *mut COMPTR, *mut u32) -> HRES,
	pub Skip: fn(COMPTR, u32) -> HRES,
	pub Reset: fn(COMPTR) -> HRES,
	pub Clone: fn(COMPTR, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IEnumMediaTypesVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(COMPTR, u32, *mut PVOID, *mut u32) -> HRES,
	pub Skip: fn(COMPTR, u32) -> HRES,
	pub Reset: fn(COMPTR) -> HRES,
	pub Clone: fn(COMPTR, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IEnumPinsVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(COMPTR, u32, *mut COMPTR, *mut u32) -> HRES,
	pub Skip: fn(COMPTR, u32) -> HRES,
	pub Reset: fn(COMPTR) -> HRES,
	pub Clone: fn(COMPTR, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IFileSinkFilterVT {
	pub IUnknownVT: IUnknownVT,
	pub SetFileName: fn(COMPTR, PCSTR, PCVOID) -> HRES,
	pub GetCurFile: fn(COMPTR, *mut PSTR, PVOID) -> HRES,
}

#[repr(C)]
pub struct IFilterGraphVT {
	pub IUnknownVT: IUnknownVT,
	pub AddFilter: fn(COMPTR, COMPTR, PCSTR) -> HRES,
	pub RemoveFilter: fn(COMPTR, COMPTR) -> HRES,
	pub EnumFilters: fn(COMPTR, *mut COMPTR) -> HRES,
	pub FindFilterByName: fn(COMPTR, PCSTR, *mut COMPTR) -> HRES,
	pub ConnectDirect: fn(COMPTR, COMPTR, COMPTR, PCVOID) -> HRES,
	pub Reconnect: fn(COMPTR, COMPTR) -> HRES,
	pub Disconnect: fn(COMPTR, COMPTR) -> HRES,
	pub SetDefaultSyncSource: fn(COMPTR) -> HRES,
}

#[repr(C)]
pub struct IFilterGraph2VT {
	pub IGraphBuilderVT: IGraphBuilderVT,
	pub AddSourceFilterForMoniker: fn(COMPTR, COMPTR, COMPTR, PCSTR, *mut COMPTR) -> HRES,
	pub ReconnectEx: fn(COMPTR, COMPTR, PCVOID) -> HRES,
	pub RenderEx: fn(COMPTR, COMPTR, u32, *mut u32) -> HRES,
}

#[repr(C)]
pub struct IGraphBuilderVT {
	pub IFilterGraphVT: IFilterGraphVT,
	pub Connect: fn(COMPTR, COMPTR, COMPTR) -> HRES,
	pub Render: fn(COMPTR, COMPTR) -> HRES,
	pub RenderFile: fn(COMPTR, PCSTR, PCSTR) -> HRES,
	pub AddSourceFilter: fn(COMPTR, PCSTR, PCSTR, *mut COMPTR) -> HRES,
	pub SetLogFile: fn(COMPTR, HANDLE) -> HRES,
	pub Abort: fn(COMPTR) -> HRES,
	pub ShouldOperationContinue: fn(COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMediaControlVT {
	pub IDispatchVT: IDispatchVT,
	pub Run: fn(COMPTR) -> HRES,
	pub Pause: fn(COMPTR) -> HRES,
	pub Stop: fn(COMPTR) -> HRES,
	pub GetState: fn(COMPTR, i32, *mut u32) -> HRES,
	pub RenderFile: fn(COMPTR, PSTR) -> HRES,
	pub AddSourceFilter: fn(COMPTR, PSTR, *mut COMPTR) -> HRES,
	pub GetFilterCollection: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetRegFilterCollection: fn(COMPTR, *mut COMPTR) -> HRES,
	pub StopWhenReady: fn(COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMediaFilterVT {
	pub IPersistVT: IPersistVT,
	pub Stop: fn(COMPTR) -> HRES,
	pub Pause: fn(COMPTR) -> HRES,
   pub Run: fn(COMPTR, i64) -> HRES,
	pub GetState: fn(COMPTR, u32, *mut u32) -> HRES,
	pub SetSyncSource: fn(COMPTR, COMPTR) -> HRES,
	pub GetSyncSource: fn(COMPTR, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMediaSeekingVT {
	pub IUnknownVT: IUnknownVT,
	pub GetCapabilities: fn(COMPTR, *mut u32) -> HRES,
	pub CheckCapabilities: fn(COMPTR, *mut u32) -> HRES,
	pub IsFormatSupported: fn(COMPTR, PCVOID) -> HRES,
	pub QueryPreferredFormat: fn(COMPTR, PVOID) -> HRES,
	pub GetTimeFormat: fn(COMPTR, PVOID) -> HRES,
	pub IsUsingTimeFormat: fn(COMPTR, PCVOID) -> HRES,
	pub SetTimeFormat: fn(COMPTR, PCVOID) -> HRES,
	pub GetDuration: fn(COMPTR, *mut i64) -> HRES,
	pub GetStopPosition: fn(COMPTR, *mut i64) -> HRES,
	pub GetCurrentPosition: fn(COMPTR, *mut i64) -> HRES,
	pub ConvertTimeFormat: fn(COMPTR, *mut i64, PCVOID, i64, PCVOID) -> HRES,
	pub SetPositions: fn(COMPTR, *mut i64, u32, *mut i64, u32) -> HRES,
	pub GetPositions: fn(COMPTR, *mut i64, *mut i64) -> HRES,
	pub GetAvailable: fn(COMPTR, *mut i64, *mut i64) -> HRES,
	pub SetRate: fn(COMPTR, f64) -> HRES,
	pub GetRate: fn(COMPTR, *mut f64) -> HRES,
	pub GetPreroll: fn(COMPTR, *mut i64) -> HRES,
}

#[repr(C)]
pub struct IPinVT {
	pub IUnknownVT: IUnknownVT,
	pub Connect: fn(COMPTR, COMPTR, PCVOID) -> HRES,
	pub ReceiveConnection: fn(COMPTR, COMPTR, PCVOID) -> HRES,
	pub Disconnect: fn(COMPTR) -> HRES,
	pub ConnectedTo: fn(COMPTR, *mut COMPTR) -> HRES,
	pub ConnectionMediaType: fn(COMPTR, PVOID) -> HRES,
	pub QueryPinInfo: fn(COMPTR, PVOID) -> HRES,
	pub QueryDirection: fn(COMPTR, PVOID) -> HRES,
	pub QueryId: fn(COMPTR, *mut PSTR) -> HRES,
	pub QueryAccept: fn(COMPTR, PCVOID) -> HRES,
	pub EnumMediaTypes: fn(COMPTR, *mut COMPTR) -> HRES,
	pub QueryInternalConnections: fn(COMPTR, *mut COMPTR, *mut u32) -> HRES,
	pub EndOfStream: fn(COMPTR) -> HRES,
	pub BeginFlush: fn(COMPTR) -> HRES,
	pub EndFlush: fn(COMPTR) -> HRES,
	pub NewSegment: fn(COMPTR, i64, i64, f64) -> HRES,
}
