#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::mf::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

com_interface! { IMFByteStream: "ad4c1b00-4bf7-422f-9175-756693d9130d";
	/// [`IIMFByteStream`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nn-mfobjects-imfbytestream)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with
	/// [`MFCreateMFByteStreamOnStream`](crate::MFCreateMFByteStreamOnStream)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let raw_data: Vec<u8>; // initialized somewhere
	/// # let raw_data = Vec::<u8>::new();
	///
	/// let stream = w::SHCreateMemStream(&raw_data)?;
	/// let byte_stream = w::MFCreateMFByteStreamOnStream(&stream)?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl mf_IMFByteStream for IMFByteStream {}

/// This trait is enabled with the `mf` feature, and provides methods for
/// [`IMFByteStream`](crate::IMFByteStream).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait mf_IMFByteStream: ole_IUnknown {
	/// [`IMFByteStream::BeginRead`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-beginread)
	/// method.
	fn BeginRead(
		&self,
		buffer: &mut [u8],
		callback: &IMFAsyncCallback,
		state: Option<&impl ole_IUnknown>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFByteStreamVT>(self).BeginRead)(
				self.ptr(),
				buffer.as_mut_ptr(),
				buffer.len() as _,
				callback.ptr(),
				state.map_or(std::ptr::null_mut(), |s| s.ptr()),
			)
		})
		.to_hrresult()
	}

	/// [`IMFByteStream::BeginWrite`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-beginwrite)
	/// method.
	fn BeginWrite(
		&self,
		buffer: &[u8],
		callback: &IMFAsyncCallback,
		state: Option<&impl ole_IUnknown>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IMFByteStreamVT>(self).BeginWrite)(
				self.ptr(),
				vec_ptr(buffer),
				buffer.len() as _,
				callback.ptr(),
				state.map_or(std::ptr::null_mut(), |s| s.ptr()),
			)
		})
		.to_hrresult()
	}

	fn_com_noparm! { Close: IMFByteStreamVT;
		/// [`IMFByteStream::Close`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-close)
		/// method.
	}

	/// [`IMFByteStream::EndRead`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-endread)
	/// method.
	fn EndRead(&self, result: &impl mf_IMFAsyncResult) -> HrResult<u32> {
		let mut read = 0u32;
		HrRet(unsafe { (vt::<IMFByteStreamVT>(self).EndRead)(self.ptr(), result.ptr(), &mut read) })
			.to_hrresult()
			.map(|_| read)
	}

	/// [`IMFByteStream::EndWrite`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-endwrite)
	/// method.
	fn EndWrite(&self, result: &impl mf_IMFAsyncResult) -> HrResult<u32> {
		let mut written = 0u32;
		HrRet(unsafe {
			(vt::<IMFByteStreamVT>(self).EndWrite)(self.ptr(), result.ptr(), &mut written)
		})
		.to_hrresult()
		.map(|_| written)
	}

	fn_com_noparm! { Flush: IMFByteStreamVT;
		/// [`IMFByteStream::Flush`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-flush)
		/// method.
	}

	/// [`IMFByteStream::GetCapabilities`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-getcapabilities)
	/// method.
	#[must_use]
	fn GetCapabilities(&self) -> HrResult<co::MFBYTESTREAM> {
		let mut cap = co::MFBYTESTREAM::default();
		HrRet(unsafe { (vt::<IMFByteStreamVT>(self).GetCapabilities)(self.ptr(), cap.as_mut()) })
			.to_hrresult()
			.map(|_| cap)
	}

	/// [`IMFByteStream::GetCurrentPosition`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-getcurrentposition)
	/// method.
	#[must_use]
	fn GetCurrentPosition(&self) -> HrResult<u64> {
		let mut pos = 0u64;
		HrRet(unsafe { (vt::<IMFByteStreamVT>(self).GetCurrentPosition)(self.ptr(), &mut pos) })
			.to_hrresult()
			.map(|_| pos)
	}

	/// [`IMFByteStream::GetLength`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-getlength)
	/// method.
	#[must_use]
	fn GetLength(&self) -> HrResult<u64> {
		let mut len = 0u64;
		HrRet(unsafe { (vt::<IMFByteStreamVT>(self).GetLength)(self.ptr(), &mut len) })
			.to_hrresult()
			.map(|_| len)
	}

	/// [`IMFByteStream::IsEndOfStream`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-isendofstream)
	/// method.
	#[must_use]
	fn IsEndOfStream(&self) -> HrResult<bool> {
		let mut is = 0;
		HrRet(unsafe { (vt::<IMFByteStreamVT>(self).IsEndOfStream)(self.ptr(), &mut is) })
			.to_hrresult()
			.map(|_| is != 0)
	}

	/// [`IMFByteStream::Read`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-read)
	/// method.
	fn Read(&self, buffer: &mut [u8]) -> HrResult<u32> {
		let mut read = 0u32;
		HrRet(unsafe {
			(vt::<IMFByteStreamVT>(self).Read)(
				self.ptr(),
				buffer.as_mut_ptr(),
				buffer.len() as _,
				&mut read,
			)
		})
		.to_hrresult()
		.map(|_| read)
	}

	/// [`IMFByteStream::Seek`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-seek)
	/// method.
	fn Seek(
		&self,
		origin: co::MFBYTESTREAM_SEEK_ORIGIN,
		offset: i64,
		flags: Option<co::MFBYTESTREAM_SEEK_FLAG>,
	) -> HrResult<u64> {
		let mut pos = 0u64;
		HrRet(unsafe {
			(vt::<IMFByteStreamVT>(self).Seek)(
				self.ptr(),
				origin.raw(),
				offset,
				flags.unwrap_or_default().raw(),
				&mut pos,
			)
		})
		.to_hrresult()
		.map(|_| pos)
	}

	/// [`IMFByteStream::SetCurrentPosition`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-setcurrentposition)
	/// method.
	fn SetCurrentPosition(&self, position: u64) -> HrResult<()> {
		HrRet(unsafe { (vt::<IMFByteStreamVT>(self).SetCurrentPosition)(self.ptr(), position) })
			.to_hrresult()
	}

	/// [`IMFByteStream::SetLength`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-setlength)
	/// method.
	fn SetLength(&self, length: u64) -> HrResult<()> {
		HrRet(unsafe { (vt::<IMFByteStreamVT>(self).SetLength)(self.ptr(), length) }).to_hrresult()
	}

	/// [`IMFByteStream::Write`](https://learn.microsoft.com/en-us/windows/win32/api/mfobjects/nf-mfobjects-imfbytestream-write)
	/// method.
	fn Write(&self, buffer: &[u8]) -> HrResult<u32> {
		let mut written = 0u32;
		HrRet(unsafe {
			(vt::<IMFByteStreamVT>(self).Write)(
				self.ptr(),
				vec_ptr(buffer),
				buffer.len() as _,
				&mut written,
			)
		})
		.to_hrresult()
		.map(|_| written)
	}
}
