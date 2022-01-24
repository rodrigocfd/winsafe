#![allow(non_snake_case)]

use crate::co;
use crate::{CoCreateInstance, CLSID};
use crate::ffi_types::{BOOL, HANDLE, HRES};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{OleIUnknown, ShellIFileDialog, ShellIModalWindow};
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

/// [`IFileSaveDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifilesavedialog)
/// COM interface over [`IFileSaveDialogVT`](crate::vt::IFileSaveDialogVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{CLSID, co, CoCreateInstance, IFileSaveDialog};
///
/// let obj = CoCreateInstance::<IFileSaveDialog>(
///     &CLSID::FileSaveDialog,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// # Ok::<_, co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub struct IFileSaveDialog(ComPtr);

impl_iunknown!(IFileSaveDialog, 0x84bccd23, 0x5fde, 0x4cdb, 0xaea4, 0xaf64b83d78ab);
impl ShellIModalWindow for IFileSaveDialog {}
impl ShellIFileDialog for IFileSaveDialog {}
impl ShellIFileSaveDialog for IFileSaveDialog {}

/// [`IFileSaveDialog`](crate::IFileSaveDialog) methods from `shell` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait ShellIFileSaveDialog: ShellIFileDialog {
	/// Calls [`CoCreateInstance`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance)
	/// function to create a new file save dialog.
	///
	/// To customize CLSCTX and such, use [`CoCreateInstance`](crate::CoCreateInstance) function
	/// directly.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IFileSaveDialog;
	///
	/// let fod = IFileSaveDialog::new()?;
	/// // setup file save dialog to your taste
	/// let _ = fod.Show()?;
	/// ```
	fn new() -> HrResult<IFileSaveDialog> {
		CoCreateInstance::<IFileSaveDialog>(
			&CLSID::FileSaveDialog,
			None,
			co::CLSCTX::INPROC_SERVER,
		)
	}

	/// [`IFileSaveDialog::SetSaveAsItem`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-setsaveasitem)
	/// method.
	fn SetSaveAsItem(&self, psi: IShellItem) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileSaveDialogVT);
			ok_to_hrresult((vt.SetSaveAsItem)(self.ptr(), psi.ptr()))
		}
	}
}
