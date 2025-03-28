#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;

#[repr(C)]
pub struct IAdviseSinkVT {
	pub IUnknownVT: IUnknownVT,
	pub OnDataChange: fn(COMPTR, PVOID, PVOID),
	pub OnViewChange: fn(COMPTR, u32, i32),
	pub OnRename: fn(COMPTR, COMPTR),
	pub OnSave: fn(COMPTR),
	pub OnClose: fn(COMPTR),
}

#[repr(C)]
pub struct IBindCtxVT {
	pub IUnknownVT: IUnknownVT,
	pub RegisterObjectBound: fn(COMPTR, COMPTR) -> HRES,
	pub RevokeObjectBound: fn(COMPTR, COMPTR) -> HRES,
	pub ReleaseBoundObjects: fn(COMPTR) -> HRES,
	pub SetBindOptions: fn(COMPTR, PVOID) -> HRES,
	pub GetBindOptions: fn(COMPTR, PVOID) -> HRES,
	pub GetRunningObjectTable: fn(COMPTR, *mut COMPTR) -> HRES,
	pub RegisterObjectParam: fn(COMPTR, PCSTR, COMPTR) -> HRES,
	pub GetObjectParam: fn(COMPTR, PCSTR, *mut COMPTR) -> HRES,
	pub EnumObjectParam: fn(COMPTR, *mut COMPTR) -> HRES,
	pub RevokeObjectParam: fn(COMPTR, PCSTR) -> HRES,
}

#[repr(C)]
pub struct IDataObjectVT {
	pub IUnknownVT: IUnknownVT,
	pub GetData: fn(COMPTR, PVOID, PVOID) -> HRES,
	pub GetDataHere: fn(COMPTR, PVOID, PVOID) -> HRES,
	pub QueryGetData: fn(COMPTR, PVOID) -> HRES,
	pub GetCanonicalFormatEtc: fn(COMPTR, PVOID, PVOID) -> HRES,
	pub SetData: fn(COMPTR, PVOID, PVOID, BOOL) -> HRES,
	pub EnumFormatEtc: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub DAdvise: fn(COMPTR, PVOID, u32, COMPTR, *mut u32) -> HRES,
	pub DUnadvise: fn(COMPTR, u32) -> HRES,
	pub EnumDAdvise: fn(COMPTR, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IDropTargetVT {
	pub IUnknownVT: IUnknownVT,
	pub DragEnter: fn(COMPTR, COMPTR, u32, u64, *mut u32) -> HRES,
	pub DragOver: fn(COMPTR, u32, u64, *mut u32) -> HRES,
	pub DragLeave: fn(COMPTR) -> HRES,
	pub Drop: fn(COMPTR, COMPTR, u32, u64, *mut u32) -> HRES,
}

#[repr(C)]
pub struct IMonikerVT {
	pub IPersistStreamVT: IPersistStreamVT,
	pub BindToObject: fn(COMPTR, COMPTR, COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub BindToStorage: fn(COMPTR, COMPTR, COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub Reduce: fn(COMPTR, COMPTR, u32, *mut COMPTR, *mut COMPTR) -> HRES,
	pub ComposeWith: fn(COMPTR, COMPTR, BOOL, *mut COMPTR) -> HRES,
	pub Enum: fn(COMPTR, BOOL, *mut COMPTR) -> HRES,
	pub IsEqual: fn(COMPTR, COMPTR) -> HRES,
	pub Hash: fn(COMPTR, *mut u32) -> HRES,
	pub IsRunning: fn(COMPTR, COMPTR, COMPTR, COMPTR) -> HRES,
	pub GetTimeOfLastChange: fn(COMPTR, COMPTR, COMPTR, PVOID) -> HRES,
	pub Inverse: fn(COMPTR, *mut COMPTR) -> HRES,
	pub CommonPrefixWith: fn(COMPTR, COMPTR, *mut COMPTR) -> HRES,
	pub RelativePathTo: fn(COMPTR, COMPTR, *mut COMPTR) -> HRES,
	pub GetDisplayName: fn(COMPTR, COMPTR, COMPTR, *mut PSTR) -> HRES,
	pub ParseDisplayName: fn(COMPTR, COMPTR, COMPTR, PCSTR, *mut u32, *mut COMPTR) -> HRES,
	pub IsSystemMoniker: fn(COMPTR, *mut u32) -> HRES,
}

#[repr(C)]
pub struct IPersistVT {
	pub IUnknownVT: IUnknownVT,
	pub GetClassID: fn(COMPTR, PVOID) -> HRES,
}

#[repr(C)]
pub struct IPersistFileVT {
	pub IPersistVT: IPersistVT,
	pub IsDirty: fn(COMPTR) -> HRES,
	pub Load: fn(COMPTR, PCSTR, u32) -> HRES,
	pub Save: fn(COMPTR, PCSTR, i32) -> HRES,
	pub SaveCompleted: fn(COMPTR, PCSTR) -> HRES,
	pub GetCurFile: fn(COMPTR, *mut PSTR) -> HRES,
}

#[repr(C)]
pub struct IPersistStreamVT {
	pub IPersistVT: IPersistVT,
	pub IsDirty: fn(COMPTR) -> HRES,
	pub Load: fn(COMPTR, COMPTR) -> HRES,
	pub Save: fn(COMPTR, COMPTR, BOOL) -> HRES,
	pub GetSizeMax: fn(COMPTR, *mut u64) -> HRES,
}

#[repr(C)]
pub struct IPictureVT {
	pub IUnknownVT: IUnknownVT,
	pub get_Handle: fn(COMPTR, *mut u32) -> HRES,
	pub get_hPal: fn(COMPTR, *mut HANDLE) -> HRES,
	pub get_Type: fn(COMPTR, *mut i16) -> HRES,
	pub get_Width: fn(COMPTR, *mut i32) -> HRES,
	pub get_Height: fn(COMPTR, *mut i32) -> HRES,
	pub Render: fn(COMPTR, HANDLE, i32, i32, i32, i32, i32, i32, i32, i32, PCVOID) -> HRES,
	pub set_hPal: fn(COMPTR, HANDLE) -> HRES,
	pub get_CurDC: fn(COMPTR, *mut HANDLE) -> HRES,
	pub SelectPicture: fn(COMPTR, HANDLE, *mut HANDLE, *mut HANDLE) -> HRES,
	pub get_KeepOriginalFormat: fn(COMPTR, *mut BOOL) -> HRES,
	pub put_KeepOriginalFormat: fn(COMPTR, BOOL) -> HRES,
	pub PictureChanged: fn(COMPTR) -> HRES,
	pub SaveAsFile: fn(COMPTR, COMPTR, BOOL, *mut i32) -> HRES,
	pub get_Attributes: fn(COMPTR, *mut u32) -> HRES,
}

#[repr(C)]
pub struct ISequentialStreamVT {
	pub IUnknownVT: IUnknownVT,
	pub Read: fn(COMPTR, PVOID, u32, *mut u32) -> HRES,
	pub Write: fn(COMPTR, PCVOID, u32, *mut u32) -> HRES,
}

#[repr(C)]
pub struct IStorageVT {
	pub IUnknownVT: IUnknownVT,
	pub CreateStream: fn(COMPTR, PCSTR, u32, u32, u32, *mut COMPTR) -> HRES,
	pub OpenStream: fn(COMPTR, PCSTR, PVOID, u32, u32, *mut COMPTR) -> HRES,
	pub CreateStorage: fn(COMPTR, PCSTR, u32, u32, u32, *mut COMPTR) -> HRES,
	pub OpenStorage: fn(COMPTR, PCSTR, COMPTR, u32, *mut PSTR, u32, *mut COMPTR) -> HRES,
	pub CopyTo: fn(COMPTR, u32, PCVOID, *mut PSTR, COMPTR) -> HRES,
	pub MoveElementTo: fn(COMPTR, PCSTR, COMPTR, PCSTR, u32) -> HRES,
	pub Commit: fn(COMPTR, u32) -> HRES,
	pub Revert: fn(COMPTR) -> HRES,
	pub EnumElements: fn(COMPTR, u32, PVOID, u32, *mut COMPTR) -> HRES,
	pub DestroyElement: fn(COMPTR, PCSTR) -> HRES,
	pub RenameElement: fn(COMPTR, PCSTR, PCSTR) -> HRES,
	pub SetElementTimes: fn(COMPTR, PCSTR, PCVOID, PCVOID, PCVOID) -> HRES,
	pub SetClass: fn(COMPTR, PCVOID) -> HRES,
	pub SetStateBits: fn(COMPTR, u32, u32) -> HRES,
	pub Stat: fn(COMPTR, PVOID, u32) -> HRES,
}

#[repr(C)]
pub struct IStreamVT {
	pub ISequentialStreamVT: ISequentialStreamVT,
	pub Seek: fn(COMPTR, i64, u32, *mut u64) -> HRES,
	pub SetSize: fn(COMPTR, u64) -> HRES,
	pub CopyTo: fn(COMPTR, COMPTR, u64, *mut u64, *mut u64) -> HRES,
	pub Commit: fn(COMPTR, u32) -> HRES,
	pub Revert: fn(COMPTR) -> HRES,
	pub LockRegion: fn(COMPTR, u64, u64, u32) -> HRES,
	pub UnlockRegion: fn(COMPTR, u64, u64, u32) -> HRES,
	pub Stat: fn(COMPTR, PVOID, u32) -> HRES,
	pub Clone: fn(COMPTR, *mut COMPTR) -> HRES,
}

#[repr(C)]
pub struct IUnknownVT {
	pub QueryInterface: fn(COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub AddRef: fn(COMPTR) -> u32,
	pub Release: fn(COMPTR) -> u32,
}
