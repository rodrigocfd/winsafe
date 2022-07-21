#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::ffi_types::BOOL;
use crate::kernel::decl::{
	FILETIME, GetLastError, HACCESSTOKEN, PROCESS_INFORMATION,
	SECURITY_ATTRIBUTES, STARTUPINFO, WinResult, WString,
};
use crate::kernel::privs::{bool_to_winresult, INFINITE, MAX_PATH};
use crate::prelude::{Handle, HandleClose};

impl_handle! { HPROCESS: "kernel";
	/// Handle to a
	/// [process](https://docs.microsoft.com/en-us/windows/win32/procthread/processes-and-threads).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HPROCESS {}
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
	/// [`CreateProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)
	/// static method.
	///
	/// **Note:** Process and thread handles are returned in the
	/// [`PROCESS_INFORMATION`](crate::PROCESS_INFORMATION) struct, and they
	/// must be paired with their respective
	/// [`HPROCESS::CloseHandle`](crate::prelude::HandleClose::CloseHandle) and
	/// [`HTHREAD::CloseHandle`](crate::prelude::HandleClose::CloseHandle)
	/// calls.
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
		si: &mut STARTUPINFO) -> WinResult<PROCESS_INFORMATION>
	{
		let mut buf_cmd_line = command_line.map_or(WString::default(), |lp| WString::from_str(lp));
		let mut pi = PROCESS_INFORMATION::default();

		bool_to_winresult(
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
		).map(|_| pi)
	}

	/// [`ExitProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitprocess)
	/// static method.
	fn ExitProcess(exit_code: u32) {
		unsafe { kernel::ffi::ExitProcess(exit_code) }
	}

	/// [`FlushInstructionCache`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushinstructioncache)
	/// method.
	fn FlushInstructionCache(self,
		base_address: *mut std::ffi::c_void, size: usize) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel::ffi::FlushInstructionCache(
					self.as_ptr(), base_address, size,
				)
			},
		)
	}

	/// [`FlushProcessWriteBuffers`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushprocesswritebuffers)
	/// static method.
	fn FlushProcessWriteBuffers() {
		unsafe { kernel::ffi::FlushProcessWriteBuffers() }
	}

	/// [`GetCurrentProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)
	/// static method.
	#[must_use]
	fn GetCurrentProcess() -> HPROCESS {
		HPROCESS(unsafe { kernel::ffi::GetCurrentProcess() })
	}

	/// [`GetExitCodeProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)
	/// method.
	#[must_use]
	fn GetExitCodeProcess(self) -> WinResult<u32> {
		let mut exit_code = u32::default();
		bool_to_winresult(
			unsafe {
				kernel::ffi::GetExitCodeProcess(self.as_ptr(), &mut exit_code)
			},
		).map(|_| exit_code)
	}

	/// [`GetGuiResources`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getguiresources)
	/// method.
	#[must_use]
	fn GetGuiResources(self, flags: co::GR) -> WinResult<u32> {
		match unsafe { kernel::ffi::GetGuiResources(self.as_ptr(), flags.0) } {
			0 => Err(GetLastError()),
			count => Ok(count),
		}
	}

	/// [`GetProcessId`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessid)
	/// method.
	#[must_use]
	fn GetProcessId(self) -> WinResult<u32> {
		match unsafe { kernel::ffi::GetProcessId(self.as_ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetProcessTimes`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes)
	/// method.
	fn GetProcessTimes(self,
		creation: &mut FILETIME,
		exit: &mut FILETIME,
		kernel: &mut FILETIME,
		user: &mut FILETIME) -> WinResult<()>
	{
		bool_to_winresult(
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

	/// [`IsWow64Process`](https://docs.microsoft.com/en-us/windows/win32/api/wow64apiset/nf-wow64apiset-iswow64process)
	/// method.
	#[must_use]
	fn IsWow64Process(self) -> WinResult<bool> {
		let mut wow64: BOOL = 0;
		match unsafe { kernel::ffi::IsWow64Process(self.as_ptr(), &mut wow64) } {
			0 => Err(GetLastError()),
			_ => Ok(wow64 != 0),
		}
	}

	/// [`OpenProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HPROCESS::CloseHandle`](crate::prelude::HandleClose::CloseHandle)
	/// call.
	#[must_use]
	fn OpenProcess(
		desired_access: co::PROCESS,
		inherit_handle: bool, process_id: u32) -> WinResult<HPROCESS>
	{
		unsafe {
			kernel::ffi::OpenProcess(
				desired_access.0,
				inherit_handle as _,
				process_id,
			).as_mut()
		}.map(|ptr| HPROCESS(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`OpenProcessToken`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocesstoken)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HACCESSTOKEN::CloseHandle`](crate::prelude::HandleClose::CloseHandle)
	/// call.
	#[must_use]
	fn OpenProcessToken(self,
		desired_access: co::TOKEN) -> WinResult<HACCESSTOKEN>
	{
		let mut handle = HACCESSTOKEN::NULL;
		bool_to_winresult(
			unsafe {
				kernel::ffi::OpenProcessToken(
					self.as_ptr(),
					desired_access.0,
					&mut handle.0,
				)
			},
		).map(|_| handle)
	}

	/// [`QueryFullProcessImageName`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-queryfullprocessimagenamew)
	/// method.
	#[must_use]
	fn QueryFullProcessImageName(self,
		flags: co::PROCESS_NAME) -> WinResult<String>
	{
		let mut buf = WString::new_alloc_buffer(MAX_PATH + 1);
		let mut sz = buf.buffer_size() as u32;

		bool_to_winresult(
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

	/// [`WaitForSingleObject`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)
	/// method.
	fn WaitForSingleObject(self,
		milliseconds: Option<u32>) -> WinResult<co::WAIT>
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
