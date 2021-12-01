#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::autom::itypeinfo::ITypeInfo;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{HRES, PCVOID, PVOID};
use crate::privs::ok_to_hrresult;
use crate::structs::LCID;

/// [`IDispatch`](crate::autom::IDispatch) virtual table.
#[repr(C)]
pub struct IDispatchVT {
	pub IUnknownVT: IUnknownVT,
	pub GetTypeInfoCount: fn(ComPtr, *mut u32) -> HRES,
	pub GetTypeInfo: fn(ComPtr, u32, u32, *mut ComPtr) -> HRES,
	pub GetIDsOfNames: fn(ComPtr, PCVOID, PVOID, u32, u32, PVOID) -> HRES,
	pub Invoke: fn(ComPtr, i32, PCVOID, u32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRES,
}

/// [`IDispatch`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-idispatch)
/// COM interface over [`IDispatchVT`](crate::autom::vt::IDispatchVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IDispatch(ComPtr);

impl_iunknown!(IDispatch, 0x00020400, 0x0000, 0x0000, 0xc000, 0x000000000046);
impl IDispatchT for IDispatch {}

/// Exposes the [`IDispatch`](crate::autom::IDispatch) methods.
pub trait IDispatchT: IUnknownT {
	/// [`IDispatch::GetTypeInfoCount`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfocount)
	/// method.
	fn GetTypeInfoCount(&self) -> HrResult<u32> {
		let mut count = u32::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IDispatchVT);
			ok_to_hrresult((vt.GetTypeInfoCount)(self.ptr(), &mut count))
		}.map(|_| count)

	}

	/// [`IDispatch::GetTypeInfo`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfo)
	/// method.
	fn GetTypeInfo(&self, info_type: u32, lcid: LCID) -> HrResult<ITypeInfo> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IDispatchVT);
			ok_to_hrresult(
				(vt.GetTypeInfo)(
					self.ptr(),
					info_type,
					lcid.0,
					&mut ppv_queried,
				),
			)
		}.map(|_| ITypeInfo::from(ppv_queried))
	}
}
