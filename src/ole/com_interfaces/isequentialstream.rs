#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::{privs::*, vts::*};
use crate::prelude::*;

com_interface! { ISequentialStream: "0c733a30-2a1c-11ce-ade5-00aa0044773d";
	/// [`ISequentialStream`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-isequentialstream)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl ole_ISequentialStream for ISequentialStream {}

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`ISequentialStream`](crate::ISequentialStream).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait ole_ISequentialStream: ole_IUnknown {
	/// [`ISequentialStream::Read`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-isequentialstream-read)
	/// method.
	///
	/// Returns the number of bytes written; if this value is lower than the
	/// requested size, it means the end of stream was reached.
	fn Read(&self, buffer: &mut [u8]) -> HrResult<u32> {
		let mut num_read = 0u32;
		HrRet(unsafe {
			(vt::<ISequentialStreamVT>(self).Read)(
				self.ptr(),
				buffer.as_mut_ptr() as _,
				buffer.len() as _,
				&mut num_read,
			)
		})
		.to_bool_hrresult()
		.map(|_| num_read)
	}

	/// [`ISequentialStream::Write`](https://learn.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-isequentialstream-write)
	/// method.
	///
	/// Returns the number of bytes written.
	fn Write(&self, data: &[u8]) -> HrResult<u32> {
		let mut num_written = 0u32;
		HrRet(unsafe {
			(vt::<ISequentialStreamVT>(self).Read)(
				self.ptr(),
				vec_ptr(data) as _,
				data.len() as _,
				&mut num_written,
			)
		})
		.to_hrresult()
		.map(|_| num_written)
	}
}
