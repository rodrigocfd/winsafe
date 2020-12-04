#![allow(non_snake_case)]

use crate::ffi::Void;

type PPVtbl = *const *const IUnknownVtbl;

/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// interface is the base to all COM interfaces.
pub struct IUnknown {
	vtbl: PPVtbl,
}

#[repr(C)]
pub struct IUnknownVtbl {
	QueryInterface: *const Void,
	AddRef: fn(PPVtbl) -> u32,
	Release: fn(PPVtbl) -> u32,
}

impl From<PPVtbl> for IUnknown {
	/// Creates a new object from a pointer to a pointer to its virtual table.
	fn from(ppv: PPVtbl) -> Self {
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
	pub unsafe fn ppv<T>(&self) -> *const *const T {
		self.vtbl as *const *const T
	}

	/// [`IUnknown::AddRef`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
	/// method.
	pub fn AddRef(&self) -> u32 {
		let pfun = unsafe { (*(*self.vtbl)).AddRef };
		pfun(self.vtbl)
	}

	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// method.
	///
	/// Can be called any number of times, will actually release only while the
	/// internal ref count is greater than zero.
	///
	/// This method will be automatically called by the destructor, but note that
	/// this must happen **before** the last
	/// [`CoUninitialize`](crate::CoUninitialize) call.
	pub fn Release(&mut self) -> u32 {
		if self.vtbl.is_null() {
			0
		} else {
			let ptrFun = unsafe { (*(*self.vtbl)).Release };
			let refCount = ptrFun(self.vtbl);

			if refCount == 0 {
				self.vtbl = std::ptr::null();
			}
			refCount
		}
	}
}