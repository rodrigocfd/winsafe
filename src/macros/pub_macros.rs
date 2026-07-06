#![allow(unused_macros)]

/// Generates sequential `u16` constants starting from the given value.
///
/// This macro is useful to generate constants for loaded resources, like menus
/// or dialog windows.
///
/// Each constant also supports individual documentation.
///
/// # Examples
///
/// ```no_run
/// use winsafe::seq_ids;
///
/// seq_ids! {
///     MNU_FILE = 3000;
///     MNU_FILE_OPEN
///     MNU_FILE_SAVE
///     /// This menu closes the application.
///     MNU_FILE_CLOSE
/// }
/// ```
///
/// The code above will generate the following:
///
/// ```no_run
/// pub const MNU_FILE: u16 = 3000;
/// pub const MNU_FILE_OPEN: u16 = 3001;
/// pub const MNU_FILE_SAVE: u16 = 3002;
/// /// This menu closes the application.
/// pub const MNU_FILE_CLOSE: u16 = 3003;
/// ```
#[macro_export] // https://internals.rust-lang.org/t/pub-on-macro-rules/19358
macro_rules! seq_ids {
	() => {};

	($val:expr,) => {};

	(
		$( #[$comment:meta] )*
		$name:ident = $val:expr;
		$( $others:tt )*
	) => {
		$( #[$comment] )*
		pub const $name: u16 = $val;
		seq_ids!($val + 1, $( $others )*);
	};

	(
		$next_val:expr,
		$( #[$comment:meta] )*
		$name:ident = $val:expr;
		$( $others:tt )*
	) => {
		$( #[$comment] )*
		pub const $name: u16 = $val;
		seq_ids!($val + 1, $( $others )*);
	};

	(
		$next_val:expr,
		$( #[$comment:meta] )*
		$name:ident
		$( $others:tt )*
	) => {
		$( #[$comment] )*
		pub const $name: u16 = $next_val;
		seq_ids!($next_val + 1, $( $others )*);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! evt {
	( $self:ident, $wnd:expr, $ev:ident, $fun:ident ) => {
		let self2 = $self.clone();
		$wnd.on().$ev(move || self2.$fun()); // event without parameter
	};
	( $self:ident, $ev:ident, $fun:ident ) => {
		evt!($self, $self.wnd, $ev, $fun);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! evp {
	( $self:ident, $wnd:expr, $ev:ident, $fun:ident ) => {
		let self2 = $self.clone();
		$wnd.on().$ev(move |p| self2.$fun(p)); // event with parameter
	};
	( $self:ident, $ev:ident, $fun:ident ) => {
		evp!($self, $self.wnd, $ev, $fun);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! cmd {
	( $self:ident, $wnd:expr, $cmd:expr, $fun:ident ) => {
		let self2 = $self.clone();
		$wnd.on().wm_command_acc_menu($cmd, move || self2.$fun());
	};
	( $self:ident, $cmd:expr, $fun:ident ) => {
		cmd!($self, $self.wnd, $cmd, $fun);
	};
}
