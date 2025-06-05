#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

handle! { HPIPE;
	/// Handle to an
	/// [anonymous pipe](https://learn.microsoft.com/en-us/windows/win32/ipc/anonymous-pipes).
	/// Originally just a `HANDLE`.
}

impl HPIPE {
	/// [`CreatePipe`](https://learn.microsoft.com/en-us/windows/win32/api/namedpipeapi/nf-namedpipeapi-createpipe)
	/// function.
	///
	/// Returns handles to the read and write pipes.
	#[must_use]
	pub fn CreatePipe(
		attrs: Option<&SECURITY_ATTRIBUTES>,
		size: u32,
	) -> SysResult<(CloseHandleGuard<HPIPE>, CloseHandleGuard<HPIPE>)> {
		let (mut hread, mut hwrite) = (HPIPE::NULL, HPIPE::NULL);
		unsafe {
			bool_to_sysresult(ffi::CreatePipe(
				hread.as_mut(),
				hwrite.as_mut(),
				pcvoid_or_null(attrs),
				size,
			))
			.map(|_| (CloseHandleGuard::new(hread), CloseHandleGuard::new(hwrite)))
		}
	}

	/// [`ReadFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile)
	/// function.
	///
	/// Returns the number of bytes read.
	///
	/// Note that asynchronous reading – which use the
	/// [`OVERLAPPED`](crate::OVERLAPPED) struct – is not currently supported by
	/// this method, because the buffer must remain untouched until the async
	/// operation is complete, thus making the method unsound.
	pub fn ReadFile(&self, buffer: &mut [u8]) -> SysResult<u32> {
		unsafe { HFILE::from_ptr(self.ptr()) }.ReadFile(buffer)
	}

	/// [`WriteFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-writefile)
	/// function.
	///
	/// Returns the number of bytes written.
	///
	/// Note that asynchronous writing – which use the
	/// [`OVERLAPPED`](crate::OVERLAPPED) struct – is not currently supported by
	/// this method, because the buffer must remain untouched until the async
	/// operation is complete, thus making the method unsound.
	pub fn WriteFile(&self, data: &[u8]) -> SysResult<u32> {
		unsafe { HFILE::from_ptr(self.ptr()) }.WriteFile(data)
	}
}
