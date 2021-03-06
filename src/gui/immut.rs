use std::cell::UnsafeCell;
use std::ops::Deref;

/// Wraps an object providing internal immutability, with no performance penalty
/// but also no race condition guarantees. Be sure to use it safely.
pub struct Immut<T>(UnsafeCell<T>);

impl<T> Deref for Immut<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0.get() }
	}
}

impl<T> Immut<T> {
	/// Instantiates a new object.
	pub fn new(obj: T) -> Immut<T> {
		Self(UnsafeCell::new(obj))
	}

	/// Returns a mutable reference to the underlying object, which can lead to
	/// race conditions.
	pub fn as_mut(&self) -> &mut T {
		unsafe { &mut *self.0.get() }
	}
}
