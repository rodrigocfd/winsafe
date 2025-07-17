#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};

handle! { HPROCESS;
	/// Handle to a
	/// [process](https://learn.microsoft.com/en-us/windows/win32/procthread/processes-and-threads).
	/// Originally just a `HANDLE`.
}

impl HPROCESS {
	/// [`CheckRemoteDebuggerPresent`](https://learn.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-checkremotedebuggerpresent)
	/// function.
	#[must_use]
	pub fn CheckRemoteDebuggerPresent(&self) -> SysResult<bool> {
		let mut present = 0;
		BoolRet(unsafe { ffi::CheckRemoteDebuggerPresent(self.ptr(), &mut present) })
			.to_sysresult()
			.map(|_| present != 0)
	}

	/// [`FlushInstructionCache`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushinstructioncache)
	/// function.
	pub fn FlushInstructionCache(
		&self,
		base_address: *mut std::ffi::c_void,
		size: usize,
	) -> SysResult<()> {
		BoolRet(unsafe { ffi::FlushInstructionCache(self.ptr(), base_address, size) })
			.to_sysresult()
	}

	/// [`GetCurrentProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getcurrentprocess)
	/// function.
	#[must_use]
	pub fn GetCurrentProcess() -> HPROCESS {
		HPROCESS(unsafe { ffi::GetCurrentProcess() })
	}

	/// [`GetExitCodeProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess)
	/// function.
	#[must_use]
	pub fn GetExitCodeProcess(&self) -> SysResult<u32> {
		let mut exit_code = 0u32;
		BoolRet(unsafe { ffi::GetExitCodeProcess(self.ptr(), &mut exit_code) })
			.to_sysresult()
			.map(|_| exit_code)
	}

	/// [`GetGuiResources`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getguiresources)
	/// function.
	#[must_use]
	pub fn GetGuiResources(&self, flags: co::GR) -> SysResult<u32> {
		match unsafe { ffi::GetGuiResources(self.ptr(), flags.raw()) } {
			0 => Err(GetLastError()),
			count => Ok(count),
		}
	}

	/// [`GetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getpriorityclass)
	/// function.
	#[must_use]
	pub fn GetPriorityClass(&self) -> SysResult<co::PRIORITY_CLASS> {
		match unsafe { ffi::GetPriorityClass(self.ptr()) } {
			0 => Err(GetLastError()),
			pc => Ok(unsafe { co::PRIORITY_CLASS::from_raw(pc) }),
		}
	}

	/// [`GetProcessHandleCount`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesshandlecount)
	/// function.
	#[must_use]
	pub fn GetProcessHandleCount(&self) -> SysResult<u32> {
		let mut count = 0u32;
		BoolRet(unsafe { ffi::GetProcessHandleCount(self.ptr(), &mut count) })
			.to_sysresult()
			.map(|_| count)
	}

	/// [`GetProcessId`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessid)
	/// function.
	#[must_use]
	pub fn GetProcessId(&self) -> SysResult<u32> {
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
	pub fn GetProcessTimes(&self) -> SysResult<(FILETIME, FILETIME, FILETIME, FILETIME)> {
		let (mut creation, mut exit, mut kernel, mut user) =
			(FILETIME::default(), FILETIME::default(), FILETIME::default(), FILETIME::default());

		BoolRet(unsafe {
			ffi::GetProcessTimes(
				self.ptr(),
				pvoid(&mut creation),
				pvoid(&mut exit),
				pvoid(&mut kernel),
				pvoid(&mut user),
			)
		})
		.to_sysresult()
		.map(|_| (creation, exit, kernel, user))
	}

	/// [`IsProcessCritical`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-isprocesscritical)
	/// function.
	#[must_use]
	pub fn IsProcessCritical(&self) -> SysResult<bool> {
		let mut critical = 0;
		BoolRet(unsafe { ffi::IsProcessCritical(self.ptr(), &mut critical) })
			.to_sysresult()
			.map(|_| critical != 0)
	}

	/// [`IsWow64Process`](https://learn.microsoft.com/en-us/windows/win32/api/wow64apiset/nf-wow64apiset-iswow64process)
	/// function.
	#[must_use]
	pub fn IsWow64Process(&self) -> SysResult<bool> {
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
	pub fn OpenProcess(
		desired_access: co::PROCESS,
		inherit_handle: bool,
		process_id: u32,
	) -> SysResult<CloseHandleGuard<HPROCESS>> {
		unsafe {
			PtrRet(ffi::OpenProcess(desired_access.raw(), inherit_handle as _, process_id))
				.to_sysresult_handle()
				.map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`QueryFullProcessImageName`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-queryfullprocessimagenamew)
	/// function.
	#[must_use]
	pub fn QueryFullProcessImageName(&self, flags: co::PROCESS_NAME) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		let mut sz = buf.buf_len() as u32;

		BoolRet(unsafe {
			ffi::QueryFullProcessImageNameW(self.ptr(), flags.raw(), buf.as_mut_ptr(), &mut sz)
		})
		.to_sysresult()
		.map(|_| buf.to_string())
	}

	/// [`QueryProcessAffinityUpdateMode`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-queryprocessaffinityupdatemode)
	/// function.
	#[must_use]
	pub fn QueryProcessAffinityUpdateMode(&self) -> SysResult<co::PROCESS_AFFINITY> {
		let mut affinity = co::PROCESS_AFFINITY::default();
		BoolRet(unsafe { ffi::QueryProcessAffinityUpdateMode(self.ptr(), affinity.as_mut()) })
			.to_sysresult()
			.map(|_| affinity)
	}

	/// [`QueryProcessCycleTime`](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-queryprocesscycletime)
	/// function.
	#[must_use]
	pub fn QueryProcessCycleTime(&self) -> SysResult<u64> {
		let mut t = 0u64;
		BoolRet(unsafe { ffi::QueryProcessCycleTime(self.ptr(), &mut t) })
			.to_sysresult()
			.map(|_| t)
	}

	/// [`ReadProcessMemory`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-readprocessmemory)
	/// function.
	///
	/// Reads at most `buffer.len()` bytes. Returns how many bytes were actually
	/// read.
	#[must_use]
	pub fn ReadProcessMemory(
		&self,
		base_address: *mut std::ffi::c_void,
		buffer: &mut [u8],
	) -> SysResult<usize> {
		let mut bytes_read = 0usize;
		BoolRet(unsafe {
			ffi::ReadProcessMemory(
				self.ptr(),
				base_address,
				buffer.as_ptr() as _,
				buffer.len(),
				&mut bytes_read,
			)
		})
		.to_sysresult()
		.map(|_| bytes_read)
	}

	/// [`SetPriorityClass`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass)
	/// function.
	pub fn SetPriorityClass(&self, prority_class: co::PRIORITY_CLASS) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetPriorityClass(self.ptr(), prority_class.raw()) }).to_sysresult()
	}

	/// [`SetProcessAffinityUpdateMode`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessaffinityupdatemode)
	/// function.
	pub fn SetProcessAffinityUpdateMode(&self, flags: co::PROCESS_AFFINITY) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetProcessAffinityUpdateMode(self.ptr(), flags.raw()) })
			.to_sysresult()
	}

	/// [`SetProcessPriorityBoost`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocesspriorityboost)
	/// function.
	pub fn SetProcessPriorityBoost(&self, disable_priority_boost: bool) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetProcessPriorityBoost(self.ptr(), disable_priority_boost as _) })
			.to_sysresult()
	}

	/// [`TerminateProcess`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-terminateprocess)
	/// function.
	pub fn TerminateProcess(&self, exit_code: u32) -> SysResult<()> {
		BoolRet(unsafe { ffi::TerminateProcess(self.ptr(), exit_code) }).to_sysresult()
	}

	/// [`VirtualQueryEx`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualqueryex)
	/// function.
	#[must_use]
	pub fn VirtualQueryEx(
		&self,
		address: Option<*mut std::ffi::c_void>,
	) -> SysResult<MEMORY_BASIC_INFORMATION> {
		let mut mbi = MEMORY_BASIC_INFORMATION::default();
		match unsafe {
			ffi::VirtualQueryEx(
				self.ptr(),
				address.unwrap_or(std::ptr::null_mut()),
				pvoid(&mut mbi),
				std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(mbi),
		}
	}

	/// [`WaitForSingleObject`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)
	/// function.
	pub fn WaitForSingleObject(&self, milliseconds: Option<u32>) -> SysResult<co::WAIT> {
		unsafe { HEVENT::from_ptr(self.ptr()) }.WaitForSingleObject(milliseconds)
	}

	/// [`WriteProcessMemory`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-writeprocessmemory)
	/// function.
	///
	/// Returns how many bytes were actually written.
	pub fn WriteProcessMemory(
		&self,
		base_address: *mut std::ffi::c_void,
		buffer: &[u8],
	) -> SysResult<usize> {
		let mut bytes_written = 0usize;
		BoolRet(unsafe {
			ffi::WriteProcessMemory(
				self.ptr(),
				base_address,
				buffer.as_ptr() as _,
				buffer.len(),
				&mut bytes_written,
			)
		})
		.to_sysresult()
		.map(|_| bytes_written)
	}
}
