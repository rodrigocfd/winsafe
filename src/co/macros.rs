// Declares the type of a constant with some impls.
// We use doc as string because of a bug in rust-analyzer:
// https://stackoverflow.com/q/65112749/6923555
macro_rules! decl {
	($ty: ident, $num: ty, $comm: expr) => {
		#[doc=$comm]
		#[repr(C)]
		#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
		pub struct $ty($num); // declare the newtype constant

		// Conversions from/to underlying number.
		impl From<$num> for $ty {
			fn from(n: $num) -> Self {
				Self(n) // the type can be created from the number
			}
		}
		impl From<$ty> for $num {
			fn from(n: $ty) -> $num {
				n.0 // the number can be created from the type
			}
		}

		// Formatters.
		impl std::fmt::Display for $ty {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::Display::fmt(&self.0, f)
			}
		}
		impl std::fmt::LowerHex for $ty {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::LowerHex::fmt(&self.0, f)
			}
		}
		impl std::fmt::UpperHex for $ty {
			fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
				std::fmt::UpperHex::fmt(&self.0, f)
			}
		}

		// Bitflag operations.
		impl std::ops::BitAnd for $ty {
			type Output = $ty;
			fn bitand(self, rhs: Self) -> Self::Output {
				Self(self.0 & rhs.0)
			}
		}
		impl std::ops::BitAndAssign for $ty {
			fn bitand_assign(&mut self, rhs: Self) {
				*self = Self(self.0 & rhs.0);
			}
		}
		impl std::ops::BitOr for $ty {
			type Output = $ty;
			fn bitor(self, rhs: Self) -> Self {
				Self(self.0 | rhs.0)
			}
		}
		impl std::ops::BitOrAssign for $ty {
			fn bitor_assign(&mut self, rhs: Self) {
				*self = Self(self.0 | rhs.0);
			}
		}
		impl std::ops::BitXor for $ty {
			type Output = $ty;
			fn bitxor(self, rhs: Self) -> Self::Output {
				Self(self.0 ^ rhs.0)
			}
		}
		impl std::ops::BitXorAssign for $ty {
			fn bitxor_assign(&mut self, rhs: Self) {
				*self = Self(self.0 ^ rhs.0);
			}
		}
		impl std::ops::Not for $ty {
			type Output = $ty;
			fn not(self) -> Self::Output {
				Self(!self.0)
			}
		}
	};
}

// Syntactic sugar to declare non-pub constant value.
macro_rules! priv_val {
	($name: ident, $val: expr) => {
		pub const $name: Self = Self($val);
	}
}

// Syntactic sugar to declare pub constant value.
macro_rules! val {
	($name: ident, $val: expr) => {
		pub const $name: Self = Self($val);
	}
}