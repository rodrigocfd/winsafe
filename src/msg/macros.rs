use crate::funcs::{HIWORD, LOWORD, MAKEDWORD};
use crate::msg::WndMsg;
use crate::structs::POINT;

/// Implements the `convert_ret` method for message structs that returns a `()`.
macro_rules! fn_convert_ret_void {
	() => {
		fn convert_ret(&self, _: isize) -> Self::RetType {
			()
		}
	};
}

/// Implements the `convert_ret` method for message structs that returns a
/// `bool`.
macro_rules! fn_convert_ret_bool {
	() => {
		fn convert_ret(&self, v: isize) -> Self::RetType {
			v != 0
		}
	};
}

/// Implements the `convert_ret` method for message structs that returns a
/// `WinResult<()>`.
macro_rules! fn_convert_ret_winresult_void {
	() => {
		fn convert_ret(&self, v: isize) -> Self::RetType {
			match v {
				0 => Err(co::ERROR::BAD_ARGUMENTS),
				_ => Ok(()),
			}
		}
	};
}

/// Implements the `convert_ret` method for message structs that returns a
/// `WinResult<HANDLE>`.
macro_rules! fn_convert_ret_winresult_handle {
	($handle:ident) => {
		fn convert_ret(&self, v: isize) -> Self::RetType {
			match v {
				0 => Err(co::ERROR::BAD_ARGUMENTS),
				p => Ok($handle { ptr: p as _ }),
			}
		}
	};
}

/// Implements the `convert_ret` method for message structs that returns an
/// `Option<HANDLE>`.
macro_rules! fn_convert_ret_option_handle {
	($handle:ident) => {
		fn convert_ret(&self, v: isize) -> Self::RetType {
			match v {
				0 => None,
				ptr => Some($handle { ptr: ptr as _ }),
			}
		}
	};
}

/// Struct for a message that has no parameters and no meaningful return value.
macro_rules! pub_struct_msg_empty {
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

			fn_convert_ret_void!();

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
macro_rules! pub_struct_msg_empty_handleable {
	(
		$name:ident, $wmconst:expr,
		$(#[$msdn:meta])*
	) => {
		pub_struct_msg_empty! {
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
macro_rules! pub_struct_msg_char {
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

			fn_convert_ret_void!();

			fn as_generic_wm(&self) -> WndMsg {
				WndMsg {
					msg_id: $wmconst,
					wparam: self.char_code as _,
					lparam: MAKEDWORD(
						self.repeat_count,
						MAKEWORD(
							self.scan_code,
							if self.is_extended_key { 0b0000_0001 } else { 0 } |
							if self.has_alt_key { 0b0010_0000 } else { 0 } |
							if self.key_was_previously_down { 0b0100_0000 } else { 0 } |
							if self.key_is_being_released { 0b1000_0000 } else { 0 },
						),
					) as _,
				}
			}
		}

		impl MsgSendRecv for $name {
			fn from_generic_wm(p: WndMsg) -> Self {
				Self {
					char_code: p.wparam as _,
					repeat_count: LOWORD(p.lparam as _),
					scan_code: LOBYTE(HIWORD(p.lparam as _)),
					is_extended_key: (HIBYTE(HIWORD(p.lparam as _)) & 0b0000_0001) != 0,
					has_alt_key: (HIBYTE(HIWORD(p.lparam as _)) & 0b0010_0000) != 0,
					key_was_previously_down: (HIBYTE(HIWORD(p.lparam as _)) & 0b0100_0000) != 0,
					key_is_being_released: (HIBYTE(HIWORD(p.lparam as _)) & 0b1000_0000) != 0,
				}
			}
		}
	};
}

/// Struct for WM_CTLCOLOR* handleable messages.
macro_rules! pub_struct_msg_ctlcolor {
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
				HBRUSH { ptr: v as _ }
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
					hdc: HDC { ptr: p.wparam as _ },
					hwnd: HWND { ptr: p.lparam as _ },
				}
			}
		}
	};
}

/// Struct for WM_*BUTTON* handleable messages and others.
macro_rules! pub_struct_msg_button {
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

			fn_convert_ret_void!();

			fn as_generic_wm(&self) -> WndMsg {
				WndMsg {
					msg_id: $wmconst,
					wparam: self.vkeys.0 as usize,
					lparam: MAKEDWORD(self.coords.x as _, self.coords.y as _) as _,
				}
			}
		}

		impl MsgSendRecv for $name {
			fn from_generic_wm(p: WndMsg) -> Self {
				Self {
					vkeys: co::VK(p.wparam as _),
					coords: POINT {
						x: LOWORD(p.lparam as _) as _,
						y: HIWORD(p.lparam as _) as _,
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
		LOWORD(p.lparam as _) as _,
		HIWORD(p.lparam as _) as _,
	)
}