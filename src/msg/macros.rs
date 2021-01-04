use crate::msg::Wm;

/// Struct for a message that has no parameters.
macro_rules! empty_msg {
	(
		$name:ident, $wmconst:expr,
		$(#[$attr:meta])*
	) => {
		$(#[$attr])*
		pub struct $name {}

		impl From<$name> for Wm {
			fn from(_: $name) -> Self {
				Self {
					msg_id: $wmconst,
					wparam: 0,
					lparam: 0,
				}
			}
		}

		impl From<Wm> for $name {
			fn from(_: Wm) -> Self {
				Self {}
			}
		}
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

		impl From<$name> for Wm {
			fn from(p: $name) -> Self {
				Self {
					msg_id: $wmconst,
					wparam: unsafe { p.hdc.as_ptr() } as usize,
					lparam: unsafe { p.hwnd.as_ptr() } as isize,
				}
			}
		}

		impl From<Wm> for $name {
			fn from(p: Wm) -> Self {
				Self {
					hdc: unsafe { HDC::from_ptr(p.wparam as *mut c_void) },
					hwnd: unsafe { HWND::from_ptr(p.lparam as *mut c_void) },
				}
			}
		}
	};
}

/// Struct for WM_*BUTTON* messages and others.
macro_rules! button_msg {
	(
		$name:ident, $wmconst:expr,
		$(#[$attr:meta])*
	) => {
		$(#[$attr])*
		pub struct $name {
			pub vkeys: co::VK,
			pub coords: POINT,
		}

		impl From<$name> for Wm {
			fn from(p: $name) -> Self {
				Self {
					msg_id: $wmconst,
					wparam: u16::from(p.vkeys) as usize,
					lparam: MAKEDWORD(p.coords.x as u16, p.coords.y as u16) as isize,
				}
			}
		}

		impl From<Wm> for $name {
			fn from(p: Wm) -> Self {
				Self {
					vkeys: co::VK::from(p.wparam as u16),
					coords: POINT {
						x: LOWORD(p.lparam as u32) as i32,
						y: HIWORD(p.lparam as u32) as i32,
					},
				}
			}
		}
	};
}

/// Converts a reference into `LPARAM` field, for message structs.
pub fn ref_to_lparam<T>(field: &T) -> isize {
	field as *const T as isize
}

/// Converts the `LPARAM` field to a mut reference, for message structs.
pub fn lparam_to_mut_ref<'a, T>(p: Wm) -> &'a mut T {
	unsafe { &mut *(p.lparam as *mut T) }
}

/// Converts the `LPARAM` field to a reference, for message structs.
pub fn lparam_to_ref<'a, T>(p: Wm) -> &'a T {
	unsafe { &*(p.lparam as *const T) }
}
