#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::funcs::CoTaskMemFree;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::shell::{COMDLG_FILTERSPEC, IShellItem};
use crate::com::shell::vt::{
	IFileDialogVT,
	IFileSaveDialogVT,
	IModalWindowVT,
	IShellItemVT,
};
use crate::funcs::HRESULT_FROM_WIN32;
use crate::handles::HWND;
use crate::privs::{hr_to_winresult, ref_as_pcvoid};
use crate::structs::GUID;
use crate::WString;

macro_rules! IFileSaveDialog_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IFileDialog_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IFileSaveDialog::SetSaveAsItem`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-setsaveasitem)
			/// method.
			pub fn SetSaveAsItem(&self, psi: IShellItem) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileSaveDialogVT>() };
				hr_to_winresult(
					unsafe { ((**ppvt).SetSaveAsItem)(ppvt, psi.ppvt()) },
				)
			}
		}
	};
}

IFileSaveDialog_impl! {
	/// [`IFileSaveDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifilesavedialog)
	/// COM interface.
	///
	/// Virtual table: [`IFileSaveDialogVT`](crate::shell::vt::IFileSaveDialogVT).
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
	IFileSaveDialog, IFileSaveDialogVT
}
