#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::ffi_types::{HRES, PCVOID};
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;

/// A pointer to a COM virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ComPtr(pub(crate) *mut *mut IUnknownVT);

impl ComPtr {
	pub(crate) const fn null() -> Self {
		ComPtr(std::ptr::null_mut())
	}
}

/// Any
/// [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal)
/// object, which encapsulates a COM interface pointer.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub trait ComInterface: From<ComPtr> {
	/// The COM interface ID.
	const IID: co::IID;
}

//------------------------------------------------------------------------------

/// [`IUnknown`](crate::IUnknown) virtual table, base to all COM virtual tables.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
#[repr(C)]
pub struct IUnknownVT {
	pub QueryInterface: fn(ComPtr, PCVOID, *mut ComPtr) -> HRES,
	pub AddRef: fn(ComPtr) -> u32,
	pub Release: fn(ComPtr) -> u32,
}

/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// COM interface over [`IUnknownVT`](crate::vt::IUnknownVT). It's the base to
/// all COM interfaces.
///
/// The `clone` method calls
/// [`AddRef`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
/// internally.
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub struct IUnknown(ComPtr);

impl_iunknown!(IUnknown, "00000000-0000-0000-c000-000000000046");

/// This trait is enabled with the `ole` feature, and provides methods for
/// [`IUnknown`](crate::IUnknown).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "ole")))]
pub trait ole_IUnknown: ComInterface + Clone {
	/// Returns the pointer to the underlying COM virtual table.
	#[must_use]
	unsafe fn ptr(&self) -> ComPtr;

	/// [`IUnknown::QueryInterface`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void))
	/// method.
	#[must_use]
	fn QueryInterface<T>(&self) -> HrResult<T>
		where T: ComInterface,
	{
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IUnknownVT);
			ok_to_hrresult(
				(vt.QueryInterface)(
					self.ptr(),
					&T::IID as *const _ as _,
					&mut ppv_queried,
				),
			)
		}.map(|_| T::from(ppv_queried))
	}
}
