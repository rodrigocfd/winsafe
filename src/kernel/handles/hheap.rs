#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, iterators::*, privs::*};
use crate::prelude::*;

handle! { HHEAP;
	/// Handle to a
	/// [heap object](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcreate).
	/// Originally just a `HANDLE`.
}

impl HHEAP {
	/// [`GetProcessHeap`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap)
	/// function.
	#[must_use]
	pub fn GetProcessHeap() -> SysResult<HHEAP> {
		PtrRet(unsafe { ffi::GetProcessHeap() }).to_sysresult_handle()
	}

	/// [`GetProcessHeaps`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheaps)
	/// function.
	#[must_use]
	pub fn GetProcessHeaps() -> SysResult<Vec<HHEAP>> {
		let num = match unsafe { ffi::GetProcessHeaps(0, std::ptr::null_mut()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => return Ok(Vec::new()), // actual zero heaps
				err => return Err(err),
			},
			num => num,
		};

		let mut buf = [0..num].iter().map(|_| HHEAP::NULL).collect::<Vec<_>>();
		BoolRet(unsafe { ffi::GetProcessHeaps(num, buf.as_mut_ptr() as _) } as _)
			.to_sysresult()
			.map(|_| buf)
	}

	/// [`HeapCreate`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcreate)
	/// function.
	#[must_use]
	pub fn HeapCreate(
		options: Option<co::HEAP_CREATE>,
		initial_size: usize,
		maximum_size: usize,
	) -> SysResult<HeapDestroyGuard> {
		unsafe {
			PtrRet(ffi::HeapCreate(options.unwrap_or_default().raw(), initial_size, maximum_size))
				.to_sysresult_handle()
				.map(|h| HeapDestroyGuard::new(h))
		}
	}

	/// [`HeapAlloc`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let heap = w::HHEAP::GetProcessHeap()?;
	///
	/// let mut block = heap.HeapAlloc(Some(co::HEAP_ALLOC::ZERO_MEMORY), 40)?;
	/// block.as_mut_slice()[0] = 10;
	/// block.as_mut_slice()[1] = 12;
	///
	/// for byte in block.as_slice().iter() {
	///     println!("{} ", byte);
	/// }
	///
	/// // HeapFree() automatically called
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn HeapAlloc(
		&self,
		flags: Option<co::HEAP_ALLOC>,
		num_bytes: usize,
	) -> SysResult<HeapFreeGuard<'_>> {
		SetLastError(co::ERROR::SUCCESS);
		unsafe {
			PtrRet(ffi::HeapAlloc(self.ptr(), flags.unwrap_or_default().raw(), num_bytes))
				.to_sysresult()
				.map(|p| HeapFreeGuard::new(self, p, num_bytes))
		}
	}

	/// [`HeapCompact`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapcompact)
	/// function.
	pub fn HeapCompact(&self, flags: Option<co::HEAP_SIZE>) -> SysResult<usize> {
		match unsafe { ffi::HeapCompact(self.ptr(), flags.unwrap_or_default().raw()) } {
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
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let heap = w::HHEAP::HeapCreate(
	///     Some(co::HEAP_CREATE::NO_SERIALIZE), 0, 0)?;
	///
	/// let _lock = heap.HeapLock()?;
	///
	/// // heap operations...
	///
	/// // HeapUnlock() automatically called
	///
	/// // HeapDestroy() automatically called
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn HeapLock(&self) -> SysResult<HeapUnlockGuard<'_>> {
		unsafe {
			BoolRet(ffi::HeapLock(self.ptr()))
				.to_sysresult()
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
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let heap = w::HHEAP::GetProcessHeap()?;
	/// let mut array = heap.HeapAlloc(Some(co::HEAP_ALLOC::ZERO_MEMORY), 40)?;
	///
	/// heap.HeapReAlloc(Some(co::HEAP_REALLOC::ZERO_MEMORY), &mut array, 65)?;
	///
	/// // HeapFree() automatically called
	/// # w::SysResult::Ok(())
	/// ```
	pub fn HeapReAlloc<'a>(
		&'a self,
		flags: Option<co::HEAP_REALLOC>,
		mem: &mut HeapFreeGuard<'a>,
		num_bytes: usize,
	) -> SysResult<()> {
		SetLastError(co::ERROR::SUCCESS);
		PtrRet(unsafe {
			ffi::HeapReAlloc(
				self.ptr(),
				flags.unwrap_or_default().raw(),
				mem.as_ptr() as _,
				num_bytes,
			)
		})
		.to_sysresult()
		.map(|p| {
			let _ = mem.leak();
			*mem = unsafe { HeapFreeGuard::new(self, p, num_bytes) };
		})
	}

	/// [`HeapSetInformation`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapsetinformation)
	/// function.
	pub fn HeapSetInformation(
		&self,
		information_class: co::HEAP_INFORMATION,
		information: &[u8],
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::HeapSetInformation(
				self.ptr(),
				information_class.raw(),
				vec_ptr(information) as _,
				information.len() as _,
			)
		})
		.to_sysresult()
	}

	/// [`HeapSize`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapsize)
	/// function.
	#[must_use]
	pub fn HeapSize(
		&self,
		flags: Option<co::HEAP_SIZE>,
		mem: &HeapFreeGuard<'_>,
	) -> SysResult<usize> {
		SetLastError(co::ERROR::SUCCESS);
		const FAILED: usize = -1isize as usize;

		match unsafe {
			ffi::HeapSize(self.ptr(), flags.unwrap_or_default().raw(), mem.as_ptr() as _)
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
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let heap = w::HHEAP::GetProcessHeap()?;
	/// let mut array = heap.HeapAlloc(Some(co::HEAP_ALLOC::ZERO_MEMORY), 40)?;
	///
	/// let is_ok = heap.HeapValidate(None, Some(&array));
	///
	/// // HeapFree() automatically called
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn HeapValidate(
		&self,
		flags: Option<co::HEAP_SIZE>,
		mem: Option<&HeapFreeGuard<'_>>,
	) -> bool {
		SetLastError(co::ERROR::SUCCESS);
		unsafe {
			ffi::HeapValidate(
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
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let heap = w::HHEAP::GetProcessHeap()?;
	///
	/// for block in heap.HeapWalk() {
	///     let block = block?;
	///     println!("Size: {}, overhead? {}",
	///         block.cbData, block.cbOverhead);
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn HeapWalk(&self) -> impl Iterator<Item = SysResult<&PROCESS_HEAP_ENTRY>> + '_ {
		HheapHeapwalkIter::new(self)
	}
}
