use std::cell::UnsafeCell;
use std::ops::Deref;

/// Syntactic sugar to `UnsafeCell`.
///
/// **Extremely** unsafe, intended only to provide less verbose internal
/// mutability within the `gui` module.
pub(crate) struct VeryUnsafeCell<T>(UnsafeCell<T>);

unsafe impl<T> Send for VeryUnsafeCell<T> {}
unsafe impl<T> Sync for VeryUnsafeCell<T> {}

impl<T> Deref for VeryUnsafeCell<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0.get() } // immutable reference
	}
}

impl<T> VeryUnsafeCell<T> {
	/// Instantiates a new object.
	pub fn new(obj: T) -> VeryUnsafeCell<T> {
		Self(UnsafeCell::new(obj))
	}

	/// Returns a mutable reference to the underlying object.
	pub fn as_mut(&self) -> &mut T {
		unsafe { &mut *self.0.get() }
	}
}
