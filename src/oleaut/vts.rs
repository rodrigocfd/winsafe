#![allow(non_snake_case)]

use crate::kernel::ffi_types::*;
use crate::macros::*;
use crate::ole::vts::*;

com_vtbl! { IDispatchVT : IUnknownVT
	GetTypeInfoCount(*mut u32) -> HRES
	GetTypeInfo(u32, u32, *mut COMPTR) -> HRES
	GetIDsOfNames(PCVOID, *const PCSTR, u32, u32, PVOID) -> HRES
	Invoke(i32, PCVOID, u32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRES
}

com_vtbl! { IPropertyStoreVT : IUnknownVT
	GetCount(*mut u32) -> HRES
	GetAt(u32, PVOID) -> HRES
	GetValue(PCVOID, PVOID) -> HRES
	SetValue(PCVOID, PCVOID) -> HRES
	Commit() -> HRES
}

com_vtbl! { ITypeInfoVT : IUnknownVT
	GetTypeAttr(*mut PVOID) -> HRES
	GetTypeComp(*mut COMPTR) -> HRES
	GetFuncDesc(u32, *mut PVOID) -> HRES
	GetVarDesc(u32, *mut PVOID) -> HRES
	GetNames(i32, *mut PSTR, u32, *mut u32) -> HRES
	GetRefTypeOfImplType(u32, *mut u32) -> HRES
	GetImplTypeFlags(u32, *mut i32) -> HRES
	GetIDsOfNames(*const PCSTR, u32, *mut i32) -> HRES
	Invoke(PVOID, i32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRES
	GetDocumentation(i32, *mut PSTR, *mut PSTR, *mut u32, *mut PSTR) -> HRES
	GetDllEntry(i32, u32, *mut PSTR, *mut PSTR, *mut u16) -> HRES
	GetRefTypeInfo(u32, *mut COMPTR) -> HRES
	AddressOfMember(i32, u32, *mut PVOID) -> HRES
	CreateInstance(COMPTR, PCVOID, *mut COMPTR) -> HRES
	GetMops(i32, *mut PSTR) -> HRES
	GetContainingTypeLib(*mut COMPTR, *mut u32) -> HRES
	ReleaseTypeAttr(PVOID) -> HRES
	ReleaseFuncDesc(PVOID) -> HRES
	ReleaseVarDesc(PVOID) -> HRES
}
