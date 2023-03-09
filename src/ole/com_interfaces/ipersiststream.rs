#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{BOOL, HRES};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::{ole_IPersist, ole_IStream};
use crate::vt::IPersistVT;

/// [`IPersistStream`](crate::IPersistStream) virtual table.
#[repr(C)]
pub struct IPersistStreamVT {
	pub IPersistVT: IPersistVT,
	pub IsDirty: fn(ComPtr) -> HRES,
	pub Load: fn(ComPtr, ComPtr) -> HRES,
	pub Save: fn(ComPtr, ComPtr, BOOL) -> HRES,
	pub GetSizeMax: fn(ComPtr, *mut u64) -> HRES,
}

com_interface! { IPersistStream: "00000109-0000-0000-c000-000000000046";
	/// [`IPersistStream`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersiststream)
	/// COM interface over [`IPersistStreamVT`](crate::vt::IPersistStreamVT).
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_IPersist for IPersistStream {}
impl ole_IPersistStream for IPersistStream {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IPersistStream`](crate::IPersistStream).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_IPersistStream: ole_IPersist {
	/// [`IPersistStream::GetSizeMax`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-getsizemax)
	/// method.
	#[must_use]
	fn GetSizeMax(&self) -> HrResult<u64> {
		let mut max = u64::default();
		ok_to_hrresult(
			unsafe {
				let vt = self.vt_ref::<IPersistStreamVT>();
				(vt.GetSizeMax)(self.ptr(), &mut max)
			},
		).map(|_| max)
	}

	/// [`IPersistStream::IsDirty`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-isdirty)
	/// method.
	#[must_use]
	fn IsDirty(&self) -> HrResult<bool> {
		okfalse_to_hrresult(
			unsafe {
				let vt = self.vt_ref::<IPersistStreamVT>();
				(vt.IsDirty)(self.ptr())
			},
		)
	}

	/// [`IPersistStream::Load`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-load)
	/// method.
	fn Load(&self, stream: &impl ole_IStream) -> HrResult<()> {
		ok_to_hrresult(
			unsafe {
				let vt = self.vt_ref::<IPersistStreamVT>();
				(vt.Load)(self.ptr(), stream.ptr())
			},
		)
	}

	/// [`IPersistStream::Save`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersiststream-save)
	/// method.
	fn Save(&self,
		stream: &impl ole_IStream, clear_dirty: bool) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				let vt = self.vt_ref::<IPersistStreamVT>();
				(vt.Save)(self.ptr(), stream.ptr(), clear_dirty as _)
			},
		)
	}
}
