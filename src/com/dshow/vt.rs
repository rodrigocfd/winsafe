//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! COM virtual tables.

#![allow(non_snake_case)]

use crate::com::{ComVT, IDispatchVT, IPersistVT, IUnknownVT, PPComVT};
use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PSTR, PVOID};
use crate::structs::IID;

com_virtual_table! { IBaseFilterVT,
	/// [`IBaseFilter`](crate::dshow::IBaseFilter) virtual table.
	->
	0x56a86895, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IMediaFilterVT, IMediaFilterVT

	EnumPins, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> i32
	FindPin, fn(PPComVT<Self>, PCSTR, *mut PPComVT<IUnknownVT>) -> i32
	QueryFilterInfo, fn(PPComVT<Self>, PVOID) -> i32
	JoinFilterGraph, fn(PPComVT<Self>, PPComVT<IUnknownVT>, PCSTR) -> i32
	QueryVendorInfo, fn(PPComVT<Self>, *mut PSTR) -> i32
}

com_virtual_table! { IFilterGraphVT,
	/// [`IFilterGraph`](crate::dshow::IFilterGraph) virtual table.
	->
	0x56a8689f, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IUnknownVT, IUnknownVT

	AddFilter, fn(PPComVT<Self>, PPComVT<IUnknownVT>, PCSTR) -> i32
	RemoveFilter, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> i32
	EnumFilters, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> i32
	FindFilterByName, fn(PPComVT<Self>, PCSTR, *mut PPComVT<IUnknownVT>) -> i32
	ConnectDirect, fn(PPComVT<Self>, PPComVT<IUnknownVT>, PPComVT<IUnknownVT>, PCVOID) -> i32
	Reconnect, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> i32
	Disconnect, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> i32
	SetDefaultSyncSource, fn(PPComVT<Self>) -> i32
}

com_virtual_table! { IGraphBuilderVT,
	/// [`IGraphBuilder`](crate::dshow::IGraphBuilder) virtual table.
	->
	0x56a868a9, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IFilterGraphVT, IFilterGraphVT

	Connect, fn(PPComVT<Self>, PPComVT<IUnknownVT>, PPComVT<IUnknownVT>) -> i32
	Render, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> i32
	RenderFile, fn(PPComVT<Self>, PCSTR, PCSTR) -> i32
	AddSourceFilter, fn(PPComVT<Self>, PCSTR, PCSTR, *mut PPComVT<IUnknownVT>) -> i32
	SetLogFile, fn(PPComVT<Self>, HANDLE) -> i32
	Abort, fn(PPComVT<Self>) -> i32
	ShouldOperationContinue, fn(PPComVT<Self>) -> i32
}

com_virtual_table! { IMediaControlVT,
	/// [`IMediaControl`](crate::dshow::IMediaControl) virtual table.
	->
	0x56a868b1, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IDispatchVT, IDispatchVT

	Run, fn(PPComVT<Self>) -> i32
	Pause, fn(PPComVT<Self>) -> i32
	Stop, fn(PPComVT<Self>) -> i32
	GetState, fn(PPComVT<Self>, i32, *mut i32) -> i32
	RenderFile, fn(PPComVT<Self>, PSTR) -> i32
	AddSourceFilter, fn(PPComVT<Self>, PSTR, *mut PPComVT<IUnknownVT>) -> i32
	GetFilterCollection, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> i32
	GetRegFilterCollection, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> i32
	StopWhenReady, fn(PPComVT<Self>) -> i32
}

com_virtual_table! { IMediaFilterVT,
	/// [`IMediaFilter`](crate::dshow::IMediaFilter) virtual table.
	->
	0x56a86899, 0x0ad4, 0x11ce, 0xb03a, 0x0020af0ba770,
	IPersistVT, IPersistVT

	Stop, fn(PPComVT<Self>) -> i32
	Pause, fn(PPComVT<Self>) -> i32
   Run, fn(PPComVT<Self>, i64) -> i32
	GetState, fn(PPComVT<Self>, i64, PVOID, *mut u32) -> i32
	SetSyncSource, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> i32
	GetSyncSource, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> i32
}

com_virtual_table! { IMediaSeekingVT,
	/// [`IMediaSeeking`](crate::dshow::IMediaSeeking) virtual table.
	->
	0x36b73880, 0xc2c8, 0x11cf, 0x8b46, 0x00805f6cef60,
	IUnknownVT, IUnknownVT

	GetCapabilities, fn(PPComVT<Self>, *mut u32) -> i32
	CheckCapabilities, fn(PPComVT<Self>, *mut u32) -> i32
	IsFormatSupported, fn(PPComVT<Self>, PCVOID) -> i32
	QueryPreferredFormat, fn(PPComVT<Self>, PVOID) -> i32
	GetTimeFormat, fn(PPComVT<Self>, PVOID) -> i32
	IsUsingTimeFormat, fn(PPComVT<Self>, PCVOID) -> i32
	SetTimeFormat, fn(PPComVT<Self>, PCVOID) -> i32
   GetDuration, fn(PPComVT<Self>, *mut i64) -> i32
	GetStopPosition, fn(PPComVT<Self>, *mut i64) -> i32
	GetCurrentPosition, fn(PPComVT<Self>, *mut i64) -> i32
	ConvertTimeFormat, fn(PPComVT<Self>, *mut i64, PCVOID, i64, PCVOID) -> i32
	SetPositions, fn(PPComVT<Self>, *mut i64, u32, *mut i64, u32) -> i32
	GetPositions, fn(PPComVT<Self>, *mut i64, *mut i64) -> i32
	GetAvailable, fn(PPComVT<Self>, *mut i64, *mut i64) -> i32
	SetRate, fn(PPComVT<Self>, f64) -> i32
	GetRate, fn(PPComVT<Self>, *mut f64) -> i32
	GetPreroll, fn(PPComVT<Self>, *mut i64) -> i32
}

com_virtual_table! { IMFGetServiceVT,
	/// [`IMFGetService`](crate::dshow::IMFGetService) virtual table.
	->
	0xfa993888, 0x4383, 0x415a, 0xa930, 0xdd472a8cf6f7,
	IUnknownVT, IUnknownVT

	GetService, fn(PPComVT<Self>, PCVOID, PCVOID, *mut PPComVT<IUnknownVT>) -> i32
}

com_virtual_table! { IMFVideoDisplayControlVT,
	/// [`IMFVideoDisplayControl`](crate::dshow::IMFVideoDisplayControl) virtual
	/// table.
	->
	0xa490b1e4, 0xab84, 0x4d31, 0xa1b2, 0x181e03b1077a,
	IUnknownVT, IUnknownVT

	GetNativeVideoSize, fn(PPComVT<Self>, PVOID, PVOID) -> i32
	GetIdealVideoSize, fn(PPComVT<Self>, PVOID, PVOID) -> i32
	SetVideoPosition, fn(PPComVT<Self>, PCVOID, PCVOID) -> i32
	GetVideoPosition, fn(PPComVT<Self>, PVOID, PCVOID) -> i32
	SetAspectRatioMode, fn(PPComVT<Self>, u32) -> i32
	GetAspectRatioMode, fn(PPComVT<Self>, *mut u32) -> i32
	SetVideoWindow, fn(PPComVT<Self>, HANDLE) -> i32
	GetVideoWindow, fn(PPComVT<Self>, *mut HANDLE) -> i32
	RepaintVideo, fn(PPComVT<Self>) -> i32
	GetCurrentImage, fn(PPComVT<Self>, PVOID, *mut *mut u8, *mut u32, *mut i64) -> i32
	SetBorderColor, fn(PPComVT<Self>, u32) -> i32
	GetBorderColor, fn(PPComVT<Self>, *mut u32) -> i32
	SetRenderingPrefs, fn(PPComVT<Self>, u32) -> i32
	GetRenderingPrefs, fn(PPComVT<Self>, *mut u32) -> i32
	SetFullscreen, fn(PPComVT<Self>, BOOL) -> i32
	GetFullscreen, fn(PPComVT<Self>, *mut BOOL) -> i32
}
