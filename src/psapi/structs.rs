#![allow(non_camel_case_types, non_snake_case)]

/// [`MODULEINFO`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-moduleinfo)
/// struct.
#[repr(C)]
pub struct MODULEINFO {
	pub lpBaseOfDll: *mut std::ffi::c_void,
	pub SizeOfImage: u32,
	pub EntryPoint: *mut std::ffi::c_void,
}

impl_default!(MODULEINFO);

/// [`PERFORMANCE_INFORMATION`](https://learn.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-performance_information)
/// struct.
#[repr(C)]
pub struct PERFORMANCE_INFORMATION {
	cb: u32,
	pub CommitTotal: usize,
	pub CommitLimit: usize,
	pub CommitPeak: usize,
	pub PhysicalTotal: usize,
	pub PhysicalAvailable: usize,
	pub SystemCache: usize,
	pub KernelTotal: usize,
	pub KernelPaged: usize,
	pub KernelNonpaged: usize,
	pub PageSize: usize,
	pub HandleCount: u32,
	pub ProcessCount: u32,
	pub ThreadCount: u32,
}

impl_default!(PERFORMANCE_INFORMATION, cb);

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
