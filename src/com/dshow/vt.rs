//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! COM virtual tables.

#![allow(non_snake_case)]

use crate::com::{IDispatchVT, IPersistVT, IUnknownVT, PPComVT};
use crate::ffi::{BOOL, HANDLE, HRESULT, PCSTR, PCVOID, PSTR, PVOID};

type IUnkPP = PPComVT<IUnknownVT>;

pub_struct_vtable! { IBaseFilterVT,
	/// [`IBaseFilter`](crate::dshow::IBaseFilter) virtual table.
	->
	0x56a86895, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IMediaFilterVT, IMediaFilterVT

	EnumPins, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	FindPin, fn(IUnkPP, PCSTR, *mut IUnkPP) -> HRESULT
	QueryFilterInfo, fn(IUnkPP, PVOID) -> HRESULT
	JoinFilterGraph, fn(IUnkPP, IUnkPP, PCSTR) -> HRESULT
	QueryVendorInfo, fn(IUnkPP, *mut PSTR) -> HRESULT
}

pub_struct_vtable! { IEnumFiltersVT,
	/// [`IEnumFilters`](crate::dshow::IEnumFilters) virtual table.
	->
	0x56a86893, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IUnknownVT, IUnknownVT

	Next, fn(IUnkPP, u32, *mut IUnkPP, *mut u32) -> HRESULT
	Skip, fn(IUnkPP, u32) -> HRESULT
	Reset, fn(IUnkPP) -> HRESULT
	Clone, fn(IUnkPP, *mut IUnkPP) -> HRESULT
}

pub_struct_vtable! { IFilterGraphVT,
	/// [`IFilterGraph`](crate::dshow::IFilterGraph) virtual table.
	->
	0x56a8689f, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IUnknownVT, IUnknownVT

	AddFilter, fn(IUnkPP, IUnkPP, PCSTR) -> HRESULT
	RemoveFilter, fn(IUnkPP, IUnkPP) -> HRESULT
	EnumFilters, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	FindFilterByName, fn(IUnkPP, PCSTR, *mut IUnkPP) -> HRESULT
	ConnectDirect, fn(IUnkPP, IUnkPP, IUnkPP, PCVOID) -> HRESULT
	Reconnect, fn(IUnkPP, IUnkPP) -> HRESULT
	Disconnect, fn(IUnkPP, IUnkPP) -> HRESULT
	SetDefaultSyncSource, fn(IUnkPP) -> HRESULT
}

pub_struct_vtable! { IGraphBuilderVT,
	/// [`IGraphBuilder`](crate::dshow::IGraphBuilder) virtual table.
	->
	0x56a868a9, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IFilterGraphVT, IFilterGraphVT

	Connect, fn(IUnkPP, IUnkPP, IUnkPP) -> HRESULT
	Render, fn(IUnkPP, IUnkPP) -> HRESULT
	RenderFile, fn(IUnkPP, PCSTR, PCSTR) -> HRESULT
	AddSourceFilter, fn(IUnkPP, PCSTR, PCSTR, *mut IUnkPP) -> HRESULT
	SetLogFile, fn(IUnkPP, HANDLE) -> HRESULT
	Abort, fn(IUnkPP) -> HRESULT
	ShouldOperationContinue, fn(IUnkPP) -> HRESULT
}

pub_struct_vtable! { IMediaControlVT,
	/// [`IMediaControl`](crate::dshow::IMediaControl) virtual table.
	->
	0x56a868b1, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IDispatchVT, IDispatchVT

	Run, fn(IUnkPP) -> HRESULT
	Pause, fn(IUnkPP) -> HRESULT
	Stop, fn(IUnkPP) -> HRESULT
	GetState, fn(IUnkPP, i32, *mut u32) -> HRESULT
	RenderFile, fn(IUnkPP, PSTR) -> HRESULT
	AddSourceFilter, fn(IUnkPP, PSTR, *mut IUnkPP) -> HRESULT
	GetFilterCollection, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	GetRegFilterCollection, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	StopWhenReady, fn(IUnkPP) -> HRESULT
}

pub_struct_vtable! { IMediaFilterVT,
	/// [`IMediaFilter`](crate::dshow::IMediaFilter) virtual table.
	->
	0x56a86899, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IPersistVT, IPersistVT

	Stop, fn(IUnkPP) -> HRESULT
	Pause, fn(IUnkPP) -> HRESULT
   Run, fn(IUnkPP, i64) -> HRESULT
	GetState, fn(IUnkPP, i64, PVOID, *mut u32) -> HRESULT
	SetSyncSource, fn(IUnkPP, IUnkPP) -> HRESULT
	GetSyncSource, fn(IUnkPP, *mut IUnkPP) -> HRESULT
}

pub_struct_vtable! { IMediaSeekingVT,
	/// [`IMediaSeeking`](crate::dshow::IMediaSeeking) virtual table.
	->
	0x36b73880, 0xc2c8, 0x11cf, 0x8b46, 0x00805f6cef60,
	IUnknownVT, IUnknownVT

	GetCapabilities, fn(IUnkPP, *mut u32) -> HRESULT
	CheckCapabilities, fn(IUnkPP, *mut u32) -> HRESULT
	IsFormatSupported, fn(IUnkPP, PCVOID) -> HRESULT
	QueryPreferredFormat, fn(IUnkPP, PVOID) -> HRESULT
	GetTimeFormat, fn(IUnkPP, PVOID) -> HRESULT
	IsUsingTimeFormat, fn(IUnkPP, PCVOID) -> HRESULT
	SetTimeFormat, fn(IUnkPP, PCVOID) -> HRESULT
   GetDuration, fn(IUnkPP, *mut i64) -> HRESULT
	GetStopPosition, fn(IUnkPP, *mut i64) -> HRESULT
	GetCurrentPosition, fn(IUnkPP, *mut i64) -> HRESULT
	ConvertTimeFormat, fn(IUnkPP, *mut i64, PCVOID, i64, PCVOID) -> HRESULT
	SetPositions, fn(IUnkPP, *mut i64, u32, *mut i64, u32) -> HRESULT
	GetPositions, fn(IUnkPP, *mut i64, *mut i64) -> HRESULT
	GetAvailable, fn(IUnkPP, *mut i64, *mut i64) -> HRESULT
	SetRate, fn(IUnkPP, f64) -> HRESULT
	GetRate, fn(IUnkPP, *mut f64) -> HRESULT
	GetPreroll, fn(IUnkPP, *mut i64) -> HRESULT
}

pub_struct_vtable! { IMFGetServiceVT,
	/// [`IMFGetService`](crate::dshow::IMFGetService) virtual table.
	->
	0xfa993888, 0x4383, 0x415a, 0xa930, 0xdd472a8cf6f7,
	IUnknownVT, IUnknownVT

	GetService, fn(IUnkPP, PCVOID, PCVOID, *mut IUnkPP) -> HRESULT
}

pub_struct_vtable! { IMFVideoDisplayControlVT,
	/// [`IMFVideoDisplayControl`](crate::dshow::IMFVideoDisplayControl) virtual
	/// table.
	->
	0xa490b1e4, 0xab84, 0x4d31, 0xa1b2, 0x181e03b1077a,
	IUnknownVT, IUnknownVT

	GetNativeVideoSize, fn(IUnkPP, PVOID, PVOID) -> HRESULT
	GetIdealVideoSize, fn(IUnkPP, PVOID, PVOID) -> HRESULT
	SetVideoPosition, fn(IUnkPP, PCVOID, PCVOID) -> HRESULT
	GetVideoPosition, fn(IUnkPP, PVOID, PCVOID) -> HRESULT
	SetAspectRatioMode, fn(IUnkPP, u32) -> HRESULT
	GetAspectRatioMode, fn(IUnkPP, *mut u32) -> HRESULT
	SetVideoWindow, fn(IUnkPP, HANDLE) -> HRESULT
	GetVideoWindow, fn(IUnkPP, *mut HANDLE) -> HRESULT
	RepaintVideo, fn(IUnkPP) -> HRESULT
	GetCurrentImage, fn(IUnkPP, PVOID, *mut *mut u8, *mut u32, *mut i64) -> HRESULT
	SetBorderColor, fn(IUnkPP, u32) -> HRESULT
	GetBorderColor, fn(IUnkPP, *mut u32) -> HRESULT
	SetRenderingPrefs, fn(IUnkPP, u32) -> HRESULT
	GetRenderingPrefs, fn(IUnkPP, *mut u32) -> HRESULT
	SetFullscreen, fn(IUnkPP, BOOL) -> HRESULT
	GetFullscreen, fn(IUnkPP, *mut BOOL) -> HRESULT
}

pub_struct_vtable! { IPinVT,
	/// [`IPin`](crate::dshow::IPin) virtual table.
	->
	0x56a86891, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IUnknownVT, IUnknownVT

	Connect, fn(IUnkPP, IUnkPP, IUnkPP, PCVOID) -> HRESULT
	ReceiveConnection, fn(IUnkPP, IUnkPP, PCVOID) -> HRESULT
	Disconnect, fn(IUnkPP) -> HRESULT
	ConnectedTo, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	ConnectionMediaType, fn(IUnkPP, PVOID) -> HRESULT
	QueryPinInfo, fn(IUnkPP, PVOID) -> HRESULT
	QueryDirection, fn(IUnkPP, PVOID) -> HRESULT
	QueryId, fn(IUnkPP, *mut PSTR) -> HRESULT
	QueryAccept, fn(IUnkPP, PCVOID) -> HRESULT
	EnumMediaTypes, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	QueryInternalConnections, fn(IUnkPP, *mut IUnkPP, *mut u32) -> HRESULT
	EndOfStream, fn(IUnkPP) -> HRESULT
	BeginFlush, fn(IUnkPP) -> HRESULT
	EndFlush, fn(IUnkPP) -> HRESULT
	NewSegment, fn(IUnkPP, i64, i64, f64) -> HRESULT
}
