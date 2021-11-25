/// Implements `Handle` trait to handle object, plus required bounds.
macro_rules! impl_handle {
	($name:ident) => {
		unsafe impl Send for $name {}

		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "{} {:#010x}", stringify!($name), self.0 as usize)
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

		impl crate::handles::traits::Handle for $name {
			const NULL: Self = Self(std::ptr::null_mut());

			unsafe fn from_ptr<T>(p: *mut T) -> Self {
				Self(p as _)
			}

			unsafe fn as_ptr(self) -> *mut std::ffi::c_void {
				self.0
			}
		}
	};
}
