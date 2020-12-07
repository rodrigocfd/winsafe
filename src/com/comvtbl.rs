#![allow(non_snake_case)]

use crate::IID;

/// Trait for any COM virtual table.
pub trait ComVtbl {
	/// Returns the interface ID.
	fn IID() -> IID;
}