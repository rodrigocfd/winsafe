#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::co;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::HANDLE;
use crate::handles::HWND;

/// [`IModalWindow`](crate::shell::IModalWindow) virtual table.
#[repr(C)]
pub struct IModalWindowVT {
	pub IUnknownVT: IUnknownVT,
	pub Show: fn(ComPtr, HANDLE) -> u32,
}

/// [`IModalWindow`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-imodalwindow)
/// COM interface over [`IModalWindowVT`](crate::shell::vt::IModalWindowVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IModalWindow(ComPtr);

impl_iunknown!(IModalWindow, 0xb4db1657, 0x70d7, 0x485e, 0x8e3e, 0x6fcb5a5c1802);
impl IModalWindowT for IModalWindow {}

/// Exposes the [`IModalWindow`](crate::shell::IModalWindow) methods.
pub trait IModalWindowT: IUnknownT {
	/// [`IModalWindow::Show`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-imodalwindow-show)
	/// method.
	///
	/// Returns false if user clicked Cancel.
	fn Show(&self, hwnd_owner: HWND) -> HrResult<bool> {
		match co::ERROR(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IModalWindowVT);
				(vt.Show)(self.ptr(), hwnd_owner.0)
			},
		) {
			co::ERROR::SUCCESS => Ok(true),
			co::ERROR::CANCELLED => Ok(false),
			e => Err(e.to_hresult()),
		}
	}
}
