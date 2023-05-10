#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{BOOL, COMPTR, HRES, PVOID};
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IDataObject`](crate::IDataObject) virtual table.
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

com_interface! { IDataObject: "0000010e-0000-0000-c000-000000000046";
	/// [`IDataObject`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-idataobject)
	/// COM interface over [`IDataObjectVT`](crate::vt::IDataObjectVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IDataObject for IDataObject {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IDataObject`](crate::IDataObject).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IDataObject: ole_IUnknown {

}
