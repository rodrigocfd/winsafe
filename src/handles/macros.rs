// Declares the type of a handle.
// We use doc as string because of a bug in rust-analyzer:
// https://stackoverflow.com/q/65112749/6923555
macro_rules! handle_type {
	($name:ident, $doc:expr) => {
		#[doc=$doc]
		#[repr(C)]
		#[derive(Copy, Clone, Eq, PartialEq)]
		pub struct $name(*const Void);

		impl $name {
			/// Returns the raw underlying pointer for this handle.
			pub unsafe fn as_ptr(&self) -> *const Void {
				self.0
			}
		}
	};
}

// Transforms a pointer into an option, which is None if the pointer is null.
macro_rules! ptr_to_opt {
	($ptr:expr) => {
		if $ptr.is_null() {
			None
		} else {
			Some($ptr)
		}
	};
}