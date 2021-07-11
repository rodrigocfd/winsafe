#![allow(non_snake_case)]

// use crate::com::itypeinfo::ITypeInfoVT;
use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPI};
use crate::ffi::{HRESULT, PCVOID, PVOID};
use crate::structs::IID;

/// [`IDispatch`](crate::IDispatch) virtual table.
pub struct IDispatchVT {
	pub IUnknownVT: IUnknownVT,
	pub GetTypeInfoCount: fn(PPI, *mut u32) -> HRESULT,
	pub GetTypeInfo: fn(PPI, u32, u32, *mut PPI) -> HRESULT,
	pub GetIDsOfNames: fn(PPI, PCVOID, PVOID, u32, u32, PVOID) -> HRESULT,
	pub Invoke: fn(PPI, i32, PCVOID, u32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRESULT,
}

/// [`IDispatch`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-idispatch)
/// COM interface over [`IDispatchVT`](crate::IDispatchVT). Inherits from
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IDispatch {
	pub(crate) ppvt: PPI,
}

impl_send_sync_fromppvt!(IDispatch);

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
			pub fn GetTypeInfo(&self, iTInfo: u32, lcid: LCID) -> WinResult<ITypeInfo> {
				let mut ppvQueried: PPI = std::ptr::null_mut();
				hr_to_winresult(
					(self.idispatch_vt().GetTypeInfo)(
						self.ppvt,
						iTInfo,
						lcid.0,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| ITypeInfo::from(ppvQueried))
			}
		}
	};
}

impl_IUnknown!(IDispatch, IDispatchVT);
impl_IDispatch!(IDispatch, IDispatchVT);
