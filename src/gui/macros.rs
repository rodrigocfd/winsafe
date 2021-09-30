/// Implements Debug trait to leaf window.
macro_rules! impl_debug {
	($name:ident) => {
		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "HWND {}, {}",
					self.hwnd(),
					match self.raw_dlg {
						RawDlg::Raw(_) => "non-dialog",
						RawDlg::Dlg(_) => "dialog",
					},
				)
			}
		}
	};
}

/// Implements Parent trait to leaf window.
macro_rules! impl_parent {
	($name:ident) => {
		impl crate::gui::traits::Parent for $name {
			fn as_any(&self) -> &dyn std::any::Any {
				self
			}
		}
	};
}

/// Implements base_ref() method to leaf window.
macro_rules! fn_base_ref {
	() => {
		pub(in crate::gui) fn base_ref(&self) -> &Base {
			match &self.raw_dlg {
				RawDlg::Raw(r) => r.base_ref(),
				RawDlg::Dlg(d) => d.base_ref(),
			}
		}
	};
}

/// Implements hwnd() method to leaf window.
macro_rules! pub_fn_hwnd {
	() => {
		/// Returns the underlying handle for this control.
		///
		/// Note that the handle is initially null, receiving an actual value
		/// only after the control is created.
		pub fn hwnd(&self) -> HWND {
			*self.base_ref().hwnd_ref()
		}
	};
}

/// Implements on() method to leaf window.
macro_rules! pub_fn_on {
	() => {
		/// Exposes the window events.
		///
		/// # Panics
		///
		/// Panics if the window is already created. Events must be set before
		/// window creation.
		pub fn on(&self) -> &WindowEvents {
			self.base_ref().user_events_ref()
		}
	};
}

/// Implements run_ui_thread() method to leaf window.
macro_rules! pub_fn_run_ui_thread {
	() => {
		/// If you perform a very long task in the UI thread, the UI freezes until
		/// the task is complete – this may cause the impression that your
		/// application crashed. That's why long tasks are performed in parallel
		/// threads. However, at some point you'll want to update the UI to reflect
		/// the task progress, but if you update the UI from another thread
		/// (different from the original UI thread), the UI may deadlock, and you
		/// application crashes.
		///
		/// The `run_ui_thread` method allows UI updates by running a closure
		/// synchronously in the window's original UI thread.
		///
		/// This is what this `run_ui_thread` does, step-by-step:
		///
		/// 1. blocks current thread;
		/// 2. switches to the window's original UI thread;
		/// 3. runs the given `FnOnce`;
		/// 4. switches back to the first thread, which is then unblocked.
		///
		/// When working in a parallel thread, you **must** call `run_ui_thread` to
		/// update the UI.
		///
		/// # Examples
		///
		/// The example below shows the event of a
		/// [button click](crate::gui::events::ButtonEvents::bn_clicked) which
		/// starts a long task in a parallel thread. As it progresses, the status is
		/// printed at the windows's titlebar.
		///
		/// ```rust,ignore
		/// use winsafe::{gui, BoxResult, GetCurrentThreadId, Sleep};
		///
		/// let wnd: gui::WindowMain; // initialized somewhere
		/// let btn: gui::Button;
		///
		/// btn.on().bn_clicked({
		///     let wnd = wnd.clone();
		///     move || -> BoxResult<()> {
		///         println!("Click event at {:#x}", GetCurrentThreadId());
		///
		///         std::thread::spawn({
		///             let wnd = wnd.clone();
		///             move || {
		///                 println!("Parallel task starts at {:#x}", GetCurrentThreadId());
		///                 Sleep(2000);
		///
		///                 wnd.run_ui_thread({
		///                     let wnd = wnd.clone();
		///                     move || -> BoxResult<()> {
		///                         println!("Updating UI at {:#x}", GetCurrentThreadId());
		///                         wnd.hwnd().SetWindowText("Status... 50%")?;
		///                         Ok(())
		///                     }
		///                 });
		///
		///                 println!("Parallel task keeps going at {:#x}", GetCurrentThreadId());
		///                 Sleep(2000);
		///
		///                 wnd.run_ui_thread({
		///                     let wnd = wnd.clone();
		///                     move || -> BoxResult<()> {
		///                         println!("Updating UI at {:#x}", GetCurrentThreadId());
		///                         wnd.hwnd().SetWindowText("Status... 100%")?;
		///                         Ok(())
		///                     }
		///                 });
		///             }
		///         });
		///
		///         Ok(())
		///     }
		/// });
		/// ```
		pub fn run_ui_thread<F>(&self, func: F)
			where F: FnOnce() -> BoxResult<()>,
		{
			match &self.raw_dlg {
				RawDlg::Raw(r) => r.run_ui_thread(func),
				RawDlg::Dlg(d) => d.run_ui_thread(func),
			}
		}
	};
}
