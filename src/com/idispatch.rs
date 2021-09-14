#![allow(non_snake_case)]

// use crate::com::itypeinfo::ITypeInfoVT;
use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::{HRESULT, PCVOID, PVOID};
use crate::structs::IID;

/// [`IDispatch`](crate::IDispatch) virtual table.
pub struct IDispatchVT {
	pub IUnknownVT: IUnknownVT,
	pub GetTypeInfoCount: fn(PPVT, *mut u32) -> HRESULT,
	pub GetTypeInfo: fn(PPVT, u32, u32, *mut PPVT) -> HRESULT,
	pub GetIDsOfNames: fn(PPVT, PCVOID, PVOID, u32, u32, PVOID) -> HRESULT,
	pub Invoke: fn(PPVT, i32, PCVOID, u32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRESULT,
}

/// [`IDispatch`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-idispatch)
/// COM interface over [`IDispatchVT`](crate::IDispatchVT). Inherits from
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IDispatch {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for IDispatch {
	const IID: IID = IID::new(0x00020400, 0x0000, 0x0000, 0xc000, 0x000000000046);
}

macro_rules! impl_IDispatch {
	($name:ty, $vt:ty) => {
		use crate::com::ITypeInfo;
		use crate::structs::LCID;

		impl $name {
			fn idispatch_vt(&self) -> &IDispatchVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`IDispatch::GetTypeInfoCount`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfocount)
			/// method.
			pub fn GetTypeInfoCount(&self) -> WinResult<u32> {
				let mut count: u32 = 0;
				hr_to_winresult(
					(self.idispatch_vt().GetTypeInfoCount)(self.ppvt, &mut count),
				).map(|_| count)
			}

			/// [`IDispatch::GetTypeInfo`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfo)
			/// method.
			pub fn GetTypeInfo(&self,
				info_type: u32, lcid: LCID) -> WinResult<ITypeInfo>
			{
				let mut ppv_queried: PPVT = std::ptr::null_mut();
				hr_to_winresult(
					(self.idispatch_vt().GetTypeInfo)(
						self.ppvt,
						info_type,
						lcid.0,
						&mut ppv_queried as *mut _ as _,
					),
				).map(|_| ITypeInfo::from(ppv_queried))
			}
		}
	};
}

impl_IUnknown!(IDispatch, IDispatchVT);
impl_IDispatch!(IDispatch, IDispatchVT);
