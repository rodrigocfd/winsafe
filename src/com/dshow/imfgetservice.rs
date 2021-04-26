#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{ComVT, IUnknown, IUnknownVT, PPComVT};
use crate::com::dshow::vt::IMFGetServiceVT;
use crate::privs::{hr_to_winresult, ref_as_pcvoid};
use crate::structs::GUID;

/// [`IMFGetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfgetservice)
/// COM interface.
///
/// Virtual table: [`IMFGetServiceVT`](crate::dshow::vt::IMFGetServiceVT).
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IMFGetService {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<IMFGetServiceVT>> for IMFGetService {
	fn from(ppv: PPComVT<IMFGetServiceVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl IMFGetService {
	unsafe fn ppv(&self) -> PPComVT<IMFGetServiceVT> {
		self.IUnknown.ppv::<IMFGetServiceVT>()
	}

	/// [`IMFGetService::GetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfgetservice-getservice)
	/// method.
	pub fn GetService<VT: ComVT, RetInterf: From<PPComVT<VT>>>(&self,
		guidService: &GUID) -> WinResult<RetInterf>
	{
		let mut ppvQueried: PPComVT<VT> = std::ptr::null_mut();
		hr_to_winresult(
			unsafe {
				((**self.ppv()).GetService)(
					self.ppv(),
					ref_as_pcvoid(guidService),
					ref_as_pcvoid(&VT::IID()),
					&mut ppvQueried as *mut _ as *mut _,
				)
			},
		).map(|_| RetInterf::from(ppvQueried))
	}
}
