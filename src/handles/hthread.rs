#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::privs::bool_to_winresult;
use crate::structs::{FILETIME, SECURITY_ATTRIBUTES};

pub_struct_handle_closeable! {
	/// Handle to a
	/// [thread](https://docs.microsoft.com/en-us/windows/win32/procthread/processes-and-threads).
	/// Originally just a `HANDLE`.
	HTHREAD
}

impl HTHREAD {
	/// [`CreateThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)
	/// static method.
	///
	/// Returns the thread handle and ID.
	///
	/// **Note:** Must be paired with an
	/// [`HTHREAD::CloseHandle`](crate::HTHREAD::CloseHandle) call.
	pub fn CreateThread(
		thread_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		stack_size: u64,
		start_addr: *mut std::ffi::c_void,
		parameter: *mut std::ffi::c_void,
		flags: co::THREAD_CREATE) -> WinResult<(HTHREAD, u32)>
	{
		let mut thread_id = u32::default();
		unsafe {
			kernel32::CreateThread(
				thread_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				stack_size,
				start_addr,
				parameter,
				flags.0,
				&mut thread_id,
			).as_mut()
		}.map(|ptr| (Self { ptr }, thread_id))
			.ok_or_else(|| GetLastError())
	}

	/// [`ExitThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)
	/// static method.
	pub fn ExitThread(exit_code: u32) {
		unsafe { kernel32::ExitThread(exit_code) }
	}

	/// [`GetCurrentThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)
	/// static method.
	pub fn GetCurrentThread() -> HTHREAD {
		Self { ptr: unsafe { kernel32::GetCurrentThread() } }
	}

	/// [`GetExitCodeThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)
	/// method.
	pub fn GetExitCodeThread(self) -> WinResult<u32> {
		let mut exit_code: u32 = 0;
		bool_to_winresult(
			unsafe { kernel32::GetExitCodeThread(self.ptr, &mut exit_code) },
		).map(|_| exit_code)
	}

	/// [`GetProcessIdOfThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessidofthread)
	/// method.
	pub fn GetProcessIdOfThread(self) -> WinResult<u32> {
		match unsafe { kernel32::GetProcessIdOfThread(self.ptr) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetThreadId`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadid)
	/// method.
	pub fn GetThreadId(self) -> WinResult<u32> {
		match unsafe { kernel32::GetThreadId(self.ptr) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetThreadTimes`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadtimes)
	/// method.
	pub fn GetThreadTimes(self,
		creation: &mut FILETIME,
		exit: &mut FILETIME,
		kernel: &mut FILETIME,
		user: &mut FILETIME) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel32::GetThreadTimes(
					self.ptr,
					creation as *mut _ as _,
					exit as *mut _ as _,
					kernel as *mut _ as _,
					user as *mut _ as _,
				)
			},
		)
	}
}
