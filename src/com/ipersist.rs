#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPComVT};
use crate::ffi::{HRESULT, PVOID};
use crate::structs::IID;

type PP = PPComVT<IUnknownVT>;

/// [`IPersist`](crate::IPersist) virtual table.
pub struct IPersistVT {
	pub IUnknownVT: IUnknownVT,
	pub GetClassID: fn(PP, PVOID) -> HRESULT,
}

/// [`IPersist`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nn-objidl-ipersist)
/// COM interface over [`IPersistVT`](crate::IPersistVT). Inherits from
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IPersist {
	pub(crate) ppvt: PPComVT<IUnknownVT>,
}

impl_send_sync_fromppvt!(IPersist);

impl ComInterface for IPersist {
	const IID: IID = IID::new(0x0000010c, 0x0000, 0x0000, 0xc000, 0x000000000046);
}

macro_rules! impl_IPersist {
	($name:ty, $vt:ty) => {
		use crate::structs::CLSID;

		impl $name {
			fn ipersist_vt(&self) -> &IPersistVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IPersist::GetClassID`](https://docs.microsoft.com/en-us/windows/win32/api/objidl/nf-objidl-ipersist-getclassid)
			/// method.
			pub fn GetClassID(&self) -> WinResult<CLSID> {
				let mut clsid = CLSID::new(0, 0, 0, 0, 0);
				hr_to_winresult(
					(self.ipersist_vt().GetClassID)(
						self.ppvt,
						&mut clsid as *mut _ as _,
					),
				).map(|_| clsid)
			}
		}
	};
}

impl_IUnknown!(IPersist, IPersistVT);
impl_IPersist!(IPersist, IPersistVT);
