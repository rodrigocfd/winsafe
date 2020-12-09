// Declares the type of a handle.
macro_rules! handle_type {
	(
		$(#[$attr:meta])*
		$name:ident
	) => {
		$(#[$attr])*
		#[repr(C)]
		#[derive(Copy, Clone, Eq, PartialEq)]
		pub struct $name(HANDLE);

		unsafe impl Send for $name {}
		unsafe impl Sync for $name {}

		impl Default for $name {
			/// Creates a null handle.
			fn default() -> Self {
				Self(std::ptr::null_mut())
			}
		}

		impl $name {
			/// Wraps a pointer.
			pub unsafe fn from_ptr<T>(p: *mut T) -> $name {
				Self(p as HANDLE)
			}

			/// Returns the raw underlying pointer for this handle.
			pub unsafe fn as_ptr(self) -> HANDLE {
				self.0
			}
		}
	};
}

// Transforms a pointer into an option, which is None if the pointer is null.
// https://stackoverflow.com/q/65144143/6923555
macro_rules! ptr_to_opt {
	($ptr:expr) => {
		unsafe {
			if $ptr.is_null() {
				None
			} else {
				Some($ptr)
			}
		}
	};
}

/// Converts a reference to a const void pointer.
pub fn const_void<T>(val: &T) -> *const std::ffi::c_void {
	val as *const T as *const std::ffi::c_void
}
/// Converts a mut reference to a mut void pointer.
pub fn mut_void<T>(val: &mut T) -> *mut std::ffi::c_void {
	val as *mut T as *mut std::ffi::c_void
}