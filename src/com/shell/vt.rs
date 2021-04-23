//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM virtual tables.

#![allow(non_snake_case)]

use crate::com::{ComVT, IUnknownVT, PPComVT};
use crate::ffi::{BOOL, HANDLE, HRESULT, PCSTR, PCVOID, PSTR, PVOID};
use crate::structs::IID;

com_virtual_table! { IFileDialogVT,
	/// [`IFileDialog`](crate::shell::IFileDialog) virtual table.
	->
	0x42f85136, 0xdb7e, 0x439c, 0x85f1, 0xe4075d135fc8,
	IModalWindowVT, IModalWindowVT

	SetFileTypes, fn(PPComVT<Self>, u32, PCVOID) -> HRESULT
	SetFileTypeIndex, fn(PPComVT<Self>, u32) -> HRESULT
	GetFileTypeIndex, fn(PPComVT<Self>, *mut u32) -> HRESULT
	Advise, fn(PPComVT<Self>, PVOID, *mut u32) -> HRESULT
	Unadvise, fn(PPComVT<Self>, u32) -> HRESULT
	SetOptions, fn(PPComVT<Self>, u32) -> HRESULT
	GetOptions, fn(PPComVT<Self>, *mut u32) -> HRESULT
	SetDefaultFolder, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> HRESULT
	SetFolder, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> HRESULT
	GetFolder, fn(PPComVT<Self>, *mut PVOID) -> HRESULT
	GetCurrentSelection, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
	SetFileName, fn(PPComVT<Self>, PCSTR) -> HRESULT
	GetFileName, fn(PPComVT<Self>, *mut PSTR) -> HRESULT
	SetTitle, fn(PPComVT<Self>, PCSTR) -> HRESULT
	SetOkButtonLabel, fn(PPComVT<Self>, PCSTR) -> HRESULT
	SetFileNameLabel, fn(PPComVT<Self>, PCSTR) -> HRESULT
	GetResult, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
	AddPlace, fn(PPComVT<Self>, PVOID, u32) -> HRESULT
	SetDefaultExtension, fn(PPComVT<Self>, PCSTR) -> HRESULT
	Close, fn(PPComVT<Self>, HRESULT) -> HRESULT
	SetClientGuid, fn(PPComVT<Self>, PCVOID) -> HRESULT
	ClearClientData, fn(PPComVT<Self>) -> HRESULT
  	SetFilter, fn(PPComVT<Self>, PVOID) -> HRESULT
}

com_virtual_table! { IFileOpenDialogVT,
	/// [`IFileOpenDialog`](crate::shell::IFileOpenDialog) virtual table.
	->
	0xd57c7288, 0xd4ad, 0x4768, 0xbe02, 0x9d969532d960,
	IFileDialogVT, IFileDialogVT

	GetResults, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
	GetSelectedItems, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
}

com_virtual_table! { IFileSaveDialogVT,
	/// [`IFileSaveDialog`](crate::shell::IFileSaveDialog) virtual table.
	->
	0x84bccd23, 0x5fde, 0x4cdb, 0xaea4, 0xaf64b83d78ab,
	IFileDialogVT, IFileDialogVT

	SetSaveAsItem, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> HRESULT
	SetProperties, fn(PPComVT<Self>, PPComVT<IUnknownVT>) -> HRESULT
	SetCollectedProperties, fn(PPComVT<Self>, PPComVT<IUnknownVT>, BOOL) -> HRESULT
	GetProperties, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
	ApplyProperties, fn(PPComVT<Self>, PPComVT<IUnknownVT>, PPComVT<IUnknownVT>, HANDLE, PPComVT<IUnknownVT>) -> HRESULT
}

com_virtual_table! { IModalWindowVT,
	/// [`IModalWindow`](crate::shell::IModalWindow) virtual table.
	->
	0xb4db1657, 0x70d7, 0x485e, 0x8e3e, 0x6fcb5a5c1802,
	IUnknownVT, IUnknownVT

	Show, fn(PPComVT<Self>, HANDLE) -> HRESULT
}

com_virtual_table! { IShellItemArrayVT,
	/// [`IShellItemArray`](crate::shell::IShellItemArray) virtual table.
	->
	0xb63ea76d, 0x1f85, 0x456f, 0xa19c, 0x48159efa858b,
	IUnknownVT, IUnknownVT

	BindToHandler, fn(PPComVT<Self>, PVOID, PCVOID, PCVOID, *mut PPComVT<IUnknownVT>) -> HRESULT
	GetPropertyStore, fn(PPComVT<Self>, u32, PCVOID, *mut PPComVT<IUnknownVT>) -> HRESULT
	GetPropertyDescriptionList, fn(PPComVT<Self>, PVOID, PCVOID, *mut PPComVT<IUnknownVT>) -> HRESULT
	GetAttributes, fn(PPComVT<Self>, u32, u32, PVOID) -> HRESULT
	GetCount, fn(PPComVT<Self>, *mut u32) -> HRESULT
	GetItemAt, fn(PPComVT<Self>, u32, *mut PPComVT<IUnknownVT>) -> HRESULT
	EnumItems, fn(PPComVT<Self>, *mut PVOID) -> HRESULT
}

com_virtual_table! { IShellItemVT,
	/// [`IShellItem`](crate::shell::IShellItem) virtual table.
	->
	0x43826d1e, 0xe718, 0x42ee, 0xbc55, 0xa1e261c37bfe,
	IUnknownVT, IUnknownVT

	BindToHandler, fn(PPComVT<Self>, PVOID, PCVOID, PCVOID, *mut PPComVT<IUnknownVT>) -> HRESULT
	GetParent, fn(PPComVT<Self>, *mut PPComVT<IUnknownVT>) -> HRESULT
	GetDisplayName, fn(PPComVT<Self>, u32, *mut PSTR) -> HRESULT
	GetAttributes, fn(PPComVT<Self>, u32, *mut u32) -> HRESULT
	Compare, fn(PPComVT<Self>, PVOID, u32, *mut i32) -> HRESULT
}

com_virtual_table! { ITaskbarListVT,
	/// [`ITaskbarList`](crate::shell::ITaskbarList) virtual table.
	->
	0x56fdf342, 0xfd6d, 0x11d0, 0x958a, 0x006097c9a090,
	IUnknownVT, IUnknownVT

	HrInit, fn(PPComVT<Self>) -> HRESULT
	AddTab, fn(PPComVT<Self>, HANDLE) -> HRESULT
	DeleteTab, fn(PPComVT<Self>, HANDLE) -> HRESULT
	ActivateTab, fn(PPComVT<Self>, HANDLE) -> HRESULT
	SetActiveAlt, fn(PPComVT<Self>, HANDLE) -> HRESULT
}

com_virtual_table! { ITaskbarList2VT,
	/// [`ITaskbarList2`](crate::shell::ITaskbarList2) virtual table.
	->
	0x602d4995, 0xb13a, 0x429b, 0xa66e, 0x1935e44f4317,
	ITaskbarListVT, ITaskbarListVT

	MarkFullscreenWindow, fn(PPComVT<Self>, HANDLE, BOOL) -> HRESULT
}

com_virtual_table! { ITaskbarList3VT,
	/// [`ITaskbarList3`](crate::shell::ITaskbarList3) virtual table.
	->
	0xea1afb91, 0x9e28, 0x4b86, 0x90e9, 0x9e9f8a5eefaf,
	ITaskbarList2VT, ITaskbarList2VT

	SetProgressValue, fn(PPComVT<Self>, HANDLE, u64, u64) -> HRESULT
	SetProgressState, fn(PPComVT<Self>, HANDLE, u32) -> HRESULT
	RegisterTab, fn(PPComVT<Self>, HANDLE, HANDLE) -> HRESULT
	UnregisterTab, fn(PPComVT<Self>, HANDLE) -> HRESULT
	SetTabOrder, fn(PPComVT<Self>, HANDLE, HANDLE) -> HRESULT
	SetTabActive, fn(PPComVT<Self>, HANDLE, HANDLE, u32) -> HRESULT
	ThumbBarAddButtons, fn(PPComVT<Self>, HANDLE, u32, PVOID) -> HRESULT
	ThumbBarUpdateButtons, fn(PPComVT<Self>, HANDLE, u32, PVOID) -> HRESULT
	ThumbBarSetImageList, fn(PPComVT<Self>, HANDLE, HANDLE) -> HRESULT
	SetOverlayIcon, fn(PPComVT<Self>, HANDLE, HANDLE, PCSTR) -> HRESULT
	SetThumbnailTooltip, fn(PPComVT<Self>, HANDLE, PCSTR) -> HRESULT
	SetThumbnailClip, fn(PPComVT<Self>, HANDLE, PVOID) -> HRESULT
}
