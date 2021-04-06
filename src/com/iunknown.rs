#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::{ComVT, PPComVT};
use crate::ffi::PCVOID;
use crate::structs::{GUID, IID};

com_virtual_table! { IUnknownVT,
	/// [`IUnknownVT`](crate::IUnknown) is the base to all COM interface virtual
	/// tables.
	->
	0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046,
	QueryInterface, fn(PPComVT<Self>, PCVOID, *mut PPComVT<IUnknownVT>) -> u32
	AddRef, fn(PPComVT<Self>) -> u32
	Release, fn(PPComVT<Self>) -> u32
}

/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
/// interface is the base to all COM interfaces. Backed by
/// [`IUnknownVT`](crate::IUnknownVT) virtual table.
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

	/// [`IUnknown::QueryInterface`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void))
	/// method.
	pub fn QueryInterface<VT: ComVT, RetInterf: From<PPComVT<VT>>>(&self)
		-> WinResult<RetInterf>
	{
		let mut ppvQueried: PPComVT<VT> = std::ptr::null_mut();

		match co::ERROR(
			(unsafe { (**self.ppv).QueryInterface })(
				self.ppv,
				VT::IID().as_ref() as *const GUID as *const _,
				&mut ppvQueried
					as *mut PPComVT<VT>
					as *mut *mut _,
			)
		) {
			co::ERROR::S_OK => Ok(RetInterf::from(ppvQueried)),
			err => Err(err),
		}
	}
}
