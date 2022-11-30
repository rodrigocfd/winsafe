#![allow(non_camel_case_types, non_snake_case)]

use std::ops::Deref;

use crate::{co, kernel};
use crate::kernel::decl::{
	FILETIME, GetLastError, HACCESSTOKEN, PROCESS_INFORMATION,
	SECURITY_ATTRIBUTES, STARTUPINFO, SysResult, WString,
};
use crate::kernel::ffi_types::BOOL;
use crate::kernel::guard::HandleGuard;
use crate::kernel::privs::{bool_to_sysresult, INFINITE, MAX_PATH};
use crate::prelude::Handle;

impl_handle! { HPROCESS: "kernel";
	/// Handle to a
	/// [process](https://learn.microsoft.com/en-us/windows/win32/procthread/processes-and-threads).
	/// Originally just a `HANDLE`.
}

impl kernel_Hprocess for HPROCESS {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HPROCESS`](crate::HPROCESS).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait kernel_Hprocess: Handle {
	/// [`CreateProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)
	/// static method.
	#[must_use]
	fn CreateProcess(
		application_name: Option<&str>,
		command_line: Option<&str>,
		process_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		thread_attrs: Option<&mut SECURITY_ATTRIBUTES>,
		inherit_handles: bool,
		creation_flags: co::CREATE,
		environment: Option<Vec<String>>,
		current_dir: Option<&str>,
		si: &mut STARTUPINFO) -> SysResult<ProcessInformationGuard>
	{
		let mut buf_cmd_line = WString::from_opt_str(command_line);
		let mut pi = PROCESS_INFORMATION::default();

		bool_to_sysresult(
			unsafe {
				kernel::ffi::CreateProcessW(
					WString::from_opt_str(application_name).as_ptr(),
					buf_cmd_line.as_mut_ptr(),
					process_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					thread_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					inherit_handles as _,
					creation_flags.0,
					environment.as_ref()
						.map_or(std::ptr::null_mut(), |lp| WString::from_str_vec(lp).as_ptr() as _),
					WString::from_opt_str(current_dir).as_ptr(),
					si as *mut _ as _,
					&mut pi as *mut _ as _,
				)
			},
		).map(|_| ProcessInformationGuard { pi })
	}

	/// [`ExitProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitprocess)
	/// static method.
	fn ExitProcess(exit_code: u32) {
		unsafe { kernel::ffi::ExitProcess(exit_code) }
	}

	/// [`FlushInstructionCache`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushinstructioncache)
	/// method.
	fn FlushInstructionCache(&self,
		base_address: *mut std::ffi::c_void, size: usize) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::FlushInstructionCache(
					self.as_ptr(), base_address, size,
				)
			},
		)
	}

	/// [`FlushProcessWriteBuffers`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushprocesswritebuffers)
	/// static method.
	fn FlushProcessWriteBuffers() {
		unsafe { kernel::ffi::FlushProcessWriteBuffers() }
	}

	/// [`GetCurrentProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)
	/// static method.
	#[must_use]
	fn GetCurrentProcess() -> HPROCESS {
		HPROCESS(unsafe { kernel::ffi::GetCurrentProcess() })
	}

	/// [`GetCurrentProcessId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocessid)
	/// static method.
	#[must_use]
	fn GetCurrentProcessId() -> u32 {
		unsafe { kernel::ffi::GetCurrentProcessId() }
	}

	/// [`GetExitCodeProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)
	/// method.
	#[must_use]
	fn GetExitCodeProcess(&self) -> SysResult<u32> {
		let mut exit_code = u32::default();
		bool_to_sysresult(
			unsafe {
				kernel::ffi::GetExitCodeProcess(self.as_ptr(), &mut exit_code)
			},
		).map(|_| exit_code)
	}

	/// [`GetGuiResources`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getguiresources)
	/// method.
	#[must_use]
	fn GetGuiResources(&self, flags: co::GR) -> SysResult<u32> {
		match unsafe { kernel::ffi::GetGuiResources(self.as_ptr(), flags.0) } {
			0 => Err(GetLastError()),
			count => Ok(count),
		}
	}

	/// [`GetProcessHandleCount`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesshandlecount)
	/// method.
	#[must_use]
	fn GetProcessHandleCount(&self) -> SysResult<u32> {
		let mut count = u32::default();
		bool_to_sysresult(
			unsafe {
				kernel::ffi::GetProcessHandleCount(self.as_ptr(), &mut count)
			},
		).map(|_| count)
	}

	/// [`GetProcessId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessid)
	/// method.
	#[must_use]
	fn GetProcessId(&self) -> SysResult<u32> {
		match unsafe { kernel::ffi::GetProcessId(self.as_ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetProcessTimes`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes)
	/// method.
	fn GetProcessTimes(&self,
		creation: &mut FILETIME,
		exit: &mut FILETIME,
		kernel: &mut FILETIME,
		user: &mut FILETIME) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::GetProcessTimes(
					self.as_ptr(),
					creation as *mut _ as _,
					exit as *mut _ as _,
					kernel as *mut _ as _,
					user as *mut _ as _,
				)
			},
		)
	}

	/// [`IsDebuggerPresent`](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-isdebuggerpresent)
	/// static method.
	#[must_use]
	fn IsDebuggerPresent() -> bool {
		unsafe { kernel::ffi::IsDebuggerPresent() != 0 }
	}

	/// [`IsProcessCritical`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-isprocesscritical)
	/// method.
	#[must_use]
	fn IsProcessCritical(&self) -> SysResult<bool> {
		let mut critical: BOOL = 0;
		match unsafe {
			kernel::ffi::IsProcessCritical(self.as_ptr(), &mut critical) }
		{
			0 => Err(GetLastError()),
			_ => Ok(critical != 0),
		}
	}

	/// [`IsWow64Process`](https://learn.microsoft.com/en-us/windows/win32/api/wow64apiset/nf-wow64apiset-iswow64process)
	/// method.
	#[must_use]
	fn IsWow64Process(&self) -> SysResult<bool> {
		let mut wow64: BOOL = 0;
		match unsafe { kernel::ffi::IsWow64Process(self.as_ptr(), &mut wow64) } {
			0 => Err(GetLastError()),
			_ => Ok(wow64 != 0),
		}
	}

	/// [`OpenProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)
	/// static method.
	///
	/// This method will return
	/// [`ERROR::INVALID_PARAMETER`](crate::co::ERROR::INVALID_PARAMETER) if you
	/// try to open a system process.
	#[must_use]
	fn OpenProcess(
		desired_access: co::PROCESS,
		inherit_handle: bool,
		process_id: u32) -> SysResult<HandleGuard<HPROCESS>>
	{
		unsafe {
			kernel::ffi::OpenProcess(
				desired_access.0,
				inherit_handle as _,
				process_id,
			).as_mut()
		}.map(|ptr| HandleGuard { handle: HPROCESS(ptr) })
			.ok_or_else(|| GetLastError())
	}

	/// [`OpenProcessToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)
	/// method.
	#[must_use]
	fn OpenProcessToken(&self,
		desired_access: co::TOKEN) -> SysResult<HandleGuard<HACCESSTOKEN>>
	{
		let mut handle = HACCESSTOKEN::NULL;
		bool_to_sysresult(
			unsafe {
				kernel::ffi::OpenProcessToken(
					self.as_ptr(),
					desired_access.0,
					&mut handle.0,
				)
			},
		).map(|_| HandleGuard { handle })
	}

	/// [`QueryFullProcessImageName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-queryfullprocessimagenamew)
	/// method.
	#[must_use]
	fn QueryFullProcessImageName(&self,
		flags: co::PROCESS_NAME) -> SysResult<String>
	{
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		let mut sz = buf.buf_len() as u32;

		bool_to_sysresult(
			unsafe {
				kernel::ffi::QueryFullProcessImageNameW(
					self.as_ptr(),
					flags.0,
					buf.as_mut_ptr(),
					&mut sz,
				)
			},
		).map(|_| buf.to_string())
	}

	/// [`WaitForSingleObject`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)
	/// method.
	fn WaitForSingleObject(&self,
		milliseconds: Option<u32>) -> SysResult<co::WAIT>
	{
		match unsafe {
			co::WAIT(
				kernel::ffi::WaitForSingleObject(
					self.as_ptr(),
					milliseconds.unwrap_or(INFINITE),
				),
			)
		} {
			co::WAIT::FAILED => Err(GetLastError()),
			wait => Ok(wait),
		}
	}
}

//------------------------------------------------------------------------------

/// RAII implementation for [`PROCESS_INFORMATION`](crate::PROCESS_INFORMATION)
/// which automatically calls
/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
/// on `hProcess` and `hThread` fields when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub struct ProcessInformationGuard {
	pub(crate) pi: PROCESS_INFORMATION,
}

impl Drop for ProcessInformationGuard {
	fn drop(&mut self) {
		if let Some(h) = self.pi.hProcess.as_opt() {
			unsafe { kernel::ffi::CloseHandle(h.as_ptr()); } // ignore errors
		}
		if let Some(h) = self.pi.hThread.as_opt() {
			unsafe { kernel::ffi::CloseHandle(h.as_ptr()); }
		}
	}
}

impl Deref for ProcessInformationGuard {
	type Target = PROCESS_INFORMATION;

	fn deref(&self) -> &Self::Target {
		&self.pi
	}
}
