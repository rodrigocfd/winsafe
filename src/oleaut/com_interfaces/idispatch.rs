#![allow(non_snake_case)]

use crate::ffi_types::{HRES, PCVOID, PVOID};
use crate::kernel::decl::LCID;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::ITypeInfo;
use crate::prelude::OleIUnknown;
use crate::vt::IUnknownVT;

/// [`IDispatch`](crate::IDispatch) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
#[repr(C)]
pub struct IDispatchVT {
	pub IUnknownVT: IUnknownVT,
	pub GetTypeInfoCount: fn(ComPtr, *mut u32) -> HRES,
	pub GetTypeInfo: fn(ComPtr, u32, u32, *mut ComPtr) -> HRES,
	pub GetIDsOfNames: fn(ComPtr, PCVOID, PVOID, u32, u32, PVOID) -> HRES,
	pub Invoke: fn(ComPtr, i32, PCVOID, u32, u16, PVOID, PVOID, PVOID, *mut u32) -> HRES,
}

/// [`IDispatch`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-idispatch)
/// COM interface over [`IDispatchVT`](crate::vt::IDispatchVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
pub struct IDispatch(ComPtr);

impl_iunknown!(IDispatch, "00020400-0000-0000-c000-000000000046");
impl OleautIDispatch for IDispatch {}

/// [`IDispatch`](crate::IDispatch) methods from `oleaut` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
pub trait OleautIDispatch: OleIUnknown {
	/// [`IDispatch::GetTypeInfoCount`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfocount)
	/// method.
	#[must_use]
	fn GetTypeInfoCount(&self) -> HrResult<u32> {
		let mut count = u32::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IDispatchVT);
			ok_to_hrresult((vt.GetTypeInfoCount)(self.ptr(), &mut count))
		}.map(|_| count)

	}

	/// [`IDispatch::GetTypeInfo`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfo)
	/// method.
	#[must_use]
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
