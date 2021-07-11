use crate::com::IUnknownVT;
use crate::structs::IID;

/// Trait to any
/// [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal)
/// object, which encapsulates a COM interface pointer.
pub trait ComInterface: From<PPComVT<IUnknownVT>> {
	/// The COM interface ID.
	const IID: IID;
}

/// Type alias to pointer to pointer to a
/// [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal)
/// virtual table.
pub type PPComVT<T> = *mut *mut T;
