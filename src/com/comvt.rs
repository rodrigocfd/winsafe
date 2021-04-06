#![allow(non_snake_case)]

use crate::IID;

/// Type alias to pointer to pointer to a
/// [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal)
/// virtual table.
pub type PPComVT<T> = *mut *mut T;

/// Trait for any
/// [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal)
/// virtual table.
pub trait ComVT {
	/// Returns the COM interface ID.
	fn IID() -> IID;
}
