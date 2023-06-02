use crate::co;
use crate::kernel::decl::{HHEAP, SysResult};
use crate::kernel::guard::HeapFreeGuard;
use crate::prelude::{kernel_Hheap, Handle};

/// Manages an [`HHEAP`](crate::HHEAP) memory block which uses the default heap
/// of the calling process – that is, calls
/// [`HHEAP::GetProcessHeap`](crate::prelude::kernel_Hheap::GetProcessHeap).
///
/// The memory block is freed automatically when the object goes out of scope.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::HeapBlock;
///
/// let mut bl = HeapBlock::alloc(10)?;
///
/// for (idx, b) in bl.as_mut_slice().iter_mut().enumerate() {
///     *b = (idx * 10) as _;
/// }
///
/// for b in bl.as_slice().iter() {
///     print!("{} ", b);
/// }
/// println!("");
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
pub struct HeapBlock {
	hheap: HHEAP,
	pmem: *mut std::ffi::c_void,
	sz: usize,
}

impl Drop for HeapBlock {
	fn drop(&mut self) {
		let _ = unsafe { HeapFreeGuard::new(&self.hheap, self.pmem, self.sz) };
	}
}

impl HeapBlock {
	/// Creates a new memory block by allocating the given number of bytes by
	/// calling
	/// [`HHEAP::GetProcessHeap`](crate::prelude::kernel_Hheap::GetProcessHeap)
	/// and [`HHEAP::HeapAlloc`](crate::prelude::kernel_Hheap::HeapAlloc), with
	/// [`co::HEAP_ALLOC::ZERO_MEMORY`](crate::co::HEAP_ALLOC::ZERO_MEMORY).
	#[must_use]
	pub fn alloc(num_bytes: usize) -> SysResult<Self> {
		let hheap = HHEAP::GetProcessHeap()?;
		let mut block = hheap.HeapAlloc(Some(co::HEAP_ALLOC::ZERO_MEMORY), num_bytes)?;
		let (pmem, sz) = block.leak();
		Ok(Self {
			hheap: unsafe { hheap.raw_copy() },
			pmem,
			sz,
		})
	}

	/// Resizes the memory block to the given number of bytes by calling
	/// [`HHEAP::HeapReAlloc`](crate::prelude::kernel_Hheap::HeapReAlloc), with
	/// [`co::HEAP_REALLOC::ZERO_MEMORY`](crate::co::HEAP_REALLOC::ZERO_MEMORY).
	pub fn realloc(&mut self, num_bytes: usize) -> SysResult<()> {
		let mut block = unsafe { HeapFreeGuard::new(&self.hheap, self.pmem, self.sz) };
		self.hheap.HeapReAlloc(Some(co::HEAP_REALLOC::ZERO_MEMORY), &mut block, num_bytes)?;
		let (pmem, _) = block.leak();
		self.pmem = pmem;
		self.sz = num_bytes;
		Ok(())
	}

	/// Returns a reference to the current process [`HHEAP`](crate::HHEAP).
	#[must_use]
	pub const fn hheap(&self) -> &HHEAP {
		&self.hheap
	}

	/// Returns the size of the allocated memory block.
	#[must_use]
	pub const fn len(&self) -> usize {
		self.sz
	}

	/// Returns a pointer to the allocated memory block.
	#[must_use]
	pub const fn as_ptr(&self) -> *const std::ffi::c_void {
		self.pmem
	}

	/// Returns a mutable pointer to the allocated memory block.
	#[must_use]
	pub fn as_mut_ptr(&mut self) -> *mut std::ffi::c_void {
		self.pmem
	}

	/// Returns a slice over the allocated memory block.
	#[must_use]
	pub const fn as_slice(&self) -> &[u8] {
		unsafe { std::slice::from_raw_parts(self.pmem.cast(), self.sz) }
	}

	/// Returns a mutable slice over the allocated memory block.
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [u8] {
		unsafe { std::slice::from_raw_parts_mut(self.pmem.cast(), self.sz) }
	}

	/// Returns a slice over the allocated memory block, aligned to the given
	/// type.
	///
	/// # Safety
	///
	/// Make sure the alignment is correct.
	#[must_use]
	pub const unsafe fn as_slice_aligned<T>(&self) -> &[T] {
		std::slice::from_raw_parts(
			self.pmem.cast(),
			self.sz / std::mem::size_of::<T>(),
		)
	}

	/// Returns a mutable slice over the allocated memory block, aligned to the
	/// given type.
	///
	/// # Safety
	///
	/// Make sure the alignment is correct.
	#[must_use]
	pub unsafe fn as_mut_slice_aligned<T>(&mut self) -> &mut [T] {
		std::slice::from_raw_parts_mut(
			self.pmem.cast(),
			self.sz / std::mem::size_of::<T>(),
		)
	}
}
