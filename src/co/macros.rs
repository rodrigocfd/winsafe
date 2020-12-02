macro_rules! decl {
	($ty: ident, $num: ty, $comm: expr) => {
		#[doc=$comm]
		#[repr(C)]
		#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
		pub struct $ty($num); // declare the newtype constant

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