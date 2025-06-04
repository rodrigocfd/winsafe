#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::ole::vts::*;

#[repr(C)]
pub struct IDispatchVT {
	pub IUnknownVT: IUnknownVT,
	pub GetTypeInfoCount: fn(COMPTR, *mut u32) -> HRES,
	pub GetTypeInfo: fn(COMPTR, u32, u32, *mut COMPTR) -> HRES,
	pub GetIDsOfNames: fn(COMPTR, PCVOID, *const PCSTR, u32, u32, PVOID) -> HRES,
	pub Invoke: fn(COMPTR, i32, PCVOID, u32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRES,
}

#[repr(C)]
pub struct IPropertyStoreVT {
	pub IUnknownVT: IUnknownVT,
	pub GetCount: fn(COMPTR, *mut u32) -> HRES,
	pub GetAt: fn(COMPTR, u32, PVOID) -> HRES,
	pub GetValue: fn(COMPTR, PCVOID, PVOID) -> HRES,
	pub SetValue: fn(COMPTR, PCVOID, PCVOID) -> HRES,
	pub Commit: fn(COMPTR) -> HRES,
}

#[repr(C)]
pub struct ITypeInfoVT {
	pub IUnknownVT: IUnknownVT,
	pub GetTypeAttr: fn(COMPTR, *mut PVOID) -> HRES,
	pub GetTypeComp: fn(COMPTR, *mut COMPTR) -> HRES,
	pub GetFuncDesc: fn(COMPTR, u32, *mut PVOID) -> HRES,
	pub GetVarDesc: fn(COMPTR, u32, *mut PVOID) -> HRES,
	pub GetNames: fn(COMPTR, i32, *mut PSTR, u32, *mut u32) -> HRES,
	pub GetRefTypeOfImplType: fn(COMPTR, u32, *mut u32) -> HRES,
	pub GetImplTypeFlags: fn(COMPTR, u32, *mut i32) -> HRES,
	pub GetIDsOfNames: fn(COMPTR, *const PCSTR, u32, *mut i32) -> HRES,
	pub Invoke: fn(COMPTR, PVOID, i32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRES,
	pub GetDocumentation: fn(COMPTR, i32, *mut PSTR, *mut PSTR, *mut u32, *mut PSTR) -> HRES,
	pub GetDllEntry: fn(COMPTR, i32, u32, *mut PSTR, *mut PSTR, *mut u16) -> HRES,
	pub GetRefTypeInfo: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub AddressOfMember: fn(COMPTR, i32, u32, *mut PVOID) -> HRES,
	pub CreateInstance: fn(COMPTR, COMPTR, PCVOID, *mut COMPTR) -> HRES,
	pub GetMops: fn(COMPTR, i32, *mut PSTR) -> HRES,
	pub GetContainingTypeLib: fn(COMPTR, *mut COMPTR, *mut u32) -> HRES,
	pub ReleaseTypeAttr: fn(COMPTR, PVOID) -> HRES,
	pub ReleaseFuncDesc: fn(COMPTR, PVOID) -> HRES,
	pub ReleaseVarDesc: fn(COMPTR, PVOID) -> HRES,
}
