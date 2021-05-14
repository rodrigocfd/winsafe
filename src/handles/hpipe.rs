#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::kernel32;
use crate::handles::HFILE;
use crate::privs::{bool_to_winresult, ref_as_pvoid};
use crate::structs::{OVERLAPPED, SECURITY_ATTRIBUTES};

pub_struct_handle_closeable! {
	/// Handle to a
	/// [pipe](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe).
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
	/// [`CloseHandle`](crate::HPIPE::CloseHandle) calls.
	pub fn CreatePipe(
		lpPipeAttributes: Option<&mut SECURITY_ATTRIBUTES>,
		nSize: u32) -> WinResult<(HPIPE, HPIPE)>
	{
		let (mut hread, mut hwrite) = (Self::NULL, Self::NULL);
		bool_to_winresult(
			unsafe {
				kernel32::CreatePipe(
					&mut hread.ptr,
					&mut hwrite.ptr,
					lpPipeAttributes.map_or(std::ptr::null_mut(), |lp| ref_as_pvoid(lp)),
					nSize,
				)
			},
		).map(|_| (hread, hwrite))
	}

	/// [`ReadFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)
	/// method.
	pub fn ReadFile(self,
		numBytesToRead: u32,
		lpOverlapped: Option<&mut OVERLAPPED>) -> WinResult<Vec<u8>>
	{
		HFILE { ptr: self.ptr }.ReadFile(numBytesToRead, lpOverlapped)
	}

	/// [`WriteFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)
	/// method.
	pub fn WriteFile(self,
		buffer: &[u8],
		lpOverlapped: Option<&mut OVERLAPPED>) -> WinResult<u32>
	{
		HFILE { ptr: self.ptr }.WriteFile(buffer, lpOverlapped)
	}
}
