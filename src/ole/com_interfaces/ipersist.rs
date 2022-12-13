#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::{HRES, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IPersist`](crate::IPersist) virtual table.
#[repr(C)]
pub struct IPersistVT {
	pub IUnknownVT: IUnknownVT,
	pub GetClassID: fn(ComPtr, PVOID) -> HRES,
}

com_interface! { IPersist: "0000010c-0000-0000-c000-000000000046";
	/// [`IPersist`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist)
	/// COM interface over [`IPersistVT`](crate::vt::IPersistVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPersist for IPersist {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IPersist`](crate::IPersist).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IPersist: ole_IUnknown {
	/// [`IPersist::GetClassID`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersist-getclassid)
	/// method.
	#[must_use]
	fn GetClassID(&self) -> HrResult<co::CLSID> {
		let mut clsid = co::CLSID::new("00000000-0000-0000-0000-000000000000"); // just a placeholder
		unsafe {
			let vt = self.vt_ref::<IPersistVT>();
			ok_to_hrresult(
				(vt.GetClassID)(self.ptr(), &mut clsid as *mut _ as _),
			)
		}.map(|_| clsid)
	}
}
