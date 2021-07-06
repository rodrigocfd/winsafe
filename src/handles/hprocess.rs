#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::privs::{bool_to_winresult, INFINITE};
use crate::structs::{
	FILETIME,
	PROCESS_INFORMATION,
	SECURITY_ATTRIBUTES,
	STARTUPINFO,
};
use crate::WString;

pub_struct_handle_closeable! {
	/// Handle to a
	/// [process](https://docs.microsoft.com/en-us/windows/win32/procthread/processes-and-threads).
	/// Originally just a `HANDLE`.
	HPROCESS
}

impl HPROCESS {
	/// [`CreateProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)
	/// static method.
	///
	/// Process and thread handles are returned in the
	/// [`PROCESS_INFORMATION`](crate::PROCESS_INFORMATION) struct, and they
	/// must be paired with their respective
	/// [`HPROCESS::CloseHandle`](crate::HPROCESS::CloseHandle) and
	/// [`HTHREAD::CloseHandle`](crate::HTHREAD::CloseHandle) calls.
	pub fn CreateProcess(
		lpApplicationName: Option<&str>,
		lpCommandLine: Option<&str>,
		lpProcessAttributes: Option<&mut SECURITY_ATTRIBUTES>,
		lpThreadAttributes: Option<&mut SECURITY_ATTRIBUTES>,
		nInheritHandles: bool,
		dwCreationFlags: co::CREATE,
		lpEnvironment: *mut u8,
		lpCurrentDirectory: Option<&str>,
		lpStartupInfo: &mut STARTUPINFO) -> WinResult<PROCESS_INFORMATION>
	{
		let mut bufCommandLine = lpCommandLine.map_or(WString::default(), |lp| WString::from_str(lp));
		let mut lpProcessInformation = PROCESS_INFORMATION::default();
		bool_to_winresult(
			unsafe {
				kernel32::CreateProcessW(
					WString::from_opt_str(lpApplicationName).as_ptr(),
					bufCommandLine.as_mut_ptr(),
					lpProcessAttributes.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					lpThreadAttributes.map_or(std::ptr::null_mut(), |lp| lp as *mut _ as _),
					nInheritHandles as _,
					dwCreationFlags.0,
					lpEnvironment as _,
					WString::from_opt_str(lpCurrentDirectory).as_ptr(),
					lpStartupInfo as *mut _ as _,
					&mut lpProcessInformation as *mut _ as _,
				)
			},
		).map(|_| lpProcessInformation)
	}

	/// [`ExitProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitprocess)
	/// static method.
	pub fn ExitProcess(dwExitCode: u32) {
		unsafe { kernel32::ExitProcess(dwExitCode) }
	}

	/// [`FlushInstructionCache`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushinstructioncache)
	/// method.
	pub fn FlushInstructionCache(self,
		lpBaseAddress: *mut std::ffi::c_void, dwSize: u64) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel32::FlushInstructionCache(self.ptr, lpBaseAddress, dwSize)
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
		Self { ptr: unsafe { kernel32::GetCurrentProcess() } }
	}

	/// [`GetExitCodeProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)
	/// method.
	pub fn GetExitCodeProcess(self) -> WinResult<u32> {
		let mut lpExitCode: u32 = 0;
		bool_to_winresult(
			unsafe { kernel32::GetExitCodeProcess(self.ptr, &mut lpExitCode) },
		).map(|_| lpExitCode)
	}

	/// [`GetGuiResources`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getguiresources)
	/// method.
	pub fn GetGuiResources(self, uiFlags: co::GR) -> WinResult<u32> {
		match unsafe { kernel32::GetGuiResources(self.ptr, uiFlags.0) } {
			0 => Err(GetLastError()),
			count => Ok(count),
		}
	}

	/// [`GetProcessId`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessid)
	/// method.
	pub fn GetProcessId(self) -> WinResult<u32> {
		match unsafe { kernel32::GetProcessId(self.ptr) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetProcessTimes`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes)
	/// method.
	pub fn GetProcessTimes(self,
		lpCreationTime: &mut FILETIME,
		lpExitTime: &mut FILETIME,
		lpKernelTime: &mut FILETIME,
		lpUserTime: &mut FILETIME) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel32::GetProcessTimes(
					self.ptr,
					lpCreationTime as *mut _ as _,
					lpExitTime as *mut _ as _,
					lpKernelTime as *mut _ as _,
					lpUserTime as *mut _ as _,
				)
			},
		)
	}

	/// [`WaitForSingleObject`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)
	/// method.
	pub fn WaitForSingleObject(self,
		dwMilliseconds: Option<u32>) -> WinResult<co::WAIT>
	{
		match unsafe {
			co::WAIT(
				kernel32::WaitForSingleObject(
					self.ptr,
					dwMilliseconds.unwrap_or(INFINITE),
				),
			)
		} {
			co::WAIT::FAILED => Err(GetLastError()),
			wait => Ok(wait),
		}
	}
}
