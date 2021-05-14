#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::privs::{bool_to_winresult, ref_as_pvoid};
use crate::structs::SECURITY_ATTRIBUTES;

pub_struct_handle_closeable! {
	/// Handle to a
	/// [thread](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread).
	/// Originally just a `HANDLE`.
	HTHREAD
}

impl HTHREAD {
	/// [`CreateThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)
	/// static method.
	///
	/// Returns the thread handle and ID.
	///
	/// **Note:** Must be paired with a
	/// [`CloseHandle`](crate::HTHREAD::CloseHandle) call.
	pub fn CreateThread(
		lpThreadAttributes: Option<&mut SECURITY_ATTRIBUTES>,
		dwStackSize: u64,
		lpStartAddress: *mut std::ffi::c_void,
		lpParameter: *mut std::ffi::c_void,
		dwCreationFlags: co::THREAD_CREATE) -> WinResult<(HTHREAD, u32)>
	{
		let mut lpThreadId: u32 = 0;
		unsafe {
			kernel32::CreateThread(
				lpThreadAttributes.map_or(std::ptr::null_mut(), |lp| ref_as_pvoid(lp)),
				dwStackSize,
				lpStartAddress,
				lpParameter,
				dwCreationFlags.0,
				&mut lpThreadId,
			).as_mut()
		}.map(|ptr| (Self { ptr }, lpThreadId))
			.ok_or_else(|| GetLastError())
	}

	/// [`ExitThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)
	/// static method.
	pub fn ExitThread(dwExitCode: u32) {
		unsafe { kernel32::ExitThread(dwExitCode) }
	}

	/// [`GetExitCodeThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)
	/// method.
	pub fn GetExitCodeThread(self) -> WinResult<u32> {
		let mut lpExitCode: u32 = 0;
		bool_to_winresult(
			unsafe { kernel32::GetExitCodeThread(self.ptr, &mut lpExitCode) },
		).map(|_| lpExitCode)
	}
}
