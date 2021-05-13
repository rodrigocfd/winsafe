//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM virtual tables.

#![allow(non_snake_case)]

use crate::com::{IUnknownVT, PPComVT};
use crate::ffi::{BOOL, HANDLE, HRESULT, PCSTR, PCVOID, PSTR, PVOID};

type IUnkPP = PPComVT<IUnknownVT>;

pub_struct_vtable! { IFileDialogVT,
	/// [`IFileDialog`](crate::shell::IFileDialog) virtual table.
	->
	0x42f85136, 0xdb7e, 0x439c, 0x85f1, 0xe4075d135fc8,
	IModalWindowVT, IModalWindowVT

	SetFileTypes, fn(IUnkPP, u32, PCVOID) -> HRESULT
	SetFileTypeIndex, fn(IUnkPP, u32) -> HRESULT
	GetFileTypeIndex, fn(IUnkPP, *mut u32) -> HRESULT
	Advise, fn(IUnkPP, PVOID, *mut u32) -> HRESULT
	Unadvise, fn(IUnkPP, u32) -> HRESULT
	SetOptions, fn(IUnkPP, u32) -> HRESULT
	GetOptions, fn(IUnkPP, *mut u32) -> HRESULT
	SetDefaultFolder, fn(IUnkPP, IUnkPP) -> HRESULT
	SetFolder, fn(IUnkPP, IUnkPP) -> HRESULT
	GetFolder, fn(IUnkPP, *mut PVOID) -> HRESULT
	GetCurrentSelection, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	SetFileName, fn(IUnkPP, PCSTR) -> HRESULT
	GetFileName, fn(IUnkPP, *mut PSTR) -> HRESULT
	SetTitle, fn(IUnkPP, PCSTR) -> HRESULT
	SetOkButtonLabel, fn(IUnkPP, PCSTR) -> HRESULT
	SetFileNameLabel, fn(IUnkPP, PCSTR) -> HRESULT
	GetResult, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	AddPlace, fn(IUnkPP, PVOID, u32) -> HRESULT
	SetDefaultExtension, fn(IUnkPP, PCSTR) -> HRESULT
	Close, fn(IUnkPP, HRESULT) -> HRESULT
	SetClientGuid, fn(IUnkPP, PCVOID) -> HRESULT
	ClearClientData, fn(IUnkPP) -> HRESULT
  	SetFilter, fn(IUnkPP, PVOID) -> HRESULT
}

pub_struct_vtable! { IFileOpenDialogVT,
	/// [`IFileOpenDialog`](crate::shell::IFileOpenDialog) virtual table.
	->
	0xd57c7288, 0xd4ad, 0x4768, 0xbe02, 0x9d969532d960,
	IFileDialogVT, IFileDialogVT

	GetResults, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	GetSelectedItems, fn(IUnkPP, *mut IUnkPP) -> HRESULT
}

pub_struct_vtable! { IFileSaveDialogVT,
	/// [`IFileSaveDialog`](crate::shell::IFileSaveDialog) virtual table.
	->
	0x84bccd23, 0x5fde, 0x4cdb, 0xaea4, 0xaf64b83d78ab,
	IFileDialogVT, IFileDialogVT

	SetSaveAsItem, fn(IUnkPP, IUnkPP) -> HRESULT
	SetProperties, fn(IUnkPP, IUnkPP) -> HRESULT
	SetCollectedProperties, fn(IUnkPP, IUnkPP, BOOL) -> HRESULT
	GetProperties, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	ApplyProperties, fn(IUnkPP, IUnkPP, IUnkPP, HANDLE, IUnkPP) -> HRESULT
}

pub_struct_vtable! { IModalWindowVT,
	/// [`IModalWindow`](crate::shell::IModalWindow) virtual table.
	->
	0xb4db1657, 0x70d7, 0x485e, 0x8e3e, 0x6fcb5a5c1802,
	IUnknownVT, IUnknownVT

	Show, fn(IUnkPP, HANDLE) -> HRESULT
}

pub_struct_vtable! { IShellItemArrayVT,
	/// [`IShellItemArray`](crate::shell::IShellItemArray) virtual table.
	->
	0xb63ea76d, 0x1f85, 0x456f, 0xa19c, 0x48159efa858b,
	IUnknownVT, IUnknownVT

	BindToHandler, fn(IUnkPP, PVOID, PCVOID, PCVOID, *mut IUnkPP) -> HRESULT
	GetPropertyStore, fn(IUnkPP, u32, PCVOID, *mut IUnkPP) -> HRESULT
	GetPropertyDescriptionList, fn(IUnkPP, PVOID, PCVOID, *mut IUnkPP) -> HRESULT
	GetAttributes, fn(IUnkPP, u32, u32, PVOID) -> HRESULT
	GetCount, fn(IUnkPP, *mut u32) -> HRESULT
	GetItemAt, fn(IUnkPP, u32, *mut IUnkPP) -> HRESULT
	EnumItems, fn(IUnkPP, *mut PVOID) -> HRESULT
}

pub_struct_vtable! { IShellItemVT,
	/// [`IShellItem`](crate::shell::IShellItem) virtual table.
	->
	0x43826d1e, 0xe718, 0x42ee, 0xbc55, 0xa1e261c37bfe,
	IUnknownVT, IUnknownVT

	BindToHandler, fn(IUnkPP, PVOID, PCVOID, PCVOID, *mut IUnkPP) -> HRESULT
	GetParent, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	GetDisplayName, fn(IUnkPP, u32, *mut PSTR) -> HRESULT
	GetAttributes, fn(IUnkPP, u32, *mut u32) -> HRESULT
	Compare, fn(IUnkPP, PVOID, u32, *mut i32) -> HRESULT
}

pub_struct_vtable! { ITaskbarListVT,
	/// [`ITaskbarList`](crate::shell::ITaskbarList) virtual table.
	->
	0x56fdf342, 0xfd6d, 0x11d0, 0x958a, 0x006097c9a090,
	IUnknownVT, IUnknownVT

	HrInit, fn(IUnkPP) -> HRESULT
	AddTab, fn(IUnkPP, HANDLE) -> HRESULT
	DeleteTab, fn(IUnkPP, HANDLE) -> HRESULT
	ActivateTab, fn(IUnkPP, HANDLE) -> HRESULT
	SetActiveAlt, fn(IUnkPP, HANDLE) -> HRESULT
}

pub_struct_vtable! { ITaskbarList2VT,
	/// [`ITaskbarList2`](crate::shell::ITaskbarList2) virtual table.
	->
	0x602d4995, 0xb13a, 0x429b, 0xa66e, 0x1935e44f4317,
	ITaskbarListVT, ITaskbarListVT

	MarkFullscreenWindow, fn(IUnkPP, HANDLE, BOOL) -> HRESULT
}

pub_struct_vtable! { ITaskbarList3VT,
	/// [`ITaskbarList3`](crate::shell::ITaskbarList3) virtual table.
	->
	0xea1afb91, 0x9e28, 0x4b86, 0x90e9, 0x9e9f8a5eefaf,
	ITaskbarList2VT, ITaskbarList2VT

	SetProgressValue, fn(IUnkPP, HANDLE, u64, u64) -> HRESULT
	SetProgressState, fn(IUnkPP, HANDLE, u32) -> HRESULT
	RegisterTab, fn(IUnkPP, HANDLE, HANDLE) -> HRESULT
	UnregisterTab, fn(IUnkPP, HANDLE) -> HRESULT
	SetTabOrder, fn(IUnkPP, HANDLE, HANDLE) -> HRESULT
	SetTabActive, fn(IUnkPP, HANDLE, HANDLE, u32) -> HRESULT
	ThumbBarAddButtons, fn(IUnkPP, HANDLE, u32, PVOID) -> HRESULT
	ThumbBarUpdateButtons, fn(IUnkPP, HANDLE, u32, PVOID) -> HRESULT
	ThumbBarSetImageList, fn(IUnkPP, HANDLE, HANDLE) -> HRESULT
	SetOverlayIcon, fn(IUnkPP, HANDLE, HANDLE, PCSTR) -> HRESULT
	SetThumbnailTooltip, fn(IUnkPP, HANDLE, PCSTR) -> HRESULT
	SetThumbnailClip, fn(IUnkPP, HANDLE, PVOID) -> HRESULT
}
