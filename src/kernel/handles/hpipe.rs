#![allow(non_snake_case)]

use crate::kernel;
use crate::kernel::decl::{HFILE, OVERLAPPED, SECURITY_ATTRIBUTES, WinResult};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::{Handle, HandleClose, KernelHfile};

impl_handle! { HPIPE: "kernel";
	/// Handle to an
	/// [anonymous pipe](https://docs.microsoft.com/en-us/windows/win32/ipc/anonymous-pipes).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HPIPE {}
impl KernelHpipe for HPIPE {}

/// [`HPIPE`](crate::HPIPE) methods from `kernel` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait KernelHpipe: Handle {
	/// [`CreatePipe`](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)
	/// static method.
	///
	/// Returns handles to the read and write pipes.
	///
	/// **Note:** Must be paired with
	/// [`HPIPE::CloseHandle`](crate::prelude::HandleClose::CloseHandle) calls.
	#[must_use]
	fn CreatePipe(
		attrs: Option<&mut SECURITY_ATTRIBUTES>,
		size: u32) -> WinResult<(HPIPE, HPIPE)>
	{
		let (mut hread, mut hwrite) = (HPIPE::NULL, HPIPE::NULL);
		bool_to_winresult(
			unsafe {
				kernel::ffi::CreatePipe(
					&mut hread.0,
					&mut hwrite.0,
					attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					size,
				)
			},
		).map(|_| (hread, hwrite))
	}

	/// [`ReadFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)
	/// method.
	///
	/// Returns the number of bytes read.
	fn ReadFile(self,
		buffer: &mut [u8],
		overlapped: Option<&mut OVERLAPPED>) -> WinResult<u32>
	{
		HFILE(unsafe { self.as_ptr() }).ReadFile(buffer, overlapped)
	}

	/// [`WriteFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)
	/// method.
	fn WriteFile(self,
		data: &[u8],
		overlapped: Option<&mut OVERLAPPED>) -> WinResult<u32>
	{
		HFILE(unsafe { self.as_ptr() }).WriteFile(data, overlapped)
	}
}
