#![allow(non_snake_case)]

use crate::IID;

/// Type alias to pointer to pointer to any COM virtual table.
pub type PPVtbl<T> = *mut *mut T;

/// Trait for any COM virtual table.
pub trait Vtbl {
	/// Returns the interface ID.
	fn IID() -> IID;
}