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

// Declares as_ptr method.
macro_rules! as_ptr_method {
	() => {
		/// Returns the raw underlying pointer for this handle.
		pub unsafe fn as_ptr(&self) -> *const Void {
			self.0
		}
	};
}