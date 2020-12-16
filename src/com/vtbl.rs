#![allow(non_snake_case)]

use crate::IID;

/// Type alias to pointer to pointer to any
/// [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal)
/// virtual table.
pub type PPVtbl<T> = *mut *mut T;

/// Trait for any
/// [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal)
/// virtual table.
pub trait Vtbl {
	/// Returns the interface ID.
	fn IID() -> IID;
}