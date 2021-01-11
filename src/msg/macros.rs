use crate::msg::Wm;

/// Struct for a message that has no parameters and no meaningful return value.
macro_rules! empty_msg {
	(
		$name:ident, $wmconst:expr,
		$(#[$attr:meta])*
	) => {
		$(#[$attr])*
		pub struct $name {}

		impl Message for $name {
			type RetType = ();

			fn convert_ret(_: isize) -> () {
				()
			}

			fn into_generic_wm(self) -> Wm {
				Wm {
					msg_id: $wmconst,
					wparam: 0,
					lparam: 0,
				}
			}

			fn from_generic_wm(_: Wm) -> Self {
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

		impl Message for $name {
			type RetType = HBRUSH;

			fn convert_ret(v: isize) -> HBRUSH {
				HBRUSH { ptr: v as *mut _ }
			}

			fn into_generic_wm(self) -> Wm {
				Wm {
					msg_id: $wmconst,
					wparam: self.hdc.ptr as usize,
					lparam: self.hwnd.ptr as isize,
				}
			}

			fn from_generic_wm(p: Wm) -> Self {
				Self {
					hdc: HDC { ptr: p.wparam as *mut _ },
					hwnd: HWND { ptr: p.lparam as *mut _ },
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
			pub coords: s::POINT,
		}

		impl Message for $name {
			type RetType = ();

			fn convert_ret(_: isize) -> () {
				()
			}

			fn into_generic_wm(self) -> Wm {
				Wm {
					msg_id: $wmconst,
					wparam: u16::from(self.vkeys) as usize,
					lparam: MAKEDWORD(self.coords.x as u16, self.coords.y as u16) as isize,
				}
			}

			fn from_generic_wm(p: Wm) -> Self {
				Self {
					vkeys: co::VK::from(p.wparam as u16),
					coords: s::POINT {
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
