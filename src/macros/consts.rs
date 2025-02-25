#![allow(unused_macros)]

/// Basic declaration of a numeric newtype constant.
macro_rules! const_basic_decl {
	(
		$name:ident : $ntype:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		#[repr(transparent)]
		#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
		pub struct $name($ntype);

		impl_intunderlying!($name, $ntype);

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
	};
}

/// Implements bitflag operations for numeric newtype constants.
macro_rules! const_impl_bitflag {
	( $name:ident ) => {
		impl crate::prelude::NativeBitflag for $name {
			fn has(&self, other: Self) -> bool {
				(self.0 & other.0) != 0
			}
		}

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
	};
}

/// Implements Debug and Display for numeric newtype constants.
macro_rules! const_impl_debug_display {
	( $name:ident ) => {
		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				if self.0 as usize > 0xffff {
					write!(f, "{}({:#010x})", stringify!($name), self.0)
				} else {
					write!(f, "{}({:#06x})", stringify!($name), self.0)
				}
			}
		}
		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				if self.0 as usize > 0xffff {
					write!(f, "{}({:#010x} {})",
						stringify!($name), self.0, self.0)
				} else {
					write!(f, "{}({:#06x} {})",
						stringify!($name), self.0, self.0)
				}
			}
		}
	};
}

/// Writes multiple pub values of a numeric newtype constant.
macro_rules! const_values_pub {
	(
		$name:ident;
		$(
			$( #[$valdoc:meta] )*
			$valname:ident $val:expr
		)*
	) => {
		impl $name {
			$(
				$( #[$valdoc] )*
				pub const $valname: Self = unsafe { Self::from_raw($val) };
			)*
		}
	};
}

/// Writes multiple pub(crate) values of a numeric newtype constant.
macro_rules! const_values_pubcrate {
	(
		$name:ident;
		$(
			$( #[$valdoc:meta] )*
			$valname:ident $val:expr
		)*
	) => {
		impl $name {
			$(
				$( #[$valdoc] )*
				pub(crate) const $valname: Self = unsafe { Self::from_raw($val) };
			)*
		}
	};
}

/// Writes multiple pub(crate) values of an arbitrary numeric type; used in
/// internal privs.rs files.
macro_rules! const_values_num_privs {
	(
		$( $name:ident $ty:ty = $val:expr )*
	) => {
		$( pub(crate) const $name: $ty = $val; )*
	};
}

/// Complete declaration of ordinary, non-bitflag numeric newtype constants.
macro_rules! const_ordinary {
	(
		$name:ident : $ntype:ty;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$valdoc:meta] )*
			$valname:ident $val:expr
		)*
	) => {
		const_basic_decl! {
			$name: $ntype;
			$( #[$doc] )*
		}
		const_impl_debug_display!($name);
		const_values_pub! {
			$name;
			$(
				$( #[$valdoc] )*
				$valname $val
			)*
		}
	};
}

/// Complete declaration of bitflag numeric newtype constants.
macro_rules! const_bitflag {
	(
		$name:ident : $ntype:ty;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$valdoc:meta] )*
			$valname:ident $val:expr
		)*
	) => {
		const_basic_decl! {
			$name: $ntype;
			$( #[$doc] )*
			///
			/// This is a bitflag constant.
		}
		const_impl_bitflag!($name);
		const_impl_debug_display!($name);
		const_values_pub! {
			$name;
			$(
				$( #[$valdoc] )*
				$valname $val
			)*
		}
	};
}

/// Complete declaration of a constant for a window message, convertible to the
/// co::WM.
macro_rules! const_wm {
	(
		$name:ident;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$valdoc:meta] )*
			$valname:ident $val:expr
		)*
	) => {
		const_basic_decl! {
			$name: u32;
			$( #[$doc] )*
			///
			/// This is a window message, convertible to [`WM`](crate::co::WM).
		}
		const_impl_bitflag!($name);
		const_impl_debug_display!($name);
		const_values_pub! {
			$name;
			$(
				$( #[$valdoc] )*
				$valname $val
			)*
		}
		impl From<$name> for crate::co::WM {
			fn from(v: $name) -> Self {
				unsafe { Self::from_raw(v.0) }
			}
		}
	};
}

/// Complete declaration of a constant for a WM_COMMAND notification code,
/// convertible to co::CMD.
macro_rules! const_cmd {
	(
		$name:ident;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$valdoc:meta] )*
			$valname:ident $val:expr
		)*
	) => {
		const_basic_decl! {
			$name: u16;
			$( #[$doc] )*
			///
			/// This is a [`wm::Command`](crate::msg::wm::Command) notification
			/// code, convertible to [`CMD`](crate::co::CMD).
		}
		const_impl_bitflag!($name);
		const_impl_debug_display!($name);
		const_values_pub! {
			$name;
			$(
				$( #[$valdoc] )*
				$valname $val
			)*
		}
		impl From<$name> for crate::co::CMD {
			fn from(v: $name) -> Self {
				Self(v.0)
			}
		}
	};
}

/// Complete declaration of a constant for a WM_NOTIFY notification code,
/// convertible to NmhdrCode.
macro_rules! const_nm {
	(
		$name:ident;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$valdoc:meta] )*
			$valname:ident $val:expr
		)*
	) => {
		const_basic_decl! {
			$name: i32;
			$( #[$doc] )*
			///
			/// This is a [`wm::Notify`](crate::msg::wm::Notify) notification
			/// code, convertible to/from [`NmhdrCode`](crate::NmhdrCode).
		}
		const_impl_bitflag!($name);
		const_impl_debug_display!($name);
		const_values_pub! {
			$name;
			$(
				$( #[$valdoc] )*
				$valname $val
			)*
		}
		impl From<$name> for crate::NmhdrCode {
			fn from(v: $name) -> Self {
				Self::new(v.raw())
			}
		}
		impl TryFrom<crate::NmhdrCode> for $name {
			type Error = crate::co::ERROR;

			fn try_from(value: crate::NmhdrCode) -> Result<Self, Self::Error> {
				// Can't use match because some values are defined as a sum, then:
				// "arbitrary expressions aren't allowed in patterns"
				$(
					if value.raw() == $val { return Ok(Self::$valname); }
				)*
				Err(crate::co::ERROR::INVALID_DATA)
			}
		}
	};
}

/// Complete declaration of a constant for a window style, convertible to
/// co::WS.
macro_rules! const_ws {
	(
		$name:ident : $ntype:ty;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$valdoc:meta] )*
			$valname:ident $val:expr
		)*
	) => {
		const_basic_decl! {
			$name: $ntype;
			$( #[$doc] )*
			///
			/// This is a window style, convertible to [`WS`](crate::co::WS).
		}
		const_impl_bitflag!($name);
		const_impl_debug_display!($name);
		const_values_pub! {
			$name;
			$(
				$( #[$valdoc] )*
				$valname $val
			)*
		}
		impl From<$name> for crate::co::WS {
			fn from(v: $name) -> Self {
				unsafe { Self::from_raw(v.0 as _) }
			}
		}
		impl From<crate::co::WS> for $name {
			fn from(v: crate::co::WS) -> Self {
				unsafe { Self::from_raw(v.raw() as _) }
			}
		}
	};
}

/// Complete declaration of a constant for an extended window style, convertible
/// to WS_EX.
macro_rules! const_wsex {
	(
		$name:ident;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$valdoc:meta] )*
			$valname:ident $val:expr
		)*
	) => {
		const_basic_decl! {
			$name: u32;
			$( #[$doc] )*
			///
			/// This is an extended windoow style, convertible to
			/// [`WS_EX`](crate::co::WS_EX).
		}
		const_impl_bitflag!($name);
		const_impl_debug_display!($name);
		const_values_pub! {
			$name;
			$(
				$( #[$valdoc] )*
				$valname $val
			)*
		}
		impl From<$name> for crate::co::WS_EX {
			fn from(v: $name) -> Self {
				unsafe { Self::from_raw(v.0) }
			}
		}
		impl From<crate::co::WS_EX> for $name {
			fn from(v: crate::co::WS_EX) -> Self {
				unsafe { Self::from_raw(v.raw() as _) }
			}
		}
	};
}

/// Complete declaration of a constant with a literal string as its underlying
/// type. Pretty rare.
macro_rules! const_str {
	(
		$name:ident;
		$( #[$doc:meta] )*
		=>
		$(
			$( #[$valdoc:meta] )*
			$valname:ident $val:literal
		)*
	) => {
		$( #[$doc] )*
		#[derive(Clone, Copy, PartialEq, Eq)]
		pub struct $name(&'static str);

		impl crate::prelude::NativeStrConst for $name {}

		impl TryFrom<&str> for $name {
			type Error = crate::co::ERROR;

			fn try_from(value: &str) -> Result<Self, Self::Error> {
				match value {
					$( $val => Ok(Self::$valname), )*
					_ => Err(crate::co::ERROR::INVALID_DATA),
				}
			}
		}

		impl From<$name> for crate::kernel::decl::WString {
			fn from(v: $name) -> Self {
				crate::kernel::decl::WString::from_str(v.0)
			}
		}

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				std::fmt::Display::fmt(self.0, f)
			}
		}
		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "\"{}\" {}", self.0, stringify!($name))
			}
		}

		impl $name {
			$(
				$( #[$valdoc] )*
				pub const $valname: Self = Self($val);
			)*
		}
	};
}
