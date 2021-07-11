#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPComVT};
use crate::ffi::{HRESULT, PCVOID};
use crate::structs::IID;

type PP = PPComVT<IUnknownVT>;

/// [`IMFGetService`](crate::dshow::IMFGetService) virtual table.
pub struct IMFGetServiceVT {
	pub IUnknownVT: IUnknownVT,
	pub GetService: fn(PP, PCVOID, PCVOID, *mut PP) -> HRESULT,
}

/// [`IMFGetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfgetservice)
/// COM interface over
/// [`IMFGetServiceVT`](crate::dshow::vt::IMFGetServiceVT). Inherits from
/// [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object g
pub struct IMFGetService {
	pub(crate) ppvt: PPComVT<IUnknownVT>,
}

impl_send_sync_fromppvt!(IMFGetService);

impl ComInterface for IMFGetService {
	const IID: IID = IID::new(0xfa993888, 0x4383, 0x415a, 0xa930, 0xdd472a8cf6f7);
}

macro_rules! impl_IMFGetService {
	($name:ty, $vt:ty) => {
		use crate::structs::GUID;

		impl $name {
			fn imfgetservice_vt(&self) -> &IMFGetServiceVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IMFGetService::GetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfgetservice-getservice)
			/// method.
			pub fn GetService<T: ComInterface>(&self,
				guidService: &GUID) -> WinResult<T>
			{
				let mut ppvQueried: PPComVT<IUnknownVT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.imfgetservice_vt().GetService)(
						self.ppvt,
						guidService as *const _ as _,
						&T::IID as *const _ as _,
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| T::from(ppvQueried))
			}
		}
	};
}

impl_IUnknown!(IMFGetService, IMFGetServiceVT);
impl_IMFGetService!(IMFGetService, IMFGetServiceVT);
