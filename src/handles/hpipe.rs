#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::kernel32;
use crate::handles::HFILE;
use crate::privs::bool_to_winresult;
use crate::structs::{OVERLAPPED, SECURITY_ATTRIBUTES};

pub_struct_handle_closeable! {
	/// Handle to an
	/// [anonymous pipe](https://docs.microsoft.com/en-us/windows/win32/ipc/anonymous-pipes).
	/// Originally just a `HANDLE`.
	HPIPE
}

impl HPIPE {
	/// [`CreatePipe`](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)
	/// static method.
	///
	/// Returns handles to the read and write pipes.
	///
	/// **Note:** Must be paired with
	/// [`HPIPE::CloseHandle`](crate::HPIPE::CloseHandle) calls.
	pub fn CreatePipe(
		attrs: Option<&mut SECURITY_ATTRIBUTES>,
		size: u32) -> WinResult<(HPIPE, HPIPE)>
	{
		let (mut hread, mut hwrite) = (Self::NULL, Self::NULL);
		bool_to_winresult(
			unsafe {
				kernel32::CreatePipe(
					&mut hread.ptr,
					&mut hwrite.ptr,
					attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					size,
				)
			},
		).map(|_| (hread, hwrite))
	}

	/// [`ReadFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)
	/// method.
	///
	/// `buffer` will be automatically resized to `num_bytes_to_read`.
	pub fn ReadFile(self,
		buffer: &mut Vec<u8>,
		num_bytes_to_read: u32,
		overlapped: Option<&mut OVERLAPPED>) -> WinResult<()>
	{
		HFILE { ptr: self.ptr }.ReadFile(buffer, num_bytes_to_read, overlapped)
	}

	/// [`WriteFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)
	/// method.
	pub fn WriteFile(self,
		data: &[u8],
		overlapped: Option<&mut OVERLAPPED>) -> WinResult<u32>
	{
		HFILE { ptr: self.ptr }.WriteFile(data, overlapped)
	}
}
