#![allow(non_snake_case)]

use crate::com::traits::{ComInterface, PPComVT};
use crate::ffi::{HRESULT, PCVOID};
use crate::structs::IID;

type PP = PPComVT<IUnknownVT>;

/// [`IUnknown`](crate::IUnknown) virtual table, base to all COM virtual tables.
pub struct IUnknownVT {
	pub QueryInterface: fn(PP, PCVOID, *mut PP) -> HRESULT,
	pub AddRef: fn(PP) -> u32,
	pub Release: fn(PP) -> u32,
}

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
pub struct IUnknown {
	pub(crate) ppvt: PPComVT<IUnknownVT>,
}

impl_send_sync_fromppvt!(IUnknown);

impl ComInterface for IUnknown {
	const IID: IID = IID::new(0x00000000, 0x0000, 0x0000, 0xc000, 0x000000000046);
}

macro_rules! impl_IUnknown {
	($name:ty, $vt:ty) => {
		use crate::aliases::WinResult;
		use crate::privs::hr_to_winresult;

		impl Drop for $name {
			fn drop(&mut self) {
				if !self.ppvt.is_null() {
					let count = (self.iunknown_vt().Release)(self.ppvt); // call Release()
					if count == 0 {
						self.ppvt = std::ptr::null_mut();
					}
				}
			}
		}

		impl Clone for $name {
			fn clone(&self) -> Self {
				(self.iunknown_vt().AddRef)(self.ppvt); // call AddRef()
				Self { ppvt: self.ppvt }
			}
		}

		impl $name {
			fn iunknown_vt(&self) -> &IUnknownVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// Returns the raw pointer to pointer to the COM virtual table.
			pub unsafe fn as_ptr(&self) -> PPComVT<$vt> {
				self.ppvt as _
			}

			/// [`IUnknown::QueryInterface`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-queryinterface(refiid_void))
			/// method.
			pub fn QueryInterface<T: ComInterface>(&self) -> WinResult<T> {
				let mut ppvQueried: PPComVT<IUnknownVT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.iunknown_vt().QueryInterface)(
						self.ppvt,
						&T::IID as *const _ as _,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| T::from(ppvQueried))
			}
		}
	};
}

impl_IUnknown!(IUnknown, IUnknownVT);
