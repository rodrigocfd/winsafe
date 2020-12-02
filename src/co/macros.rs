macro_rules! decl {
	($ty: ident, $num: ty, $comm: expr) => {
		#[doc=$comm]
		#[repr(C)]
		#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
		pub struct $ty($num); // declare the newtype constant

		// Conversions from/to underlying number.
		impl From<$num> for $ty {
			fn from(n: $num) -> $ty {
				$ty(n) // the type can be created from the number
			}
		}
		impl From<$ty> for $num {
			fn from(n: $ty) -> $num {
				n.0 // the number can be created from the type
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

macro_rules! priv_val {
	($ty: ident, $name: ident, $val: expr) => {
		pub const $name: $ty = $ty($val); // syntactic sugar to declare non-pub constants
	}
}

macro_rules! val {
	($ty: ident, $name: ident, $val: expr) => {
		pub const $name: $ty = $ty($val); // syntactic sugar to declare pub constants
	}
}