#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::macros::*;

com_vtbl! { IAdviseSinkVT : IUnknownVT
	OnDataChange(PVOID, PVOID)
	OnViewChange(u32, i32)
	OnRename(COMPTR)
	OnSave()
	OnClose()
}

com_vtbl! { IBindCtxVT : IUnknownVT
	RegisterObjectBound(COMPTR) -> HRES
	RevokeObjectBound(COMPTR) -> HRES
	ReleaseBoundObjects() -> HRES
	SetBindOptions(PVOID) -> HRES
	GetBindOptions(PVOID) -> HRES
	GetRunningObjectTable(*mut COMPTR) -> HRES
	RegisterObjectParam(PCSTR, COMPTR) -> HRES
	GetObjectParam(PCSTR, *mut COMPTR) -> HRES
	EnumObjectParam(*mut COMPTR) -> HRES
	RevokeObjectParam(PCSTR) -> HRES
}

com_vtbl! { IDataObjectVT : IUnknownVT
	GetData(PCVOID, PVOID) -> HRES
	GetDataHere(PVOID, PVOID) -> HRES
	QueryGetData(PCVOID) -> HRES
	GetCanonicalFormatEtc(PVOID, PVOID) -> HRES
	SetData(PVOID, PVOID, BOOL) -> HRES
	EnumFormatEtc(u32, *mut COMPTR) -> HRES
	DAdvise(PCVOID, u32, COMPTR, *mut u32) -> HRES
	DUnadvise(u32) -> HRES
	EnumDAdvise(*mut COMPTR) -> HRES
}

com_vtbl! { IDropTargetVT : IUnknownVT
	DragEnter(COMPTR, u32, u64, *mut u32) -> HRES
	DragOver(u32, u64, *mut u32) -> HRES
	DragLeave() -> HRES
	Drop(COMPTR, u32, u64, *mut u32) -> HRES
}

com_vtbl! { IMonikerVT : IPersistStreamVT
	BindToObject(COMPTR, COMPTR, PCVOID, *mut COMPTR) -> HRES
	BindToStorage(COMPTR, COMPTR, PCVOID, *mut COMPTR) -> HRES
	Reduce(COMPTR, u32, *mut COMPTR, *mut COMPTR) -> HRES
	ComposeWith(COMPTR, BOOL, *mut COMPTR) -> HRES
	Enum(BOOL, *mut COMPTR) -> HRES
	IsEqual(COMPTR) -> HRES
	Hash(*mut u32) -> HRES
	IsRunning(COMPTR, COMPTR, COMPTR) -> HRES
	GetTimeOfLastChange(COMPTR, COMPTR, PVOID) -> HRES
	Inverse(*mut COMPTR) -> HRES
	CommonPrefixWith(COMPTR, *mut COMPTR) -> HRES
	RelativePathTo(COMPTR, *mut COMPTR) -> HRES
	GetDisplayName(COMPTR, COMPTR, *mut PSTR) -> HRES
	ParseDisplayName(COMPTR, COMPTR, PCSTR, *mut u32, *mut COMPTR) -> HRES
	IsSystemMoniker(*mut u32) -> HRES
}

com_vtbl! { IPersistVT : IUnknownVT
	GetClassID(PVOID) -> HRES
}

com_vtbl! { IPersistFileVT : IPersistVT
	IsDirty() -> HRES
	Load(PCSTR, u32) -> HRES
	Save(PCSTR, i32) -> HRES
	SaveCompleted(PCSTR) -> HRES
	GetCurFile(*mut PSTR) -> HRES
}

com_vtbl! { IPersistStreamVT : IPersistVT
	IsDirty() -> HRES
	Load(COMPTR) -> HRES
	Save(COMPTR, BOOL) -> HRES
	GetSizeMax(*mut u64) -> HRES
}

com_vtbl! { IPictureVT : IUnknownVT
	get_Handle(*mut u32) -> HRES
	get_hPal(*mut HANDLE) -> HRES
	get_Type(*mut i16) -> HRES
	get_Width(*mut i32) -> HRES
	get_Height(*mut i32) -> HRES
	Render(HANDLE, i32, i32, i32, i32, i32, i32, i32, i32, PCVOID) -> HRES
	set_hPal(HANDLE) -> HRES
	get_CurDC(*mut HANDLE) -> HRES
	SelectPicture(HANDLE, *mut HANDLE, *mut HANDLE) -> HRES
	get_KeepOriginalFormat(*mut BOOL) -> HRES
	put_KeepOriginalFormat(BOOL) -> HRES
	PictureChanged() -> HRES
	SaveAsFile(COMPTR, BOOL, *mut i32) -> HRES
	get_Attributes(*mut u32) -> HRES
}

com_vtbl! { ISequentialStreamVT  : IUnknownVT
	Read(PVOID, u32, *mut u32) -> HRES
	Write(PCVOID, u32, *mut u32) -> HRES
}

com_vtbl! { IStorageVT : IUnknownVT
	CreateStream(PCSTR, u32, u32, u32, *mut COMPTR) -> HRES
	OpenStream(PCSTR, PVOID, u32, u32, *mut COMPTR) -> HRES
	CreateStorage(PCSTR, u32, u32, u32, *mut COMPTR) -> HRES
	OpenStorage(PCSTR, COMPTR, u32, *mut PSTR, u32, *mut COMPTR) -> HRES
	CopyTo(u32, PCVOID, *mut PSTR, COMPTR) -> HRES
	MoveElementTo(PCSTR, COMPTR, PCSTR, u32) -> HRES
	Commit(u32) -> HRES
	Revert() -> HRES
	EnumElements(u32, PVOID, u32, *mut COMPTR) -> HRES
	DestroyElement(PCSTR) -> HRES
	RenameElement(PCSTR, PCSTR) -> HRES
	SetElementTimes(PCSTR, PCVOID, PCVOID, PCVOID) -> HRES
	SetClass(PCVOID) -> HRES
	SetStateBits(u32, u32) -> HRES
	Stat(PVOID, u32) -> HRES
}

com_vtbl! { IStreamVT : ISequentialStreamVT
	Seek(i64, u32, *mut u64) -> HRES
	SetSize(u64) -> HRES
	CopyTo(COMPTR, u64, *mut u64, *mut u64) -> HRES
	Commit(u32) -> HRES
	Revert() -> HRES
	LockRegion(u64, u64, u32) -> HRES
	UnlockRegion(u64, u64, u32) -> HRES
	Stat(PVOID, u32) -> HRES
	Clone(*mut COMPTR) -> HRES
}

com_vtbl! { IUnknownVT
	QueryInterface(PCVOID, *mut COMPTR) -> HRES
	AddRef() -> u32
	Release() -> u32
}
