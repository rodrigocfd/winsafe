#![allow(non_snake_case)]

use crate::com::{ComVT, PPComVT};
use crate::ffi::PVOID;
use crate::structs::IID;

/// [`IUnknownVtbl`](crate::IUnknown) is the base to all COM interface
/// virtual tables.
#[repr(C)]
pub struct IUnknownVT {
	pub QueryInterface: fn(PPComVT<Self>, PVOID, *mut PPComVT<IUnknownVT>),
	pub AddRef: fn(PPComVT<Self>) -> u32,
	pub Release: fn(PPComVT<Self>) -> u32,
}

impl_iid!(IUnknownVT, 0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046);

//------------------------------------------------------------------------------

/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// interface is the base to all COM interfaces.
///
/// The `clone` method calls
/// [`AddRef`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
/// internally.
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IUnknown {
	ppv: PPComVT<IUnknownVT>,
}

impl From<PPComVT<IUnknownVT>> for IUnknown {
	fn from(ppv: PPComVT<IUnknownVT>) -> Self {
		Self { ppv } // converts a **vtbl to the interface object
	}
}

impl Drop for IUnknown {
	fn drop(&mut self) {
		if !self.ppv.is_null() {
			let count = unsafe { (**self.ppv).Release }(self.ppv); // call Release()
			if count == 0 {
				self.ppv = std::ptr::null_mut();
			}
		}
	}
}

impl Clone for IUnknown {
	fn clone(&self) -> Self {
		(unsafe { (**self.ppv).AddRef })(self.ppv); // call AddRef()
		Self { ppv: self.ppv }
	}
}

impl IUnknown {
	/// Returns a pointer to a pointer to the underlying COM virtual table.
	///
	/// This method is used internally by COM interface implementations.
	pub unsafe fn ppv<T>(&self) -> PPComVT<T> {
		self.ppv as PPComVT<T>
	}
}
