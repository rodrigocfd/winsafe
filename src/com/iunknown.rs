#![allow(non_snake_case)]

use crate::ffi::*;

/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// is the base to all COM interfaces.
#[repr(C)]
pub struct IUnknown(pub *mut *mut IUnknownVtbl);

#[repr(C)]
pub struct IUnknownVtbl {
	queryInterface: *mut Void,
	addRef: fn(*mut *mut IUnknownVtbl) -> u32,
	release: fn(*mut *mut IUnknownVtbl) -> u32,
}

impl IUnknown {
	pub fn AddRef(&self) -> u32 {
		let pfun = unsafe { (*(*self.0)).addRef };
		pfun(self.0)
	}

	pub fn Release(&self) -> u32 {
		let pfun = unsafe { (*(*self.0)).release };
		pfun(self.0)
	}
}