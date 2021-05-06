#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::PPComVT;
use crate::ffi::{HRESULT, PCVOID};
use crate::privs::hr_to_winresult;

com_virtual_table! { IUnknownVT,
	/// [`IUnknown`](crate::IUnknown) virtual table.
	->
	0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046,
	QueryInterface, fn(PPComVT<Self>, PCVOID, *mut PPComVT<IUnknownVT>) -> HRESULT
	AddRef, fn(PPComVT<Self>) -> u32
	Release, fn(PPComVT<Self>) -> u32
}

macro_rules! IUnknown_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		$(#[$doc])*
		pub struct $name {
			ppvt_iunk: PPComVT<IUnknownVT>,
		}

		impl From<PPComVT<$vt>> for $name {
			fn from(ppvt: PPComVT<$vt>) -> Self {
				Self { ppvt_iunk: ppvt as PPComVT<IUnknownVT> } // converts a **vtbl to the interface object
			}
		}

		impl Drop for $name {
			fn drop(&mut self) {
				if !self.ppvt_iunk.is_null() {
					let count = unsafe { (**self.ppvt_iunk).Release }(self.ppvt_iunk); // call Release()
					if count == 0 {
						self.ppvt_iunk = std::ptr::null_mut();
					}
				}
			}
		}

		impl Clone for $name {
			fn clone(&self) -> Self {
				(unsafe { (**self.ppvt_iunk).AddRef })(self.ppvt_iunk); // call AddRef()
				Self { ppvt_iunk: self.ppvt_iunk }
			}
		}

		impl $name {
			/// Returns a pointer to a pointer to the underlying IUnknownVT
			/// converted to any virtual table.
			///
			/// This method is used internally by COM interface implementations.
			pub unsafe fn ppvt<T>(&self) -> PPComVT<T> {
				self.ppvt_iunk as PPComVT<T>
			}

			/// [`IUnknown::QueryInterface`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void))
			/// method.
			pub fn QueryInterface<VT: crate::com::ComVT, RetInterf: From<PPComVT<VT>>>(&self)
				-> WinResult<RetInterf>
			{
				let mut ppvQueried: PPComVT<VT> = std::ptr::null_mut();
				hr_to_winresult(
					(unsafe { (**self.ppvt_iunk).QueryInterface })(
						self.ppvt_iunk,
						crate::privs::ref_as_pcvoid(&VT::IID()),
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| RetInterf::from(ppvQueried))
			}
		}
	};
}

IUnknown_impl! {
	/// [`IUnknown`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)
	/// COM interface over [`IUnknownVT`](crate::IUnknownVT). It's the base to
	/// all COM interfaces.
	///
	/// The `clone` method calls
	/// [`AddRef`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-addref)
	/// internally.
	///
	/// Automatically calls
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IUnknown, IUnknownVT
}
