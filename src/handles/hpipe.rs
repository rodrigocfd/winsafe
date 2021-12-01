#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::kernel32;
use crate::handles::HFILE;
use crate::handles::prelude::{Handle, HandleClose};
use crate::privs::bool_to_winresult;
use crate::structs::{OVERLAPPED, SECURITY_ATTRIBUTES};

/// Handle to an
/// [anonymous pipe](https://docs.microsoft.com/en-us/windows/win32/ipc/anonymous-pipes).
/// Originally just a `HANDLE`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HPIPE(pub(crate) *mut std::ffi::c_void);

impl_handle!(HPIPE);
impl HandleClose for HPIPE {}

impl HPIPE {
	/// [`CreatePipe`](https://docs.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)
	/// static method.
	///
	/// Returns handles to the read and write pipes.
	///
	/// **Note:** Must be paired with
	/// [`HPIPE::CloseHandle`](crate::prelude::HandleClose::CloseHandle) calls.
	pub fn CreatePipe(
		attrs: Option<&mut SECURITY_ATTRIBUTES>,
		size: u32) -> WinResult<(HPIPE, HPIPE)>
	{
		let (mut hread, mut hwrite) = (Self::NULL, Self::NULL);
		bool_to_winresult(
			unsafe {
				kernel32::CreatePipe(
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
	pub fn ReadFile(self,
		buffer: &mut [u8],
		overlapped: Option<&mut OVERLAPPED>) -> WinResult<u32>
	{
		HFILE(self.0).ReadFile(buffer, overlapped)
	}

	/// [`WriteFile`](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)
	/// method.
	pub fn WriteFile(self,
		data: &[u8],
		overlapped: Option<&mut OVERLAPPED>) -> WinResult<u32>
	{
		HFILE(self.0).WriteFile(data, overlapped)
	}
}
