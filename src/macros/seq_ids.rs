/// Generates sequential `u16` constants starting from the given value.
///
/// This macro is useful to generate constants for loaded resources, like menus
/// or dialog windows.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::seq_ids;
///
/// seq_ids! {
///     MNU_FILE = 3000;
///     MNU_FILE_OPEN
///     MNU_FILE_SAVE
///     MNU_FILE_CLOSE
/// }
/// ```
///
/// The code above will generate the following:
///
/// ```rust,no_run
/// pub const MNU_FILE: u16 = 3000;
/// pub const MNU_FILE_OPEN: u16 = 3001;
/// pub const MNU_FILE_SAVE: u16 = 3002;
/// pub const MNU_FILE_CLOSE: u16 = 3003;
/// ```
#[macro_export]
macro_rules! seq_ids {
	(
		$name:ident = $val:expr;
		$( $others:ident )*
	) => {
		pub const $name: u16 = $val;
		seq_ids!($val + 1, $($others,)*);
	};

	($val:expr,) => {};

	($val:expr, $name:ident,) => {
		pub const $name: u16 = $val;
	};

	($val:expr, $name:ident, $( $others:ident, )+) => {
		pub const $name: u16 = $val;
		seq_ids!($val + 1, $($others,)*);
	};
}
