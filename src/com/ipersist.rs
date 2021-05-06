#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknownVT, PPComVT};
use crate::ffi::{HRESULT, PVOID};
use crate::privs::{hr_to_winresult, ref_as_pvoid};
use crate::structs::CLSID;

com_virtual_table! { IPersistVT,
	/// [`IPersist`](crate::IPersist) virtual table.
	->
	0x0000010c, 0x0000, 0x0000, 0xc000, 0x000000000046,
	IUnknownVT, IUnknownVT

	GetClassID, fn(PPComVT<Self>, PVOID) -> HRESULT
}

macro_rules! IPersist_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IPersist::GetClassID`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersist-getclassid)
			/// method.
			pub fn GetClassID(&self) -> WinResult<CLSID> {
				let ppvt = unsafe { self.ppvt::<IPersistVT>() };
				let mut clsid = CLSID::new(0, 0, 0, 0, 0);
				hr_to_winresult(
					unsafe { ((**ppvt).GetClassID)(ppvt, ref_as_pvoid(&mut clsid)) },
				).map(|_| clsid)
			}
		}
	};
}

IPersist_impl! {
	/// [`IPersist`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist)
	/// COM interface. Backed by [`IPersistVT`](crate::IPersistVT) virtual table.
	///
	/// Inherits from:
	/// * [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IPersist, IPersistVT
}
