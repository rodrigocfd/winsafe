/// Declares a method for `EventsWm` trait, which has no parameters and returns
/// zero (or a non-meaningful value).
macro_rules! pub_fn_wm_ret0 {
	(
		$name:ident, $wmconst:expr,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		fn $name<F>(&self, func: F)
			where F: Fn() -> crate::aliases::ErrResult<()> + 'static,
		{
			self.add_msg($wmconst, move |_| {
				func()?;
				Ok(None) // return value is never meaningful
			});
		}
	};
}

/// Declares a method for `EventsWm` trait, which carries an object with its
/// parameters, and returns zero (or a non-meaningful value).
macro_rules! pub_fn_wm_ret0_param {
	(
		$name:ident, $wmconst:expr, $parm:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		fn $name<F>(&self, func: F)
			where F: Fn($parm) -> crate::aliases::ErrResult<()> + 'static,
		{
			self.add_msg($wmconst, move |p| {
				func(<$parm>::from_generic_wm(p))?;
				Ok(None) // return value is never meaningful
			});
		}
	};
}

/// Declares a method for `EventsWm` trait, which carries an object with its
/// parameters, and returns `bool`.
macro_rules! pub_fn_wm_retbool_param {
	(
		$name:ident, $wmconst:expr, $parm:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		fn $name<F>(&self, func: F)
			where F: Fn($parm) -> crate::aliases::ErrResult<bool> + 'static,
		{
			self.add_msg($wmconst,
				move |p| Ok(Some(func(<$parm>::from_generic_wm(p))? as _)));
		}
	};
}

/// Declares a method for `EventsWm` trait, which carroes an object with its
/// parameters, and returns a constant.
macro_rules! pub_fn_wm_retco_param {
	(
		$name:ident, $wmconst:expr, $parm:ty, $retco:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		fn $name<F>(&self, func: F)
			where F: Fn($parm) -> crate::aliases::ErrResult<$retco> + 'static,
		{
			self.add_msg($wmconst,
				move |p| Ok(Some(func(<$parm>::from_generic_wm(p))?.0 as _)));
		}
	};
}

/// Declares a method for a WM_CTLCOLOR* message.
macro_rules! pub_fn_wm_ctlcolor {
	(
		$name:ident, $wmconst:expr, $parm:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		fn $name<F>(&self, func: F)
			where F: Fn($parm) -> crate::aliases::ErrResult<HBRUSH> + 'static,
		{
			self.add_msg($wmconst,
				move |p| Ok(Some(func(<$parm>::from_generic_wm(p))?.ptr as _)));
		}
	};
}

/// Declares a method for a `WM_COMMAND` notification.
macro_rules! pub_fn_cmd_ret0 {
	(
		$name:ident, $cmd:expr,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: Fn() -> crate::aliases::ErrResult<()> + 'static,
		{
			self.0.wm_command($cmd, move || func());
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives an `NMHDR`
/// parameter, which is not passed because it carries no useful data, and whose
/// callback has no return.
macro_rules! pub_fn_nfy_ret0 {
	(
		$name:ident, $nfy:expr,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: Fn() -> crate::aliases::ErrResult<()> + 'static,
		{
			self.0.add_nfy($nfy, move |_| { func()?; Ok(None) });
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives a parameter,
/// and whose callback has no return.
macro_rules! pub_fn_nfy_ret0_param {
	(
		$name:ident, $nfy:expr, $param:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: Fn(&$param) -> crate::aliases::ErrResult<()> + 'static,
		{
			self.0.add_nfy($nfy,
				move |p| { func(unsafe { p.cast_nmhdr::<$param>() })?; Ok(None) });
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives a mutable
/// parameter, and whose callback has no return.
macro_rules! pub_fn_nfy_ret0_mutparam {
	(
		$name:ident, $nfy:expr, $param:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: Fn(&mut $param) -> crate::aliases::ErrResult<()> + 'static,
		{
			self.0.add_nfy($nfy,
				move |p| { func(unsafe { p.cast_nmhdr_mut::<$param>() })?; Ok(None) });
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives a parameter,
/// and whose callback returns `bool`.
macro_rules! pub_fn_nfy_retbool_param {
	(
		$name:ident, $nfy:expr, $param:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: Fn(&$param) -> crate::aliases::ErrResult<bool> + 'static,
		{
			self.0.add_nfy($nfy,
				move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<$param>() })? as _)));
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives a mutable
/// parameter, and whose callback returns `bool`.
macro_rules! pub_fn_nfy_retbool_mutparam {
	(
		$name:ident, $nfy:expr, $param:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: Fn(&mut $param) -> crate::aliases::ErrResult<bool> + 'static,
		{
			self.0.add_nfy($nfy,
				move |p| Ok(Some(func(unsafe { p.cast_nmhdr_mut::<$param>() })? as _)));
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives an `NMHDR`
/// parameter, which is not passed because it carries no useful data, and whose
/// callback returns `i32`.
macro_rules! pub_fn_nfy_reti32 {
	(
		$name:ident, $nfy:expr,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: Fn() -> crate::aliases::ErrResult<i32> + 'static,
		{
			self.0.add_nfy($nfy, move |_| Ok(Some(func()? as _)));
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification which receives a parameter,
/// and whose callback returns `i32`.
macro_rules! pub_fn_nfy_reti32_param {
	(
		$name:ident, $nfy:expr, $param:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: Fn(&$param) -> crate::aliases::ErrResult<i32> + 'static,
		{
			self.0.add_nfy($nfy,
				move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<$param>() })? as _)));
		}
	};
}
