#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{HRES, PCVOID, PVOID};
use crate::privs::{ok_to_hrresult, okfalse_to_hrresult};

/// [`ISequentialStream`](crate::idl::ISequentialStream) virtual table.
#[repr(C)]
pub struct ISequentialStreamVT {
	pub IUnknownVT: IUnknownVT,
	pub Read: fn(ComPtr, PVOID, u32, *mut u32) -> HRES,
	pub Write: fn(ComPtr, PCVOID, u32, *mut u32) -> HRES,
}

/// [`ISequentialStream`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-isequentialstream)
/// COM interface over
/// [`ISequentialStreamVT`](crate::idl::vt::ISequentialStreamVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct ISequentialStream(ComPtr);

impl_iunknown!(ISequentialStream, 0x0c733a30, 0x2a1c, 0x11ce, 0xade5, 0x00aa0044773d);
impl ISequentialStreamT for ISequentialStream {}

/// Exposes the [`ISequentialStream`](crate::idl::ISequentialStream) methods.
pub trait ISequentialStreamT: IUnknownT {
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
