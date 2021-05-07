#![allow(non_snake_case)]

macro_rules! IUnknown_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::aliases::WinResult;
		use crate::com::vt::{ComVT, IUnknownVT, PPComVT};
		use crate::privs::{hr_to_winresult, ref_as_pcvoid};

		$(#[$doc])*
		pub struct $name {
			pub(crate) ppvt: PPComVT<IUnknownVT>,
		}

		impl From<PPComVT<$vt>> for $name {
			fn from(ppvt: PPComVT<$vt>) -> Self {
				Self { ppvt: ppvt as _ } // converts a **vtbl to **IUnknownVT
			}
		}

		impl Drop for $name {
			fn drop(&mut self) {
				if !self.ppvt.is_null() {
					let count = unsafe { (**self.ppvt).Release }(self.ppvt); // call Release()
					if count == 0 {
						self.ppvt = std::ptr::null_mut();
					}
				}
			}
		}

		impl Clone for $name {
			fn clone(&self) -> Self {
				(unsafe { (**self.ppvt).AddRef })(self.ppvt); // call AddRef()
				Self { ppvt: self.ppvt }
			}
		}

		impl $name {
			/// Returns the raw pointer to pointer to the COM virtual table.
			pub unsafe fn as_ptr(&self) -> PPComVT<$vt> {
				self.ppvt as PPComVT<_>
			}

			/// [`IUnknown::QueryInterface`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void))
			/// method.
			pub fn QueryInterface<VT: ComVT, RetInterf: From<PPComVT<VT>>>(&self)
				-> WinResult<RetInterf>
			{
				let mut ppvQueried: PPComVT<VT> = std::ptr::null_mut();
				hr_to_winresult(
					(unsafe { (**self.ppvt).QueryInterface })(
						self.ppvt,
						ref_as_pcvoid(&VT::IID()),
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
	IUnknown, crate::com::vt::IUnknownVT
}
