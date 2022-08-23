#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::ffi_types::HRES;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{shell_IFileDialog, shell_IModalWindow};
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

impl_iunknown!(IFileOpenDialog, "d57c7288-d4ad-4768-be02-9d969532d960");
impl shell_IModalWindow for IFileOpenDialog {}
impl shell_IFileDialog for IFileOpenDialog {}
impl shell_IFileOpenDialog for IFileOpenDialog {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IFileOpenDialog`](crate::IFileOpenDialog).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_IFileOpenDialog: shell_IFileDialog {
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
	/// # let fo = IFileOpenDialog::from(unsafe { winsafe::ComPtr::null() });
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
	#[must_use]
	fn GetResults(&self) -> HrResult<IShellItemArray> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = &**(self.ptr().0 as *mut *mut IFileOpenDialogVT);
			ok_to_hrresult((vt.GetResults)(self.ptr(), &mut ppv_queried))
				.map(|_| IShellItemArray::from(ppv_queried))
		}
	}

	/// [`IFileOpenDialog::GetSelectedItems`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileopendialog-getselecteditems)
	/// method.
	#[must_use]
	fn GetSelectedItems(&self) -> HrResult<IShellItemArray> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = &**(self.ptr().0 as *mut *mut IFileOpenDialogVT);
			ok_to_hrresult((vt.GetSelectedItems)(self.ptr(), &mut ppv_queried))
				.map(|_| IShellItemArray::from(ppv_queried))
		}
	}
}
