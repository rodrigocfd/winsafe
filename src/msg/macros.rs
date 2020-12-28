use crate::msg::WmAny;

/// Implements standard `return` method that returns 0 as window, 1 as dialog.
macro_rules! msg_lresult_zero {
	($name:ident $(, $life:lifetime)*) => {
		impl<$($life),*> $name<$($life),*> {
			/// Generates the message result value.
			///
			/// You should return zero for ordinary windows, and 1 for dialogs.
			pub fn lresult(&self, val: isize) -> LResult {
				LResult(val)
			}
		}
	};
}

/// Struct for a message that has no parameters.
macro_rules! empty_msg {
	(
		$name:ident, $wmconst:expr,
		$(#[$attr:meta])*
	) => {
		$(#[$attr])*
		pub struct $name {}

		impl From<$name> for WmAny {
			fn from(_: $name) -> Self {
				Self {
					msg_id: $wmconst,
					wparam: 0,
					lparam: 0,
				}
			}
		}

		impl From<WmAny> for $name {
			fn from(_: WmAny) -> Self {
				Self {}
			}
		}

		msg_lresult_zero!($name);
	};
}

/// Struct for WM_CTLCOLOR* messages.
macro_rules! ctl_color_msg {
	(
		$name:ident, $wmconst:expr,
		$(#[$attr:meta])*
	) => {
		$(#[$attr])*
		pub struct $name {
			pub hdc: HDC,
			pub hwnd: HWND,
		}

		impl From<$name> for WmAny {
			fn from(p: $name) -> Self {
				Self {
					msg_id: $wmconst,
					wparam: unsafe { p.hdc.as_ptr() } as usize,
					lparam: unsafe { p.hwnd.as_ptr() } as isize,
				}
			}
		}

		impl From<WmAny> for $name {
			fn from(p: WmAny) -> Self {
				Self {
					hdc: unsafe { HDC::from_ptr(p.wparam as *mut c_void) },
					hwnd: unsafe { HWND::from_ptr(p.lparam as *mut c_void) },
				}
			}
		}

		impl $name {
			/// Generates the message result value.
			pub fn lresult(&self, val: HBRUSH) -> LResult {
				LResult(unsafe { val.as_ptr() } as isize)
			}
		}
	};
}

/// Converts a reference into `LPARAM` field, for message structs.
pub fn ref_to_lparam<T>(field: &T) -> isize {
	field as *const T as isize
}

/// Converts the `LPARAM` field to a mut reference, for message structs.
pub fn lparam_to_mut_ref<'a, T>(p: WmAny) -> &'a mut T {
	unsafe { (p.lparam as *mut T).as_mut() }.unwrap()
}

/// Converts the `LPARAM` field to a reference, for message structs.
pub fn lparam_to_ref<'a, T>(p: WmAny) -> &'a T {
	unsafe { (p.lparam as *const T).as_ref() }.unwrap()
}