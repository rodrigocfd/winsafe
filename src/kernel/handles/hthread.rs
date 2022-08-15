#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	FILETIME, GetLastError, HACCESSTOKEN, SECURITY_ATTRIBUTES, SysResult,
};
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::{Handle, HandleClose};

impl_handle! { HTHREAD: "kernel";
	/// Handle to a
	/// [thread](https://docs.microsoft.com/en-us/windows/win32/procthread/processes-and-threads).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HTHREAD {}
impl kernel_Hthread for HTHREAD {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HTHREAD`](crate::HTHREAD).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait kernel_Hthread: Handle {
	/// [`CreateThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)
	/// static method.
	///
	/// Returns the thread handle and ID.
	///
	/// **Note:** Must be paired with an
	/// [`HTHREAD::CloseHandle`](crate::prelude::HandleClose::CloseHandle) call.
	#[must_use]
	fn CreateThread(
		thread_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		stack_size: usize,
		start_addr: *mut std::ffi::c_void,
		parameter: *mut std::ffi::c_void,
		flags: co::THREAD_CREATE) -> SysResult<(HTHREAD, u32)>
	{
		let mut thread_id = u32::default();
		unsafe {
			kernel::ffi::CreateThread(
				thread_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
				stack_size,
				start_addr,
				parameter,
				flags.0,
				&mut thread_id,
			).as_mut()
		}.map(|ptr| (HTHREAD(ptr), thread_id))
			.ok_or_else(|| GetLastError())
	}

	/// [`ExitThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread)
	/// static method.
	fn ExitThread(exit_code: u32) {
		unsafe { kernel::ffi::ExitThread(exit_code) }
	}

	/// [`GetCurrentThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)
	/// static method.
	#[must_use]
	fn GetCurrentThread() -> HTHREAD {
		HTHREAD(unsafe { kernel::ffi::GetCurrentThread() })
	}

	/// [`GetExitCodeThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)
	/// method.
	#[must_use]
	fn GetExitCodeThread(self) -> SysResult<u32> {
		let mut exit_code = u32::default();
		bool_to_sysresult(
			unsafe {
				kernel::ffi::GetExitCodeThread(self.as_ptr(), &mut exit_code)
			},
		).map(|_| exit_code)
	}

	/// [`GetProcessIdOfThread`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessidofthread)
	/// method.
	#[must_use]
	fn GetProcessIdOfThread(self) -> SysResult<u32> {
		match unsafe { kernel::ffi::GetProcessIdOfThread(self.as_ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetThreadId`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadid)
	/// method.
	#[must_use]
	fn GetThreadId(self) -> SysResult<u32> {
		match unsafe { kernel::ffi::GetThreadId(self.as_ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetThreadTimes`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadtimes)
	/// method.
	fn GetThreadTimes(self,
		creation: &mut FILETIME,
		exit: &mut FILETIME,
		kernel: &mut FILETIME,
		user: &mut FILETIME) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::GetThreadTimes(
					self.as_ptr(),
					creation as *mut _ as _,
					exit as *mut _ as _,
					kernel as *mut _ as _,
					user as *mut _ as _,
				)
			},
		)
	}

	/// [`OpenThreadToken`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthreadtoken)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HACCESSTOKEN::CloseHandle`](crate::prelude::HandleClose::CloseHandle)
	/// call.
	#[must_use]
	fn OpenThreadToken(self,
		desired_access: co::TOKEN, open_as_self: bool) -> SysResult<HACCESSTOKEN>
	{
		let mut handle = HACCESSTOKEN::NULL;
		bool_to_sysresult(
			unsafe {
				kernel::ffi::OpenThreadToken(
					self.as_ptr(),
					desired_access.0,
					open_as_self as _,
					&mut handle.0,
				)
			},
		).map(|_| handle)
	}
}
