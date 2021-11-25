use std::fmt;
use std::hash::Hash;
use std::ops;

/// Any native Windows constant.
pub trait NativeConstant: Default + Copy + Clone + Eq + PartialEq + Hash
	+ From<Self::Concrete> + Into<Self::Concrete>
	+ fmt::LowerHex + fmt::UpperHex + fmt::Binary + fmt::Octal
	+ ops::BitAnd + ops::BitAndAssign
	+ ops::BitOr + ops::BitOrAssign
	+ ops::BitXor + ops::BitXorAssign
	+ ops::Not
{
	/// The underlying concrete type for this constant type.
	type Concrete;

	/// Tells whether other bitflag style is present.
	///
	/// Equivalent to `(val & other) != 0`.
	fn has(&self, other: Self) -> bool;
}
