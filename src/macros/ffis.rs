#![allow(unused_macros)]

/// Builds one single FFI binding function.
macro_rules! one_func {
	($dll:literal $func:ident( $( $parm:ty ),* ) -> $ret:ty) => {
		::windows_targets::link! {
			$dll
			"system"
			fn $func( $( x: $parm ),* ) -> $ret
		}
	};

	($dll:literal $func:ident( $( $parm:ty ),* )) => {
		one_func!($dll $func( $( $parm ),* ) -> () );
	};
}

/// Builds a block of FFI bindings.
macro_rules! extern_sys {
	(
		$dll:expr;
		$(
			$func:ident( $( $parm:ty ),* ) $( -> $ret:ty )?
		)*
	) => {
		$(
			one_func!( $dll $func( $( $parm ),* ) $(-> $ret)? );
		)*
	};
}
