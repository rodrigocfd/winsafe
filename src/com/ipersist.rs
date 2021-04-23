#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{ComVT, IUnknown, IUnknownVT, PPComVT};
use crate::com::funcs::hr_to_winresult;
use crate::ffi::{HRESULT, PVOID};
use crate::structs::{CLSID, IID};

com_virtual_table! { IPersistVT,
	/// [`IPersist`](crate::IPersist) virtual table.
	->
	0x0000010c, 0x0000, 0x0000, 0xc000, 0x000000000046,
	IUnknownVT, IUnknownVT

	GetClassID, fn(PPComVT<Self>, PVOID) -> HRESULT
}

/// [`IPersist`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist)
/// COM interface. Backed by [`IPersistVT`](crate::IPersistVT) virtual table.
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IPersist {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<IPersistVT>> for IPersist {
	fn from(ppv: PPComVT<IPersistVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl IPersist {
	unsafe fn ppv(&self) -> PPComVT<IPersistVT> {
		self.IUnknown.ppv::<IPersistVT>()
	}

	/// [`IPersist::GetClassID`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersist-getclassid)
	/// method.
	pub fn GetClassID(&self) -> WinResult<CLSID> {
		let mut clsid = CLSID::new(0, 0, 0, 0, 0);
		hr_to_winresult(
			unsafe {
				((**self.ppv()).GetClassID)(
					self.ppv(), &mut clsid as *mut _ as *mut _,
				)
			},
		).map(|_| clsid)
	}
}
