/// Implements Send and Sync traits to leaf window/control.
macro_rules! impl_send_sync {
	($name:ident) => {
		unsafe impl Send for $name {}
		unsafe impl Sync for $name {}
	};
}

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

/// Implements Window trait to leaf window.
macro_rules! impl_window {
	($name:ident) => {
		impl Window for $name {
			fn hwnd(&self) -> HWND {
				match &self.raw_dlg {
					RawDlg::Raw(r) => r.hwnd(),
					RawDlg::Dlg(d) => d.hwnd(),
				}
			}
		}
	};
}

/// Implements AsWindow trait to leaf window.
macro_rules! impl_aswindow {
	($name:ident) => {
		impl crate::gui::traits::AsWindow for $name {
			fn as_window(&self) -> std::sync::Arc<dyn Window> {
				match &self.raw_dlg {
					RawDlg::Raw(r) => r.as_window(),
					RawDlg::Dlg(d) => d.as_window(),
				}
			}
		}
	};
}

/// Implements UiThread trait to leaf window.
macro_rules! impl_uithread {
	($name:ident) => {
		impl UiThread for $name {
			fn run_ui_thread<F>(&self, func: F)
				where F: FnOnce() -> ErrResult<()>,
			{
				match &self.raw_dlg {
					RawDlg::Raw(r) => r.run_ui_thread(func),
					RawDlg::Dlg(d) => d.run_ui_thread(func),
				}
			}
		}
	};
}

/// Implements ParentEvents trait to leaf window.
macro_rules! impl_parentevents {
	($name:ident) => {
		impl ParentEvents for $name {
			fn on(&self) -> &WindowEventsAll {
				match &self.raw_dlg {
					RawDlg::Raw(r) => r.on(),
					RawDlg::Dlg(d) => d.on(),
				}
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
