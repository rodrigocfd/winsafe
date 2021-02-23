/// Declares multiple public constant values for the given type.
macro_rules! const_type_pub_values {
	(
		$name:ident
		$($pubcname:ident, $pubcval:expr)*
	) => {
		impl $name {
			$( pub const $pubcname: Self = Self($pubcval); )*
		}
	};
}

/// Declares multiple private constant values for the given type.
macro_rules! const_type_priv_values {
	(
		$name:ident
		$($privcname:ident, $privcval:expr)*
	) => {
		impl $name {
			$( pub(crate) const $privcname: Self = Self($privcval); )*
		}
	};
}

/// Declares the type of a constant with some impls, but not Debug/Display.
/// Optionally declares multiple public constant values.
macro_rules! const_type_no_debug_display {
	(
		$name:ident, $num:ty,
		$(#[$attr:meta])*
		$($pubcname:ident, $pubcval:expr)*
	) => {
		$(#[$attr])*
		#[repr(C)]
		#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
		pub struct $name(pub(crate) $num);

		// Conversions from/to underlying number.
		impl From<$num> for $name {
			fn from(n: $num) -> Self {
				Self(n) // the type can be created from the number
			}
		}
		impl From<$name> for $num {
			fn from(n: $name) -> Self {
				n.0 // the number can be created from the type
			}
		}

		// Formatters.
		impl std::fmt::LowerHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::LowerHex::fmt(&self.0, f)
			}
		}
		impl std::fmt::UpperHex for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::UpperHex::fmt(&self.0, f)
			}
		}
		impl std::fmt::Binary for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Binary::fmt(&self.0, f)
			}
		}
		impl std::fmt::Octal for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Octal::fmt(&self.0, f)
			}
		}

		// Bitflag operations.
		impl std::ops::BitAnd for $name {
			type Output = $name;
			fn bitand(self, rhs: Self) -> Self::Output {
				Self(self.0 & rhs.0)
			}
		}
		impl std::ops::BitAndAssign for $name {
			fn bitand_assign(&mut self, rhs: Self) {
				*self = Self(self.0 & rhs.0);
			}
		}
		impl std::ops::BitOr for $name {
			type Output = $name;
			fn bitor(self, rhs: Self) -> Self {
				Self(self.0 | rhs.0)
			}
		}
		impl std::ops::BitOrAssign for $name {
			fn bitor_assign(&mut self, rhs: Self) {
				*self = Self(self.0 | rhs.0);
			}
		}
		impl std::ops::BitXor for $name {
			type Output = $name;
			fn bitxor(self, rhs: Self) -> Self::Output {
				Self(self.0 ^ rhs.0)
			}
		}
		impl std::ops::BitXorAssign for $name {
			fn bitxor_assign(&mut self, rhs: Self) {
				*self = Self(self.0 ^ rhs.0);
			}
		}
		impl std::ops::Not for $name {
			type Output = $name;
			fn not(self) -> Self::Output {
				Self(!self.0)
			}
		}

		impl $name {
			/// Tells whether other bitflag style is present. Equivalent to
			/// `(val & other) != 0`.
			pub fn has(&self, other: $name) -> bool {
				(self.0 & other.0) != 0
			}
		}

		// All public const values.
		const_type_pub_values! { $name
			$($pubcname, $pubcval)*
		}
	};
}

/// Declares the type of a constant with some impls.
/// Optionally declares multiple public constant values.
macro_rules! const_type {
	(
		$name:ident, $num:ty,
		$(#[$attr:meta])*
		$($pubcname:ident, $pubcval:expr)*
	) => {
		const_type_no_debug_display! {
			$name, $num,
			$(#[$attr])*
			#[derive(Debug)]
			$($pubcname, $pubcval)*
		}

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Display::fmt(&self.0, f) // delegate
			}
		}
	};
}

/// Declares the type of a constant for a WM_COMMAND notification code,
/// convertible to [`CMD`](crate::co::CMD) constant type.
/// Optionally declares multiple public constant values.
macro_rules! const_type_cmd {
	(
		$name:ident,
		$(#[$attr:meta])*
		$($pubcname:ident, $pubcval:expr)*
	) => {
		const_type! {
			$name, u16,
			$(#[$attr])*
			$($pubcname, $pubcval)*
		}

		// Conversion to CMD notification code.
		impl From<$name> for CMD {
			fn from(v: $name) -> Self {
				Self(v.0)
			}
		}
	};
}

/// Declares the type of a constant for a WM_NOTIFY notification code,
/// convertible to [`NM`](crate::co::NM) constant type.
/// Optionally declares multiple public constant values.
macro_rules! const_type_nm {
	(
		$name:ident,
		$(#[$attr:meta])*
		$($pubcname:ident, $pubcval:expr)*
	) => {
		const_type! {
			$name, i32,
			$(#[$attr])*
			$($pubcname, $pubcval)*
		}

		// Conversion to NM notification code.
		impl From<$name> for NM {
			fn from(v: $name) -> Self {
				Self(v.0)
			}
		}
	};
}

/// Declares the type of a constant for a window style,
/// convertible to [`WS`](crate::co::WS) constant type.
/// Optionally declares multiple public constant values.
macro_rules! const_type_ws {
	(
		$name:ident,
		$(#[$attr:meta])*
		$($pubcname:ident, $pubcval:expr)*
	) => {
		const_type! {
			$name, u32,
			$(#[$attr])*
			$($pubcname, $pubcval)*
		}

		// Conversion to WS style.
		impl From<$name> for WS {
			fn from(v: $name) -> Self {
				Self(v.0)
			}
		}
	};
}

/// Declares the type of a constant for an extended window style,
/// convertible to [`WS_EX`](crate::co::WS_EX) constant type.
/// Optionally declares multiple public constant values.
macro_rules! const_type_wsex {
	(
		$name:ident,
		$(#[$attr:meta])*
		$($pubcname:ident, $pubcval:expr)*
	) => {
		const_type! {
			$name, u32,
			$(#[$attr])*
			$($pubcname, $pubcval)*
		}

		// Conversion to WS_EX style.
		impl From<$name> for WS_EX {
			fn from(v: $name) -> Self {
				Self(v.0)
			}
		}
	};
}
