use crate::co;
use crate::decl::*;
use crate::gui::privs::*;
use crate::prelude::*;

/// This trait is enabled with the `gui` feature, and exposes property sheet
/// page control
/// [notifications](https://learn.microsoft.com/en-us/windows/win32/controls/bumper-property-sheets-reference-notifications).
///
/// These event methods are just proxies to the
/// [`GuiEventsParent`](crate::prelude::GuiEventsParent) of the parent window,
/// who is the real responsible for the child event handling.
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait GuiEventsPropSheetPage: GuiEventsParent {
	/// [`PSN_APPLY`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-apply)
	/// notification.
	fn psn_apply<F>(&self, func: F)
	where
		F: Fn() -> AnyResult<co::PSNRET> + 'static,
	{
		self.wm_notify(0u16, co::PSN::APPLY, move |_| Ok(func()?.raw() as _));
	}

	/// [`PSN_GETOBJECT`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-getobject)
	/// notification.
	fn psn_get_object<F>(&self, func: F)
	where
		F: Fn(&NMOBJECTNOTIFY) -> AnyResult<()> + 'static,
	{
		let def_proc_val = self.wnd_ty().def_proc_val();
		self.wm_notify(0u16, co::PSN::GETOBJECT, move |p| {
			func(unsafe { p.cast_nmhdr_mut::<NMOBJECTNOTIFY>() })?;
			Ok(def_proc_val)
		});
	}

	/// [`PSN_HELP`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-help)
	/// notification.
	fn psn_help<F>(&self, func: F)
	where
		F: Fn(&PSHNOTIFY) -> AnyResult<()> + 'static,
	{
		let def_proc_val = self.wnd_ty().def_proc_val();
		self.wm_notify(0u16, co::PSN::HELP, move |p| {
			func(unsafe { p.cast_nmhdr_mut::<PSHNOTIFY>() })?;
			Ok(def_proc_val)
		});
	}

	/// [`PSN_KILLACTIVE`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-killactive)
	/// notification.
	fn psn_kill_active<F>(&self, func: F)
	where
		F: Fn(&PSHNOTIFY) -> AnyResult<bool> + 'static,
	{
		self.wm_notify(0u16, co::PSN::KILLACTIVE, move |p| {
			Ok(func(unsafe { p.cast_nmhdr::<PSHNOTIFY>() })? as _)
		});
	}

	/// [`PSN_QUERYCANCEL`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-querycancel)
	/// notification.
	fn psn_query_cancel<F>(&self, func: F)
	where
		F: Fn(&PSHNOTIFY) -> AnyResult<bool> + 'static,
	{
		self.wm_notify(0u16, co::PSN::QUERYCANCEL, move |p| {
			Ok(func(unsafe { p.cast_nmhdr::<PSHNOTIFY>() })? as _)
		});
	}

	/// [`PSN_QUERYINITIALFOCUS`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-queryinitialfocus)
	/// notification.
	fn psn_query_initial_focus<F>(&self, func: F)
	where
		F: Fn(&PSHNOTIFY) -> AnyResult<Option<HWND>> + 'static,
	{
		self.wm_notify(0u16, co::PSN::QUERYINITIALFOCUS, move |p| {
			Ok(func(unsafe { p.cast_nmhdr::<PSHNOTIFY>() })?.map_or(0, |h| h.ptr() as _))
		});
	}

	/// [`PSN_RESET`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-reset)
	/// notification.
	fn psn_reset<F>(&self, func: F)
	where
		F: Fn(&PSHNOTIFY) -> AnyResult<()> + 'static,
	{
		let def_proc_val = self.wnd_ty().def_proc_val();
		self.wm_notify(0u16, co::PSN::RESET, move |p| {
			func(unsafe { p.cast_nmhdr_mut::<PSHNOTIFY>() })?;
			Ok(def_proc_val)
		});
	}

	/// [`PSN_SETACTIVE`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-setactive)
	/// notification.
	fn psn_set_active<F>(&self, func: F)
	where
		F: Fn(&PSHNOTIFY) -> AnyResult<i32> + 'static,
	{
		self.wm_notify(0u16, co::PSN::SETACTIVE, move |p| {
			Ok(func(unsafe { p.cast_nmhdr::<PSHNOTIFY>() })? as _)
		});
	}

	/// [`PSN_TRANSLATEACCELERATOR`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-translateaccelerator)
	/// notification.
	fn psn_translate_accelerator<F>(&self, func: F)
	where
		F: Fn() -> AnyResult<co::PSNRET> + 'static,
	{
		self.wm_notify(0u16, co::PSN::TRANSLATEACCELERATOR, move |_| Ok(func()?.raw() as _));
	}

	/// [`PSN_WIZBACK`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-wizback)
	/// notification.
	fn psn_wiz_back<F>(&self, func: F)
	where
		F: Fn(&PSHNOTIFY) -> AnyResult<i32> + 'static,
	{
		self.wm_notify(0u16, co::PSN::WIZBACK, move |p| {
			Ok(func(unsafe { p.cast_nmhdr::<PSHNOTIFY>() })? as _)
		});
	}

	/// [`PSN_WIZFINISH`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-wizfinish)
	/// notification.
	fn psn_wiz_finish<F>(&self, func: F)
	where
		F: Fn(&PSHNOTIFY) -> AnyResult<bool> + 'static,
	{
		self.wm_notify(0u16, co::PSN::WIZFINISH, move |p| {
			Ok(func(unsafe { p.cast_nmhdr::<PSHNOTIFY>() })? as _)
		});
	}

	/// [`PSN_WIZNEXT`](https://learn.microsoft.com/en-us/windows/win32/controls/psn-wiznext)
	/// notification.
	fn psn_wiz_next<F>(&self, func: F)
	where
		F: Fn(&PSHNOTIFY) -> AnyResult<i32> + 'static,
	{
		self.wm_notify(0u16, co::PSN::WIZNEXT, move |p| {
			Ok(func(unsafe { p.cast_nmhdr::<PSHNOTIFY>() })? as _)
		});
	}
}

impl GuiEventsPropSheetPage for BaseWndEvents {}
