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
	/// [`GetProcessMemoryInfo`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getprocessmemoryinfo)
	/// function.
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

	/// [`EnumProcessModules`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-enumprocessmodules)
	/// function.
	#[must_use]
	fn EnumProcessModules(
		&self,
		buf_size: usize,
	) -> SysResult<Vec<HMODULE>> {
		let mut hmodule_buffer: Vec<HMODULE> = (0..buf_size).map(|_| HMODULE::NULL).collect();
		let mut cb_needed = 0;

		bool_to_sysresult(unsafe {
			ffi::EnumProcessModules(
				self.ptr(),
				hmodule_buffer.as_mut_ptr() as _,
				hmodule_buffer.len() as _,
				&mut cb_needed
			)
		})
		.map(|_| {
			let actual_len = (cb_needed as usize) / std::mem::size_of::<HMODULE>();
			hmodule_buffer.truncate(actual_len.min(buf_size));
			hmodule_buffer
		})
	}

	/// [`GetModuleBaseNameA`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulebasenamea)
	/// function.
	#[must_use]
	fn GetModuleBaseNameA(
		&self,
		hmodule: Option<HMODULE>,
		sz: usize,
	) -> SysResult<WString> {
		let mut buf = WString::new_alloc_buf(sz);
		unsafe {
			ffi::GetModuleBaseNameA(
				self.ptr(),
				hmodule.map(|x| x.ptr()).unwrap_or_else(std::ptr::null_mut),
				buf.as_mut_ptr(),
				buf.buf_len() as u32,
			)
		}
			.eq(&0)
			.then(|| buf)
			.ok_or_else(GetLastError)
	}

	/// [`GetModuleInformation`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmoduleinformation)
    /// function.
    #[must_use]
    fn GetModuleInformation(
		&self,
		hmodule: HMODULE,
	) -> SysResult<MODULEINFO> {
		let mut mod_info = MODULEINFO::default();
		bool_to_sysresult(unsafe {
			ffi::GetModuleInformation(
				self.ptr(),
				hmodule.ptr(),
				&mut mod_info,
				std::mem::size_of::<MODULEINFO>() as u32,
			)
		})
		.map(|_| mod_info)
    }
}
