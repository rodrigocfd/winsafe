#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::ffi_types::{HRES, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`IPersist`](crate::IPersist) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct IPersistVT {
	pub IUnknownVT: IUnknownVT,
	pub GetClassID: fn(ComPtr, PVOID) -> HRES,
}

/// [`IPersist`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist)
/// COM interface over [`IPersistVT`](crate::vt::IPersistVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub struct IPersist(ComPtr);

impl_iunknown!(IPersist, "0000010c-0000-0000-c000-000000000046");
impl shell_IPersist for IPersist {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IPersist`](crate::IPersist).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_IPersist: ole_IUnknown {
	/// [`IPersist::GetClassID`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersist-getclassid)
	/// method.
	#[must_use]
	fn GetClassID(&self) -> HrResult<co::CLSID> {
		let mut clsid = co::CLSID::new("00000000-0000-0000-0000-000000000000"); // just a placeholder
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPersistVT);
			ok_to_hrresult(
				(vt.GetClassID)(self.ptr(), &mut clsid as *mut _ as _),
			)
		}.map(|_| clsid)
	}
}
