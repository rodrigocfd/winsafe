/// Struct for a message that has no parameters.
macro_rules! empty_msg {
	(
		$(#[$attr:meta])*
		$name:ident, $wmconst:expr
	) => {
		$(#[$attr])*
		pub struct $name {}

		impl From<$name> for WmAny {
			fn from(_: $name) -> Self {
				Self {
					msg: $wmconst,
					wparam: 0,
					lparam: 0,
				}
			}
		}

		impl From<WmAny> for $name {
			fn from(_: WmAny) -> Self {
				Self {}
			}
		}
	};
}