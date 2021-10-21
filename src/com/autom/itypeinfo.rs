#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::iunknown::{
	ComInterface,
	ComPtr,
	IUnknown,
	IUnknownT,
	IUnknownVT,
};
use crate::ffi::{HRESULT, PCVOID, PSTR, PVOID};
use crate::privs::hr_to_winresult;

/// [`ITypeInfo`](crate::autom::ITypeInfo) virtual table.
pub struct ITypeInfoVT {
	pub IUnknownVT: IUnknownVT,
	pub GetTypeAttr: fn(ComPtr, *mut PVOID) -> HRESULT,
	pub GetTypeComp: fn(ComPtr, *mut ComPtr) -> HRESULT,
	pub GetFuncDesc: fn(ComPtr, u32, *mut PVOID) -> HRESULT,
	pub GetVarDesc: fn(ComPtr, u32, *mut PVOID) -> HRESULT,
	pub GetNames: fn(ComPtr, i32, *mut PSTR, u32, *mut u32) -> HRESULT,
	pub GetRefTypeOfImplType: fn(ComPtr, u32, *mut u32) -> HRESULT,
	pub GetImplTypeFlags: fn(ComPtr, u32, *mut i32) -> HRESULT,
	pub GetIDsOfNames: fn(ComPtr, *mut PSTR, u32, *mut i32) -> HRESULT,
	pub Invoke: fn(ComPtr, PVOID, i32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRESULT,
	pub GetDocumentation: fn(ComPtr, i32, *mut PSTR, *mut PSTR, *mut u32, PSTR) -> HRESULT,
	pub GetDllEntry: fn(ComPtr, i32, u32, *mut PSTR, *mut PSTR, *mut u16) -> HRESULT,
	pub GetRefTypeInfo: fn(ComPtr, u32, *mut ComPtr) -> HRESULT,
	pub AddressOfMember: fn(ComPtr, i32, u32, *mut PVOID) -> HRESULT,
	pub CreateInstance: fn(ComPtr, *mut ComPtr, PCVOID, *mut PVOID) -> HRESULT,
	pub GetMops: fn(ComPtr, i32, *mut PSTR) -> HRESULT,
	pub GetContainingTypeLib: fn(ComPtr, *mut ComPtr, *mut u32) -> HRESULT,
	pub ReleaseTypeAttr: fn(ComPtr, PVOID) -> HRESULT,
	pub ReleaseFuncDesc: fn(ComPtr, PVOID) -> HRESULT,
	pub ReleaseVarDesc: fn(ComPtr, PVOID) -> HRESULT,
}

/// [`ITypeInfo`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-itypeinfo)
/// COM interface over [`ITypeInfoVT`](crate::autom::vt::ITypeInfoVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct ITypeInfo(ComPtr);

impl_iunknown!(ITypeInfo, 0x00020401, 0x0000, 0x0000, 0xc000, 0x000000000046);
impl ITypeInfoT for ITypeInfo {}

/// Exposes the [`ITypeInfo`](crate::autom::ITypeInfo) methods.
pub trait ITypeInfoT: IUnknownT {
	/// [`ITypeInfo::CreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-itypeinfo-createinstance)
	/// method.
	fn CreateInstance<T: ComInterface>(&self,
		iunk_outer: Option<&mut IUnknown>) -> WinResult<T>
	{
		let mut ppv_queried = ComPtr::null();
		let mut ppv_outer = ComPtr::null();

		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITypeInfoVT);
			hr_to_winresult(
				(vt.CreateInstance)(
					self.ptr(),
					iunk_outer.as_ref()
						.map_or(std::ptr::null_mut(), |_| &mut ppv_outer as *mut _ as _),
					&T::IID as *const _ as _,
					&mut ppv_queried as *mut _ as _,
				),
			)
		}.map(|_| T::from(ppv_queried))
	}
}
