#![allow(unused_macros)]

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
				pub(crate) fn $func( $( _x: $parm, )* ) $( -> $ret )?;
			)*
		}
	};
}
pub(crate) use extern_sys;
