/// Declares multiple public constant values for the given type.
macro_rules! const_type_pub_values {
	(
		$tname:ident,
		$(
			$(#[$pubcdoc:meta])*
			$pubcname:ident, $pubcval:expr
		)*
	) => {
		impl $tname {
			$(
				$(#[$pubcdoc])*
				pub const $pubcname: Self = Self($pubcval);
			)*
		}
	};
}

/// Declares multiple private constant values for the given type.
macro_rules! const_type_priv_values {
	(
		$tname:ident,
		$(
			$(#[$privcdoc:meta])*
			$privcname:ident, $privcval:expr
		)*
	) => {
		impl $tname {
			$(
				$(#[$privcdoc])*
				pub(crate) const $privcname: Self = Self($privcval);
			)*
		}
	};
}

/// Declares the type of a constant with some impls, but not Debug/Display.
/// Optionally declares multiple public constant values.
macro_rules! const_type_no_debug_display {
	(
		$tname:ident, $ttype:ty,
		$(#[$tdoc:meta])*
		->
		$(
			$(#[$pubcdoc:meta])*
			$pubcname:ident, $pubcval:expr
		)*
	) => {
		$(#[$tdoc])*
		#[repr(C)]
		#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
		pub struct $tname(pub(crate) $ttype);

		// Conversions from/to underlying number.
		impl From<$ttype> for $tname {
			fn from(n: $ttype) -> Self {
				Self(n) // the type can be created from the number
			}
		}
		impl From<$tname> for $ttype {
			fn from(n: $tname) -> Self {
				n.0 // the number can be created from the type
			}
		}

		// Formatters.
		impl std::fmt::LowerHex for $tname {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::LowerHex::fmt(&self.0, f)
			}
		}
		impl std::fmt::UpperHex for $tname {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::UpperHex::fmt(&self.0, f)
			}
		}
		impl std::fmt::Binary for $tname {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Binary::fmt(&self.0, f)
			}
		}
		impl std::fmt::Octal for $tname {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Octal::fmt(&self.0, f)
			}
		}

		// Bitflag operations.
		impl std::ops::BitAnd for $tname {
			type Output = $tname;
			fn bitand(self, rhs: Self) -> Self::Output {
				Self(self.0 & rhs.0)
			}
		}
		impl std::ops::BitAndAssign for $tname {
			fn bitand_assign(&mut self, rhs: Self) {
				*self = Self(self.0 & rhs.0);
			}
		}
		impl std::ops::BitOr for $tname {
			type Output = $tname;
			fn bitor(self, rhs: Self) -> Self {
				Self(self.0 | rhs.0)
			}
		}
		impl std::ops::BitOrAssign for $tname {
			fn bitor_assign(&mut self, rhs: Self) {
				*self = Self(self.0 | rhs.0);
			}
		}
		impl std::ops::BitXor for $tname {
			type Output = $tname;
			fn bitxor(self, rhs: Self) -> Self::Output {
				Self(self.0 ^ rhs.0)
			}
		}
		impl std::ops::BitXorAssign for $tname {
			fn bitxor_assign(&mut self, rhs: Self) {
				*self = Self(self.0 ^ rhs.0);
			}
		}
		impl std::ops::Not for $tname {
			type Output = $tname;
			fn not(self) -> Self::Output {
				Self(!self.0)
			}
		}

		impl $tname {
			/// Tells whether other bitflag style is present. Equivalent to
			/// `(val & other) != 0`.
			///
			/// This method is common to all constant types.
			pub fn has(&self, other: $tname) -> bool {
				(self.0 & other.0) != 0
			}
		}

		// All public const values.
		const_type_pub_values! { $tname,
			$(
				$(#[$pubcdoc])*
				$pubcname, $pubcval
			)*
		}
	};
}

/// Declares the type of a constant with some impls.
/// Optionally declares multiple public constant values.
macro_rules! const_type {
	(
		$tname:ident, $ttype:ty,
		$(#[$tdoc:meta])*
		->
		$(
			$(#[$pubcdoc:meta])*
			$pubcname:ident, $pubcval:expr
		)*
	) => {
		const_type_no_debug_display! {
			$tname, $ttype,
			$(#[$tdoc])*
			#[derive(Debug)]
			->
			$(
				$(#[$pubcdoc])*
				$pubcname, $pubcval
			)*
		}

		impl std::fmt::Display for $tname {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Display::fmt(&self.0, f) // delegate
			}
		}
	};
}

/// Declares the type of a constant for a window message,
/// convertible to [`WM`](crate::co::WM) constant type.
/// Optionally declares multiple public constant values.
macro_rules! const_type_wm {
	(
		$tname:ident,
		$(#[$tdoc:meta])*
		->
		$(
			$(#[$pubcdoc:meta])*
			$pubcname:ident, $pubcval:expr
		)*
	) => {
		const_type! {
			$tname, u32,
			$(#[$tdoc])*
			->
			$(
				$(#[$pubcdoc])*
				$pubcname, $pubcval
			)*
		}

		impl From<$tname> for WM {
			fn from(v: $tname) -> Self {
				Self(v.0)
			}
		}
	};
}

/// Declares the type of a constant for a WM_COMMAND notification code,
/// convertible to [`CMD`](crate::co::CMD) constant type.
/// Optionally declares multiple public constant values.
macro_rules! const_type_cmd {
	(
		$tname:ident,
		$(#[$tdoc:meta])*
		->
		$(
			$(#[$pubcdoc:meta])*
			$pubcname:ident, $pubcval:expr
		)*
	) => {
		const_type! {
			$tname, u16,
			$(#[$tdoc])*
			->
			$(
				$(#[$pubcdoc])*
				$pubcname, $pubcval
			)*
		}

		impl From<$tname> for CMD {
			fn from(v: $tname) -> Self {
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
		$tname:ident,
		$(#[$tdoc:meta])*
		->
		$(
			$(#[$pubcdoc:meta])*
			$pubcname:ident, $pubcval:expr
		)*
	) => {
		const_type! {
			$tname, i32,
			$(#[$tdoc])*
			->
			$(
				$(#[$pubcdoc])*
				$pubcname, $pubcval
			)*
		}

		impl From<$tname> for NM {
			fn from(v: $tname) -> Self {
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
		$tname:ident,
		$(#[$tdoc:meta])*
		->
		$(
			$(#[$pubcdoc:meta])*
			$pubcname:ident, $pubcval:expr
		)*
	) => {
		const_type! {
			$tname, u32,
			$(#[$tdoc])*
			->
			$(
				$(#[$pubcdoc])*
				$pubcname, $pubcval
			)*
		}

		impl From<$tname> for WS {
			fn from(v: $tname) -> Self {
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
		$tname:ident,
		$(#[$tdoc:meta])*
		->
		$(
			$(#[$pubcdoc:meta])*
			$pubcname:ident, $pubcval:expr
		)*
	) => {
		const_type! {
			$tname, u32,
			$(#[$tdoc])*
			->
			$(
				$(#[$pubcdoc])*
				$pubcname, $pubcval
			)*
		}

		impl From<$tname> for WS_EX {
			fn from(v: $tname) -> Self {
				Self(v.0)
			}
		}
	};
}
