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
/// for (idx, b) in bl.mem_mut().iter_mut().enumerate() {
///     *b = (idx * 10) as _;
/// }
///
/// for b in bl.mem().iter() {
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

	/// Returns the size of the allocated memory block.
	#[must_use]
	pub const fn len(&self) -> usize {
		self.sz
	}

	/// Returns a slice over the allocated memory block.
	#[must_use]
	pub const fn mem(&self) -> &[u8] {
		unsafe { std::slice::from_raw_parts(self.pmem as _, self.sz) }
	}

	/// Returns a slice over the allocated memory block, casting the pointer to
	/// the given type.
	///
	/// # Safety
	///
	/// Be sure the alignment and block length are correct.
	#[must_use]
	pub const unsafe fn mem_align<T>(&self) -> &[T] {
		std::slice::from_raw_parts(
			self.pmem as _,
			self.sz / std::mem::size_of::<T>(),
		)
	}

	/// Returns a mutable slice over the allocated memory block.
	#[must_use]
	pub fn mem_mut(&mut self) -> &mut [u8] {
		unsafe { std::slice::from_raw_parts_mut(self.pmem as _, self.sz) }
	}

	/// Returns a mutable slice over the allocated memory block, casting the
	/// pointer to the given type.
	///
	/// # Safety
	///
	/// Be sure the alignment and block length are correct.
	#[must_use]
	pub unsafe fn mem_mut_align<T>(&mut self) -> &mut [T] {
		std::slice::from_raw_parts_mut(
			self.pmem as _,
			self.sz / std::mem::size_of::<T>(),
		)
	}

	/// Returns the pointer to the allocated memory block.
	#[must_use]
	pub const fn ptr(&self) -> *mut std::ffi::c_void {
		self.pmem
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
}
