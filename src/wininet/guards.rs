use std::ops::{Deref, DerefMut};

use crate::prelude::*;
use crate::wininet::ffi;

/// RAII implementation for wininet handles which automatically calls
/// [`InternetCloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetclosehandle)
/// when the object goes out of scope.
pub struct InternetCloseHandleGuard<T>
where
	T: Handle,
{
	handle: T,
}

impl<T> Drop for InternetCloseHandleGuard<T>
where
	T: Handle,
{
	fn drop(&mut self) {
		if let Some(h) = self.handle.as_opt() {
			unsafe {
				ffi::InternetCloseHandle(h.ptr()); // ignore errors
			}
		}
	}
}

impl<T> Deref for InternetCloseHandleGuard<T>
where
	T: Handle,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.handle
	}
}

impl<T> DerefMut for InternetCloseHandleGuard<T>
where
	T: Handle,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.handle
	}
}

impl<T> InternetCloseHandleGuard<T>
where
	T: Handle,
{
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`InternetCloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetclosehandle)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(handle: T) -> Self {
		Self { handle }
	}

	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> T {
		std::mem::replace(&mut self.handle, T::INVALID)
	}
}
