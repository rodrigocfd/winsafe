#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::vts::*;

com_interface! { IFileOperation: "947aab5f-0a5c-4c13-b4d6-4bf7836fc9f8";
	/// [`IFileOperation`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifileoperation)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let fo = w::CoCreateInstance::<w::IFileOperation>(
	///     &co::CLSID::FileOperation,
	///     None,
	///     co::CLSCTX::INPROC_SERVER,
	/// )?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl shell_IFileOperation for IFileOperation {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IFileOperation`](crate::IFileOperation).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IFileOperation: ole_IUnknown {
	/// [`IFileOperation::NewItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-newitem)
	/// method.
	fn NewItem(
		&self,
		destination_folder: &impl shell_IShellItem,
		file_attributes: co::FILE_ATTRIBUTE,
		name: &str,
		template_name: Option<&str>,
		fopts: Option<&IFileOperationProgressSink>,
	) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IFileOperationVT>(self).NewItem)(
				self.ptr(),
				destination_folder.ptr(),
				file_attributes.raw(),
				WString::from_str(name).as_ptr(),
				WString::from_opt_str(template_name).as_ptr(),
				fopts.map_or(std::ptr::null_mut(), |p| p.ptr()),
			)
		})
	}

	/// [`IFileOperation::SetOperationFlags`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-setoperationflags)
	/// method.
	fn SetOperationFlags(&self, flags: co::FOF) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			(vt::<IFileOperationVT>(self).SetOperationFlags)(self.ptr(), flags.raw() as _)
		})
	}
}
