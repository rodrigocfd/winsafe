#![allow(non_snake_case)]

use crate::co;
use crate::ffi_types::HANDLE;
use crate::ole::decl::{ComPtr, HrResult};
use crate::prelude::OleIUnknown;
use crate::user::decl::HWND;
use crate::vt::IUnknownVT;

/// [`IModalWindow`](crate::IModalWindow) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct IModalWindowVT {
	pub IUnknownVT: IUnknownVT,
	pub Show: fn(ComPtr, HANDLE) -> u32,
}

/// [`IModalWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-imodalwindow)
/// COM interface over [`IModalWindowVT`](crate::vt::IModalWindowVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub struct IModalWindow(ComPtr);

impl_iunknown!(IModalWindow, "b4db1657-70d7-485e-8e3e-6fcb5a5c1802");
impl ShellIModalWindow for IModalWindow {}

/// [`IModalWindow`](crate::IModalWindow) methods from `shell` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait ShellIModalWindow: OleIUnknown {
	/// [`IModalWindow::Show`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-imodalwindow-show)
	/// method.
	///
	/// Returns false if user clicked Cancel.
	fn Show(&self, hwnd_owner: HWND) -> HrResult<bool> {
		const CANCELLED: co::HRESULT = co::ERROR::CANCELLED.to_hresult();
		match co::HRESULT(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IModalWindowVT);
				(vt.Show)(self.ptr(), hwnd_owner.0)
			},
		) {
			co::HRESULT::S_OK => Ok(true),
			CANCELLED => Ok(false),
			e => Err(e),
		}
	}
}
