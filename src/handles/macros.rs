// Declares a handle type.
macro_rules! ty_handle {
	($ty: ident, $comm: expr) => {
		#[doc=$comm]
		#[repr(C)]
		#[derive(Copy, Clone, Eq, PartialEq)]
		pub struct $ty(pub(crate) *mut Void);
	};
}

// Transforms a pointer into an option.
macro_rules! ptr_to_opt {
	($p: expr) => {
		if $p.is_null() {
			None
		} else {
			Some($p)
		}
	};
}