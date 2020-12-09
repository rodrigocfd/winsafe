// Declares the type of a handle.
macro_rules! handle_type {
	(
		$(#[$attr:meta])*
		$name:ident
	) => {
		$(#[$attr])*
		#[repr(C)]
		#[derive(Copy, Clone, Eq, PartialEq)]
		pub struct $name(*const c_void);

		unsafe impl Send for $name {}
		unsafe impl Sync for $name {}

		impl Default for $name {
			/// Creates a null handle.
			fn default() -> Self {
				Self(std::ptr::null())
			}
		}

		impl $name {
			/// Wraps a const pointer.
			pub unsafe fn from_ptr<T>(p: *const T) -> Self {
				Self(p as *const c_void)
			}

			/// Wraps a mut pointer.
			pub unsafe fn from_mut_ptr<T>(p: *mut T) -> Self {
				Self(p as *mut T as *const c_void)
			}

			/// Returns the raw underlying pointer for this handle.
			pub unsafe fn as_ptr(&self) -> *const c_void {
				self.0
			}
		}
	};
}

// Transforms a pointer into an option, which is None if the pointer is null.
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