use crate::funcs::{HIWORD, LOWORD, MAKEDWORD};
use crate::msg::WndMsg;
use crate::structs::POINT;

/// Struct for a message that has no parameters and no meaningful return value.
macro_rules! empty_msg {
	(
		$name:ident, $wmconst:expr,
		$(#[$msdn:meta])*
	) => {
		$(#[$msdn])*
		/// message, which has no parameters.
		///
		/// Return type: `()`.
		pub struct $name {}

		impl MsgSend for $name {
			type RetType = ();

			fn convert_ret(&self, _: isize) -> Self::RetType {
				()
			}

			fn as_generic_wm(&self) -> WndMsg {
				WndMsg {
					msg_id: $wmconst,
					wparam: 0,
					lparam: 0,
				}
			}
		}
	};
}

/// Struct for a handleable message that has no parameters and no meaningful
/// return value.
macro_rules! empty_msg_handleable {
	(
		$name:ident, $wmconst:expr,
		$(#[$msdn:meta])*
	) => {
		empty_msg! {
			$name, $wmconst,
			$(#[$msdn])*
		}

		impl MsgSendRecv for $name {
			fn from_generic_wm(_: WndMsg) -> Self {
				Self {}
			}
		}
	};
}

/// Struct for WM_CHAR-based handleable messages.
macro_rules! char_msg {
	(
		$name:ident, $wmconst:expr,
		$(#[$msdn:meta])*
	) => {
		$(#[$msdn])*
		/// message parameters.
		///
		/// Return type: `()`.
		pub struct $name {
			pub char_code: u32,
			pub repeat_count: u16,
			pub scan_code: u8,
			pub is_extended_key: bool,
			pub has_alt_key: bool,
			pub key_was_previously_down: bool,
			pub key_is_being_released: bool,
		}

		impl MsgSend for $name {
			type RetType = ();

			fn convert_ret(&self, _: isize) -> Self::RetType {
				()
			}

			fn as_generic_wm(&self) -> WndMsg {
				WndMsg {
					msg_id: $wmconst,
					wparam: self.char_code as usize,
					lparam: MAKEDWORD(
						self.repeat_count,
						MAKEWORD(
							self.scan_code,
							if self.is_extended_key { 0b0000_0001 } else { 0 } |
							if self.has_alt_key { 0b0010_0000 } else { 0 } |
							if self.key_was_previously_down { 0b0100_0000 } else { 0 } |
							if self.key_is_being_released { 0b1000_0000 } else { 0 },
						),
					) as isize,
				}
			}
		}

		impl MsgSendRecv for $name {
			fn from_generic_wm(p: WndMsg) -> Self {
				Self {
					char_code: p.wparam as u32,
					repeat_count: LOWORD(p.lparam as u32),
					scan_code: LOBYTE(HIWORD(p.lparam as u32)),
					is_extended_key: (HIBYTE(HIWORD(p.lparam as u32)) & 0b0000_0001) != 0,
					has_alt_key: (HIBYTE(HIWORD(p.lparam as u32)) & 0b0010_0000) != 0,
					key_was_previously_down: (HIBYTE(HIWORD(p.lparam as u32)) & 0b0100_0000) != 0,
					key_is_being_released: (HIBYTE(HIWORD(p.lparam as u32)) & 0b1000_0000) != 0,
				}
			}
		}
	};
}

/// Struct for WM_CTLCOLOR* handleable messages.
macro_rules! ctl_color_msg {
	(
		$name:ident, $wmconst:expr,
		$(#[$msdn:meta])*
	) => {
		$(#[$msdn])*
		/// message parameters.
		///
		/// Return type: `HBRUSH`.
		pub struct $name {
			pub hdc: HDC,
			pub hwnd: HWND,
		}

		impl MsgSend for $name {
			type RetType = HBRUSH;

			fn convert_ret(&self, v: isize) -> Self::RetType {
				HBRUSH { ptr: v as *mut _ }
			}

			fn as_generic_wm(&self) -> WndMsg {
				WndMsg {
					msg_id: $wmconst,
					wparam: self.hdc.ptr as usize,
					lparam: self.hwnd.ptr as isize,
				}
			}
		}

		impl MsgSendRecv for $name {
			fn from_generic_wm(p: WndMsg) -> Self {
				Self {
					hdc: HDC { ptr: p.wparam as *mut _ },
					hwnd: HWND { ptr: p.lparam as *mut _ },
				}
			}
		}
	};
}

/// Struct for WM_*BUTTON* handleable messages and others.
macro_rules! button_msg {
	(
		$name:ident, $wmconst:expr,
		$(#[$doc:meta])*
	) => {
		$(#[$doc])*
		/// message parameters.
		///
		/// Return type: `()`.
		pub struct $name {
			pub vkeys: co::VK,
			pub coords: POINT,
		}

		impl MsgSend for $name {
			type RetType = ();

			fn convert_ret(&self, _: isize) -> Self::RetType {
				()
			}

			fn as_generic_wm(&self) -> WndMsg {
				WndMsg {
					msg_id: $wmconst,
					wparam: self.vkeys.0 as usize,
					lparam: MAKEDWORD(self.coords.x as u16, self.coords.y as u16) as isize,
				}
			}
		}

		impl MsgSendRecv for $name {
			fn from_generic_wm(p: WndMsg) -> Self {
				Self {
					vkeys: co::VK(p.wparam as u16),
					coords: POINT {
						x: LOWORD(p.lparam as u32) as i32,
						y: HIWORD(p.lparam as u32) as i32,
					},
				}
			}
		}
	};
}

/// Converts a `POINT` to a an `LPARAM` field.
pub fn point_to_lp(p: POINT) -> isize {
	MAKEDWORD(p.x as u16, p.y as u16) as isize
}

/// Converts the `LPARAM` field to a `POINT`.
pub fn lp_to_point(p: WndMsg) -> POINT {
	POINT::new(
		LOWORD(p.lparam as u32) as i32,
		HIWORD(p.lparam as u32) as i32,
	)
}
