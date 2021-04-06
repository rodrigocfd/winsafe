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
/// Automatically calls [`Release`](crate::IUnknown::Release) when the object
/// goes out of scope.
pub struct IUnknown {
	vtbl: PPComVT<IUnknownVT>,
}

impl From<PPComVT<IUnknownVT>> for IUnknown {
	fn from(ppv: PPComVT<IUnknownVT>) -> Self {
		Self { vtbl: ppv } // converts a **vtbl to the interface object
	}
}

impl Drop for IUnknown {
	fn drop(&mut self) {
		self.Release();
	}
}

impl IUnknown {
	/// Returns a pointer to a pointer to the underlying COM virtual table.
	///
	/// This method is used internally by COM interface implementations.
	pub unsafe fn ppv<T>(&self) -> PPComVT<T> {
		self.vtbl as PPComVT<T>
	}

	/// [`IUnknown::AddRef`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
	/// method.
	///
	/// **Note:** This method increments the internal COM reference counter, and
	/// will cause a memory leak if not paired with a
	/// [`Release`](crate::IUnknown::Release) call.
	pub unsafe fn AddRef(&self) -> u32 {
		((**self.vtbl).AddRef)(self.vtbl)
	}

	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// method.
	///
	/// Automatically called when the object goes out of scope, so you don't need
	/// to call it manually. But note that the last call to
	/// [`CoUninitialize`](crate::CoUninitialize) must happen after `Release` is
	/// called.
	///
	/// This method can be called any number of times, it will be effectively
	/// fired only while the internal ref count is greater than zero.
	pub fn Release(&mut self) -> u32 {
		if self.vtbl.is_null() {
			0
		} else {
			let refCount = unsafe { (**self.vtbl).Release }(self.vtbl);
			if refCount == 0 {
				self.vtbl = std::ptr::null_mut();
			}
			refCount
		}
	}
}
