#![allow(non_snake_case)]

use crate::ffi_types::HRES;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{ShellIFileDialog, ShellIModalWindow};
use crate::shell::decl::IShellItemArray;
use crate::vt::IFileDialogVT;

/// [`IFileOpenDialog`](crate::IFileOpenDialog) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct IFileOpenDialogVT {
	pub IFileDialogVT: IFileDialogVT,
	pub GetResults: fn(ComPtr, *mut ComPtr) -> HRES,
	pub GetSelectedItems: fn(ComPtr, *mut ComPtr) -> HRES,
}

/// [`IFileOpenDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileopendialog)
/// COM interface over [`IFileOpenDialogVT`](crate::vt::IFileOpenDialogVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, CoCreateInstance, IFileOpenDialog};
///
/// let obj = CoCreateInstance::<IFileOpenDialog>(
///     &co::CLSID::FileOpenDialog,
///     None,
///     co::CLSCTX::INPROC_SERVER,
/// )?;
/// # Ok::<_, co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub struct IFileOpenDialog(ComPtr);

impl_iunknown!(IFileOpenDialog, 0xd57c7288, 0xd4ad, 0x4768, 0xbe02, 0x9d969532d960);
impl ShellIModalWindow for IFileOpenDialog {}
impl ShellIFileDialog for IFileOpenDialog {}
impl ShellIFileOpenDialog for IFileOpenDialog {}

/// [`IFileOpenDialog`](crate::IFileOpenDialog) methods from `shell` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait ShellIFileOpenDialog: ShellIFileDialog {
	/// [`IFileOpenDialog::GetResults`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getresults)
	/// method.
	///
	/// # Examples
	///
	/// Collecting the file paths into a
	/// [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html):
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HrResult, IFileOpenDialog};
	///
	/// let fo: IFileOpenDialog; // initialized somewhere
	/// # use winsafe::{co::CLSID, co::CLSCTX, CoCreateInstance};
	/// # let fo = CoCreateInstance::<IFileOpenDialog>(&CLSID::new(0,0,0,0,0), None, CLSCTX::INPROC_SERVER)?;
	///
	/// let paths = fo.GetResults()?.iter()?
	///     .map(|shi|
	///         shi.and_then(|shi|
	///             shi.GetDisplayName(co::SIGDN::FILESYSPATH)
	///         ),
	///     )
	///     .collect::<HrResult<Vec<_>>>()?;
	/// # Ok::<_, co::HRESULT>(())
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
