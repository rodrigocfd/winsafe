#![allow(non_snake_case)]

use crate::IID;
use crate::ffi::{HRESULT, PCVOID, PSTR, PVOID};

/// Type alias to pointer to pointer to a
/// [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal)
/// virtual table.
pub type PPComVT<T> = *mut *mut T;

/// Trait for any
/// [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal)
/// virtual table.
pub trait ComVT {
	/// Returns the COM interface ID.
	fn IID() -> IID;
}

type IUnkPP = PPComVT<IUnknownVT>;

com_virtual_table! { IUnknownVT,
	/// [`IUnknown`](crate::IUnknown) virtual table, base to all COM virtual
	/// tables.
	->
	0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046,

	QueryInterface, fn(IUnkPP, PCVOID, *mut IUnkPP) -> HRESULT
	AddRef, fn(IUnkPP) -> u32
	Release, fn(IUnkPP) -> u32
}

com_virtual_table! { IDispatchVT,
	/// [`IDispatch`](crate::IDispatch) virtual table.
	->
	0x00020400, 0x0000, 0x0000, 0xc000, 0x000000000046,
	IUnknownVT, IUnknownVT

	GetTypeInfoCount, fn(IUnkPP, *mut u32) -> HRESULT
	GetTypeInfo, fn(IUnkPP, u32, u32, *mut IUnkPP) -> HRESULT
	GetIDsOfNames, fn(IUnkPP, PCVOID, PVOID, u32, u32, PVOID) -> HRESULT
	Invoke, fn(IUnkPP, i32, PCVOID, u32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRESULT
}

com_virtual_table! { IPersistVT,
	/// [`IPersist`](crate::IPersist) virtual table.
	->
	0x0000010c, 0x0000, 0x0000, 0xc000, 0x000000000046,
	IUnknownVT, IUnknownVT

	GetClassID, fn(IUnkPP, PVOID) -> HRESULT
}

com_virtual_table! { ITypeInfoVT,
	/// [`ITypeInfo`](crate::ITypeInfo) virtual table.
	->
	0x00020401, 0x0000, 0x0000, 0xc000, 0x000000000046,
	IUnknownVT, IUnknownVT

	GetTypeAttr, fn(IUnkPP, *mut PVOID) -> HRESULT
	GetTypeComp, fn(IUnkPP, *mut IUnkPP) -> HRESULT
	GetFuncDesc, fn(IUnkPP, u32, *mut PVOID) -> HRESULT
	GetVarDesc, fn(IUnkPP, u32, *mut PVOID) -> HRESULT
	GetNames, fn(IUnkPP, i32, *mut PSTR, u32, *mut u32) -> HRESULT
	GetRefTypeOfImplType, fn(IUnkPP, u32, *mut u32) -> HRESULT
	GetImplTypeFlags, fn(IUnkPP, u32, *mut i32) -> HRESULT
	GetIDsOfNames, fn(IUnkPP, *mut PSTR, u32, *mut i32) -> HRESULT
	Invoke, fn(IUnkPP, PVOID, i32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRESULT
	GetDocumentation, fn(IUnkPP, i32, *mut PSTR, *mut PSTR, *mut u32, PSTR) -> HRESULT
	GetDllEntry, fn(IUnkPP, i32, u32, *mut PSTR, *mut PSTR, *mut u16) -> HRESULT
	GetRefTypeInfo, fn(IUnkPP, u32, *mut IUnkPP) -> HRESULT
	AddressOfMember, fn(IUnkPP, i32, u32, *mut PVOID) -> HRESULT
	CreateInstance, fn(IUnkPP, *mut IUnkPP, PCVOID, *mut PVOID) -> HRESULT
	GetMops, fn(IUnkPP, i32, *mut PSTR) -> HRESULT
	GetContainingTypeLib, fn(IUnkPP, *mut IUnkPP, *mut u32) -> HRESULT
	ReleaseTypeAttr, fn(IUnkPP, PVOID) -> HRESULT
	ReleaseFuncDesc, fn(IUnkPP, PVOID) -> HRESULT
	ReleaseVarDesc, fn(IUnkPP, PVOID) -> HRESULT
}
