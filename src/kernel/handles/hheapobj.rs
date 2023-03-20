#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	GetLastError, HHEAPMEM, PROCESS_HEAP_ENTRY, SetLastError, SysResult,
};
use crate::kernel::guard::{HeapDestroyGuard, HeapFreeGuard, HeapUnlockGuard};
use crate::kernel::privs::{bool_to_sysresult, ptr_to_sysresult_handle};
use crate::prelude::Handle;

impl_handle! { HHEAPOBJ;
	/// Handle to a
	/// [heap object](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcreate).
	/// Originally just a `HANDLE`.
	/// 
	/// Not to be confused with [`HHEAPMEM`](crate::HHEAPMEM).
}

impl kernel_Hheapobj for HHEAPOBJ {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HHEAPOBJ`](crate::HHEAPOBJ).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hheapobj: Handle {
	/// [`GetProcessHeap`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap)
	/// static method.
	#[must_use]
	fn GetProcessHeap() -> SysResult<HHEAPOBJ> {
		ptr_to_sysresult_handle(unsafe { kernel::ffi::GetProcessHeap() })
	}

	/// [`GetProcessHeaps`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheaps)
	/// static method.
	#[must_use]
	fn GetProcessHeaps() -> SysResult<Vec<HHEAPOBJ>> {
		let num = match unsafe {
			kernel::ffi::GetProcessHeaps(0, std::ptr::null_mut())
		} {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => return Ok(Vec::default()), // actual zero heaps
				err => return Err(err),
			},
			num => num,
		};

		let mut buf = [0..num].iter()
			.map(|_| HHEAPOBJ::NULL)
			.collect::<Vec<_>>();

		match unsafe {
			kernel::ffi::GetProcessHeaps(num, buf.as_mut_ptr() as _)
		} {
			0 => Err(GetLastError()),
			_ => Ok(buf)
		}
	}

	/// [`HeapCreate`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcreate)
	/// static method.
	#[must_use]
	fn HeapCreate(
		options: co::HEAP_CREATE,
		initial_size: usize,
		maximum_size: usize,
	) -> SysResult<HeapDestroyGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				kernel::ffi::HeapCreate(options.0, initial_size, maximum_size)
			).map(|h| HeapDestroyGuard::new(h))
		}
	}

	/// [`HeapAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc)
	/// method.
	#[must_use]
	fn HeapAlloc(&self,
		flags: co::HEAP_ALLOC, bytes: usize) -> SysResult<HeapFreeGuard<Self>>
	{
		SetLastError(co::ERROR::SUCCESS);
		unsafe {
			ptr_to_sysresult_handle(
				kernel::ffi::HeapAlloc(self.as_ptr(), flags.0, bytes),
			).map(|h| HeapFreeGuard::new(self.raw_copy(), h))
		}
	}

	/// [`HeapCompact`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcompact)
	/// method.
	fn HeapCompact(&self, flags: co::HEAP_SIZE) -> SysResult<usize> {
		match unsafe { kernel::ffi::HeapCompact(self.as_ptr(), flags.0) } {
			0 => Err(GetLastError()),
			n => Ok(n),
		}
	}

	/// [`HeapLock`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaplock)
	/// method.
	#[must_use]
	fn HeapLock(&self) -> SysResult<HeapUnlockGuard<'_, Self>> {
		unsafe {
			bool_to_sysresult(kernel::ffi::HeapLock(self.as_ptr()))
				.map(|_| HeapUnlockGuard::new(self))
		}
	}

	/// [`HeapReAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc)
	/// method.
	/// 
	/// Originally this method returns the handle to the reallocated memory
	/// object; here the original handle is automatically updated.
	fn HeapReAlloc(&self,
		flags: co::HEAP_REALLOC,
		mem: &mut HHEAPMEM,
		bytes: usize,
	) -> SysResult<()>
	{
		SetLastError(co::ERROR::SUCCESS);
		ptr_to_sysresult_handle(
			unsafe {
				kernel::ffi::HeapReAlloc(
					self.as_ptr(),
					flags.0,
					mem.as_ptr(),
					bytes,
				)
			},
		).map(|h| { *mem = h; })
	}

	/// [`HeapSize`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapsize)
	/// method.
	#[must_use]
	fn HeapSize(&self,
		flags: co::HEAP_SIZE, mem: &HHEAPMEM) -> SysResult<usize>
	{
		SetLastError(co::ERROR::SUCCESS);
		const FAILED: usize = -1isize as usize;

		match unsafe {
			kernel::ffi::HeapSize(self.as_ptr(), flags.0, mem.as_ptr())
		} {
			FAILED => Err(GetLastError()),
			n => Ok(n),
		}
	}

	/// [`HeapWalk`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapwalk)
	/// method.
	#[must_use]
	unsafe fn HeapWalk(&self,
		entry: &mut PROCESS_HEAP_ENTRY) -> SysResult<bool>
	{
		match kernel::ffi::HeapWalk(self.as_ptr(), entry as *mut _ as _) {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_ITEMS => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}
