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