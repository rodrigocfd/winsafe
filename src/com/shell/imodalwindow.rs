#![allow(non_snake_case)]

macro_rules! pub_struct_IModalWindow {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::co;
		use crate::com::shell::vt::IModalWindowVT;
		use crate::funcs::HRESULT_FROM_WIN32;
		use crate::handles::HWND;

		pub_struct_IUnknown! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn imodalwindow_vt(&self) -> &IModalWindowVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

			/// [`IModalWindow::Show`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-imodalwindow-show)
			/// method.
			///
			/// Returns false if user clicked Cancel.
			pub fn Show(&self, hwndOwner: HWND) -> WinResult<bool> {
				let hr = (self.imodalwindow_vt().Show)(self.ppvt, hwndOwner.ptr);
				match HRESULT_FROM_WIN32(hr) {
					co::ERROR::S_OK => Ok(true),
					co::ERROR::CANCELLED => Ok(false), // ordinary error, not a COM error
					_ => Err(co::ERROR(hr as _)),
				}
			}
		}
	};
}

pub_struct_IModalWindow! {
	/// [`IModalWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-imodalwindow)
	/// COM interface over [`IModalWindowVT`](crate::shell::vt::IModalWindowVT).
	/// Inherits from [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IModalWindow, crate::com::shell::vt::IModalWindowVT
}
