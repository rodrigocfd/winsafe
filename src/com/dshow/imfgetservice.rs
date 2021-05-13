#![allow(non_snake_case)]

macro_rules! pub_struct_IMFGetService {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::dshow::vt::IMFGetServiceVT;
		use crate::structs::GUID;

		pub_struct_IUnknown! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn imfgetservice_vt(&self) -> &IMFGetServiceVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IMFGetService::GetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfgetservice-getservice)
			/// method.
			pub fn GetService<VT: ComVT, RetInterf: From<PPComVT<VT>>>(&self,
				guidService: &GUID) -> WinResult<RetInterf>
			{
				let mut ppvQueried: PPComVT<VT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.imfgetservice_vt().GetService)(
						self.ppvt,
						ref_as_pcvoid(guidService),
						ref_as_pcvoid(&VT::IID()),
						&mut ppvQueried as *mut _ as _,
					),
				).map(|_| RetInterf::from(ppvQueried))
			}
		}
	};
}

pub_struct_IMFGetService! {
	/// [`IMFGetService`](https://docs.microsoft.com/en-us/windows/win32/api/mfidl/nn-mfidl-imfgetservice)
	/// COM interface over
	/// [`IMFGetServiceVT`](crate::dshow::vt::IMFGetServiceVT). Inherits from
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IMFGetService, crate::com::dshow::vt::IMFGetServiceVT
}
