#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::privs::{bool_to_winresult, ref_as_pvoid};
use crate::structs::{PROCESS_INFORMATION, SECURITY_ATTRIBUTES, STARTUPINFO};
use crate::WString;

pub_struct_handle_closeable! {
	/// Handle to a
	/// [process](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information).
	/// Originally just a `HANDLE`.
	HPROCESS
}

impl HPROCESS {
	/// [`CreateProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw)
	/// static method.
	///
	/// Process and thread handles are returned in the
	/// [`PROCESS_INFORMATION`](crate::PROCESS_INFORMATION) struct, and must be
	/// paired with [`CloseHandle`](crate::HPROCESS::CloseHandle) and
	/// [`CloseHandle`](crate::HTHREAD::CloseHandle) calls.
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
					lpApplicationName.map_or(std::ptr::null_mut(), |lp| WString::from_str(lp).as_ptr()),
					bufCommandLine.as_mut_ptr(),
					lpProcessAttributes.map_or(std::ptr::null_mut(), |lp| ref_as_pvoid(lp)),
					lpThreadAttributes.map_or(std::ptr::null_mut(), |lp| ref_as_pvoid(lp)),
					nInheritHandles as _,
					dwCreationFlags.0,
					lpEnvironment as _,
					lpCurrentDirectory.map_or(std::ptr::null_mut(), |lp| WString::from_str(lp).as_ptr()),
					ref_as_pvoid(lpStartupInfo),
					ref_as_pvoid(&mut lpProcessInformation),
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

	/// [`GetExitCodeProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)
	/// method.
	pub fn GetExitCodeProcess(self) -> WinResult<u32> {
		let mut lpExitCode: u32 = 0;
		bool_to_winresult(
			unsafe { kernel32::GetExitCodeProcess(self.ptr, &mut lpExitCode) },
		).map(|_| lpExitCode)
	}
}
