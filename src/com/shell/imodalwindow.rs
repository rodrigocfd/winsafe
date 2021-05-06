#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::shell::vt::IModalWindowVT;
use crate::funcs::HRESULT_FROM_WIN32;
use crate::handles::HWND;
use crate::privs::hr_to_winresult;

macro_rules! IModalWindow_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IModalWindow::Show`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-imodalwindow-show)
			/// method.
			///
			/// Returns false if user clicked Cancel.
			pub fn Show(&self, hwndOwner: HWND) -> WinResult<bool> {
				let ppvt = unsafe { self.ppvt::<IModalWindowVT>() };
				let hr = unsafe { ((**ppvt).Show)(ppvt, hwndOwner.ptr) };
				match HRESULT_FROM_WIN32(hr) {
					co::ERROR::S_OK => Ok(true),
					co::ERROR::CANCELLED => Ok(false), // ordinary error, not a COM error
					_ => Err(co::ERROR(hr as _)),
				}
			}
		}
	};
}

IModalWindow_impl! {
	/// [`IModalWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-imodalwindow)
	/// COM interface.
	///
	/// Virtual table: [`IModalWindowVT`](crate::shell::vt::IModalWindowVT).
	///
	/// Inherits from:
	/// * [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IModalWindow, IModalWindowVT
}
