#![allow(non_snake_case)]

use crate::com::{PPVtbl, Vtbl};
use crate::ffi::PVOID;
use crate::structs::IID;

vtbl_type! {
	/// [`IUnknown`](crate::IUnknown) virtual table.
	IUnknownVtbl,
	0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046,

	QueryInterface, fn(PPVtbl<Self>, PVOID, *mut PPVtbl<IUnknownVtbl>)
	AddRef, fn(PPVtbl<Self>) -> u32
	Release, fn(PPVtbl<Self>) -> u32
}

//------------------------------------------------------------------------------

/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// interface is the base to all COM interfaces.
///
/// Automatically calls [`Release`](crate::IUnknown::Release) when the object
/// goes out of scope.
pub struct IUnknown {
	vtbl: PPVtbl<IUnknownVtbl>,
}

impl From<PPVtbl<IUnknownVtbl>> for IUnknown {
	fn from(ppv: PPVtbl<IUnknownVtbl>) -> Self {
		Self { vtbl: ppv }
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
	/// This method is used internally by COM interface implementations, and may
	/// cause segmentation faults. Don't use unless you know what you're doing.
	pub unsafe fn ppv<T>(&self) -> PPVtbl<T> {
		self.vtbl as PPVtbl<T>
	}

	/// [`IUnknown::AddRef`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
	/// method.
	///
	/// This method increments the internal COM reference counter, and will cause
	/// a memory leak if not paired with a [`Release`](crate::IUnknown::Release)
	/// call. Don't use unless you know what you're doing.
	pub unsafe fn AddRef(&self) -> u32 {
		((*(*self.vtbl)).AddRef)(self.vtbl)
	}

	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// method.
	///
	/// Can be called any number of times, will actually release only while the
	/// internal ref count is greater than zero.
	///
	/// This method is automatically called when the object goes out of scope, so
	/// you don't need to call it manually. But note that the last call to
	/// [`CoUninitialize`](crate::CoUninitialize) must happen after `Release` is
	/// called.
	pub fn Release(&mut self) -> u32 {
		if self.vtbl.is_null() {
			0
		} else {
			let refCount = unsafe { (*(*self.vtbl)).Release }(self.vtbl);
			if refCount == 0 {
				self.vtbl = std::ptr::null_mut();
			}
			refCount
		}
	}
}