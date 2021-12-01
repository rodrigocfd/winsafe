#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::com::iunknown::ComPtr;
use crate::com::shell::ifiledialog::{IFileDialogT, IFileDialogVT};
use crate::com::shell::imodalwindow::IModalWindowT;
use crate::com::shell::ishellitemarray::IShellItemArray;
use crate::ffi::HRES;
use crate::privs::ok_to_hrresult;

/// [`IFileOpenDialog`](crate::shell::IFileOpenDialog) virtual table.
#[repr(C)]
pub struct IFileOpenDialogVT {
	pub IFileDialogVT: IFileDialogVT,
	pub GetResults: fn(ComPtr, *mut ComPtr) -> HRES,
	pub GetSelectedItems: fn(ComPtr, *mut ComPtr) -> HRES,
}

/// [`IFileOpenDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileopendialog)
/// COM interface over
/// [`IFileOpenDialogVT`](crate::shell::vt::IFileOpenDialogVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::prelude::*;
/// use winsafe::{co, CoCreateInstance, shell};
///
/// let obj = CoCreateInstance::<shell::IFileOpenDialog>(
///     &shell::clsid::FileOpenDialog,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// ```
pub struct IFileOpenDialog(ComPtr);

impl_iunknown!(IFileOpenDialog, 0xd57c7288, 0xd4ad, 0x4768, 0xbe02, 0x9d969532d960);
impl IModalWindowT for IFileOpenDialog {}
impl IFileDialogT for IFileOpenDialog {}
impl IFileOpenDialogT for IFileOpenDialog {}

/// Exposes the [`IFileOpenDialog`](crate::shell::IFileOpenDialog) methods.
pub trait IFileOpenDialogT: IFileDialogT {
	/// [`IFileOpenDialog::GetResults`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getresults)
	/// method.
	///
	/// # Examples
	///
	/// Collecting the file paths into a
	/// [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html):
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::{HrResult, shell, shell::co::SIGDN};
	///
	/// let fo: shell::IFileOpenDialog; // initialized somewhere
	///
	/// let paths = fo.GetResults()?.iter()
	///     .map(|shi|
	///         shi.and_then(|shi|
	///             shi.GetDisplayName(SIGDN::FILESYSPATH)
	///         ),
	///     )
	///     .collect::<HrResult<Vec<_>>>()?,
	/// ```
	fn GetResults(&self) -> HrResult<IShellItemArray> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileOpenDialogVT);
			ok_to_hrresult((vt.GetResults)(self.ptr(), &mut ppv_queried))
		}.map(|_| IShellItemArray::from(ppv_queried))
	}

	/// [`IFileOpenDialog::GetSelectedItems`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getselecteditems)
	/// method.
	fn GetSelectedItems(&self) -> HrResult<IShellItemArray> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileOpenDialogVT);
			ok_to_hrresult((vt.GetSelectedItems)(self.ptr(), &mut ppv_queried))
		}.map(|_| IShellItemArray::from(ppv_queried))
	}
}
