#![allow(unused_macros)]

/// Declares a handle.
macro_rules! handle {
	(
		$name:ident;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[repr(transparent)]
		#[derive(PartialEq, Eq, Hash)]
		pub struct $name(*mut std::ffi::c_void);

		unsafe impl Send for $name {}

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{:#010x}", self.0 as usize)
			}
		}
		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "[{:#010x} {}] {}",
					self.0 as usize, self.0 as usize, stringify!($name))
			}
		}

		impl std::fmt::LowerHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				std::fmt::LowerHex::fmt(&(self.0 as usize), f)
			}
		}
		impl std::fmt::UpperHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				std::fmt::UpperHex::fmt(&(self.0 as usize), f)
			}
		}

		impl crate::prelude::Handle for $name {
			const NULL: Self = Self(std::ptr::null_mut());
			const INVALID: Self = Self(-1 as _);

			unsafe fn from_ptr(p: *mut std::ffi::c_void) -> Self {
				unsafe { Self::from_ptr(p) }
			}

			unsafe fn raw_copy(&self) -> Self {
				unsafe { self.raw_copy() }
			}

			unsafe fn as_mut(&mut self) -> &mut *mut std::ffi::c_void {
				unsafe { self.as_mut() }
			}

			fn ptr(&self) -> *mut std::ffi::c_void {
				self.ptr()
			}
		}

		impl $name {
			/// Constructs a new handle object by wrapping a pointer.
			///
			/// This method can be used as an escape hatch to interoperate with
			/// other libraries.
			///
			/// # Safety
			///
			/// Be sure the pointer has the correct type and isn't owned by
			/// anyone else, otherwise you may cause memory access violations.
			#[must_use]
			pub const unsafe fn from_ptr(p: *mut std::ffi::c_void) -> Self {
				Self(p)
			}

			/// Returns a raw copy of the underlying handle pointer.
			///
			/// # Safety
			///
			/// As the name implies, `raw_copy` returns a raw copy of the
			/// handle, so closing one of the copies won't close the others.
			/// This means a handle can be used after it has been closed, what
			/// can lead to errors and undefined behavior. Even worse: sometimes
			/// Windows reuses handle values, so you can call a method on a
			/// completely different handle type, what can be catastrophic.
			///
			/// However, in some cases the Windows API *demands* a copy of the
			/// handle – `raw_copy` is an escape hatch to fill this gap.
			#[must_use]
			pub const unsafe fn raw_copy(&self) -> Self {
				unsafe { Self::from_ptr(self.ptr()) }
			}

			/// Returns a mutable reference to the underlying raw pointer.
			///
			/// This method can be used as an escape hatch to interoperate with
			/// other libraries.
			///
			/// # Safety
			///
			/// This method exposes the raw pointer used by raw Windows calls.
			/// It's an opaque pointer to an internal Windows structure, and no
			/// dereferencings should be attempted.
			#[must_use]
			pub const unsafe fn as_mut(&mut self) -> &mut *mut std::ffi::c_void {
				&mut self.0
			}

			/// Returns the underlying raw pointer.
			///
			/// This method exposes the raw pointer used by raw Windows calls.
			/// It's an opaque pointer to an internal Windows structure, and no
			/// dereferencings should be attempted.
			///
			/// This method can be used as an escape hatch to interoperate with
			/// other libraries.
			#[must_use]
			pub const fn ptr(&self) -> *mut std::ffi::c_void {
				self.0
			}
		}
	};
}

/// Declares a handle guard which has a simple cleaner function.
macro_rules! handle_guard {
	(
		$name:ident : $handle:ty;
		$cleaner:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub struct $name {
			handle: $handle,
		}

		impl Drop for $name {
			fn drop(&mut self) {
				if let Some(h) = self.handle.as_opt() {
					unsafe { $cleaner(h.ptr()); } // ignore errors
				}
			}
		}

		impl std::ops::Deref for $name {
			type Target = $handle;

			fn deref(&self) -> &Self::Target {
				&self.handle
			}
		}
		impl std::ops::DerefMut for $name {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.handle
			}
		}

		impl $name {
			/// Constructs the guard by taking ownership of the handle.
			///
			/// This method can be used as an escape hatch to interoperate with
			/// other libraries.
			///
			/// # Safety
			///
			/// Be sure the handle must be freed with the specified function at
			/// the end of scope.
			#[must_use]
			pub const unsafe fn new(handle: $handle) -> Self {
				Self { handle }
			}

			/// Ejects the underlying handle, leaving a
			/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its
			/// place.
			///
			/// Since the internal handle will be invalidated, the destructor
			/// will not run. It's your responsability to run it, otherwise
			/// you'll cause a resource leak.
			#[must_use]
			pub fn leak(&mut self) -> $handle {
				std::mem::replace(&mut self.handle, Handle::INVALID)
			}
		}
	};
}
