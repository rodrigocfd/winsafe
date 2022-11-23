#![allow(unused_macros)]

/// Declares a handle.
macro_rules! impl_handle {
	(
		$name:ident : $feature:literal;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
		#[repr(transparent)]
		#[derive(PartialEq, Eq, Hash)]
		pub struct $name(pub(crate) *mut std::ffi::c_void);

		unsafe impl Send for $name {}

		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "[{:#010x} {}] {}",
					self.0 as usize, self.0 as usize, stringify!($name))
			}
		}
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

		impl crate::prelude::Handle for $name {
			const NULL: Self = Self(std::ptr::null_mut());
			const INVALID: Self = Self(-1 as _);

			unsafe fn from_ptr(p: *mut std::ffi::c_void) -> Self {
				Self(p)
			}

			unsafe fn as_ptr(&self) -> *mut std::ffi::c_void {
				self.0
			}

			unsafe fn raw_copy(&self) -> Self {
				Self(self.0)
			}
		}
	};
}
