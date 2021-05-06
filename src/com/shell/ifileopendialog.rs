#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::funcs::CoTaskMemFree;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::shell::{COMDLG_FILTERSPEC, IShellItem, IShellItemArray};
use crate::com::shell::vt::{
	IFileDialogVT,
	IFileOpenDialogVT,
	IModalWindowVT,
	IShellItemArrayVT,
	IShellItemVT,
};
use crate::funcs::HRESULT_FROM_WIN32;
use crate::handles::HWND;
use crate::privs::{hr_to_winresult, ref_as_pcvoid};
use crate::structs::GUID;
use crate::WString;

macro_rules! IFileOpenDialog_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IFileDialog_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IFileOpenDialog::GetResults`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getresults)
			/// method.
			pub fn GetResults(&self) -> WinResult<IShellItemArray> {
				let ppvt = unsafe { self.ppvt::<IFileOpenDialogVT>() };
				let mut ppvQueried: PPComVT<IShellItemArrayVT> = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						((**ppvt).GetResults)(
							ppvt,
							&mut ppvQueried as *mut _ as _,
						)
					},
				).map(|_| IShellItemArray::from(ppvQueried))
			}

			/// [`IFileOpenDialog::GetSelectedItems`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getselecteditems)
			/// method.
			pub fn GetSelectedItems(&self) -> WinResult<IShellItemArray> {
				let ppvt = unsafe { self.ppvt::<IFileOpenDialogVT>() };
				let mut ppvQueried: PPComVT<IShellItemArrayVT> = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						((**ppvt).GetSelectedItems)(
							ppvt,
							&mut ppvQueried as *mut _ as _,
						)
					},
				).map(|_| IShellItemArray::from(ppvQueried))
			}
		}
	};
}

IFileOpenDialog_impl! {
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
	IFileOpenDialog, IFileOpenDialogVT
}
