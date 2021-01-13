/// Declares multiple constant values for the given type.
macro_rules! const_type_values {
	(
		$name:ident,
		$($cname:ident, $cval:expr)*
	) => {
		impl $name {
			$( pub const $cname: Self = Self($cval); )*
		}
	};
}

/// Declares the type of a constant with some impls, but not Debug/Display.
macro_rules! const_type_no_debug_display {
	(
		$name:ident, $num:ty,
		$(#[$attr:meta])*
		$($cname:ident, $cval:expr)*
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
			fn from(n: $name) -> $num {
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

		// All const values.
		const_type_values! { $name,
			$($cname, $cval)*
		}
	};
}

/// Declares the type of a constant with some impls.
macro_rules! const_type {
	(
		$name:ident, $num:ty,
		$(#[$attr:meta])*
		$($cname:ident, $cval:expr)*
	) => {
		const_type_no_debug_display! {
			$name, $num,
			$(#[$attr])*
			#[derive(Debug)]
			$($cname, $cval)*
		}

		impl std::fmt::Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Display::fmt(&self.0, f) // delegate
			}
		}
	};
}
