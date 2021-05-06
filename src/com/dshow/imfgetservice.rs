#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{ComVT, IUnknownVT, PPComVT};
use crate::com::dshow::vt::IMFGetServiceVT;
use crate::privs::{hr_to_winresult, ref_as_pcvoid};
use crate::structs::GUID;

macro_rules! IMFGetService_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IMFGetService::GetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfgetservice-getservice)
			/// method.
			pub fn GetService<VT: ComVT, RetInterf: From<PPComVT<VT>>>(&self,
				guidService: &GUID) -> WinResult<RetInterf>
			{
				let mut ppvQueried: PPComVT<VT> = std::ptr::null_mut();
				let ppvt = unsafe { self.ppvt::<IMFGetServiceVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).GetService)(
							ppvt,
							ref_as_pcvoid(guidService),
							ref_as_pcvoid(&VT::IID()),
							&mut ppvQueried as *mut _ as _,
						)
					},
				).map(|_| RetInterf::from(ppvQueried))
			}
		}
	};
}

IMFGetService_impl! {
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
	IMFGetService, IMFGetServiceVT
}
