#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::iunknown::{ComPtr, IUnknownT};
use crate::com::shell::ifiledialog::{IFileDialogT, IFileDialogVT};
use crate::com::shell::imodalwindow::IModalWindowT;
use crate::com::shell::ishellitem::IShellItem;
use crate::ffi::{BOOL, HANDLE, HRES};
use crate::privs::ok_to_hrresult;

/// [`IFileSaveDialog`](crate::shell::IFileSaveDialog) virtual table.
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
/// COM interface over
/// [`IFileSaveDialogVT`](crate::shell::vt::IFileSaveDialogVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::prelude::*;
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj = CoCreateInstance::<shell::IFileSaveDialog>(
///     &shell::clsid::FileSaveDialog,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct IFileSaveDialog(ComPtr);

impl_iunknown!(IFileSaveDialog, 0x84bccd23, 0x5fde, 0x4cdb, 0xaea4, 0xaf64b83d78ab);
impl IModalWindowT for IFileSaveDialog {}
impl IFileDialogT for IFileSaveDialog {}
impl IFileSaveDialogT for IFileSaveDialog {}

/// Exposes the [`IFileSaveDialog`](crate::shell::IFileSaveDialog) methods.
pub trait IFileSaveDialogT: IFileDialogT {
	/// [`IFileSaveDialog::SetSaveAsItem`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-setsaveasitem)
	/// method.
	fn SetSaveAsItem(&self, psi: IShellItem) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileSaveDialogVT);
			ok_to_hrresult((vt.SetSaveAsItem)(self.ptr(), psi.ptr()))
		}
	}
}
