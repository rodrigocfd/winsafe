#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::{BOOL, kernel32, user32};
use crate::funcs::GetLastError;
use crate::handles::HandleClose;
use crate::privs::{bool_to_winresult, INFINITE, MAX_PATH};
use crate::structs::{
	FILETIME,
	PROCESS_INFORMATION,
	SECURITY_ATTRIBUTES,
	STARTUPINFO,
};
use crate::various::WString;

/// Handle to a
/// [process](https://docs.microsoft.com/en-us/windows/win32/procthread/processes-and-threads).
/// Originally just a `HANDLE`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HPROCESS(pub(crate) *mut std::ffi::c_void);

impl_handle!(HPROCESS);
impl HandleClose for HPROCESS {}

impl HPROCESS {
	/// [`CreateProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)
	/// static method.
	///
	/// **Note:** Process and thread handles are returned in the
	/// [`PROCESS_INFORMATION`](crate::PROCESS_INFORMATION) struct, and they
	/// must be paired with their respective
	/// [`HPROCESS::CloseHandle`](crate::HPROCESS::CloseHandle) and
	/// [`HTHREAD::CloseHandle`](crate::HTHREAD::CloseHandle) calls.
	pub fn CreateProcess(
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
				kernel32::CreateProcessW(
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
	pub fn ExitProcess(exit_code: u32) {
		unsafe { kernel32::ExitProcess(exit_code) }
	}

	/// [`FlushInstructionCache`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushinstructioncache)
	/// method.
	pub fn FlushInstructionCache(self,
		base_address: *mut std::ffi::c_void, size: u64) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel32::FlushInstructionCache(self.0, base_address, size)
			},
		)
	}

	/// [`FlushProcessWriteBuffers`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushprocesswritebuffers)
	/// static method.
	pub fn FlushProcessWriteBuffers() {
		unsafe { kernel32::FlushProcessWriteBuffers() }
	}

	/// [`GetCurrentProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)
	/// static method.
	pub fn GetCurrentProcess() -> HPROCESS {
		Self(unsafe { kernel32::GetCurrentProcess() })
	}

	/// [`GetExitCodeProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)
	/// method.
	pub fn GetExitCodeProcess(self) -> WinResult<u32> {
		let mut exit_code = u32::default();
		bool_to_winresult(
			unsafe { kernel32::GetExitCodeProcess(self.0, &mut exit_code) },
		).map(|_| exit_code)
	}

	/// [`GetGuiResources`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getguiresources)
	/// method.
	pub fn GetGuiResources(self, flags: co::GR) -> WinResult<u32> {
		match unsafe { kernel32::GetGuiResources(self.0, flags.0) } {
			0 => Err(GetLastError()),
			count => Ok(count),
		}
	}

	/// [`GetProcessId`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessid)
	/// method.
	pub fn GetProcessId(self) -> WinResult<u32> {
		match unsafe { kernel32::GetProcessId(self.0) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetProcessTimes`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes)
	/// method.
	pub fn GetProcessTimes(self,
		creation: &mut FILETIME,
		exit: &mut FILETIME,
		kernel: &mut FILETIME,
		user: &mut FILETIME) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel32::GetProcessTimes(
					self.0,
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
	pub fn IsWow64Process(self) -> WinResult<bool> {
		let mut wow64: BOOL = 0;
		match unsafe { kernel32::IsWow64Process(self.0, &mut wow64) } {
			0 => Err(GetLastError()),
			_ => Ok(wow64 != 0),
		}
	}

	/// [`OpenProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HPROCESS::CloseHandle`](crate::HPROCESS::CloseHandle) call.
	pub fn OpenProcess(
		desired_access: co::PROCESS,
		inherit_handle: bool, process_id: u32) -> WinResult<HPROCESS>
	{
		unsafe {
			kernel32::OpenProcess(
				desired_access.0,
				inherit_handle as _,
				process_id,
			).as_mut()
		}.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`QueryFullProcessImageName`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-queryfullprocessimagenamew)
	/// method.
	pub fn QueryFullProcessImageName(self,
		flags: co::PROCESS_NAME) -> WinResult<String>
	{
		let mut buf = WString::new_alloc_buffer(MAX_PATH + 1);
		let mut sz = buf.buffer_size() as u32;

		bool_to_winresult(
			unsafe {
				kernel32::QueryFullProcessImageNameW(
					self.0,
					flags.0,
					buf.as_mut_ptr(),
					&mut sz,
				)
			},
		).map(|_| buf.to_string())
	}

	/// [`SetUserObjectInformation`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setuserobjectinformationw)
	/// method.
	///
	/// **Note:** The `pv_info` type varies according to `index`. If you set it
	/// wrong, you're likely to cause a buffer overrun.
	pub unsafe fn SetUserObjectInformation<T>(self,
		index: co::UOI, pv_info: &mut T) -> WinResult<()>
	{
		bool_to_winresult(
			user32::SetUserObjectInformationW(
				self.0,
				index.0,
				pv_info as *mut _ as _,
				std::mem::size_of::<T>() as _,
			),
		)
	}

	/// [`WaitForSingleObject`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)
	/// method.
	pub fn WaitForSingleObject(self,
		milliseconds: Option<u32>) -> WinResult<co::WAIT>
	{
		match unsafe {
			co::WAIT(
				kernel32::WaitForSingleObject(
					self.0,
					milliseconds.unwrap_or(INFINITE),
				),
			)
		} {
			co::WAIT::FAILED => Err(GetLastError()),
			wait => Ok(wait),
		}
	}
}
