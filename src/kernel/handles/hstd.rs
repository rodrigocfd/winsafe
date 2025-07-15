#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

handle! { HSTD;
	/// Handle to a
	/// [standard device](https://learn.microsoft.com/en-us/windows/console/getstdhandle).
	/// Originally just a `HANDLE`.
}

impl HSTD {
	/// [`FlushConsoleInputBuffer`](https://learn.microsoft.com/en-us/windows/console/flushconsoleinputbuffer)
	/// function.
	pub fn FlushConsoleInputBuffer(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::FlushConsoleInputBuffer(self.ptr()) })
	}

	/// [`GetConsoleMode`](https://learn.microsoft.com/en-us/windows/console/getconsolemode)
	/// function.
	#[must_use]
	pub fn GetConsoleMode(&self) -> SysResult<co::CONSOLE> {
		let mut mode = co::CONSOLE::default();
		bool_to_sysresult(unsafe { ffi::GetConsoleMode(self.ptr(), mode.as_mut()) }).map(|_| mode)
	}

	/// [`GetStdHandle`](https://learn.microsoft.com/en-us/windows/console/getstdhandle)
	/// function.
	#[must_use]
	pub fn GetStdHandle(std_handle: co::STD_HANDLE) -> SysResult<CloseHandleGuard<HSTD>> {
		unsafe {
			match HSTD::from_ptr(ffi::GetStdHandle(std_handle.raw())) {
				HSTD::INVALID => Err(GetLastError()),
				handle => Ok(CloseHandleGuard::new(handle)),
			}
		}
	}

	/// [`ReadConsole`](https://learn.microsoft.com/en-us/windows/console/readconsole)
	/// function.
	///
	/// Returns the number of chars actually written.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hstd = w::HSTD::GetStdHandle(co::STD_HANDLE::INPUT)?;
	///
	/// let mut buffer = w::WString::new_alloc_buf(2048);
	/// hstd.ReadConsole(&mut buffer, None)?;
	///
	/// let text = buffer.to_string();
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn ReadConsole(
		&self,
		buffer: &mut WString,
		input_control: Option<&CONSOLE_READCONSOLE_CONTROL>,
	) -> SysResult<u32> {
		let mut num_read = 0u32;
		bool_to_sysresult(unsafe {
			ffi::ReadConsoleW(
				self.ptr(),
				buffer.as_mut_ptr() as _,
				buffer.buf_len() as _,
				&mut num_read,
				pcvoid_or_null(input_control),
			)
		})
		.map(|_| num_read)
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

	/// [`SetConsoleMode`](https://learn.microsoft.com/en-us/windows/console/setconsolemode)
	/// function.
	pub fn SetConsoleMode(&self, mode: co::CONSOLE) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::SetConsoleMode(self.ptr(), mode.raw()) })
	}

	/// [`WriteConsole`](https://learn.microsoft.com/en-us/windows/console/writeconsole)
	/// function.
	///
	/// Returns the number of chars actually written.
	pub fn WriteConsole(&self, text: &str) -> SysResult<u32> {
		let buf = WString::from_str(text);
		let mut num_written = 0u32;

		unsafe {
			bool_to_sysresult(ffi::WriteConsoleW(
				self.ptr(),
				buf.as_ptr() as _,
				buf.str_len() as _,
				&mut num_written,
				std::ptr::null_mut(),
			))
		}
		.map(|_| num_written)
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
