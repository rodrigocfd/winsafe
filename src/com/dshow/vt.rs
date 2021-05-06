//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! COM virtual tables.

#![allow(non_snake_case)]

use crate::com::{IDispatchVT, IPersistVT, IUnknownVT, PPComVT};
use crate::ffi::{BOOL, HANDLE, HRESULT, PCSTR, PCVOID, PSTR, PVOID};

com_virtual_table! { IBaseFilterVT,
	/// [`IBaseFilter`](crate::dshow::IBaseFilter) virtual table.
	->
	0x56a86895, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IMediaFilterVT, IMediaFilterVT

	EnumPins, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
	FindPin, fn(PPComVT<Self>, PCSTR, *mut PPComVT<IUnknownVT>) -> HRESULT
	QueryFilterInfo, fn(PPComVT<Self>, PVOID) -> HRESULT
	JoinFilterGraph, fn(PPComVT<Self>, PPComVT<IUnknownVT>, PCSTR) -> HRESULT
	QueryVendorInfo, fn(PPComVT<Self>, *mut PSTR) -> HRESULT
}

com_virtual_table! { IEnumFiltersVT,
	/// [`IEnumFilters`](crate::dshow::IEnumFilters) virtual table.
	->
	0x56a86893, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IUnknownVT, IUnknownVT

	Next, fn(PPComVT<Self>, u32, *mut PPComVT<IUnknownVT>, *mut u32) -> HRESULT
	Skip, fn(PPComVT<Self>, u32) -> HRESULT
	Reset, fn(PPComVT<Self>) -> HRESULT
	Clone, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
}

com_virtual_table! { IFilterGraphVT,
	/// [`IFilterGraph`](crate::dshow::IFilterGraph) virtual table.
	->
	0x56a8689f, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IUnknownVT, IUnknownVT

	AddFilter, fn(PPComVT<Self>, PPComVT<IUnknownVT>, PCSTR) -> HRESULT
	RemoveFilter, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> HRESULT
	EnumFilters, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
	FindFilterByName, fn(PPComVT<Self>, PCSTR, *mut PPComVT<IUnknownVT>) -> HRESULT
	ConnectDirect, fn(PPComVT<Self>, PPComVT<IUnknownVT>, PPComVT<IUnknownVT>, PCVOID) -> HRESULT
	Reconnect, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> HRESULT
	Disconnect, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> HRESULT
	SetDefaultSyncSource, fn(PPComVT<Self>) -> HRESULT
}

com_virtual_table! { IGraphBuilderVT,
	/// [`IGraphBuilder`](crate::dshow::IGraphBuilder) virtual table.
	->
	0x56a868a9, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IFilterGraphVT, IFilterGraphVT

	Connect, fn(PPComVT<Self>, PPComVT<IUnknownVT>, PPComVT<IUnknownVT>) -> HRESULT
	Render, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> HRESULT
	RenderFile, fn(PPComVT<Self>, PCSTR, PCSTR) -> HRESULT
	AddSourceFilter, fn(PPComVT<Self>, PCSTR, PCSTR, *mut PPComVT<IUnknownVT>) -> HRESULT
	SetLogFile, fn(PPComVT<Self>, HANDLE) -> HRESULT
	Abort, fn(PPComVT<Self>) -> HRESULT
	ShouldOperationContinue, fn(PPComVT<Self>) -> HRESULT
}

com_virtual_table! { IMediaControlVT,
	/// [`IMediaControl`](crate::dshow::IMediaControl) virtual table.
	->
	0x56a868b1, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IDispatchVT, IDispatchVT

	Run, fn(PPComVT<Self>) -> HRESULT
	Pause, fn(PPComVT<Self>) -> HRESULT
	Stop, fn(PPComVT<Self>) -> HRESULT
	GetState, fn(PPComVT<Self>, i32, *mut u32) -> HRESULT
	RenderFile, fn(PPComVT<Self>, PSTR) -> HRESULT
	AddSourceFilter, fn(PPComVT<Self>, PSTR, *mut PPComVT<IUnknownVT>) -> HRESULT
	GetFilterCollection, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
	GetRegFilterCollection, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
	StopWhenReady, fn(PPComVT<Self>) -> HRESULT
}

com_virtual_table! { IMediaFilterVT,
	/// [`IMediaFilter`](crate::dshow::IMediaFilter) virtual table.
	->
	0x56a86899, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IPersistVT, IPersistVT

	Stop, fn(PPComVT<Self>) -> HRESULT
	Pause, fn(PPComVT<Self>) -> HRESULT
   Run, fn(PPComVT<Self>, i64) -> HRESULT
	GetState, fn(PPComVT<Self>, i64, PVOID, *mut u32) -> HRESULT
	SetSyncSource, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> HRESULT
	GetSyncSource, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
}

com_virtual_table! { IMediaSeekingVT,
	/// [`IMediaSeeking`](crate::dshow::IMediaSeeking) virtual table.
	->
	0x36b73880, 0xc2c8, 0x11cf, 0x8b46, 0x00805f6cef60,
	IUnknownVT, IUnknownVT

	GetCapabilities, fn(PPComVT<Self>, *mut u32) -> HRESULT
	CheckCapabilities, fn(PPComVT<Self>, *mut u32) -> HRESULT
	IsFormatSupported, fn(PPComVT<Self>, PCVOID) -> HRESULT
	QueryPreferredFormat, fn(PPComVT<Self>, PVOID) -> HRESULT
	GetTimeFormat, fn(PPComVT<Self>, PVOID) -> HRESULT
	IsUsingTimeFormat, fn(PPComVT<Self>, PCVOID) -> HRESULT
	SetTimeFormat, fn(PPComVT<Self>, PCVOID) -> HRESULT
   GetDuration, fn(PPComVT<Self>, *mut i64) -> HRESULT
	GetStopPosition, fn(PPComVT<Self>, *mut i64) -> HRESULT
	GetCurrentPosition, fn(PPComVT<Self>, *mut i64) -> HRESULT
	ConvertTimeFormat, fn(PPComVT<Self>, *mut i64, PCVOID, i64, PCVOID) -> HRESULT
	SetPositions, fn(PPComVT<Self>, *mut i64, u32, *mut i64, u32) -> HRESULT
	GetPositions, fn(PPComVT<Self>, *mut i64, *mut i64) -> HRESULT
	GetAvailable, fn(PPComVT<Self>, *mut i64, *mut i64) -> HRESULT
	SetRate, fn(PPComVT<Self>, f64) -> HRESULT
	GetRate, fn(PPComVT<Self>, *mut f64) -> HRESULT
	GetPreroll, fn(PPComVT<Self>, *mut i64) -> HRESULT
}

com_virtual_table! { IMFGetServiceVT,
	/// [`IMFGetService`](crate::dshow::IMFGetService) virtual table.
	->
	0xfa993888, 0x4383, 0x415a, 0xa930, 0xdd472a8cf6f7,
	IUnknownVT, IUnknownVT

	GetService, fn(PPComVT<Self>, PCVOID, PCVOID, *mut PPComVT<IUnknownVT>) -> HRESULT
}

com_virtual_table! { IMFVideoDisplayControlVT,
	/// [`IMFVideoDisplayControl`](crate::dshow::IMFVideoDisplayControl) virtual
	/// table.
	->
	0xa490b1e4, 0xab84, 0x4d31, 0xa1b2, 0x181e03b1077a,
	IUnknownVT, IUnknownVT

	GetNativeVideoSize, fn(PPComVT<Self>, PVOID, PVOID) -> HRESULT
	GetIdealVideoSize, fn(PPComVT<Self>, PVOID, PVOID) -> HRESULT
	SetVideoPosition, fn(PPComVT<Self>, PCVOID, PCVOID) -> HRESULT
	GetVideoPosition, fn(PPComVT<Self>, PVOID, PCVOID) -> HRESULT
	SetAspectRatioMode, fn(PPComVT<Self>, u32) -> HRESULT
	GetAspectRatioMode, fn(PPComVT<Self>, *mut u32) -> HRESULT
	SetVideoWindow, fn(PPComVT<Self>, HANDLE) -> HRESULT
	GetVideoWindow, fn(PPComVT<Self>, *mut HANDLE) -> HRESULT
	RepaintVideo, fn(PPComVT<Self>) -> HRESULT
	GetCurrentImage, fn(PPComVT<Self>, PVOID, *mut *mut u8, *mut u32, *mut i64) -> HRESULT
	SetBorderColor, fn(PPComVT<Self>, u32) -> HRESULT
	GetBorderColor, fn(PPComVT<Self>, *mut u32) -> HRESULT
	SetRenderingPrefs, fn(PPComVT<Self>, u32) -> HRESULT
	GetRenderingPrefs, fn(PPComVT<Self>, *mut u32) -> HRESULT
	SetFullscreen, fn(PPComVT<Self>, BOOL) -> HRESULT
	GetFullscreen, fn(PPComVT<Self>, *mut BOOL) -> HRESULT
}

com_virtual_table! { IPinVT,
	/// [`IPin`](crate::dshow::IPin) virtual table.
	->
	0x56a86891, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IUnknownVT, IUnknownVT

	Connect, fn(PPComVT<Self>, PPComVT<Self>, PPComVT<IUnknownVT>, PCVOID) -> HRESULT
	ReceiveConnection, fn(PPComVT<Self>, PPComVT<IUnknownVT>, PCVOID) -> HRESULT
	Disconnect, fn(PPComVT<Self>) -> HRESULT
	ConnectedTo, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
	ConnectionMediaType, fn(PPComVT<Self>, PVOID) -> HRESULT
	QueryPinInfo, fn(PPComVT<Self>, PVOID) -> HRESULT
	QueryDirection, fn(PPComVT<Self>, PVOID) -> HRESULT
	QueryId, fn(PPComVT<Self>, *mut PSTR) -> HRESULT
	QueryAccept, fn(PPComVT<Self>, PCVOID) -> HRESULT
	EnumMediaTypes, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
	QueryInternalConnections, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>, *mut u32) -> HRESULT
	EndOfStream, fn(PPComVT<Self>) -> HRESULT
	BeginFlush, fn(PPComVT<Self>) -> HRESULT
	EndFlush, fn(PPComVT<Self>) -> HRESULT
	NewSegment, fn(PPComVT<Self>, i64, i64, f64) -> HRESULT
}
