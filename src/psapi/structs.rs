#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::*;

/// [`PROCESS_MEMORY_COUNTERS_EX`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-process_memory_counters_ex)
/// struct.
#[repr(C)]
pub struct PROCESS_MEMORY_COUNTERS_EX {
	cb: u32,
	pub PageFaultCount: u32,
	pub PeakWorkingSetSize: usize,
	pub WorkingSetSize: usize,
	pub QuotaPeakPagedPoolUsage: usize,
	pub QuotaPagedPoolUsage: usize,
	pub QuotaPeakNonPagedPoolUsage: usize,
	pub QuotaNonPagedPoolUsage: usize,
	pub PagefileUsage: usize,
	pub PeakPagefileUsage: usize,
	pub PrivateUsage: usize,
}

impl_default!(PROCESS_MEMORY_COUNTERS_EX, cb);

/// [`MODULEINFO`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-moduleinfo)
/// struct.
#[repr(C)]
pub struct MODULEINFO {
	pub lpBaseOfDll: PVOID,
	pub SizeOfImage: u32,
	pub EntryPoint: PVOID,
}

impl_default!(MODULEINFO);