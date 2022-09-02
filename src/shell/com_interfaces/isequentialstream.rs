#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{HRES, PCVOID, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::ole_IUnknown;
use crate::vt::IUnknownVT;

/// [`ISequentialStream`](crate::ISequentialStream) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct ISequentialStreamVT {
	pub IUnknownVT: IUnknownVT,
	pub Read: fn(ComPtr, PVOID, u32, *mut u32) -> HRES,
	pub Write: fn(ComPtr, PCVOID, u32, *mut u32) -> HRES,
}

com_interface! { ISequentialStream: "shell";
	"0c733a30-2a1c-11ce-ade5-00aa0044773d";
	/// [`ISequentialStream`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-isequentialstream)
	/// COM interface over [`ISequentialStreamVT`](crate::vt::ISequentialStreamVT).
	///
	/// Automatically calls
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_ISequentialStream for ISequentialStream {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`ISequentialStream`](crate::ISequentialStream).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_ISequentialStream: ole_IUnknown {
	/// [`ISequentialStream::Read`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-isequentialstream-read)
	/// method.
	///
	/// Returns the number of bytes written; if this value is lower than the
	/// requested size, it means the end of stream was reached.
	fn Read(&self, buffer: &mut [u8]) -> HrResult<u32> {
		let mut num_read = u32::default();
		okfalse_to_hrresult(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut ISequentialStreamVT);
				(vt.Read)(
					self.ptr(),
					buffer.as_mut_ptr() as _,
					buffer.len() as _,
					&mut num_read,
				)
			},
		).map(|_| num_read)
	}

	/// [`ISequentialStream::Write`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-isequentialstream-write)
	/// method.
	///
	/// Returns the number of bytes written.
	fn Write(&self, data: &[u8]) -> HrResult<u32> {
		let mut num_written = u32::default();
		ok_to_hrresult(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut ISequentialStreamVT);
				(vt.Read)(
					self.ptr(),
					data.as_ptr() as _,
					data.len() as _,
					&mut num_written,
				)
			},
		).map(|_| num_written)
	}
}
