#![allow(non_snake_case)]

use crate::ffi_types::{HRES, PCVOID, PSTR, PVOID};
use crate::ole::decl::{ComPtr, HrResult, IUnknown};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{ComInterface, OleIUnknown};
use crate::vt::IUnknownVT;

/// [`ITypeInfo`](crate::ITypeInfo) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
#[repr(C)]
pub struct ITypeInfoVT {
	pub IUnknownVT: IUnknownVT,
	pub GetTypeAttr: fn(ComPtr, *mut PVOID) -> HRES,
	pub GetTypeComp: fn(ComPtr, *mut ComPtr) -> HRES,
	pub GetFuncDesc: fn(ComPtr, u32, *mut PVOID) -> HRES,
	pub GetVarDesc: fn(ComPtr, u32, *mut PVOID) -> HRES,
	pub GetNames: fn(ComPtr, i32, *mut PSTR, u32, *mut u32) -> HRES,
	pub GetRefTypeOfImplType: fn(ComPtr, u32, *mut u32) -> HRES,
	pub GetImplTypeFlags: fn(ComPtr, u32, *mut i32) -> HRES,
	pub GetIDsOfNames: fn(ComPtr, *mut PSTR, u32, *mut i32) -> HRES,
	pub Invoke: fn(ComPtr, PVOID, i32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRES,
	pub GetDocumentation: fn(ComPtr, i32, *mut PSTR, *mut PSTR, *mut u32, PSTR) -> HRES,
	pub GetDllEntry: fn(ComPtr, i32, u32, *mut PSTR, *mut PSTR, *mut u16) -> HRES,
	pub GetRefTypeInfo: fn(ComPtr, u32, *mut ComPtr) -> HRES,
	pub AddressOfMember: fn(ComPtr, i32, u32, *mut PVOID) -> HRES,
	pub CreateInstance: fn(ComPtr, *mut ComPtr, PCVOID, *mut ComPtr) -> HRES,
	pub GetMops: fn(ComPtr, i32, *mut PSTR) -> HRES,
	pub GetContainingTypeLib: fn(ComPtr, *mut ComPtr, *mut u32) -> HRES,
	pub ReleaseTypeAttr: fn(ComPtr, PVOID) -> HRES,
	pub ReleaseFuncDesc: fn(ComPtr, PVOID) -> HRES,
	pub ReleaseVarDesc: fn(ComPtr, PVOID) -> HRES,
}

/// [`ITypeInfo`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-itypeinfo)
/// COM interface over [`ITypeInfoVT`](crate::vt::ITypeInfoVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
pub struct ITypeInfo(ComPtr);

impl_iunknown!(ITypeInfo, "00020401-0000-0000-c000-000000000046");
impl OleautITypeInfo for ITypeInfo {}

/// [`ITypeInfo`](crate::ITypeInfo) methods from `oleaut` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
pub trait OleautITypeInfo: OleIUnknown {
	/// [`ITypeInfo::CreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-itypeinfo-createinstance)
	/// method.
	#[must_use]
	fn CreateInstance<T>(&self, iunk_outer: Option<&mut IUnknown>) -> HrResult<T>
		where T: ComInterface,
	{
		let mut ppv_queried = ComPtr::null();
		let mut ppv_outer = ComPtr::null();

		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut ITypeInfoVT);
			ok_to_hrresult(
				(vt.CreateInstance)(
					self.ptr(),
					iunk_outer.as_ref()
						.map_or(std::ptr::null_mut(), |_| &mut ppv_outer),
					&T::IID as *const _ as _,
					&mut ppv_queried,
				),
			)
		}.map(|_| T::from(ppv_queried))
	}
}
