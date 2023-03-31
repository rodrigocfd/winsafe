#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	FILETIME, GetLastError, HACCESSTOKEN, SECURITY_ATTRIBUTES, SysResult,
};
use crate::kernel::guard::CloseHandleGuard;
use crate::kernel::privs::{bool_to_sysresult, ptr_to_sysresult_handle};
use crate::prelude::Handle;

impl_handle! { HTHREAD;
	/// Handle to a
	/// [thread](https://learn.microsoft.com/en-us/windows/win32/procthread/processes-and-threads).
	/// Originally just a `HANDLE`.
}

impl kernel_Hthread for HTHREAD {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HTHREAD`](crate::HTHREAD).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hthread: Handle {
	/// [`CreateThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)
	/// static method.
	///
	/// Returns the thread handle and its ID.
	#[must_use]
	fn CreateThread(
		thread_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		stack_size: usize,
		start_addr: *mut std::ffi::c_void,
		parameter: *mut std::ffi::c_void,
		flags: co::THREAD_CREATE,
	) -> SysResult<(CloseHandleGuard<HTHREAD>, u32)>
	{
		let mut thread_id = u32::default();
		unsafe {
			ptr_to_sysresult_handle(
				kernel::ffi::CreateThread(
					thread_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					stack_size,
					start_addr,
					parameter,
					flags.0,
					&mut thread_id,
				)
			).map(|h| (CloseHandleGuard::new(h), thread_id))
		}
	}

	/// [`GetCurrentThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentthread)
	/// static method.
	#[must_use]
	fn GetCurrentThread() -> HTHREAD {
		HTHREAD(unsafe { kernel::ffi::GetCurrentThread() })
	}

	/// [`GetExitCodeThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodethread)
	/// method.
	#[must_use]
	fn GetExitCodeThread(&self) -> SysResult<u32> {
		let mut exit_code = u32::default();
		bool_to_sysresult(
			unsafe {
				kernel::ffi::GetExitCodeThread(self.as_ptr(), &mut exit_code)
			},
		).map(|_| exit_code)
	}

	/// [`GetProcessIdOfThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessidofthread)
	/// method.
	#[must_use]
	fn GetProcessIdOfThread(&self) -> SysResult<u32> {
		match unsafe { kernel::ffi::GetProcessIdOfThread(self.as_ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetThreadId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadid)
	/// method.
	#[must_use]
	fn GetThreadId(&self) -> SysResult<u32> {
		match unsafe { kernel::ffi::GetThreadId(self.as_ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetThreadTimes`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadtimes)
	/// method.
	fn GetThreadTimes(&self,
		creation: &mut FILETIME,
		exit: &mut FILETIME,
		kernel: &mut FILETIME,
		user: &mut FILETIME,
	) -> SysResult<()>
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

	/// [`OpenThreadToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthreadtoken)
	/// method.
	#[must_use]
	fn OpenThreadToken(&self,
		desired_access: co::TOKEN,
		open_as_self: bool,
	) -> SysResult<CloseHandleGuard<HACCESSTOKEN>>
	{
		let mut handle = HACCESSTOKEN::NULL;
		unsafe {
			bool_to_sysresult(
				kernel::ffi::OpenThreadToken(
					self.as_ptr(),
					desired_access.0,
					open_as_self as _,
					handle.as_mut(),
				),
			).map(|_| CloseHandleGuard::new(handle))
		}
	}

	/// [`ResumeThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread)
	/// method.
	fn ResumeThread(&self) -> SysResult<u32> {
		const MINUS_ONE: u32 = -1i32 as u32;
		match unsafe { kernel::ffi::ResumeThread(self.as_ptr()) } {
			MINUS_ONE => Err(GetLastError()),
			c => Ok(c),
		}
	}

	/// [`SuspendThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-suspendthread)
	/// method.
	fn SuspendThread(&self) -> SysResult<u32> {
		const MINUS_ONE: u32 = -1i32 as u32;
		match unsafe { kernel::ffi::SuspendThread(self.as_ptr()) } {
			MINUS_ONE => Err(GetLastError()),
			c => Ok(c),
		}
	}

	/// [`TerminateThread`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminatethread)
	/// method.
	fn TerminateThread(&self, exit_code: u32) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { kernel::ffi::TerminateThread(self.as_ptr(), exit_code) },
		)
	}
}
