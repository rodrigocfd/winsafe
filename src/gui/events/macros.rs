/// Declares a method for a `WM_COMMAND` notification.
macro_rules! cmd_event {
	(
		$name:ident, $cmd:expr,
		$(#[$attr:meta])*
	) => {
		$(#[$attr])*
		pub fn $name<F>(&self, func: F)
			where F: FnMut() + 'static,
		{
			self.parent_events().wm_command($cmd, self.ctrl_id, {
				let mut func = func;
				move || func()
			});
		}
	};
}

/// Declares a method for a `WM_NOTIFY` notification.
macro_rules! nfy_event {
	(
		$name:ident, $nfy:expr, $struc:ty,
		$(#[$attr:meta])*
	) => {
		$(#[$attr])*
		pub fn $name<F>(&self, func: F)
			where F: FnMut(&$struc) + 'static,
		{
			self.parent_events().add_nfy(self.ctrl_id, $nfy, {
				let mut func = func;
				move |p| { func(unsafe { p.cast_nmhdr::<$struc>() }); None }
			});
		}
	};
}
