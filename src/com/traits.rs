use crate::com::iunknown::IUnknownVT;
use crate::structs::IID;

/// Pointer to pointer to `IUnknownVT`.
pub(in crate::com) type PPI = *mut *mut IUnknownVT;

/// Trait to any
/// [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal)
/// object, which encapsulates a COM interface pointer.
pub trait ComInterface: From<PPI> {
	/// The COM interface ID.
	const IID: IID;
}
