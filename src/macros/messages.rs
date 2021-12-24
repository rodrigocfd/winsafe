#![allow(unused_macros)]

/// Struct for a message that has no parameters and no meaningful return value.
macro_rules! pub_struct_msg_empty {
	(
		$name:ident : $wmconst:expr; $feature:literal;
		$( #[$msdn:meta] )*
	) => {
		$( #[$msdn] )*
		/// message, which has no parameters.
		///
		/// Return type: `()`.
		#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
		pub struct $name {}

		impl MsgSend for $name {
			type RetType = ();

			fn convert_ret(&self, _: isize) -> Self::RetType {
				()
			}

			fn as_generic_wm(&mut self) -> WndMsg {
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
		$name:ident : $wmconst:expr; $feature:literal;
		$( #[$msdn:meta] )*
	) => {
		pub_struct_msg_empty! {
			$name : $wmconst; $feature;
			$( #[$msdn] )*
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
		$name:ident : $wmconst:expr; $feature:literal;
		$( #[$msdn:meta] )*
	) => {
		$( #[$msdn] )*
		/// message parameters.
		///
		/// Return type: `()`.
		#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
		pub struct $name {
			pub char_code: u32,
			pub repeat_count: u16,
			pub scan_code: u8,
			pub is_extended_key: bool,
			pub has_alt_key: bool,
			pub key_was_previously_down: bool,
			pub key_is_being_released: bool,
		}

		impl crate::prelude::MsgSend for $name {
			type RetType = ();

			fn convert_ret(&self, _: isize) -> Self::RetType {
				()
			}

			fn as_generic_wm(&mut self) -> crate::msg::WndMsg {
				crate::msg::WndMsg {
					msg_id: $wmconst,
					wparam: self.char_code as _,
					lparam: crate::kernel::decl::MAKEDWORD(
						self.repeat_count,
						crate::kernel::decl::MAKEWORD(
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

		impl crate::prelude::MsgSendRecv for $name {
			fn from_generic_wm(p: crate::msg::WndMsg) -> Self {
				use crate::kernel::decl::{HIBYTE, HIWORD, LOBYTE, LOWORD};
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
		$name:ident : $wmconst:expr; $feature:literal;
		$( #[$msdn:meta] )*
	) => {
		$( #[$msdn] )*
		/// message parameters.
		///
		/// Return type: `HBRUSH`.
		#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
		pub struct $name {
			pub hdc: crate::user::decl::HDC,
			pub hwnd: crate::user::decl::HWND,
		}

		impl crate::prelude::MsgSend for $name {
			type RetType = crate::user::decl::HBRUSH;

			fn convert_ret(&self, v: isize) -> Self::RetType {
				crate::user::decl::HBRUSH(v as _)
			}

			fn as_generic_wm(&mut self) -> crate::msg::WndMsg {
				crate::msg::WndMsg {
					msg_id: $wmconst,
					wparam: self.hdc.0 as usize,
					lparam: self.hwnd.0 as isize,
				}
			}
		}

		impl crate::prelude::MsgSendRecv for $name {
			fn from_generic_wm(p: crate::msg::WndMsg) -> Self {
				Self {
					hdc: crate::user::decl::HDC(p.wparam as _),
					hwnd: crate::user::decl::HWND(p.lparam as _),
				}
			}
		}
	};
}

/// Struct for WM_*BUTTON* handleable messages and others.
macro_rules! pub_struct_msg_button {
	(
		$name:ident : $wmconst:expr; $feature:literal;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		/// message parameters.
		///
		/// Return type: `()`.
		#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
		pub struct $name {
			pub vkey_code: co::VK,
			pub coords: POINT,
		}

		impl MsgSend for $name {
			type RetType = ();

			fn convert_ret(&self, _: isize) -> Self::RetType {
				()
			}

			fn as_generic_wm(&mut self) -> WndMsg {
				WndMsg {
					msg_id: $wmconst,
					wparam: self.vkey_code.0 as usize,
					lparam: self.coords.into_u32() as _,
				}
			}
		}

		impl MsgSendRecv for $name {
			fn from_generic_wm(p: WndMsg) -> Self {
				Self {
					vkey_code: co::VK(p.wparam as _),
					coords: POINT {
						x: LOWORD(p.lparam as _) as _,
						y: HIWORD(p.lparam as _) as _,
					},
				}
			}
		}
	};
}
