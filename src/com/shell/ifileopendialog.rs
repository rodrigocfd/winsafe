#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::PPComVT;
use crate::com::shell::{IFileDialog, IShellItemArray};
use crate::com::shell::vt::{IFileDialogVT, IFileOpenDialogVT, IShellItemArrayVT};
use crate::privs::hr_to_winresult;

/// [`IFileOpenDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileopendialog)
/// COM interface.
///
/// Virtual table: [`IFileOpenDialogVT`](crate::shell::vt::IFileOpenDialogVT).
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

	/// [`IFileOpenDialog::GetResults`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getresults)
	/// method.
	pub fn GetResults(&self) -> WinResult<IShellItemArray> {
		let mut ppvQueried: PPComVT<IShellItemArrayVT> = std::ptr::null_mut();
		hr_to_winresult(
			unsafe {
				((**self.ppv()).GetResults)(
					self.ppv(),
					&mut ppvQueried as *mut _ as *mut _,
				)
			},
		).map(|_| IShellItemArray::from(ppvQueried))
	}

	/// [`IFileOpenDialog::GetSelectedItems`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getselecteditems)
	/// method.
	pub fn GetSelectedItems(&self) -> WinResult<IShellItemArray> {
		let mut ppvQueried: PPComVT<IShellItemArrayVT> = std::ptr::null_mut();
		hr_to_winresult(
			unsafe {
				((**self.ppv()).GetSelectedItems)(
					self.ppv(),
					&mut ppvQueried as *mut _ as *mut _,
				)
			},
		).map(|_| IShellItemArray::from(ppvQueried))
	}
}
