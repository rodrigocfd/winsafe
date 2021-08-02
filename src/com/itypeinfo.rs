#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::{HRESULT, PCVOID, PSTR, PVOID};
use crate::structs::IID;

/// [`ITypeInfo`](crate::ITypeInfo) virtual table.
pub struct ITypeInfoVT {
	pub IUnknownVT: IUnknownVT,
	pub GetTypeAttr: fn(PPVT, *mut PVOID) -> HRESULT,
	pub GetTypeComp: fn(PPVT, *mut PPVT) -> HRESULT,
	pub GetFuncDesc: fn(PPVT, u32, *mut PVOID) -> HRESULT,
	pub GetVarDesc: fn(PPVT, u32, *mut PVOID) -> HRESULT,
	pub GetNames: fn(PPVT, i32, *mut PSTR, u32, *mut u32) -> HRESULT,
	pub GetRefTypeOfImplType: fn(PPVT, u32, *mut u32) -> HRESULT,
	pub GetImplTypeFlags: fn(PPVT, u32, *mut i32) -> HRESULT,
	pub GetIDsOfNames: fn(PPVT, *mut PSTR, u32, *mut i32) -> HRESULT,
	pub Invoke: fn(PPVT, PVOID, i32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRESULT,
	pub GetDocumentation: fn(PPVT, i32, *mut PSTR, *mut PSTR, *mut u32, PSTR) -> HRESULT,
	pub GetDllEntry: fn(PPVT, i32, u32, *mut PSTR, *mut PSTR, *mut u16) -> HRESULT,
	pub GetRefTypeInfo: fn(PPVT, u32, *mut PPVT) -> HRESULT,
	pub AddressOfMember: fn(PPVT, i32, u32, *mut PVOID) -> HRESULT,
	pub CreateInstance: fn(PPVT, *mut PPVT, PCVOID, *mut PVOID) -> HRESULT,
	pub GetMops: fn(PPVT, i32, *mut PSTR) -> HRESULT,
	pub GetContainingTypeLib: fn(PPVT, *mut PPVT, *mut u32) -> HRESULT,
	pub ReleaseTypeAttr: fn(PPVT, PVOID) -> HRESULT,
	pub ReleaseFuncDesc: fn(PPVT, PVOID) -> HRESULT,
	pub ReleaseVarDesc: fn(PPVT, PVOID) -> HRESULT,
}

/// [`ITypeInfo`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-itypeinfo)
/// COM interface over [`ITypeInfoVT`](crate::ITypeInfoVT). Inherits from
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct ITypeInfo {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for ITypeInfo {
	const IID: IID = IID::new(0x00020401, 0x0000, 0x0000, 0xc000, 0x000000000046);
}

macro_rules! impl_ITypeInfo {
	($name:ty, $vt:ty) => {
		use crate::com::IUnknown;

		impl $name {
			fn itypeinfo_vt(&self) -> &ITypeInfoVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`ITypeInfo::CreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-itypeinfo-createinstance)
			/// method.
			pub fn CreateInstance<T: ComInterface>(&self,
				pUnkOuter: Option<&mut IUnknown>) -> WinResult<T>
			{
				let mut ppvQueried: PPVT = std::ptr::null_mut();
				let mut ppvOuter: PPVT = std::ptr::null_mut();

				hr_to_winresult(
					(self.itypeinfo_vt().CreateInstance)(
						self.ppvt,
						pUnkOuter.as_ref()
							.map_or(std::ptr::null_mut(), |_| &mut ppvOuter as *mut _ as _),
						&T::IID as *const _ as _,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| T::from(ppvQueried))
			}
		}
	};
}

impl_IUnknown!(ITypeInfo, ITypeInfoVT);
impl_ITypeInfo!(ITypeInfo, ITypeInfoVT);
