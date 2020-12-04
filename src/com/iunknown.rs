#![allow(non_snake_case)]

use crate::ffi::*;

/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// is the base to all COM interfaces.
pub struct IUnknown {
	vtbl: *mut *mut IUnknownVtbl,
}

#[repr(C)]
pub struct IUnknownVtbl {
	queryInterface: *mut Void,
	addRef: fn(*mut *mut IUnknownVtbl) -> u32,
	release: fn(*mut *mut IUnknownVtbl) -> u32,
}

impl From<*mut *mut IUnknownVtbl> for IUnknown {
	fn from(ppv: *mut *mut IUnknownVtbl) -> Self {
		Self { vtbl: ppv }
	}
}

impl IUnknown {
	pub(crate) fn ppv<T>(&self) -> *mut *mut T {
		self.vtbl as *mut *mut T
	}

	/// [IUnknown::AddRef](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
	/// method.
	pub fn AddRef(&self) -> u32 {
		let pfun = unsafe { (*(*self.vtbl)).addRef };
		pfun(self.vtbl)
	}

	/// [IUnknown::Release](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// method.
	pub fn Release(&self) -> u32 {
		let pfun = unsafe { (*(*self.vtbl)).release };
		pfun(self.vtbl)
	}
}