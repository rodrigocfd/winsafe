#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::{IUnknown, IUnknownVT, PPComVT};
use crate::com::shell::vt::IModalWindowVT;
use crate::funcs::HRESULT_FROM_WIN32;
use crate::handles::HWND;

/// [`IModalWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-imodalwindow)
/// COM interface.
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls [`IUnknown::Release`](crate::IUnknown::Release) when the
/// object goes out of scope.
#[derive(Clone)]
pub struct IModalWindow {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<IModalWindowVT>> for IModalWindow {
	fn from(ppv: PPComVT<IModalWindowVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl IModalWindow {
	unsafe fn ppv(&self) -> PPComVT<IModalWindowVT> {
		self.IUnknown.ppv::<IModalWindowVT>()
	}

	/// [`IModalWindow::Show`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-imodalwindow-show)
	/// method.
	///
	/// Returns false if user clicked Cancel.
	pub fn Show(&self, hwndOwner: HWND) -> WinResult<bool> {
		let code = unsafe { ((**self.ppv()).Show)(self.ppv(), hwndOwner.ptr) };
		match HRESULT_FROM_WIN32(code) {
			co::ERROR::S_OK => Ok(true),
			co::ERROR::CANCELLED => Ok(false),
			_ => Err(co::ERROR(code as u32)),
		}
	}
}
