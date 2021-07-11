#![allow(non_snake_case)]

use crate::com::shell::vt::{IFileDialogVT, IModalWindowVT};
use crate::com::traits::{ComInterface, PPI};
use crate::ffi::{BOOL, HANDLE, HRESULT};
use crate::structs::IID;

/// [`IFileSaveDialog`](crate::shell::IFileSaveDialog) virtual table.
pub struct IFileSaveDialogVT {
	pub IFileDialogVT: IFileDialogVT,
	pub SetSaveAsItem: fn(PPI, PPI) -> HRESULT,
	pub SetProperties: fn(PPI, PPI) -> HRESULT,
	pub SetCollectedProperties: fn(PPI, PPI, BOOL) -> HRESULT,
	pub GetProperties: fn(PPI, *mut PPI) -> HRESULT,
	pub ApplyProperties: fn(PPI, PPI, PPI, HANDLE, PPI) -> HRESULT,
}

/// [`IFileSaveDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifilesavedialog)
/// COM interface over
/// [`IFileSaveDialogVT`](crate::shell::vt::IFileSaveDialogVT). Inherits from
/// [`IFileDialog`](crate::shell::IFileDialog),
/// [`IModalWindow`](crate::shell::IModalWindow),
/// [`IUnknown`](crate::IUnknown).
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
/// let obj = CoCreateInstance::<shell::IFileSaveDialog>(
///     &shell::clsid::FileSaveDialog,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// ).unwrap();
/// ```
pub struct IFileSaveDialog  {
	pub(crate) ppvt: PPI,
}

impl_send_sync_fromppvt!(IFileSaveDialog);

impl ComInterface for IFileSaveDialog {
	const IID: IID = IID::new(0x84bccd23, 0x5fde, 0x4cdb, 0xaea4, 0xaf64b83d78ab);
}

macro_rules! impl_IFileSaveDialog {
	($name:ty, $vt:ty) => {
		impl $name {
			fn ifilesavedialog_vt(&self) -> &IFileSaveDialogVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`IFileSaveDialog::SetSaveAsItem`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifilesavedialog-setsaveasitem)
			/// method.
			pub fn SetSaveAsItem(&self, psi: IShellItem) -> WinResult<()> {
				hr_to_winresult(
					(self.ifilesavedialog_vt().SetSaveAsItem)(self.ppvt, psi.ppvt),
				)
			}
		}
	};
}

impl_IUnknown!(IFileSaveDialog, IFileSaveDialogVT);
impl_IModalWindow!(IFileSaveDialog, IFileSaveDialogVT);
impl_IFileDialog!(IFileSaveDialog, IFileSaveDialogVT);
impl_IFileSaveDialog!(IFileSaveDialog, IFileSaveDialogVT);
