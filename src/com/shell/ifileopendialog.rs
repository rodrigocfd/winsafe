#![allow(non_snake_case)]

use crate::com::PPComVT;
use crate::com::shell::IFileDialog;
use crate::com::shell::vt::{IFileDialogVT, IFileOpenDialogVT};

/// [`IFileOpenDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileopendialog)
/// COM interface. Backed by [`IFileOpenDialogVT`](crate::shell::IFileOpenDialogVT)
/// virtual table.
///
/// Inherits from:
/// * [`IFileDialog`](crate::shell::IFileDialog);
/// * [`IModalWindow`](crate::shell::IModalWindow);
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj: shell::IFileOpenDialog = CoCreateInstance(
///     &shell::clsid::FileOpenDialog,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
#[derive(Clone)]
pub struct IFileOpenDialog {
	/// Methods of base interface [`IFileDialog`](crate::shell::IFileDialog).
	pub IFileDialog: IFileDialog,
}

impl From<PPComVT<IFileOpenDialogVT>> for IFileOpenDialog {
	fn from(ppv: PPComVT<IFileOpenDialogVT>) -> Self {
		Self {
			IFileDialog: IFileDialog::from(ppv as PPComVT<IFileDialogVT>),
		}
	}
}

impl IFileOpenDialog {
	unsafe fn ppv(&self) -> PPComVT<IFileOpenDialogVT> {
		self.IFileDialog.IModalWindow.IUnknown.ppv::<IFileOpenDialogVT>()
	}

}
