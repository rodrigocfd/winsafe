#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::psapi::ffi;

impl HPROCESS {
	/// [`EmptyWorkingSet`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-emptyworkingset)
	/// function.
	pub fn EmptyWorkingSet(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::EmptyWorkingSet(self.ptr()) }).to_sysresult()
	}

	/// [`EnumProcessModules`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodules)
	/// function.
	#[must_use]
	pub fn EnumProcessModules(&self) -> SysResult<Vec<HINSTANCE>> {
		loop {
			let mut bytes_needed = 0u32;
			BoolRet(unsafe {
				ffi::EnumProcessModules(self.ptr(), std::ptr::null_mut(), 0, &mut bytes_needed)
			})
			.to_sysresult()?;

			let elems_needed = bytes_needed / (std::mem::size_of::<HINSTANCE>() as u32);
			let mut buf = (0..elems_needed)
				.map(|_| HINSTANCE::NULL)
				.collect::<Vec<_>>();

			let mut bytes_got = 0u32;
			BoolRet(unsafe {
				ffi::EnumProcessModules(
					self.ptr(),
					buf.as_mut_ptr() as _,
					bytes_needed,
					&mut bytes_got,
				)
			})
			.to_sysresult()?;

			if bytes_needed == bytes_got {
				return Ok(buf);
			}
		}
	}

	/// [`GetMappedFileName`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmappedfilenamew)
	/// function.
	#[must_use]
	pub fn GetMappedFileName(&self, address: *mut std::ffi::c_void) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1); // arbitrary
		BoolRet(unsafe {
			ffi::GetMappedFileNameW(self.ptr(), address, buf.as_mut_ptr(), buf.buf_len() as _)
		})
		.to_sysresult()
		.map(|_| buf.to_string())
	}

	/// [`GetModuleBaseName`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulebasenamew)
	/// function.
	#[must_use]
	pub fn GetModuleBaseName(&self, hmodule: Option<&HINSTANCE>) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1); // arbitrary
		BoolRet(unsafe {
			ffi::GetModuleBaseNameW(
				self.ptr(),
				hmodule.map_or(std::ptr::null_mut(), |h| h.ptr()),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
			)
		})
		.to_sysresult()
		.map(|_| buf.to_string())
	}

	/// [`GetModuleFileNameEx`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulefilenameexw)
	/// function.
	#[must_use]
	pub fn GetModuleFileNameEx(&self, hmodule: Option<&HINSTANCE>) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1); // arbitrary
		BoolRet(unsafe {
			ffi::GetModuleFileNameExW(
				self.ptr(),
				hmodule.map_or(std::ptr::null_mut(), |h| h.ptr()),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
			)
		})
		.to_sysresult()
		.map(|_| buf.to_string())
	}

	/// [`GetModuleInformation`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation)
	/// function.
	#[must_use]
	pub fn GetModuleInformation(&self, hmodule: Option<&HINSTANCE>) -> SysResult<MODULEINFO> {
		let mut mi = MODULEINFO::default();
		BoolRet(unsafe {
			ffi::GetModuleInformation(
				self.ptr(),
				hmodule.map_or(std::ptr::null_mut(), |h| h.ptr()),
				pvoid(&mut mi),
				std::mem::size_of::<MODULEINFO>() as _,
			)
		})
		.to_sysresult()
		.map(|_| mi)
	}

	/// [`GetProcessImageFileName`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getprocessimagefilenamew)
	/// function.
	#[must_use]
	pub fn GetProcessImageFileName(&self) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1); // arbitrary
		BoolRet(unsafe {
			ffi::GetProcessImageFileNameW(self.ptr(), buf.as_mut_ptr(), buf.buf_len() as _)
		})
		.to_sysresult()
		.map(|_| buf.to_string())
	}

	/// [`GetProcessMemoryInfo`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getprocessmemoryinfo)
	/// function.
	#[must_use]
	pub fn GetProcessMemoryInfo(&self) -> SysResult<PROCESS_MEMORY_COUNTERS_EX> {
		let mut pmc = PROCESS_MEMORY_COUNTERS_EX::default();
		BoolRet(unsafe {
			ffi::GetProcessMemoryInfo(
				self.ptr(),
				pvoid(&mut pmc),
				std::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as _,
			)
		})
		.to_sysresult()
		.map(|_| pmc)
	}
}
