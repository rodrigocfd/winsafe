#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::macros::*;
use crate::ole::vts::*;

com_vtbl! { IMFAsyncCallbackVT : IUnknownVT
	GetParameters(*mut u32, *mut u32) -> HRES
	Invoke(COMPTR) -> HRES
}

com_vtbl! { IMFAsyncResultVT : IUnknownVT
	GetState(*mut COMPTR) -> HRES
	GetStatus() -> HRES
	SetStatus(HRES) -> HRES
	GetObject(*mut COMPTR) -> HRES
	GetStateNoAddRef() -> COMPTR
}

com_vtbl! { IMFAttributesVT : IUnknownVT
	GetItem(PCVOID, PVOID) -> HRES
	GetItemType(PCVOID, *mut u32) -> HRES
	CompareItem(PCVOID, PCVOID, *mut BOOL) -> HRES
	Compare(COMPTR, u32, *mut BOOL) -> HRES
	GetUINT32(PCVOID, *mut u32) -> HRES
	GetUINT64(PCVOID, *mut u64) -> HRES
	GetDouble(PCVOID, *mut f64) -> HRES
	GetGUID(PCVOID, PVOID) -> HRES
	GetStringLength(PCVOID, *mut u32) -> HRES
	GetString(PCVOID, PSTR, u32, *mut u32) -> HRES
	GetAllocatedString(PCVOID, *mut PSTR, *mut u32) -> HRES
	GetBlobSize(PCVOID, *mut u32) -> HRES
	GetBlob(PCVOID, *mut u8, u32, *mut u32) -> HRES
	GetAllocatedBlob(PCVOID, *mut *mut u8, *mut u32) -> HRES
	GetUnknown(PCVOID, PCVOID, *mut COMPTR) -> HRES
	SetItem(PCVOID, PCVOID) -> HRES
	DeleteItem(PCVOID) -> HRES
	DeleteAllItems() -> HRES
	SetUINT32(PCVOID, u32) -> HRES
	SetUINT64(PCVOID, u64) -> HRES
	SetDouble(PCVOID, f64) -> HRES
	SetGUID(PCVOID, PCVOID) -> HRES
	SetString(PCVOID, PCSTR) -> HRES
	SetBlob(PCVOID, *const u8, u32) -> HRES
	SetUnknown(PCVOID, COMPTR) -> HRES
	LockStore() -> HRES
	UnlockStore() -> HRES
	GetCount(*mut u32) -> HRES
	GetItemByIndex(u32, PVOID, PVOID) -> HRES
	CopyAllItems(COMPTR) -> HRES
}

com_vtbl! { IMFByteStreamVT : IUnknownVT
	GetCapabilities(*mut u32) -> HRES
	GetLength(*mut u64) -> HRES
	SetLength(u64) -> HRES
	GetCurrentPosition(*mut u64) -> HRES
	SetCurrentPosition(u64) -> HRES
	IsEndOfStream(*mut BOOL) -> HRES
	Read(*mut u8, u32, *mut u32) -> HRES
	BeginRead(*mut u8, u32, COMPTR, COMPTR) -> HRES
	EndRead(COMPTR, *mut u32) -> HRES
	Write(*const u8, u32, *mut u32) -> HRES
	BeginWrite(*const u8, u32, COMPTR, COMPTR) -> HRES
	EndWrite(COMPTR, *mut u32) -> HRES
	Seek(u32, i64, u32, *mut u64) -> HRES
	Flush() -> HRES
	Close() -> HRES
}

com_vtbl! { IMFClockVT : IUnknownVT
	GetClockCharacteristics(*mut u32) -> HRES
	GetCorrelatedTime(u32, *mut i64, *mut i64) -> HRES
	GetContinuityKey(*mut u32) -> HRES
	GetState(u32, *mut u32) -> HRES
	GetProperties(PVOID) -> HRES
}

com_vtbl! { IMFCollectionVT : IUnknownVT
	GetElementCount(*mut u32) -> HRES
	GetElement(u32, *mut COMPTR) -> HRES
	AddElement(COMPTR) -> HRES
	RemoveElement(u32, *mut COMPTR) -> HRES
	InsertElementAt(u32, COMPTR) -> HRES
	RemoveAllElements() -> HRES
}

com_vtbl! { IMFGetServiceVT : IUnknownVT
	GetService(PCVOID, PCVOID, *mut COMPTR) -> HRES
}

com_vtbl! { IMFMediaEventVT : IMFAttributesVT
	GetType(*mut u32) -> HRES
	GetExtendedType(PVOID) -> HRES
	GetStatus(*mut HRES) -> HRES
	GetValue(PVOID) -> HRES
}

com_vtbl! { IMFMediaEventGeneratorVT : IUnknownVT
	GetEvent(u32, *mut COMPTR) -> HRES
	BeginGetEvent(COMPTR, COMPTR) -> HRES
	EndGetEvent(COMPTR, *mut COMPTR) -> HRES
	QueueEvent(u32, PCVOID, HRES, PCVOID) -> HRES
}

com_vtbl! { IMFMediaSessionVT : IMFMediaEventGeneratorVT
	SetTopology(u32, COMPTR) -> HRES
	ClearTopologies() -> HRES
	Start(PCVOID, PCVOID) -> HRES
	Pause() -> HRES
	Stop() -> HRES
	Close() -> HRES
	Shutdown() -> HRES
	GetClock(*mut COMPTR) -> HRES
	GetSessionCapabilities(*mut u32) -> HRES
	GetFullTopology(u32, u64, *mut COMPTR) -> HRES
}

com_vtbl! { IMFMediaSourceVT : IMFMediaEventGeneratorVT
	GetCharacteristics(*mut u32) -> HRES
	CreatePresentationDescriptor(*mut COMPTR) -> HRES
	Start(COMPTR, PCVOID, PCVOID) -> HRES
	Stop() -> HRES
	Pause() -> HRES
	Shutdown() -> HRES
}

com_vtbl! { IMFMediaTypeHandlerVT : IUnknownVT
	IsMediaTypeSupported(COMPTR, *mut COMPTR) -> HRES
	GetMediaTypeCount(*mut u32) -> HRES
	GetMediaTypeByIndex(u32, *mut COMPTR) -> HRES
	SetCurrentMediaType(COMPTR) -> HRES
	GetCurrentMediaType(*mut COMPTR) -> HRES
	GetMajorType(PVOID) -> HRES
}

com_vtbl! { IMFPresentationDescriptorVT : IMFAttributesVT
	GetStreamDescriptorCount(*mut u32) -> HRES
	GetStreamDescriptorByIndex(u32, *mut BOOL, *mut COMPTR) -> HRES
	SelectStream(u32) -> HRES
	DeselectStream(u32) -> HRES
	Clone(*mut COMPTR) -> HRES
}

com_vtbl! { IMFSourceResolverVT : IUnknownVT
	CreateObjectFromURL(PCSTR, u32, COMPTR, *mut u32, *mut COMPTR) -> HRES
	CreateObjectFromByteStream(COMPTR, PCSTR, u32, COMPTR, *mut u32, *mut COMPTR) -> HRES
	BeginCreateObjectFromURL(PCSTR, u32, COMPTR, *mut COMPTR, COMPTR, COMPTR) -> HRES
	EndCreateObjectFromURL(COMPTR, *mut u32, *mut COMPTR) -> HRES
	BeginCreateObjectFromByteStream(COMPTR, PCSTR, u32, COMPTR, *mut COMPTR, COMPTR, COMPTR) -> HRES
	EndCreateObjectFromByteStream(COMPTR, *mut u32, *mut COMPTR) -> HRES
	CancelObjectCreation(COMPTR) -> HRES
}

com_vtbl! { IMFStreamDescriptorVT : IMFAttributesVT
	GetStreamIdentifier(*mut u32) -> HRES
	GetMediaTypeHandler(*mut COMPTR) -> HRES
}

com_vtbl! { IMFTopologyVT : IMFAttributesVT
	GetTopologyID(*mut u64) -> HRES
	AddNode(COMPTR) -> HRES
	RemoveNode(COMPTR) -> HRES
	GetNodeCount(*mut u16) -> HRES
	GetNode(u16, *mut COMPTR) -> HRES
	Clear() -> HRES
	CloneFrom(COMPTR) -> HRES
	GetNodeByID(u64, *mut COMPTR) -> HRES
	GetSourceNodeCollection(*mut COMPTR) -> HRES
	GetOutputNodeCollection(*mut COMPTR) -> HRES
}

com_vtbl! { IMFTopologyNodeVT : IMFAttributesVT
	SetObject(COMPTR) -> HRES
	GetObject(*mut COMPTR) -> HRES
	GetNodeType(*mut u32) -> HRES
	GetTopoNodeID(*mut u64) -> HRES
	SetTopoNodeID(u64) -> HRES
	GetInputCount(*mut u32) -> HRES
	GetOutputCount(*mut u32) -> HRES
	ConnectOutput(u32, COMPTR, u32) -> HRES
	DisconnectOutput(u32) -> HRES
	GetInput(u32, *mut COMPTR, *mut u32) -> HRES
	GetOutput(u32, *mut COMPTR, *mut u32) -> HRES
	SetOutputPrefType(u32, COMPTR) -> HRES
	GetOutputPrefType(u32, *mut COMPTR) -> HRES
	SetInputPrefType(u32, COMPTR) -> HRES
	GetInputPrefType(u32, *mut COMPTR) -> HRES
	CloneFrom(COMPTR) -> HRES
}

com_vtbl! { IMFVideoDisplayControlVT : IUnknownVT
	GetNativeVideoSize(PVOID, PVOID) -> HRES
	GetIdealVideoSize(PVOID, PVOID) -> HRES
	SetVideoPosition(PCVOID, PCVOID) -> HRES
	GetVideoPosition(PVOID, PCVOID) -> HRES
	SetAspectRatioMode(u32) -> HRES
	GetAspectRatioMode(*mut u32) -> HRES
	SetVideoWindow(HANDLE) -> HRES
	GetVideoWindow(*mut HANDLE) -> HRES
	RepaintVideo() -> HRES
	GetCurrentImage(PVOID, *mut *mut u8, *mut u32, *mut i64) -> HRES
	SetBorderColor(u32) -> HRES
	GetBorderColor(*mut u32) -> HRES
	SetRenderingPrefs(u32) -> HRES
	GetRenderingPrefs(*mut u32) -> HRES
	SetFullscreen(BOOL) -> HRES
	GetFullscreen(*mut BOOL) -> HRES
}
