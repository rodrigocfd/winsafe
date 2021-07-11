//! [Shell](https://docs.microsoft.com/en-us/windows/win32/api/_shell/)
//! COM virtual tables.

#![allow(non_snake_case)]

use crate::com::{IUnknownVT, PPComVT};
use crate::ffi::{BOOL, HANDLE, HRESULT, PCSTR, PCVOID, PSTR, PVOID};



type IUnkPP = PPComVT<IUnknownVT>;

pub_struct_vtable! { IFileDialogVT,

	=>
	,



}

pub_struct_vtable! { IFileOpenDialogVT,

	=>
	,



}

pub_struct_vtable! { IFileSaveDialogVT,

	=>
	,



}

pub_struct_vtable! { IModalWindowVT,

	=>
	,



}

pub_struct_vtable! { IShellItemArrayVT,

	=>
	,



}

pub_struct_vtable! { IShellItemVT,

	=>
	,



}

pub_struct_vtable! { ITaskbarListVT,

	=>
	,



}

pub_struct_vtable! { ITaskbarList2VT,

	=>
	,



}

pub_struct_vtable! { ITaskbarList3VT,

	=>
	,



}

pub_struct_vtable! { ITaskbarList4VT,

	=>
	,



}
