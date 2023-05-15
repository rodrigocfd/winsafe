#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	GetLastError, PROCESS_HEAP_ENTRY, SetLastError, SysResult,
};
use crate::kernel::guard::{HeapDestroyGuard, HeapFreeGuard, HeapUnlockGuard};
use crate::kernel::privs::{
	bool_to_sysresult, ptr_to_sysresult, ptr_to_sysresult_handle,
};
use crate::prelude::Handle;

impl_handle! { HHEAP;
	/// Handle to a
	/// [heap object](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcreate).
	/// Originally just a `HANDLE`.
	///
	/// If you're allocating memory using just the default process heap – that
	/// is, by calling
	/// [`HHEAP::GetProcessHeap`](crate::prelude::kernel_Hheap::GetProcessHeap)
	/// –, consider using the [`HeapBlock`](crate::HeapBlock) high-level
	/// abstraction.
}

impl kernel_Hheap for HHEAP {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HHEAP`](crate::HHEAP).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hheap: Handle {
	/// Returns an iterator over the heap memory blocks with
	/// [`PROCESS_HEAP_ENTRY`](crate::PROCESS_HEAP_ENTRY) structs. Calls
	/// [`HHEAP::HeapWalk`](crate::prelude::kernel_Hheap::HeapWalk).
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HHEAP};
	///
	/// let heap = HHEAP::GetProcessHeap()?;
	///
	/// for entry in heap.iter_walk() {
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
	fn GetProcessHeap() -> SysResult<HHEAP> {
		ptr_to_sysresult_handle(unsafe { kernel::ffi::GetProcessHeap() })
	}

	/// [`GetProcessHeaps`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheaps)
	/// static method.
	#[must_use]
	fn GetProcessHeaps() -> SysResult<Vec<HHEAP>> {
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
			.map(|_| HHEAP::NULL)
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
		options: Option<co::HEAP_CREATE>,
		initial_size: usize,
		maximum_size: usize,
	) -> SysResult<HeapDestroyGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				kernel::ffi::HeapCreate(
					options.map_or(0, |f| f.raw()),
					initial_size,
					maximum_size,
				),
			).map(|h| HeapDestroyGuard::new(h))
		}
	}

	/// [`HeapAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc)
	/// method.
	///
	/// If you're allocating memory using just the default process heap – that
	/// is, by calling
	/// [`HHEAP::GetProcessHeap`](crate::prelude::kernel_Hheap::GetProcessHeap)
	/// –, consider using the [`HeapBlock`](crate::HeapBlock) high-level
	/// abstraction.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HHEAP};
	///
	/// let heap = HHEAP::GetProcessHeap()?;
	///
	/// let mut array = heap.HeapAlloc(Some(co::HEAP_ALLOC::ZERO_MEMORY), 40)?;
	/// array[0] = 10;
	/// array[1] = 12;
	///
	/// for byte in array.iter() {
	///     println!("{} ", byte);
	/// }
	///
	/// println!("End.");
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn HeapAlloc(&self,
		flags: Option<co::HEAP_ALLOC>,
		num_bytes: usize,
	) -> SysResult<HeapFreeGuard<'_, Self>>
	{
		SetLastError(co::ERROR::SUCCESS);
		unsafe {
			ptr_to_sysresult(
				kernel::ffi::HeapAlloc(
					self.ptr(),
					flags.map_or(0, |f| f.raw()),
					num_bytes,
				),
			).map(|p| HeapFreeGuard::new(self, p, num_bytes))
		}
	}

	/// [`HeapCompact`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcompact)
	/// method.
	fn HeapCompact(&self, flags: Option<co::HEAP_SIZE>) -> SysResult<usize> {
		match unsafe {
			kernel::ffi::HeapCompact(self.ptr(), flags.map_or(0, |f| f.raw()))
		} {
			0 => Err(GetLastError()),
			n => Ok(n),
		}
	}

	/// [`HeapLock`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaplock)
	/// method.
	///
	/// You only need to call this method if the [`HHEAP`](crate::HHEAP) was
	/// created with
	/// [`co::HEAP_CREATE::NO_SERIALIZE`](crate::co::HEAP_CREATE::NO_SERIALIZE).
	/// Otherwise, the heap is already protected against concurrent thread
	/// access.
	///
	/// In the original C implementation, you must call
	/// [`HeapUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapunlock)
	/// as a cleanup operation; here, the cleanup is performed automatically,
	/// because `HeapLock` returns a
	/// [`HeapUnlockGuard`](crate::guard::HeapUnlockGuard), which automatically
	/// calls `HeapUnlock` when the guard goes out of scope. You must, however,
	/// keep the guard alive, otherwise the cleanup will be performed right
	/// away.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HHEAP};
	///
	/// let heap = HHEAP::HeapCreate(Some(co::HEAP_CREATE::NO_SERIALIZE), 0, 0)?;
	///
	/// let _lock = heap.HeapLock()?;
	///
	/// // heap operations...
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn HeapLock(&self) -> SysResult<HeapUnlockGuard<'_, Self>> {
		unsafe {
			bool_to_sysresult(kernel::ffi::HeapLock(self.ptr()))
				.map(|_| HeapUnlockGuard::new(self))
		}
	}

	/// [`HeapReAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc)
	/// method.
	///
	/// Originally this method returns the handle to the reallocated memory
	/// object; here the original handle, present inside
	/// [`HeapFreeGuard`](crate::guard::HeapFreeGuard), is automatically
	/// updated.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HHEAP};
	///
	/// let heap = HHEAP::GetProcessHeap()?;
	/// let mut array = heap.HeapAlloc(Some(co::HEAP_ALLOC::ZERO_MEMORY), 40)?;
	///
	/// heap.HeapReAlloc(Some(co::HEAP_REALLOC::ZERO_MEMORY), &mut array, 65)?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn HeapReAlloc<'a>(&'a self,
		flags: Option<co::HEAP_REALLOC>,
		mem: &mut HeapFreeGuard<'a, Self>,
		num_bytes: usize,
	) -> SysResult<()>
	{
		SetLastError(co::ERROR::SUCCESS);
		ptr_to_sysresult(
			unsafe {
				kernel::ffi::HeapReAlloc(
					self.ptr(),
					flags.map_or(0, |f| f.raw()),
					mem.as_ptr() as _,
					num_bytes,
				)
			},
		).map(|p| {
			let _ = mem.leak();
			*mem = unsafe { HeapFreeGuard::new(self, p, num_bytes) };
		})
	}

	/// [`HeapSize`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapsize)
	/// method.
	#[must_use]
	fn HeapSize(&self,
		flags: Option<co::HEAP_SIZE>,
		mem: &HeapFreeGuard<'_, Self>,
	) -> SysResult<usize>
	{
		SetLastError(co::ERROR::SUCCESS);
		const FAILED: usize = -1isize as usize;

		match unsafe {
			kernel::ffi::HeapSize(
				self.ptr(),
				flags.map_or(0, |f| f.raw()),
				mem.as_ptr() as _,
			)
		} {
			FAILED => Err(GetLastError()),
			n => Ok(n),
		}
	}

	/// [`HeapValidate`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapvalidate)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HHEAP};
	///
	/// let heap = HHEAP::GetProcessHeap()?;
	/// let mut array = heap.HeapAlloc(Some(co::HEAP_ALLOC::ZERO_MEMORY), 40)?;
	///
	/// let is_ok = heap.HeapValidate(None, Some(&array));
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn HeapValidate(&self,
		flags: Option<co::HEAP_SIZE>,
		mem: Option<&HeapFreeGuard<'_, Self>>,
	) -> bool
	{
		unsafe {
			kernel::ffi::HeapValidate(
				self.ptr(),
				flags.map_or(0, |f| f.raw()),
				mem.map_or(std::ptr::null_mut(), |mem| mem.as_ptr() as _),
			) != 0
		}
	}

	/// [`HeapWalk`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapwalk)
	/// method.
	///
	/// Prefer using
	/// [`HHEAP::iter_walk`](crate::prelude::kernel_Hheap::iter_walk), which is
	/// simpler.
	#[must_use]
	fn HeapWalk(&self, entry: &mut PROCESS_HEAP_ENTRY) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::HeapWalk(self.ptr(), entry as *mut _ as _)
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
	where H: kernel_Hheap,
{
	hheap: &'a H,
	entry: PROCESS_HEAP_ENTRY,
	has_more: bool,
}

impl<'a, H> Iterator for WalkIter<'a, H>
	where H: kernel_Hheap,
{
	type Item = SysResult<&'a PROCESS_HEAP_ENTRY>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		match self.hheap.HeapWalk(&mut self.entry) {
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
	where H: kernel_Hheap,
{
	fn new(hheap: &'a H) -> Self {
		Self {
			hheap,
			entry: PROCESS_HEAP_ENTRY::default(),
			has_more: true,
		}
	}
}
