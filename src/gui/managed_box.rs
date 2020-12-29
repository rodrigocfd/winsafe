/// Performs manual memory management by keeping a raw pointer to a
/// heap-allocated memory block. All cloned objects will have a pointer to the
/// memory block of the original object, which **must** outlive them all.
///
/// Intended to be used within the window-related objects, and should **not** be
/// used in other situations.
///
/// This goal could be safely achieved with `Arc` and `RwLock`, but it would
/// incur in an unnecessary cost, since the windows outlive their child objects
/// and messages are always processed in the UI thread.
pub struct ManagedBox<T> {
	original: bool,
	ptr: *mut T,
}

unsafe impl<T> Send for ManagedBox<T> {}
unsafe impl<T> Sync for ManagedBox<T> {}

impl<T> Drop for ManagedBox<T> {
	fn drop(&mut self) {
		if self.original {
			unsafe { Box::from_raw(self.ptr); } // release the memory
		}
	}
}

impl<T> Clone for ManagedBox<T> {
	fn clone(&self) -> Self {
		Self {
			original: false, // clones won't release the memory
			ptr: self.ptr, // simply copy away the pointer
		}
	}
}

impl<T> ManagedBox<T> {
	/// Creates a new object.
	pub fn new(obj: T) -> ManagedBox<T> {
		Self {
			original: true, // this is the object that will actually release the memory
			ptr: Box::into_raw(Box::new(obj)), // leak and keep the pointer
		}
	}

	/// Returns a mutable reference to the heap-allocated object.
	pub fn as_mut(&self) -> &mut T {
		unsafe { &mut *(self.ptr as *mut T) }
	}

	/// Returns a reference to the heap-allocated object.
	pub fn as_ref(&self) -> &T {
		unsafe { &*(self.ptr as *const T) }
	}
}