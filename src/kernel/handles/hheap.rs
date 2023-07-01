#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	GetLastError, PROCESS_HEAP_ENTRY, SetLastError, SysResult,
};
use crate::kernel::guard::{HeapDestroyGuard, HeapFreeGuard, HeapUnlockGuard};
use crate::kernel::iterators::HheapHeapwalkIter;
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
	/// [`GetProcessHeap`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap)
	/// function.
	#[must_use]
	fn GetProcessHeap() -> SysResult<HHEAP> {
		ptr_to_sysresult_handle(unsafe { kernel::ffi::GetProcessHeap() })
	}

	/// [`GetProcessHeaps`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheaps)
	/// function.
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
	/// function.
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
					options.unwrap_or_default().raw(),
					initial_size,
					maximum_size,
				),
			).map(|h| HeapDestroyGuard::new(h))
		}
	}

	/// [`HeapAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc)
	/// function.
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
	/// let mut block = heap.HeapAlloc(Some(co::HEAP_ALLOC::ZERO_MEMORY), 40)?;
	/// block.as_mut_slice()[0] = 10;
	/// block.as_mut_slice()[1] = 12;
	///
	/// for byte in block.as_slice().iter() {
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
					flags.unwrap_or_default().raw(),
					num_bytes,
				),
			).map(|p| HeapFreeGuard::new(self, p, num_bytes))
		}
	}

	/// [`HeapCompact`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcompact)
	/// function.
	fn HeapCompact(&self, flags: Option<co::HEAP_SIZE>) -> SysResult<usize> {
		match unsafe {
			kernel::ffi::HeapCompact(self.ptr(), flags.unwrap_or_default().raw())
		} {
			0 => Err(GetLastError()),
			n => Ok(n),
		}
	}

	/// [`HeapLock`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaplock)
	/// function.
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
	/// function.
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
					flags.unwrap_or_default().raw(),
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
	/// function.
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
				flags.unwrap_or_default().raw(),
				mem.as_ptr() as _,
			)
		} {
			FAILED => Err(GetLastError()),
			n => Ok(n),
		}
	}

	/// [`HeapValidate`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapvalidate)
	/// function.
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
		SetLastError(co::ERROR::SUCCESS);
		unsafe {
			kernel::ffi::HeapValidate(
				self.ptr(),
				flags.unwrap_or_default().raw(),
				mem.map_or(std::ptr::null_mut(), |mem| mem.as_ptr() as _),
			) != 0
		}
	}

	/// [`HeapWalk`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapwalk)
	/// function.
	///
	/// Returns an iterator over the heap memory blocks, exposing
	/// [`PROCESS_HEAP_ENTRY`](crate::PROCESS_HEAP_ENTRY) structs.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HHEAP};
	///
	/// let heap = HHEAP::GetProcessHeap()?;
	///
	/// for block in heap.HeapWalk() {
	///     let block = block?;
	///     println!("Size: {}, overhead? {}",
	///         block.cbData, block.cbOverhead);
	/// }
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn HeapWalk(&self
	) -> Box<dyn Iterator<Item = SysResult<&PROCESS_HEAP_ENTRY>> + '_>
	{
		Box::new(HheapHeapwalkIter::new(self))
	}
}
