/// Declares the type of a handle.
macro_rules! handle_type {
	(
		$(#[$doc:meta])*
		$name:ident
	) => {
		$(#[$doc])*
		#[repr(C)]
		#[derive(Debug, Copy, Clone, Eq, PartialEq)]
		pub struct $name {
			pub(crate) ptr: *mut std::ffi::c_void,
		}

		unsafe impl Send for $name {}
		unsafe impl Sync for $name {}

		// Formatters.
		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{:#010x}", self.ptr as usize)
			}
		}
		impl std::fmt::LowerHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::LowerHex::fmt(&(self.ptr as usize), f)
			}
		}
		impl std::fmt::UpperHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::UpperHex::fmt(&(self.ptr as usize), f)
			}
		}
		impl std::fmt::Binary for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Binary::fmt(&(self.ptr as usize), f)
			}
		}
		impl std::fmt::Octal for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Octal::fmt(&(self.ptr as usize), f)
			}
		}

		impl $name {
			/// The null, invalid handle.
			///
			/// This constant is common to all handle types.
			pub const NULL: Self = Self { ptr: std::ptr::null_mut() };

			/// Creates a new handle instance by wrapping a pointer.
			///
			/// This method is common to all handle types.
			pub unsafe fn from_ptr<T>(p: *mut T) -> $name {
				Self { ptr: p as _ }
			}

			/// Consumes the handle returning the underlying raw pointer.
			///
			/// This method is common to all handle types.
			pub unsafe fn as_ptr(self) -> *mut std::ffi::c_void {
				self.ptr
			}

			/// Tells if the handle is invalid (null).
			///
			/// This method is common to all handle types.
			pub fn is_null(self) -> bool {
				self.ptr.is_null()
			}

			/// Consumes the handle into an option, which is `None` if the handle
			/// pointer is null.
			///
			/// This method is common to all handle types.
			pub fn as_opt(self) -> Option<$name> {
				if self.ptr.is_null() {
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
		$(#[$doc:meta])*
		$name:ident
	) => {
		handle_type! {
			$(#[$doc])*
			$name
		}

		impl $name {
			/// [`DeleteObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deleteobject)
			/// method.
			///
			/// This method is common to all GDI handle types.
			pub fn DeleteObject(self) -> crate::aliases::WinResult<()> {
				crate::privs::bool_to_winresult(
					unsafe { crate::ffi::gdi32::DeleteObject(self.ptr) },
				)
			}
		}
	};
}
