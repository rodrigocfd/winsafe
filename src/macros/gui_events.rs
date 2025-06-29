#![allow(unused_macros)]

/// Ordinary window message, no parameters, no meaningful return.
macro_rules! pub_fn_wm_noparm_noret {
	(
		$name:ident, $wmconst:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn() -> crate::AnyResult<()> + 'static,
		{
			let def_proc_val = self.wnd_ty.def_proc_val();
			self.wm($wmconst, move |_| {
				func()?;
				Ok(def_proc_val)
			});
			self
		}
	};
}

/// Ordinary window message, no parameters, returns bool.
macro_rules! pub_fn_wm_noparm_boolret {
	(
		$name:ident, $wmconst:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn() -> crate::AnyResult<bool> + 'static,
		{
			self.wm($wmconst, move |_| {
				Ok(func()? as _)
			});
			self
		}
	};
}

/// Ordinary window message, with parameters, no meaningful return.
macro_rules! pub_fn_wm_withparm_noret {
	(
		$name:ident, $wmconst:expr, $parm:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn($parm) -> crate::AnyResult<()> + 'static,
		{
			let def_proc_val = self.wnd_ty.def_proc_val();
			self.wm($wmconst, move |p| {
				func(unsafe { <$parm>::from_generic_wm(p) })?;
				Ok(def_proc_val)
			});
			self
		}
	};
}

/// Ordinary window message, with parameters, returns bool.
macro_rules! pub_fn_wm_withparm_boolret {
	(
		$name:ident, $wmconst:expr, $parm:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn($parm) -> crate::AnyResult<bool> + 'static,
		{
			self.wm($wmconst, move |p| {
				Ok(func(unsafe { <$parm>::from_generic_wm(p) })? as _)
			});
			self
		}
	};
}

/// Ordinary window message, with parameters, returns constant.
macro_rules! pub_fn_wm_withparm_coret {
	(
		$name:ident, $wmconst:expr, $parm:ty, $coret:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn($parm) -> crate::AnyResult<$coret> + 'static,
		{
			self.wm($wmconst, move |p| {
				Ok(func(unsafe { <$parm>::from_generic_wm(p) })?.raw() as _)
			});
			self
		}
	};
}

/// WM_CTLCOLOR* message.
macro_rules! pub_fn_wm_ctlcolor {
	(
		$name:ident, $wmconst:expr, $parm:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn($parm) -> crate::AnyResult<crate::HBRUSH> + 'static,
		{
			self.wm($wmconst, move |p| {
				Ok(func(unsafe { <$parm>::from_generic_wm(p) })?.ptr() as _)
			});
			self
		}
	};
}

/// WM_COMMAND message, no parameters, no meaningful return.
macro_rules! pub_fn_cmd_noparm_noret {
	(
		$name:ident, $cmd:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn() -> crate::AnyResult<()> + 'static,
		{
			self.0.wm_command($cmd, move || {
				func()
			});
			self
		}
	};
}

/// WM_NOTIFY message, no parameters, no meaningful return.
macro_rules! pub_fn_nfy_noparm_noret {
	(
		$name:ident, $nfy:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn() -> crate::AnyResult<()> + 'static,
		{
			let def_proc_val = self.0.wnd_ty().def_proc_val();
			self.0.wm_notify($nfy, move |_| {
				func()?;
				Ok(def_proc_val)
			});
			self
		}
	};
}

/// WM_NOTIFY message, with parameters, no meaningful return.
macro_rules! pub_fn_nfy_withparm_noret {
	(
		$name:ident, $nfy:expr, $param:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn(&$param) -> crate::AnyResult<()> + 'static,
		{
			let def_proc_val = self.0.wnd_ty().def_proc_val();
			self.0.wm_notify($nfy, move |p| {
				func(unsafe { p.cast_nmhdr::<$param>() })?;
				Ok(def_proc_val)
			});
			self
		}
	};
}

/// WM_NOTIFY message, with mutable parameters, no meaningful return.
macro_rules! pub_fn_nfy_withmutparm_noret {
	(
		$name:ident, $nfy:expr, $param:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn(&mut $param) -> crate::AnyResult<()> + 'static,
		{
			let def_proc_val = self.0.wnd_ty().def_proc_val();
			self.0.wm_notify($nfy, move |p| {
				func(unsafe { p.cast_nmhdr_mut::<$param>() })?;
				Ok(def_proc_val)
			});
			self
		}
	};
}

/// WM_NOTIFY message, no parameters, returns bool.
macro_rules! pub_fn_nfy_noparm_boolret {
	(
		$name:ident, $nfy:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn() -> crate::AnyResult<bool> + 'static,
		{
			self.0.wm_notify($nfy, move |_| {
				Ok(func()? as _)
			});
			self
		}
	};
}

/// WM_NOTIFY message, with parameters, returns bool.
macro_rules! pub_fn_nfy_withparm_boolret {
	(
		$name:ident, $nfy:expr, $param:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn(&$param) -> crate::AnyResult<bool> + 'static,
		{
			self.0.wm_notify($nfy, move |p| {
				Ok(func(unsafe { p.cast_nmhdr::<$param>() })? as _)
			});
			self
		}
	};
}

/// WM_NOTIFY message, no parameters, returns i32.
macro_rules! pub_fn_nfy_noparm_i32ret {
	(
		$name:ident, $nfy:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn() -> crate::AnyResult<i32> + 'static,
		{
			self.0.wm_notify($nfy, move |_| {
				Ok(func()? as _)
			});
			self
		}
	};
}

/// WM_NOTIFY message, with parameters, returns i32.
macro_rules! pub_fn_nfy_withparm_i32ret {
	(
		$name:ident, $nfy:expr, $param:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F) -> &Self
			where F: Fn(&$param) -> crate::AnyResult<i32> + 'static,
		{
			self.0.wm_notify($nfy, move |p| {
				Ok(func(unsafe { p.cast_nmhdr::<$param>() })? as _)
			});
			self
		}
	};
}
