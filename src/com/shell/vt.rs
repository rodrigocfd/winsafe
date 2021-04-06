//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM virtual tables.

#![allow(non_snake_case)]

use crate::com::{ComVT, IUnknownVT, PPComVT};
use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID, PSTR, PVOID};
use crate::structs::IID;

com_virtual_table! { IFileDialogVT,
	/// [`IFileDialog`](crate::shell::IFileDialog) virtual table.
	->
	0x42f85136, 0xdb7e, 0x439c, 0x85f1, 0xe4075d135fc8,
	IModalWindowVT, IModalWindowVT

	SetFileTypes, fn(PPComVT<Self>, u32, PCVOID) -> i32
	SetFileTypeIndex, fn(PPComVT<Self>, u32) -> i32
	GetFileTypeIndex, fn(PPComVT<Self>, *mut u32) -> i32
	Advise, fn(PPComVT<Self>, PVOID, *mut u32) -> i32
	Unadvise, fn(PPComVT<Self>, u32) -> i32
	SetOptions, fn(PPComVT<Self>, u32) -> i32
	GetOptions, fn(PPComVT<Self>, *mut u32) -> i32
	SetDefaultFolder, fn(PPComVT<Self>, PVOID) -> i32
	SetFolder, fn(PPComVT<Self>, PVOID) -> i32
	GetFolder, fn(PPComVT<Self>, *mut PVOID) -> i32
	GetCurrentSelection, fn(PPComVT<Self>, *mut PVOID) -> i32
	SetFileName, fn(PPComVT<Self>, PCSTR) -> i32
	GetFileName, fn(PPComVT<Self>, PSTR) -> i32
	SetTitle, fn(PPComVT<Self>, PCSTR) -> i32
	SetOkButtonLabel, fn(PPComVT<Self>, PCSTR) -> i32
	SetFileNameLabel, fn(PPComVT<Self>, PCSTR) -> i32
	GetResult, fn(PPComVT<Self>, *mut PVOID) -> i32
	AddPlace, fn(PPComVT<Self>, PVOID, u32) -> i32
	SetDefaultExtension, fn(PPComVT<Self>, PCSTR) -> i32
	Close, fn(PPComVT<Self>, i32) -> i32
	SetClientGuid, fn(PPComVT<Self>, PCVOID) -> i32
	ClearClientData, fn(PPComVT<Self>) -> i32
  	SetFilter, fn(PPComVT<Self>, PVOID) -> i32
}

com_virtual_table! { IModalWindowVT,
	/// [`IModalWindow`](crate::shell::IModalWindow) virtual table.
	->
	0xb4db1657, 0x70d7, 0x485e, 0x8e3e, 0x6fcb5a5c1802,
	IUnknownVT, IUnknownVT

	Show, fn(PPComVT<Self>, HANDLE) -> i32
}

com_virtual_table! { ITaskbarListVT,
	/// [`ITaskbarList`](crate::shell::ITaskbarList) virtual table.
	->
	0x56fdf342, 0xfd6d, 0x11d0, 0x958a, 0x006097c9a090,
	IUnknownVT, IUnknownVT

	HrInit, fn(PPComVT<Self>) -> i32
	AddTab, fn(PPComVT<Self>, HANDLE) -> i32
	DeleteTab, fn(PPComVT<Self>, HANDLE) -> i32
	ActivateTab, fn(PPComVT<Self>, HANDLE) -> i32
	SetActiveAlt, fn(PPComVT<Self>, HANDLE) -> i32
}

com_virtual_table! { ITaskbarList2VT,
	/// [`ITaskbarList2`](crate::shell::ITaskbarList2) virtual table.
	->
	0x602d4995, 0xb13a, 0x429b, 0xa66e, 0x1935e44f4317,
	ITaskbarListVT, ITaskbarListVT

	MarkFullscreenWindow, fn(PPComVT<Self>, HANDLE, BOOL) -> i32
}

com_virtual_table! { ITaskbarList3VT,
	/// [`ITaskbarList3`](crate::shell::ITaskbarList3) virtual table.
	->
	0xea1afb91, 0x9e28, 0x4b86, 0x90e9, 0x9e9f8a5eefaf,
	ITaskbarList2VT, ITaskbarList2VT

	SetProgressValue, fn(PPComVT<Self>, HANDLE, u64, u64) -> i32
	SetProgressState, fn(PPComVT<Self>, HANDLE, u32) -> i32
	RegisterTab, fn(PPComVT<Self>, HANDLE, HANDLE) -> i32
	UnregisterTab, fn(PPComVT<Self>, HANDLE) -> i32
	SetTabOrder, fn(PPComVT<Self>, HANDLE, HANDLE) -> i32
	SetTabActive, fn(PPComVT<Self>, HANDLE, HANDLE, u32) -> i32
	ThumbBarAddButtons, fn(PPComVT<Self>, HANDLE, u32, PVOID) -> i32
	ThumbBarUpdateButtons, fn(PPComVT<Self>, HANDLE, u32, PVOID) -> i32
	ThumbBarSetImageList, fn(PPComVT<Self>, HANDLE, HANDLE) -> i32
	SetOverlayIcon, fn(PPComVT<Self>, HANDLE, HANDLE, PCSTR) -> i32
	SetThumbnailTooltip, fn(PPComVT<Self>, HANDLE, PCSTR) -> i32
	SetThumbnailClip, fn(PPComVT<Self>, HANDLE, PVOID) -> i32
}
