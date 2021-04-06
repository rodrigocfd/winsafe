#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::funcs::hr_to_winresult;
use crate::com::PPComVT;
use crate::com::shell::IModalWindow;
use crate::com::shell::vt::{IFileDialogVT, IModalWindowVT};

/// [`IFileDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialog)
/// interface. Backed by [`IFileDialogVT`](crate::shell::IFileDialogVT) virtual
/// table.
///
/// Inherits from:
/// * [`IModalWindow`](crate::shell::IModalWindow);
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls [`IUnknown::Release`](crate::IUnknown::Release) when the
/// object goes out of scope.
#[derive(Clone)]
pub struct IFileDialog {
	/// Methods of base interface
	/// [`IModalWindow`](crate::shell::IModalWindow).
	pub IModalWindow: IModalWindow,
}

impl From<PPComVT<IFileDialogVT>> for IFileDialog {
	fn from(ppv: PPComVT<IFileDialogVT>) -> Self {
		Self {
			IModalWindow: IModalWindow::from(ppv as PPComVT<IModalWindowVT>),
		}
	}
}

impl IFileDialog {
	unsafe fn ppv(&self) -> PPComVT<IFileDialogVT> {
		self.IModalWindow.IUnknown.ppv::<IFileDialogVT>()
	}

	/// [`IFileDialog::ClearClientData`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-clearclientdata)
	/// method.
	pub fn ClearClientData(&self) -> WinResult<()> {
		unsafe {
			hr_to_winresult( ((**self.ppv()).ClearClientData)(self.ppv()) )
		}
	}

	/// [`IFileDialog::Close`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-close)
	/// method.
	pub fn Close(&self, hr: co::ERROR) -> WinResult<()> {
		unsafe {
			hr_to_winresult( ((**self.ppv()).Close)(self.ppv(), hr.0 as i32) )
		}
	}

	/// [`IFileDialog::SetFileTypeIndex`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypeindex)
	/// method.
	pub fn SetFileTypeIndex(&self, iFileType: u32) -> WinResult<()> {
		unsafe {
			hr_to_winresult(
				((**self.ppv()).SetFileTypeIndex)(self.ppv(), iFileType),
			)
		}
	}
}
