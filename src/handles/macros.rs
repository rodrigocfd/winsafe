/// Declares the type of a handle.
macro_rules! handle_type {
	(
		$(#[$attr:meta])*
		$name:ident
	) => {
		$(#[$attr])*
		#[repr(C)]
		#[derive(Copy, Clone, Eq, PartialEq)]
		pub struct $name(*mut std::ffi::c_void);

		unsafe impl Send for $name {}
		unsafe impl Sync for $name {}

		impl Default for $name {
			fn default() -> Self {
				Self(std::ptr::null_mut())
			}
		}

		impl $name {
			/// Creates a new handle instance by wrapping a pointer.
			pub unsafe fn from_ptr<T>(p: *mut T) -> $name {
				Self(p as *mut std::ffi::c_void)
			}

			/// Returns the raw underlying pointer for this handle.
			pub unsafe fn as_ptr(self) -> *mut std::ffi::c_void {
				self.0
			}

			/// Tells if the handle is invalid (null).
			pub fn is_null(&self) -> bool {
				self.0.is_null()
			}
		}
	};
}

/// Declares the type of an HGDIOBJ handle.
macro_rules! hgdiobj_type {
	(
		$(#[$attr:meta])*
		$name:ident
	) => {
		handle_type! {
			$(#[$attr])*
			$name
		}

		impl $name {
			/// [`DeleteObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
			/// method.
			pub fn DeleteObject(self) -> Result<(), ()> {
				match unsafe { gdi32::DeleteObject(self.0) } {
					 0 => Err(()),
					_ => Ok(()),
				}
			}
		}
	};
}