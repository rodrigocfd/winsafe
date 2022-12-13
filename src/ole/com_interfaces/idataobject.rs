#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{BOOL, HRES, PVOID};
use crate::ole::decl::ComPtr;
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IDataObject`](crate::IDataObject) virtual table.
#[repr(C)]
pub struct IDataObjectVT {
	pub IUnknownVT: IUnknownVT,
	pub GetData: fn(ComPtr, PVOID, PVOID) -> HRES,
	pub GetDataHere: fn(ComPtr, PVOID, PVOID) -> HRES,
	pub QueryGetData: fn(ComPtr, PVOID) -> HRES,
	pub GetCanonicalFormatEtc: fn(ComPtr, PVOID, PVOID) -> HRES,
	pub SetData: fn(ComPtr, PVOID, PVOID, BOOL) -> HRES,
	pub EnumFormatEtc: fn(ComPtr, u32, *mut ComPtr) -> HRES,
	pub DAdvise: fn(ComPtr, PVOID, u32, ComPtr, *mut u32) -> HRES,
	pub DUnadvise: fn(ComPtr, u32) -> HRES,
	pub EnumDAdvise: fn(ComPtr, *mut ComPtr) -> HRES,
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
