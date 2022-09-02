#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::LCID;
use crate::kernel::ffi_types::{HRES, PCVOID, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::oleaut::decl::ITypeInfo;
use crate::prelude::ole_IUnknown;
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

com_interface! { IDispatch: "oleaut";
	"00020400-0000-0000-c000-000000000046";
	/// [`IDispatch`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nn-oaidl-idispatch)
	/// COM interface over [`IDispatchVT`](crate::vt::IDispatchVT).
	///
	/// Automatically calls
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl oleaut_IDispatch for IDispatch {}

/// This trait is enabled with the `oleaut` feature, and provides methods for
/// [`IDispatch`](crate::IDispatch).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "oleaut")))]
pub trait oleaut_IDispatch: ole_IUnknown {
	/// [`IDispatch::GetTypeInfoCount`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfocount)
	/// method.
	#[must_use]
	fn GetTypeInfoCount(&self) -> HrResult<u32> {
		let mut count = u32::default();
		unsafe {
			let vt = self.vt_ref::<IDispatchVT>();
			ok_to_hrresult((vt.GetTypeInfoCount)(self.ptr(), &mut count))
		}.map(|_| count)

	}

	/// [`IDispatch::GetTypeInfo`](https://docs.microsoft.com/en-us/windows/win32/api/oaidl/nf-oaidl-idispatch-gettypeinfo)
	/// method.
	#[must_use]
	fn GetTypeInfo(&self, info_type: u32, lcid: LCID) -> HrResult<ITypeInfo> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IDispatchVT>();
			ok_to_hrresult(
				(vt.GetTypeInfo)(
					self.ptr(),
					info_type,
					lcid.0,
					&mut ppv_queried,
				),
			).map(|_| ITypeInfo::from(ppv_queried))
		}
	}
}
