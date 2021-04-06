//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM virtual tables.

#![allow(non_snake_case)]

use crate::com::{ComVT, IUnknownVT, PPComVT};
use crate::ffi::{BOOL, HANDLE, PCSTR, PCVOID};
use crate::structs::IID;

/// [`IModalWindow`](crate::shell::IModalWindow) virtual table.
#[repr(C)]
pub struct IModalWindowVT {
	pub iUnknownVT: IUnknownVT,

	pub Show: fn(PPComVT<Self>, HANDLE) -> i32,
}

impl_iid!(IModalWindowVT, 0xb4db1657, 0x70d7, 0x485e, 0x8e3e, 0x6fcb5a5c1802);

/// [`ITaskbarList`](crate::shell::ITaskbarList) virtual table.
#[repr(C)]
pub struct ITaskbarListVT {
	pub iUnknownVT: IUnknownVT,

	pub HrInit: fn(PPComVT<Self>) -> i32,
	pub AddTab: fn(PPComVT<Self>, HANDLE) -> i32,
	pub DeleteTab: fn(PPComVT<Self>, HANDLE) -> i32,
	pub ActivateTab: fn(PPComVT<Self>, HANDLE) -> i32,
	pub SetActiveAlt: fn(PPComVT<Self>, HANDLE) -> i32,
}

impl_iid!(ITaskbarListVT, 0x56fdf342, 0xfd6d, 0x11d0, 0x958a, 0x006097c9a090);

/// [`ITaskbarList2`](crate::shell::ITaskbarList2) virtual table.
#[repr(C)]
pub struct ITaskbarList2VT {
	pub ITaskbarListVT: ITaskbarListVT,

	pub MarkFullscreenWindow: fn(PPComVT<Self>, HANDLE, BOOL) -> i32,
}

impl_iid!(ITaskbarList2VT, 0x602d4995, 0xb13a, 0x429b, 0xa66e, 0x1935e44f4317);

/// [`ITaskbarList3`](crate::shell::ITaskbarList3) virtual table.
#[repr(C)]
pub struct ITaskbarList3VT {
	pub iTaskbarList2VT: ITaskbarList2VT,

	pub SetProgressValue: fn(PPComVT<Self>, HANDLE, u64, u64) -> i32,
	pub SetProgressState: fn(PPComVT<Self>, HANDLE, u32) -> i32,
	pub RegisterTab: fn(PPComVT<Self>, HANDLE, HANDLE) -> i32,
	pub UnregisterTab: fn(PPComVT<Self>, HANDLE) -> i32,
	pub SetTabOrder: fn(PPComVT<Self>, HANDLE, HANDLE) -> i32,
	pub SetTabActive: fn(PPComVT<Self>, HANDLE, HANDLE, u32) -> i32,
	pub ThumbBarAddButtons: fn(PPComVT<Self>, HANDLE, u32, PCVOID) -> i32,
	pub ThumbBarUpdateButtons: fn(PPComVT<Self>, HANDLE, u32, PCVOID) -> i32,
	pub ThumbBarSetImageList: fn(PPComVT<Self>, HANDLE, HANDLE) -> i32,
	pub SetOverlayIcon: fn(PPComVT<Self>, HANDLE, HANDLE, PCSTR) -> i32,
	pub SetThumbnailTooltip: fn(PPComVT<Self>, HANDLE, PCSTR) -> i32,
	pub SetThumbnailClip: fn(PPComVT<Self>, HANDLE, PCVOID) -> i32,
}

impl_iid!(ITaskbarList3VT, 0xea1afb91, 0x9e28, 0x4b86, 0x90e9, 0x9e9f8a5eefaf);
