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
	/// Returns an iterator over the heap memory blocks with
	/// [`PROCESS_HEAP_ENTRY`](crate::PROCESS_HEAP_ENTRY) structs. Calls
	/// [`HHEAPOBJ::HeapWalk`](crate::prelude::kernel_Hheapobj::HeapWalk).
	/// 
	/// # Examples
	/// 
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HHEAPOBJ};
	/// 
	/// let heap_obj = HHEAPOBJ::HeapCreate(co::HEAP_CREATE::NoValue, 0, 0)?;
	/// let _lock = heap_obj.HeapLock()?;
	/// 
	/// for entry in heap_obj.iter_walk() {
	///     let entry = entry?;
	///     println!("Size: {}, overhead? {}",
	///         entry.cbData, entry.cbOverhead);
	/// }
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_walk(&self
	) -> Box<dyn Iterator<Item = SysResult<&PROCESS_HEAP_ENTRY>> + '_>
	{
		Box::new(WalkIter::new(self))
	}

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

		bool_to_sysresult(
			unsafe {
				kernel::ffi::GetProcessHeaps(num, buf.as_mut_ptr() as _)
			} as _,
		).map(|_| buf)
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
	/// 
	/// In the original C implementation, you must call
	/// [`HeapUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapunlock)
	/// as a cleanup operation.
	/// 
	/// Here, the cleanup is performed automatically, because `HeapLock` returns
	/// a [`HeapUnlockGuard`](crate::guard::HeapUnlockGuard), which
	/// automatically calls `HeapUnlock` when the guard goes out of scope. You
	/// must, however, keep the guard alive, otherwise the cleanup will be
	/// performed right away.
	/// 
	/// # Examples
	/// 
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HHEAPOBJ};
	/// 
	/// let heap_obj = HHEAPOBJ::HeapCreate(co::HEAP_CREATE::NoValue, 0, 0)?;
	/// 
	/// let _lock = heap_obj.HeapLock()?;
	/// 
	/// // heap operations...
	/// # Ok::<_, co::ERROR>(())
	/// ```
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
	/// 
	/// Prefer using
	/// [`HHEAPOBJ::iter_walk`](crate::prelude::kernel_Hheapobj::iter_walk),
	/// which is simpler.
	#[must_use]
	fn HeapWalk(&self, entry: &mut PROCESS_HEAP_ENTRY) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::HeapWalk(self.as_ptr(), entry as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_ITEMS => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}

//------------------------------------------------------------------------------

struct WalkIter<'a, H>
	where H: kernel_Hheapobj,
{
	hheapobj: &'a H,
	entry: PROCESS_HEAP_ENTRY,
	has_more: bool,
}

impl<'a, H> Iterator for WalkIter<'a, H>
	where H: kernel_Hheapobj,
{
	type Item = SysResult<&'a PROCESS_HEAP_ENTRY>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		match self.hheapobj.HeapWalk(&mut self.entry) {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(found) => {
				if found {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.entry as *const PROCESS_HEAP_ENTRY;
					Some(Ok(unsafe { &*ptr }))
				} else {
					self.has_more = false; // no further iterations
					None
				}
			},
		}
	}
}

impl<'a, H> WalkIter<'a, H>
	where H: kernel_Hheapobj,
{
	fn new(hheapobj: &'a H) -> Self {
		Self {
			hheapobj,
			entry: PROCESS_HEAP_ENTRY::default(),
			has_more: true,
		}
	}
}
