#![allow(non_snake_case)]

use crate::ffi_types::{HRES, PVOID};
use crate::ole::decl::{CLSID, ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::OleIUnknown;
use crate::vt::IUnknownVT;

/// [`IPersist`](crate::IPersist) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shlwapi")))]
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
#[cfg_attr(docsrs, doc(cfg(feature = "shlwapi")))]
pub struct IPersist(ComPtr);

impl_iunknown!(IPersist, 0x0000010c, 0x0000, 0x0000, 0xc000, 0x000000000046);
impl ShlwapiIPersist for IPersist {}

/// [`IPersist`](crate::IPersist) methods from `shlwapi` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "shlwapi")))]
pub trait ShlwapiIPersist: OleIUnknown {
	/// [`IPersist::GetClassID`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersist-getclassid)
	/// method.
	fn GetClassID(&self) -> HrResult<CLSID> {
		let mut clsid = CLSID::new(0, 0, 0, 0, 0);
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPersistVT);
			ok_to_hrresult(
				(vt.GetClassID)(self.ptr(), &mut clsid as *mut _ as _),
			)
		}.map(|_| clsid)
	}
}
