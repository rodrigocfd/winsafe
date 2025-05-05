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
#[cfg(feature = "user")]
#[macro_export]
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

/// Binds a function to an event which has no parameters.
///
/// This is just syntactic sugar to use a member function as an event closure.
///
/// # Example
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, gui, bind};
///
/// #[derive(Clone)]
/// struct MyWindow {
///     wnd: gui::WindowMain,
/// }
///
/// impl MyWindow {
///     fn new() -> Self {
///         let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
///         let new_self = Self { wnd };
///         new_self.events();
///         new_self
///     }
///
///     fn events(&self) {
///         self.wnd.on().wm_destroy(bind!(self, Self::on_destroy));
///     }
///
///     fn on_destroy(&self) -> w::AnyResult<()> {
///         Ok(())
///     }
/// }
/// ```
///
/// # Related macros
///
/// * [`bind_ig`](crate::bind_ig)
/// * [`bind_p`](crate::bind_p)
#[cfg(feature = "gui")]
#[macro_export]
macro_rules! bind {
	($arg:ident, $fun:expr) => {{
		let arg2 = $arg.clone();
		move || $fun(&arg2)
	}};
}

/// Binds, to an event closure, a function receiving the event parameter.
///
/// This is just syntactic sugar to use a member function as an event closure.
///
/// # Example
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, gui, msg, bind_p};
///
/// #[derive(Clone)]
/// struct MyWindow {
///     wnd: gui::WindowMain,
/// }
///
/// impl MyWindow {
///     fn new() -> Self {
///         let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
///         let new_self = Self { wnd };
///         new_self.events();
///         new_self
///     }
///
///     fn events(&self) {
///         self.wnd.on().wm_create(bind_p!(self, Self::on_create));
///     }
///
///     fn on_create(&self, p: msg::wm::Create) -> w::AnyResult<i32> {
///         Ok(0)
///     }
/// }
/// ```
///
/// # Related macros
///
/// * [`bind`](crate::bind)
/// * [`bind_ig`](crate::bind_ig)
#[cfg(feature = "gui")]
#[macro_export]
macro_rules! bind_p {
	($arg:ident, $fun:expr) => {{
		let arg2 = $arg.clone();
		move |p| $fun(&arg2, p)
	}};
}

/// Binds, to an event closure, a function ignoring the event parameter.
///
/// This is just syntactic sugar to use a member function as an event closure.
///
/// # Example
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, gui, bind_ig};
///
/// #[derive(Clone)]
/// struct MyWindow {
///     wnd: gui::WindowMain,
/// }
///
/// impl MyWindow {
///     fn new() -> Self {
///         let wnd = gui::WindowMain::new(gui::WindowMainOpts::default());
///         let new_self = Self { wnd };
///         new_self.events();
///         new_self
///     }
///
///     fn events(&self) {
///         self.wnd.on().wm_create(bind_ig!(self, Self::on_create));
///     }
///
///     fn on_create(&self) -> w::AnyResult<i32> {
///         Ok(0)
///     }
/// }
/// ```
///
/// # Related macros
///
/// * [`bind`](crate::bind)
/// * [`bind_p`](crate::bind_p)
#[cfg(feature = "gui")]
#[macro_export]
macro_rules! bind_ig {
	($arg:ident, $fun:expr) => {{
		let arg2 = $arg.clone();
		move |_| $fun(&arg2)
	}};
}
