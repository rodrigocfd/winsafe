#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::funcs::hr_to_winresult;
use crate::com::PPComVT;
use crate::com::shell::{IFileDialog, IShellItem};
use crate::com::shell::vt::{IFileDialogVT, IFileSaveDialogVT};

/// [`IFileSaveDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifilesavedialog)
/// COM interface. Backed by
/// [`IFileSaveDialogVT`](crate::shell::vt::IFileSaveDialogVT) virtual table.
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
/// let obj: shell::IFileSaveDialog = CoCreateInstance(
///     &shell::clsid::FileSaveDialog,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
#[derive(Clone)]
pub struct IFileSaveDialog {
	/// Methods of base interface [`IFileDialog`](crate::shell::IFileDialog).
	pub IFileDialog: IFileDialog,
}

impl From<PPComVT<IFileSaveDialogVT>> for IFileSaveDialog {
	fn from(ppv: PPComVT<IFileSaveDialogVT>) -> Self {
		Self {
			IFileDialog: IFileDialog::from(ppv as PPComVT<IFileDialogVT>),
		}
	}
}

impl IFileSaveDialog {
	unsafe fn ppv(&self) -> PPComVT<IFileSaveDialogVT> {
		self.IFileDialog.IModalWindow.IUnknown.ppv::<IFileSaveDialogVT>()
	}

	/// [`IFileSaveDialog::SetSaveAsItem`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-setsaveasitem)
	/// method.
	pub fn SetSaveAsItem(&self, psi: IShellItem) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetSaveAsItem)(
					self.ppv(), psi.IUnknown.ppv(),
				)
			},
		)
	}
}
