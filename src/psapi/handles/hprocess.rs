#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::psapi::ffi;

impl psapi_Hprocess for HPROCESS {}

/// This trait is enabled with the `psapi` feature, and provides methods for
/// [`HPROCESS`](crate::HPROCESS).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait psapi_Hprocess: kernel_Hprocess {
	/// [`EmptyWorkingSet`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-emptyworkingset)
	/// function.
	fn EmptyWorkingSet(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::EmptyWorkingSet(self.ptr()) })
	}

	/// [`EnumProcessModules`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodules)
	/// function.
	#[must_use]
	fn EnumProcessModules(&self) -> SysResult<Vec<HINSTANCE>> {
		loop {
			let mut bytes_needed = u32::default();
			bool_to_sysresult(unsafe {
				ffi::EnumProcessModules(self.ptr(), std::ptr::null_mut(), 0, &mut bytes_needed)
			})?;

			let elems_needed = bytes_needed / (std::mem::size_of::<HINSTANCE>() as u32);
			let mut buf = (0..elems_needed)
				.map(|_| HINSTANCE::NULL)
				.collect::<Vec<_>>();

			let mut bytes_got = u32::default();
			bool_to_sysresult(unsafe {
				ffi::EnumProcessModules(
					self.ptr(),
					buf.as_mut_ptr() as _,
					bytes_needed,
					&mut bytes_got,
				)
			})?;

			if bytes_needed == bytes_got {
				return Ok(buf);
			}
		}
	}

	/// [`GetMappedFileName`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmappedfilenamew)
	/// function.
	#[must_use]
	fn GetMappedFileName(&self, address: *mut std::ffi::c_void) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1); // arbitrary
		bool_to_sysresult(unsafe {
			ffi::GetMappedFileNameW(self.ptr(), address, buf.as_mut_ptr(), buf.buf_len() as _)
		})
		.map(|_| buf.to_string())
	}

	/// [`GetModuleBaseName`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulebasenamew)
	/// function.
	#[must_use]
	fn GetModuleBaseName(&self, hmodule: Option<&HINSTANCE>) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1); // arbitrary
		bool_to_sysresult(unsafe {
			ffi::GetModuleBaseNameW(
				self.ptr(),
				hmodule.map_or(std::ptr::null_mut(), |h| h.ptr()),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
			)
		})
		.map(|_| buf.to_string())
	}

	/// [`GetModuleFileNameEx`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulefilenameexw)
	/// function.
	#[must_use]
	fn GetModuleFileNameEx(&self, hmodule: Option<&HINSTANCE>) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1); // arbitrary
		bool_to_sysresult(unsafe {
			ffi::GetModuleFileNameExW(
				self.ptr(),
				hmodule.map_or(std::ptr::null_mut(), |h| h.ptr()),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
			)
		})
		.map(|_| buf.to_string())
	}

	/// [`GetModuleInformation`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation)
	/// function.
	#[must_use]
	fn GetModuleInformation(&self, hmodule: Option<&HINSTANCE>) -> SysResult<MODULEINFO> {
		let mut mi = MODULEINFO::default();
		bool_to_sysresult(unsafe {
			ffi::GetModuleInformation(
				self.ptr(),
				hmodule.map_or(std::ptr::null_mut(), |h| h.ptr()),
				pvoid(&mut mi),
				std::mem::size_of::<MODULEINFO>() as _,
			)
		})
		.map(|_| mi)
	}

	/// [`GetProcessImageFileName`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getprocessimagefilenamew)
	/// function.
	#[must_use]
	fn GetProcessImageFileName(&self) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1); // arbitrary
		bool_to_sysresult(unsafe {
			ffi::GetProcessImageFileNameW(self.ptr(), buf.as_mut_ptr(), buf.buf_len() as _)
		})
		.map(|_| buf.to_string())
	}

	/// [`GetProcessMemoryInfo`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getprocessmemoryinfo)
	/// function.
	#[must_use]
	fn GetProcessMemoryInfo(&self) -> SysResult<PROCESS_MEMORY_COUNTERS_EX> {
		let mut pmc = PROCESS_MEMORY_COUNTERS_EX::default();
		bool_to_sysresult(unsafe {
			ffi::GetProcessMemoryInfo(
				self.ptr(),
				pvoid(&mut pmc),
				std::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as _,
			)
		})
		.map(|_| pmc)
	}
}
