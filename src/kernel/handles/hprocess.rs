#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	FILETIME, GetLastError, HACCESSTOKEN, PROCESS_INFORMATION,
	SECURITY_ATTRIBUTES, STARTUPINFO, SysResult, WString,
};
use crate::kernel::ffi_types::BOOL;
use crate::kernel::guard::{CloseHandleGuard, CloseHandlePiGuard};
use crate::kernel::privs::{
	bool_to_sysresult, INFINITE, MAX_PATH, ptr_to_sysresult_handle,
};
use crate::prelude::Handle;

impl_handle! { HPROCESS;
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
pub trait kernel_Hprocess: Handle {
	/// [`CheckRemoteDebuggerPresent`](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-checkremotedebuggerpresent)
	/// method.
	#[must_use]
	fn CheckRemoteDebuggerPresent(&self) -> SysResult<bool> {
		let mut present: BOOL = 0;
		bool_to_sysresult(
			unsafe {
				kernel::ffi::CheckRemoteDebuggerPresent(self.as_ptr(), &mut present)
			},
		).map(|_| present != 0)
	}

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
		environment: Option<Vec<(&str, &str)>>,
		current_dir: Option<&str>,
		si: &mut STARTUPINFO,
	) -> SysResult<CloseHandlePiGuard>
	{
		let mut buf_cmd_line = WString::from_opt_str(command_line);
		let mut pi = PROCESS_INFORMATION::default();

		unsafe {
			bool_to_sysresult(
				kernel::ffi::CreateProcessW(
					WString::from_opt_str(application_name).as_ptr(),
					buf_cmd_line.as_mut_ptr(),
					process_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					thread_attrs.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					inherit_handles as _,
					creation_flags.0,
					environment.map_or(std::ptr::null_mut(), |environment| {
						WString::from_str_vec(
							&environment.iter()
								.map(|(name, val)| format!("{}={}", name, val))
								.collect::<Vec<_>>()
						).as_ptr() as _
					}),
					WString::from_opt_str(current_dir).as_ptr(),
					si as *mut _ as _,
					&mut pi as *mut _ as _,
				),
			).map(|_| CloseHandlePiGuard::new(pi))
		}
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

	/// [`GetCurrentProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)
	/// static method.
	#[must_use]
	fn GetCurrentProcess() -> HPROCESS {
		HPROCESS(unsafe { kernel::ffi::GetCurrentProcess() })
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
		user: &mut FILETIME,
	) -> SysResult<()>
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
		process_id: u32,
	) -> SysResult<CloseHandleGuard<HPROCESS>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				kernel::ffi::OpenProcess(
					desired_access.0,
					inherit_handle as _,
					process_id,
				),
			).map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`OpenProcessToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)
	/// method.
	#[must_use]
	fn OpenProcessToken(&self,
		desired_access: co::TOKEN) -> SysResult<CloseHandleGuard<HACCESSTOKEN>>
	{
		let mut handle = HACCESSTOKEN::NULL;
		unsafe {
			bool_to_sysresult(
				kernel::ffi::OpenProcessToken(
					self.as_ptr(),
					desired_access.0,
					handle.as_mut(),
				),
			).map(|_| CloseHandleGuard::new(handle))
		}
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
