#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{HRES, PVOID};
use crate::privs::ok_to_hrresult;
use crate::structs::CLSID;

/// [`IPersist`](crate::idl::IPersist) virtual table.
#[repr(C)]
pub struct IPersistVT {
	pub IUnknownVT: IUnknownVT,
	pub GetClassID: fn(ComPtr, PVOID) -> HRES,
}

/// [`IPersist`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist)
/// COM interface over [`IPersistVT`](crate::idl::vt::IPersistVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IPersist(ComPtr);

impl_iunknown!(IPersist, 0x0000010c, 0x0000, 0x0000, 0xc000, 0x000000000046);
impl IPersistT for IPersist {}

/// Exposes the [`IPersist`](crate::idl::IPersist) methods.
pub trait IPersistT: IUnknownT {
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
