// Builds a block of FFI bindings.
macro_rules! extern_sys {
	(
		$dll:expr,
		$(
			$func:ident, $( $parm:ty, )* => $ret:ty
		)*
	) => {
		#[link(name = $dll)]
		extern "system" {
			$(
				pub(crate) fn $func( $( _: $parm, )* ) -> $ret;
			)*
		}
	};
}
