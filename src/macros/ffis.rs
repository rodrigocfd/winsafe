#![allow(unused_macros)]

/// Builds one single FFI binding function.
macro_rules! one_func {
	($func:ident( $( $parm:ty ),* ) -> $ret:ty) => {
		pub(crate) fn $func( $( _: $parm, )* ) -> $ret;
	};

	($func:ident( $( $parm:ty ),* )) => {
		pub(crate) fn $func( $( _: $parm, )* );
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
		#[cfg_attr(not(feature = "raw-dylib"), link(name = $dll))]
		#[cfg_attr(feature = "raw-dylib", link(name = $dll, kind = "raw-dylib"))]
		unsafe extern "system" {
			$(
				one_func!( $func( $( $parm ),* ) $(-> $ret)? );
			)*
		}
	};
}
