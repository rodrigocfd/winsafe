#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::ffi::{HANDLE, HRESULT};
use crate::funcs::HRESULT_FROM_WIN32;
use crate::handles::HWND;

/// [`IModalWindow`](crate::shell::IModalWindow) virtual table.
#[repr(C)]
pub struct IModalWindowVT {
	pub IUnknownVT: IUnknownVT,
	pub Show: fn(ComPtr, HANDLE) -> HRESULT,
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
	fn Show(&self, hwnd_owner: HWND) -> WinResult<bool> {
		let hr = unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IModalWindowVT);
			(vt.Show)(self.ptr(), hwnd_owner.ptr)
		};
		match HRESULT_FROM_WIN32(hr) {
			co::ERROR::S_OK => Ok(true),
			co::ERROR::CANCELLED => Ok(false), // ordinary error, not a COM error
			_ => Err(co::ERROR(hr as _)),
		}
	}
}
