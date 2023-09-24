use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::prelude::*;

/// Manages an [`HHEAP`](crate::HHEAP) memory block which uses the default heap
/// of the calling process – that is, calls
/// [`HHEAP::GetProcessHeap`](crate::prelude::kernel_Hheap::GetProcessHeap).
///
/// The memory block is freed automatically when the object goes out of scope.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let mut bl = w::HeapBlock::alloc(10)?;
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
		if !self.pmem.is_null() {
			let _ = unsafe { HeapFreeGuard::new(&self.hheap, self.pmem, self.sz) };
		}
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

	pub_fn_mem_block!();
}
