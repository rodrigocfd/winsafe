/// Declares a method for an ordinary message notification, which has no
/// parameters and returns zero (or a non-meaningful value).
macro_rules! pub_fn_wm_ret0 {
	(
		$name:ident, $wmconst:expr,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: Fn() -> crate::aliases::ErrResult<()> + 'static,
		{
			self.add_msg($wmconst, move |_| { func()?; Ok(None) }); // return value is never meaningful
		}
	};
}

/// Declares a method for an ordinary message notification, which carries an
/// object with its parameters, and returns zero (or a non-meaningful value).
macro_rules! pub_fn_wm_ret0_param {
	(
		$name:ident, $wmconst:expr, $parm:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: Fn($parm) -> crate::aliases::ErrResult<()> + 'static,
		{
			self.add_msg($wmconst,
				move |p| { func(<$parm>::from_generic_wm(p))?; Ok(None) }); // return value is never meaningful
		}
	};
}

/// Declares a method for an ordinary message notification, which carries an
/// object with its parameters, and returns `bool`.
macro_rules! pub_fn_wm_retbool_param {
	(
		$name:ident, $wmconst:expr, $parm:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
			where F: Fn($parm) -> crate::aliases::ErrResult<bool> + 'static,
		{
			self.add_msg($wmconst,
				move |p| Ok(Some(func(<$parm>::from_generic_wm(p))? as _)));
		}
	};
}

/// Declares a method for an ordinary message notification, which carroes an
/// object with its parameters, and returns a constant.
macro_rules! pub_fn_wm_retco_param {
	(
		$name:ident, $wmconst:expr, $parm:ty, $retco:ty,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		pub fn $name<F>(&self, func: F)
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
		pub fn $name<F>(&self, func: F)
			where F: Fn($parm) -> crate::aliases::ErrResult<crate::handles::HBRUSH> + 'static,
		{
			self.add_msg($wmconst,
				move |p| Ok(Some(func(<$parm>::from_generic_wm(p))?.ptr as _)));
		}
	};
}

/// Declares a struct of control events, which is just a proxy to parent events.
macro_rules! pub_struct_ctrl_events_proxy {
	(
		$(#[$doc:meta])*
		$name:ident
	) => {
		$(#[$doc])*
		pub struct $name {
			parent_ptr: std::ptr::NonNull<crate::gui::base::Base>,
			ctrl_id: u16,
		}

		impl $name {
			pub(in crate::gui) fn new(
				parent_base_ref: &crate::gui::base::Base, ctrl_id: u16) -> $name
			{
				Self {
					parent_ptr: std::ptr::NonNull::from(parent_base_ref),
					ctrl_id,
				}
			}

			fn parent_user_events(&self) -> &crate::gui::events::WindowEvents {
				unsafe { self.parent_ptr.as_ref().user_events_ref() }
			}
		}
	}
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
			self.parent_user_events().wm_command($cmd, self.ctrl_id as _,
				move || func());
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
			self.parent_user_events().add_nfy(self.ctrl_id as _, $nfy,
				move |_| { func()?; Ok(None) });
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
			self.parent_user_events().add_nfy(self.ctrl_id as _, $nfy,
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
			self.parent_user_events().add_nfy(self.ctrl_id as _, $nfy,
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
			self.parent_user_events().add_nfy(self.ctrl_id as _, $nfy,
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
			self.parent_user_events().add_nfy(self.ctrl_id as _, $nfy,
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
			self.parent_user_events().add_nfy(self.ctrl_id as _, $nfy,
				move |_| Ok(Some(func()? as _)));
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
			self.parent_user_events().add_nfy(self.ctrl_id as _, $nfy,
				move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<$param>() })? as _)));
		}
	};
}
