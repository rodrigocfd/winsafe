#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};
use crate::prelude::*;

handle! { HPROCESS;
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
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hprocess: Handle {
	/// [`CheckRemoteDebuggerPresent`](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-checkremotedebuggerpresent)
	/// function.
	#[must_use]
	fn CheckRemoteDebuggerPresent(&self) -> SysResult<bool> {
		let mut present = 0;
		bool_to_sysresult(unsafe { ffi::CheckRemoteDebuggerPresent(self.ptr(), &mut present) })
			.map(|_| present != 0)
	}

	/// [`FlushInstructionCache`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushinstructioncache)
	/// function.
	fn FlushInstructionCache(
		&self,
		base_address: *mut std::ffi::c_void,
		size: usize,
	) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::FlushInstructionCache(self.ptr(), base_address, size) })
	}

	/// [`GetCurrentProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)
	/// function.
	#[must_use]
	fn GetCurrentProcess() -> HPROCESS {
		HPROCESS(unsafe { ffi::GetCurrentProcess() })
	}

	/// [`GetExitCodeProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)
	/// function.
	#[must_use]
	fn GetExitCodeProcess(&self) -> SysResult<u32> {
		let mut exit_code = u32::default();
		bool_to_sysresult(unsafe { ffi::GetExitCodeProcess(self.ptr(), &mut exit_code) })
			.map(|_| exit_code)
	}

	/// [`GetGuiResources`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getguiresources)
	/// function.
	#[must_use]
	fn GetGuiResources(&self, flags: co::GR) -> SysResult<u32> {
		match unsafe { ffi::GetGuiResources(self.ptr(), flags.raw()) } {
			0 => Err(GetLastError()),
			count => Ok(count),
		}
	}

	/// [`GetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass)
	/// function.
	#[must_use]
	fn GetPriorityClass(&self) -> SysResult<co::PRIORITY_CLASS> {
		match unsafe { ffi::GetPriorityClass(self.ptr()) } {
			0 => Err(GetLastError()),
			pc => Ok(unsafe { co::PRIORITY_CLASS::from_raw(pc) }),
		}
	}

	/// [`GetProcessHandleCount`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesshandlecount)
	/// function.
	#[must_use]
	fn GetProcessHandleCount(&self) -> SysResult<u32> {
		let mut count = u32::default();
		bool_to_sysresult(unsafe { ffi::GetProcessHandleCount(self.ptr(), &mut count) })
			.map(|_| count)
	}

	/// [`GetProcessId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessid)
	/// function.
	#[must_use]
	fn GetProcessId(&self) -> SysResult<u32> {
		match unsafe { ffi::GetProcessId(self.ptr()) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`GetProcessTimes`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes)
	/// function.
	///
	/// Returns, respectively:
	///
	/// 1. creation time;
	/// 2. exit time;
	/// 3. kernel time;
	/// 4. user time.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hprocess: w::HPROCESS; // initialized somewhere
	/// # let hprocess = w::HPROCESS::NULL;
	///
	/// let (creation, exit, kernel, user) = hprocess.GetProcessTimes()?;
	/// # w::SysResult::Ok(())
	/// ```
	fn GetProcessTimes(&self) -> SysResult<(FILETIME, FILETIME, FILETIME, FILETIME)> {
		let (mut creation, mut exit, mut kernel, mut user) =
			(FILETIME::default(), FILETIME::default(), FILETIME::default(), FILETIME::default());

		bool_to_sysresult(unsafe {
			ffi::GetProcessTimes(
				self.ptr(),
				pvoid(&mut creation),
				pvoid(&mut exit),
				pvoid(&mut kernel),
				pvoid(&mut user),
			)
		})
		.map(|_| (creation, exit, kernel, user))
	}

	/// [`IsProcessCritical`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-isprocesscritical)
	/// function.
	#[must_use]
	fn IsProcessCritical(&self) -> SysResult<bool> {
		let mut critical = 0;
		bool_to_sysresult(unsafe { ffi::IsProcessCritical(self.ptr(), &mut critical) })
			.map(|_| critical != 0)
	}

	/// [`IsWow64Process`](https://learn.microsoft.com/en-us/windows/win32/api/wow64apiset/nf-wow64apiset-iswow64process)
	/// function.
	#[must_use]
	fn IsWow64Process(&self) -> SysResult<bool> {
		let mut wow64 = 0;
		match unsafe { ffi::IsWow64Process(self.ptr(), &mut wow64) } {
			0 => Err(GetLastError()),
			_ => Ok(wow64 != 0),
		}
	}

	/// [`OpenProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess)
	/// function.
	///
	/// This method will return
	/// [`ERROR::INVALID_PARAMETER`](crate::co::ERROR::INVALID_PARAMETER) if you
	/// try to open a system process.
	#[must_use]
	fn OpenProcess(
		desired_access: co::PROCESS,
		inherit_handle: bool,
		process_id: u32,
	) -> SysResult<CloseHandleGuard<HPROCESS>> {
		unsafe {
			ptr_to_sysresult_handle(ffi::OpenProcess(
				desired_access.raw(),
				inherit_handle as _,
				process_id,
			))
			.map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`QueryFullProcessImageName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-queryfullprocessimagenamew)
	/// function.
	#[must_use]
	fn QueryFullProcessImageName(&self, flags: co::PROCESS_NAME) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		let mut sz = buf.buf_len() as u32;

		bool_to_sysresult(unsafe {
			ffi::QueryFullProcessImageNameW(self.ptr(), flags.raw(), buf.as_mut_ptr(), &mut sz)
		})
		.map(|_| buf.to_string())
	}

	/// [`QueryProcessAffinityUpdateMode`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-queryprocessaffinityupdatemode)
	/// function.
	#[must_use]
	fn QueryProcessAffinityUpdateMode(&self) -> SysResult<co::PROCESS_AFFINITY> {
		let mut affinity = co::PROCESS_AFFINITY::default();
		bool_to_sysresult(unsafe {
			ffi::QueryProcessAffinityUpdateMode(self.ptr(), affinity.as_mut())
		})
		.map(|_| affinity)
	}

	/// [`QueryProcessCycleTime`](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-queryprocesscycletime)
	/// function.
	#[must_use]
	fn QueryProcessCycleTime(&self) -> SysResult<u64> {
		let mut t = u64::default();
		bool_to_sysresult(unsafe { ffi::QueryProcessCycleTime(self.ptr(), &mut t) }).map(|_| t)
	}

	/// [`ReadProcessMemory`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory)
	/// function.
	///
	/// Reads at most `buffer.len()` bytes. Returns how many bytes were actually
	/// read.
	#[must_use]
	fn ReadProcessMemory(
		&self,
		base_address: *mut std::ffi::c_void,
		buffer: &mut [u8],
	) -> SysResult<usize> {
		let mut bytes_read = 0;
		bool_to_sysresult(unsafe {
			ffi::ReadProcessMemory(
				self.ptr(),
				base_address,
				buffer.as_ptr() as _,
				buffer.len() as _,
				&mut bytes_read,
			)
		})
		.map(|_| bytes_read)
	}

	/// [`SetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass)
	/// function.
	fn SetPriorityClass(&self, prority_class: co::PRIORITY_CLASS) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::SetPriorityClass(self.ptr(), prority_class.raw()) })
	}

	/// [`SetProcessAffinityUpdateMode`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessaffinityupdatemode)
	/// function.
	fn SetProcessAffinityUpdateMode(&self, flags: co::PROCESS_AFFINITY) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::SetProcessAffinityUpdateMode(self.ptr(), flags.raw()) })
	}

	/// [`SetProcessPriorityBoost`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocesspriorityboost)
	/// function.
	fn SetProcessPriorityBoost(&self, disable_priority_boost: bool) -> SysResult<()> {
		bool_to_sysresult(unsafe {
			ffi::SetProcessPriorityBoost(self.ptr(), disable_priority_boost as _)
		})
	}

	/// [`TerminateProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess)
	/// function.
	fn TerminateProcess(&self, exit_code: u32) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::TerminateProcess(self.ptr(), exit_code) })
	}

	/// [`VirtualQueryEx`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualqueryex)
	/// function.
	#[must_use]
	fn VirtualQueryEx(&self, address: Option<usize>) -> SysResult<MEMORY_BASIC_INFORMATION> {
		let mut mbi = MEMORY_BASIC_INFORMATION::default();
		let ret = unsafe {
			ffi::VirtualQueryEx(
				self.ptr(),
				address.unwrap_or_default() as _,
				pvoid(&mut mbi),
				std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
			)
		};
		if ret == 0 {
			Err(GetLastError())
		} else {
			Ok(mbi)
		}
	}

	/// [`WaitForSingleObject`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)
	/// function.
	fn WaitForSingleObject(&self, milliseconds: Option<u32>) -> SysResult<co::WAIT> {
		unsafe { HEVENT::from_ptr(self.ptr()) }.WaitForSingleObject(milliseconds)
	}
}
