#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::kernel32;
use crate::privs::bool_to_winresult;

pub_struct_handle_closeable! {
	/// Handle to a
	/// [process](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_information).
	/// Originally just a `HANDLE`.
	HPROCESS
}

impl HPROCESS {
	/// [`GetExitCodeProcess`](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)
	/// function.
	pub fn GetExitCodeProcess(self) -> WinResult<u32> {
		let mut lpExitCode: u32 = 0;
		bool_to_winresult(
			unsafe { kernel32::GetExitCodeProcess(self.ptr, &mut lpExitCode) },
		).map(|_| lpExitCode)
	}
}
