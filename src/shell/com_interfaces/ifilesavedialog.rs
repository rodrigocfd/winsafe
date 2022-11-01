#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::{BOOL, HANDLE, HRES};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{ole_IUnknown, shell_IFileDialog, shell_IModalWindow};
use crate::shell::decl::IShellItem;
use crate::vt::IFileDialogVT;

/// [`IFileSaveDialog`](crate::IFileSaveDialog) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct IFileSaveDialogVT {
	pub IFileDialogVT: IFileDialogVT,
	pub SetSaveAsItem: fn(ComPtr, ComPtr) -> HRES,
	pub SetProperties: fn(ComPtr, ComPtr) -> HRES,
	pub SetCollectedProperties: fn(ComPtr, ComPtr, BOOL) -> HRES,
	pub GetProperties: fn(ComPtr, *mut ComPtr) -> HRES,
	pub ApplyProperties: fn(ComPtr, ComPtr, ComPtr, HANDLE, ComPtr) -> HRES,
}

com_interface! { IFileSaveDialog: "shell";
	"84bccd23-5fde-4cdb-aea4-af64b83d78ab";
	/// [`IFileSaveDialog`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifilesavedialog)
	/// COM interface over [`IFileSaveDialogVT`](crate::vt::IFileSaveDialogVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, CoCreateInstance, IFileSaveDialog};
	///
	/// let obj = CoCreateInstance::<IFileSaveDialog>(
	///     &co::CLSID::FileSaveDialog,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # Ok::<_, co::HRESULT>(())
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
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_IFileSaveDialog: shell_IFileDialog {
	/// [`IFileSaveDialog::SetSaveAsItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-setsaveasitem)
	/// method.
	fn SetSaveAsItem(&self, psi: IShellItem) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IFileSaveDialogVT>();
			ok_to_hrresult((vt.SetSaveAsItem)(self.ptr(), psi.ptr()))
		}
	}
}
