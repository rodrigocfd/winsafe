#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::ole::vts::*;

#[repr(C)]
pub struct IMFAsyncCallbackVT {
	pub IUnknownVT: IUnknownVT,
	pub GetParameters: fn(COMPTR, *mut u32, *mut u32) -> HRES,
	pub Invoke: fn(COMPTR, COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMFAsyncResultVT {
	pub IUnknownVT: IUnknownVT,
	pub GetState: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetStatus: fn(COMPTR) -> HRES,
	pub SetStatus: fn(COMPTR, HRES) -> HRES,
	pub GetObject: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetStateNoAddRef: fn(COMPTR) -> COMPTR,
}

#[repr(C)]
pub struct IMFAttributesVT {
	pub IUnknownVT: IUnknownVT,
	pub GetItem: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub GetItemType: fn(COMPTR, PCVOID, *mut u32) -> HRES,
	pub CompareItem: fn(COMPTR, PCVOID, PCVOID, *mut BOOL) -> HRES,
	pub Compare: fn(COMPTR, COMPTR, u32, *mut BOOL) -> HRES,
	pub GetUINT32: fn(COMPTR, PCVOID, *mut u32) -> HRES,
	pub GetUINT64: fn(COMPTR, PCVOID, *mut u64) -> HRES,
	pub GetDouble: fn(COMPTR, PCVOID, *mut f64) -> HRES,
	pub GetGUID: fn(COMPTR, COMPTR, PVOID) -> HRES,
	pub GetStringLength: fn(COMPTR, PCVOID, *mut u32) -> HRES,
	pub GetString: fn(COMPTR, PCVOID, PSTR, u32, *mut u32) -> HRES,
	pub GetAllocatedString: fn(COMPTR, PCVOID, *mut PSTR, *mut u32) -> HRES,
	pub GetBlobSize: fn(COMPTR, PCVOID, *mut u32) -> HRES,
	pub GetBlob: fn(COMPTR, PCVOID, *mut u8, u32, *mut u32) -> HRES,
	pub GetAllocatedBlob: fn(COMPTR, PCVOID, *mut *mut u8, *mut u32) -> HRES,
	pub GetUnknown: fn(COMPTR, PCVOID, PCVOID, *mut COMPTR) -> HRES,
	pub SetItem: fn(COMPTR, PCVOID, PCVOID) -> HRES,
	pub DeleteItem: fn(COMPTR, PCVOID) -> HRES,
	pub DeleteAllItems: fn(COMPTR) -> HRES,
	pub SetUINT32: fn(COMPTR, PCVOID, u32) -> HRES,
	pub SetUINT64: fn(COMPTR, PCVOID, u64) -> HRES,
	pub SetDouble: fn(COMPTR, PCVOID, f64) -> HRES,
	pub SetGUID: fn(COMPTR, PCVOID, PCVOID) -> HRES,
	pub SetString: fn(COMPTR, PCVOID, PCSTR) -> HRES,
	pub SetBlob: fn(COMPTR, PCVOID, *const u8, u32) -> HRES,
	pub SetUnknown: fn(COMPTR, PCVOID, COMPTR) -> HRES,
	pub LockStore: fn(COMPTR) -> HRES,
	pub UnlockStore: fn(COMPTR) -> HRES,
	pub GetCount: fn(COMPTR, *mut u32) -> HRES,
	pub GetItemByIndex: fn(COMPTR, u32, PVOID, PVOID) -> HRES,
	pub CopyAllItems: fn(COMPTR, COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMFByteStreamVT {
	pub IUnknownVT: IUnknownVT,
	pub GetCapabilities: fn (COMPTR, *mut u32) -> HRES,
	pub GetLength: fn(COMPTR, *mut u64) -> HRES,
	pub SetLength: fn(COMPTR, u64) -> HRES,
	pub GetCurrentPosition: fn(COMPTR, *mut u64) -> HRES,
	pub SetCurrentPosition: fn(COMPTR, u64) -> HRES,
	pub IsEndOfStream: fn(COMPTR, *mut BOOL) -> HRES,
	pub Read: fn(COMPTR, *mut u8, u32, *mut u32) -> HRES,
	pub BeginRead: fn(COMPTR, *mut u8, u32, COMPTR, COMPTR) -> HRES,
	pub EndRead: fn(COMPTR, COMPTR, *mut u32) -> HRES,
	pub Write: fn(COMPTR, *const u8, u32, *mut u32) -> HRES,
	pub BeginWrite: fn(COMPTR, *const u8, u32, COMPTR, COMPTR) -> HRES,
	pub EndWrite: fn(COMPTR, COMPTR, *mut u32) -> HRES,
	pub Seek: fn(COMPTR, u32, i64, u32, *mut u64) -> HRES,
	pub Flush: fn(COMPTR) -> HRES,
	pub Close: fn(COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMFClockVT {
	pub IUnknownVT: IUnknownVT,
	pub GetClockCharacteristics: fn(COMPTR, *mut u32) -> HRES,
	pub GetCorrelatedTime: fn(COMPTR, u32, *mut i64, *mut i64) -> HRES,
	pub GetContinuityKey: fn(COMPTR, *mut u32) -> HRES,
	pub GetState: fn(COMPTR, u32, *mut u32) -> HRES,
	pub GetProperties: fn(COMPTR, PVOID) -> HRES,
}

#[repr(C)]
pub struct IMFGetServiceVT {
	pub IUnknownVT: IUnknownVT,
	pub GetService: fn(COMPTR, PCVOID, PCVOID, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMFMediaEventVT {
	pub IMFAttributesVT: IMFAttributesVT,
	pub GetType: fn(COMPTR, *mut u32) -> HRES,
	pub GetExtendedType: fn(COMPTR, PVOID) -> HRES,
	pub GetStatus: fn(COMPTR, *mut HRES) -> HRES,
	pub GetValue: fn(COMPTR, PVOID) -> HRES,
}

#[repr(C)]
pub struct IMFMediaEventGeneratorVT {
	pub IUnknownVT: IUnknownVT,
	pub GetEvent: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub BeginGetEvent: fn(COMPTR, COMPTR, COMPTR) -> HRES,
	pub EndGetEvent: fn(COMPTR, COMPTR, *mut COMPTR) -> HRES,
	pub QueueEvent: fn(COMPTR, u32, PCVOID, HRES, PCVOID) -> HRES,
}

#[repr(C)]
pub struct IMFMediaSessionVT {
	pub IMFMediaEventGeneratorVT: IMFMediaEventGeneratorVT,
	pub SetTopology: fn(COMPTR, u32, COMPTR) -> HRES,
	pub ClearTopologies: fn(COMPTR) -> HRES,
	pub Start: fn(COMPTR, PCVOID, PCVOID) -> HRES,
	pub Pause: fn(COMPTR) -> HRES,
	pub Stop: fn(COMPTR) -> HRES,
	pub Close: fn(COMPTR) -> HRES,
	pub Shutdown: fn(COMPTR) -> HRES,
	pub GetClock: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetSessionCapabilities: fn(COMPTR, *mut u32) -> HRES,
	pub GetFullTopology: fn(COMPTR, u32, u64, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMFMediaSourceVT {
	pub IMFMediaEventGeneratorVT: IMFMediaEventGeneratorVT,
	pub GetCharacteristics: fn(COMPTR, *mut u32) -> HRES,
	pub CreatePresentationDescriptor: fn(COMPTR, *mut COMPTR) -> HRES,
	pub Start: fn(COMPTR, COMPTR, PCVOID, PCVOID) -> HRES,
	pub Stop: fn(COMPTR) -> HRES,
	pub Pause: fn(COMPTR) -> HRES,
	pub Shutdown: fn(COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMFPresentationDescriptorVT {
	pub IMFAttributesVT: IMFAttributesVT,
	pub GetStreamDescriptorCount: fn(COMPTR, *mut u32) -> HRES,
	pub GetStreamDescriptorByIndex: fn(COMPTR, u32, *mut BOOL, *mut COMPTR) -> HRES,
	pub SelectStream: fn(COMPTR, u32) -> HRES,
	pub DeselectStream: fn(COMPTR, u32) -> HRES,
	pub Clone: fn(COMPTR, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMFSourceResolverVT {
	pub IUnknownVT: IUnknownVT,
	pub CreateObjectFromURL: fn(COMPTR, PCSTR, u32, COMPTR, *mut u32, *mut COMPTR) -> HRES,
	pub CreateObjectFromByteStream: fn(COMPTR, COMPTR, PCSTR, u32, COMPTR, *mut u32, *mut COMPTR) -> HRES,
	pub BeginCreateObjectFromURL: fn(COMPTR, PCSTR, u32, COMPTR, *mut COMPTR, COMPTR, COMPTR) -> HRES,
	pub EndCreateObjectFromURL: fn(COMPTR, COMPTR, *mut u32, *mut COMPTR) -> HRES,
	pub BeginCreateObjectFromByteStream: fn(COMPTR, COMPTR, PCSTR, u32, COMPTR, *mut COMPTR, COMPTR, COMPTR) -> HRES,
	pub EndCreateObjectFromByteStream: fn(COMPTR, COMPTR, *mut u32, *mut COMPTR) -> HRES,
	pub CancelObjectCreation: fn(COMPTR, COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMFTopologyVT {
	pub IMFAttributesVT: IMFAttributesVT,
	pub GetTopologyID: fn(COMPTR, *mut u64) -> HRES,
	pub AddNode: fn(COMPTR, COMPTR) -> HRES,
	pub RemoveNode: fn(COMPTR, COMPTR) -> HRES,
	pub GetNodeCount: fn(COMPTR, *mut u16) -> HRES,
	pub GetNode: fn(COMPTR, u16, *mut COMPTR) -> HRES,
	pub Clear: fn(COMPTR) -> HRES,
	pub CloneFrom: fn(COMPTR, COMPTR) -> HRES,
	pub GetNodeByID: fn(COMPTR, u64, *mut COMPTR) -> HRES,
	pub GetSourceNodeCollection: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetOutputNodeCollection: fn(COMPTR, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMFTopologyNodeVT {
	pub IMFAttributesVT: IMFAttributesVT,
	pub SetObject: fn(COMPTR, COMPTR) -> HRES,
	pub GetObject: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetNodeType: fn(COMPTR, *mut u32) -> HRES,
	pub GetTopoNodeID: fn(COMPTR,*mut u64) -> HRES,
	pub SetTopoNodeID: fn(COMPTR, u64) -> HRES,
	pub GetInputCount: fn(COMPTR, *mut u32) -> HRES,
	pub GetOutputCount: fn(COMPTR, *mut u32) -> HRES,
	pub ConnectOutput: fn(COMPTR, u32, COMPTR, u32) -> HRES,
	pub DisconnectOutput: fn(COMPTR, u32) -> HRES,
	pub GetInput: fn(COMPTR, u32, *mut COMPTR, *mut u32) -> HRES,
	pub GetOutput: fn(COMPTR, u32, *mut COMPTR, *mut u32) -> HRES,
	pub SetOutputPrefType: fn(COMPTR, u32, COMPTR) -> HRES,
	pub GetOutputPrefType: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub SetInputPrefType: fn(COMPTR, u32, COMPTR) -> HRES,
	pub GetInputPrefType: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub CloneFrom: fn(COMPTR, COMPTR) -> HRES,
}

#[repr(C)]
pub struct IMFVideoDisplayControlVT {
	pub IUnknownVT: IUnknownVT,
	pub GetNativeVideoSize: fn(COMPTR, PVOID, PVOID) -> HRES,
	pub GetIdealVideoSize: fn(COMPTR, PVOID, PVOID) -> HRES,
	pub SetVideoPosition: fn(COMPTR, PCVOID, PCVOID) -> HRES,
	pub GetVideoPosition: fn(COMPTR, PVOID, PCVOID) -> HRES,
	pub SetAspectRatioMode: fn(COMPTR, u32) -> HRES,
	pub GetAspectRatioMode: fn(COMPTR, *mut u32) -> HRES,
	pub SetVideoWindow: fn(COMPTR, HANDLE) -> HRES,
	pub GetVideoWindow: fn(COMPTR, *mut HANDLE) -> HRES,
	pub RepaintVideo: fn(COMPTR) -> HRES,
	pub GetCurrentImage: fn(COMPTR, PVOID, *mut *mut u8, *mut u32, *mut i64) -> HRES,
	pub SetBorderColor: fn(COMPTR, u32) -> HRES,
	pub GetBorderColor: fn(COMPTR, *mut u32) -> HRES,
	pub SetRenderingPrefs: fn(COMPTR, u32) -> HRES,
	pub GetRenderingPrefs: fn(COMPTR, *mut u32) -> HRES,
	pub SetFullscreen: fn(COMPTR, BOOL) -> HRES,
	pub GetFullscreen: fn(COMPTR, *mut BOOL) -> HRES,
}
