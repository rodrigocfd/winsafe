#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::vts::*;

com_interface! { IFileSaveDialog: "84bccd23-5fde-4cdb-aea4-af64b83d78ab";
	/// [`IFileSaveDialog`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifilesavedialog)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// Saving a TXT file:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hparent: w::HWND; // initialized somewhere
	/// # let hparent = w::HWND::NULL;
	///
	/// let file_save = w::CoCreateInstance::<w::IFileSaveDialog>(
	///     &co::CLSID::FileSaveDialog,
	///     None::<&w::IUnknown>,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	///
	/// file_save.SetFileTypes(&[
	///     ("Text files", "*.txt"),
	///     ("All files", "*.*"),
	/// ])?;
	/// file_save.SetFileTypeIndex(1)?;
	/// file_save.SetDefaultExtension("txt")?;
	///
	/// if file_save.Show(&hparent)? {
	///     let chosen_file = file_save.GetResult()?
	///         .GetDisplayName(co::SIGDN::FILESYSPATH)?;
	///     println!("{}", chosen_file);
	/// }
	/// # w::HrResult::Ok(())
	/// ```
}

impl shell_IModalWindow for IFileSaveDialog {}
impl shell_IFileDialog for IFileSaveDialog {}
impl shell_IFileSaveDialog for IFileSaveDialog {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IFileSaveDialog`](crate::IFileSaveDialog).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IFileSaveDialog: shell_IFileDialog {
	/// [`IFileSaveDialog::ApplyProperties`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-applyproperties)
	/// method.
	fn ApplyProperties(
		&self,
		item: &impl shell_IShellItem,
		store: &impl oleaut_IPropertyStore,
		hwnd: &HWND,
		sink: Option<&IFileOperationProgressSink>,
	) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IFileSaveDialogVT>(self).ApplyProperties)(
				self.ptr(),
				item.ptr(),
				store.ptr(),
				hwnd.ptr(),
				sink.map_or(std::ptr::null_mut(), |s| s.ptr()),
			)
		})
	}

	fn_com_interface_get! { GetProperties: IFileSaveDialogVT => IPropertyStore;
		/// [`IFileSaveDialog::GetProperties`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-getproperties)
		/// method.
	}

	/// [`IFileSaveDialog::SetProperties`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-setproperties)
	/// method.
	fn SetProperties(&self, store: &impl oleaut_IPropertyStore) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IFileSaveDialogVT>(self).SetProperties)(self.ptr(), store.ptr())
		})
	}

	/// [`IFileSaveDialog::SetSaveAsItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-setsaveasitem)
	/// method.
	fn SetSaveAsItem(&self, item: &impl shell_IShellItem) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IFileSaveDialogVT>(self).SetSaveAsItem)(self.ptr(), item.ptr())
		})
	}
}
