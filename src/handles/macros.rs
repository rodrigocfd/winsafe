/// Declares the type of a handle.
macro_rules! handle_type {
	(
		$(#[$attr:meta])*
		$name:ident
	) => {
		$(#[$attr])*
		#[repr(C)]
		#[derive(Debug, Copy, Clone, Eq, PartialEq)]
		pub struct $name(*mut std::ffi::c_void);

		unsafe impl Send for $name {}
		unsafe impl Sync for $name {}

		// Formatters.
		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{:#010x}", self.0 as usize)
			}
		}
		impl std::fmt::LowerHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::LowerHex::fmt(&(self.0 as usize), f)
			}
		}
		impl std::fmt::UpperHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::UpperHex::fmt(&(self.0 as usize), f)
			}
		}
		impl std::fmt::Binary for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Binary::fmt(&(self.0 as usize), f)
			}
		}
		impl std::fmt::Octal for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Octal::fmt(&(self.0 as usize), f)
			}
		}

		impl $name {
			/// Creates a new handle instance by wrapping a pointer.
			pub unsafe fn from_ptr<T>(p: *mut T) -> $name {
				Self(p as *mut std::ffi::c_void)
			}

			/// Creates a null, invalid handle.
			pub unsafe fn null_handle() -> Self {
				Self(std::ptr::null_mut())
			}

			/// Consumes the handle returning the underlying raw pointer.
			pub unsafe fn as_ptr(self) -> *mut std::ffi::c_void {
				self.0
			}

			/// Tells if the handle is invalid (null).
			pub fn is_null(&self) -> bool {
				self.0.is_null()
			}

			/// Consumes the handle into an option, which is `None` if the handle
			/// pointer is null.
			pub fn as_opt(self) -> Option<$name> {
				if self.0.is_null() {
					None
				} else {
					Some(self)
				}
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
