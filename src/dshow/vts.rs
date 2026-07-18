#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::macros::*;
use crate::ole::vts::*;
use crate::oleaut::vts::*;

com_vtbl! { IBaseFilterVT : IMediaFilterVT
	EnumPins(*mut COMPTR) -> HRES
	FindPin(PCSTR, *mut COMPTR) -> HRES
	QueryFilterInfo(PVOID) -> HRES
	JoinFilterGraph(COMPTR, PCSTR) -> HRES
	QueryVendorInfo(*mut PSTR) -> HRES
}

com_vtbl! { IEnumFiltersVT : IUnknownVT
	Next(u32, *mut COMPTR, *mut u32) -> HRES
	Skip(u32) -> HRES
	Reset() -> HRES
	Clone(*mut COMPTR) -> HRES
}

com_vtbl! { IEnumMediaTypesVT : IUnknownVT
	Next(u32, PVOID, *mut u32) -> HRES
	Skip(u32) -> HRES
	Reset() -> HRES
	Clone(*mut COMPTR) -> HRES
}

com_vtbl! { IEnumPinsVT : IUnknownVT
	Next(u32, *mut COMPTR, *mut u32) -> HRES
	Skip(u32) -> HRES
	Reset() -> HRES
	Clone(*mut COMPTR) -> HRES
}

com_vtbl! { IFileSinkFilterVT : IUnknownVT
	SetFileName(PCSTR, PCVOID) -> HRES
	GetCurFile(*mut PSTR, PVOID) -> HRES
}

com_vtbl! { IFilterGraphVT : IUnknownVT
	AddFilter(COMPTR, PCSTR) -> HRES
	RemoveFilter(COMPTR) -> HRES
	EnumFilters(*mut COMPTR) -> HRES
	FindFilterByName(PCSTR, *mut COMPTR) -> HRES
	ConnectDirect(COMPTR, COMPTR, PCVOID) -> HRES
	Reconnect(COMPTR) -> HRES
	Disconnect(COMPTR) -> HRES
	SetDefaultSyncSource() -> HRES
}

com_vtbl! { IFilterGraph2VT : IGraphBuilderVT
	AddSourceFilterForMoniker(COMPTR, COMPTR, PCSTR, *mut COMPTR) -> HRES
	ReconnectEx(COMPTR, PCVOID) -> HRES
	RenderEx(COMPTR, u32, *mut u32) -> HRES
}

com_vtbl! { IGraphBuilderVT : IFilterGraphVT
	Connect(COMPTR, COMPTR) -> HRES
	Render(COMPTR) -> HRES
	RenderFile(PCSTR, PCSTR) -> HRES
	AddSourceFilter(PCSTR, PCSTR, *mut COMPTR) -> HRES
	SetLogFile(HANDLE) -> HRES
	Abort() -> HRES
	ShouldOperationContinue() -> HRES
}

com_vtbl! { IMediaControlVT : IDispatchVT
	Run() -> HRES
	Pause() -> HRES
	Stop() -> HRES
	GetState(i32, *mut u32) -> HRES
	RenderFile(PSTR) -> HRES
	AddSourceFilter(PSTR, *mut COMPTR) -> HRES
	GetFilterCollection(*mut COMPTR) -> HRES
	GetRegFilterCollection(*mut COMPTR) -> HRES
	StopWhenReady() -> HRES
}

com_vtbl! { IMediaFilterVT : IPersistVT
	Stop() -> HRES
	Pause() -> HRES
	Run(i64) -> HRES
	GetState(u32, *mut u32) -> HRES
	SetSyncSource(COMPTR) -> HRES
	GetSyncSource(*mut COMPTR) -> HRES
}

com_vtbl! { IMediaSeekingVT : IUnknownVT
	GetCapabilities(*mut u32) -> HRES
	CheckCapabilities(*mut u32) -> HRES
	IsFormatSupported(PCVOID) -> HRES
	QueryPreferredFormat(PVOID) -> HRES
	GetTimeFormat(PVOID) -> HRES
	IsUsingTimeFormat(PCVOID) -> HRES
	SetTimeFormat(PCVOID) -> HRES
	GetDuration(*mut i64) -> HRES
	GetStopPosition(*mut i64) -> HRES
	GetCurrentPosition(*mut i64) -> HRES
	ConvertTimeFormat(*mut i64, PCVOID, i64, PCVOID) -> HRES
	SetPositions(*mut i64, u32, *mut i64, u32) -> HRES
	GetPositions(*mut i64, *mut i64) -> HRES
	GetAvailable(*mut i64, *mut i64) -> HRES
	SetRate(f64) -> HRES
	GetRate(*mut f64) -> HRES
	GetPreroll(*mut i64) -> HRES
}

com_vtbl! { IPinVT : IUnknownVT
	Connect(COMPTR, PCVOID) -> HRES
	ReceiveConnection(COMPTR, PCVOID) -> HRES
	Disconnect() -> HRES
	ConnectedTo(*mut COMPTR) -> HRES
	ConnectionMediaType(PVOID) -> HRES
	QueryPinInfo(PVOID) -> HRES
	QueryDirection(PVOID) -> HRES
	QueryId(*mut PSTR) -> HRES
	QueryAccept(PCVOID) -> HRES
	EnumMediaTypes(*mut COMPTR) -> HRES
	QueryInternalConnections(*mut COMPTR, *mut u32) -> HRES
	EndOfStream() -> HRES
	BeginFlush() -> HRES
	EndFlush() -> HRES
	NewSegment(i64, i64, f64) -> HRES
}
