#![allow(non_snake_case)]

use crate::com::shell::vt::{IFileDialogVT, IModalWindowVT};
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::HRESULT;
use crate::structs::IID;

/// [`IFileOpenDialog`](crate::shell::IFileOpenDialog) virtual table.
pub struct IFileOpenDialogVT {
	pub IFileDialogVT: IFileDialogVT,
	pub GetResults: fn(PPVT, *mut PPVT) -> HRESULT,
	pub GetSelectedItems: fn(PPVT, *mut PPVT) -> HRESULT,
}

/// [`IFileOpenDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileopendialog)
/// COM interface over
/// [`IFileOpenDialogVT`](crate::shell::vt::IFileOpenDialogVT). Inherits from
/// [`IFileDialog`](crate::shell::IFileDialog),
/// [`IModalWindow`](crate::shell::IModalWindow), [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj = CoCreateInstance::<shell::IFileOpenDialog>(
///     &shell::clsid::FileOpenDialog,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct IFileOpenDialog  {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for IFileOpenDialog {
	const IID: IID = IID::new(0xd57c7288, 0xd4ad, 0x4768, 0xbe02, 0x9d969532d960);
}

macro_rules! impl_IFileOpenDialog {
	($name:ty, $vt:ty) => {
		use crate::com::shell::IShellItemArray;

		impl $name {
			fn ifileopendialog_vt(&self) -> &IFileOpenDialogVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// [`IFileOpenDialog::GetResults`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getresults)
			/// method.
			pub fn GetResults(&self) -> WinResult<IShellItemArray> {
				let mut ppv_queried: PPVT = std::ptr::null_mut();
				hr_to_winresult(
					(self.ifileopendialog_vt().GetResults)(
						self.ppvt,
						&mut ppv_queried as *mut _ as _,
					),
				).map(|_| IShellItemArray::from(ppv_queried))
			}

			/// [`IFileOpenDialog::GetSelectedItems`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getselecteditems)
			/// method.
			pub fn GetSelectedItems(&self) -> WinResult<IShellItemArray> {
				let mut ppv_queried: PPVT = std::ptr::null_mut();
				hr_to_winresult(
					(self.ifileopendialog_vt().GetSelectedItems)(
						self.ppvt,
						&mut ppv_queried as *mut _ as _,
					),
				).map(|_| IShellItemArray::from(ppv_queried))
			}
		}
	};
}

impl_IUnknown!(IFileOpenDialog, IFileOpenDialogVT);
impl_IModalWindow!(IFileOpenDialog, IFileOpenDialogVT);
impl_IFileDialog!(IFileOpenDialog, IFileOpenDialogVT);
impl_IFileOpenDialog!(IFileOpenDialog, IFileOpenDialogVT);
