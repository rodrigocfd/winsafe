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
	/// Copying a file:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let _com_guard =
	///     w::CoInitializeEx(co::COINIT::APARTMENTTHREADED | co::COINIT::DISABLE_OLE1DDE)?;
	///
	/// let op = w::CoCreateInstance::<w::IFileOperation>(
	///     &co::CLSID::FileOperation,
	///     None::<&w::IUnknown>,
	///     co::CLSCTX::ALL,
	/// )?;
	///
	/// let file = w::SHCreateItemFromParsingName::<w::IShellItem>(
	///     "C:\\Temp\\foo.txt",
	///     None::<&w::IBindCtx>,
	/// )?;
	/// let dest = w::SHCreateItemFromParsingName::<w::IShellItem>(
	///     "C:\\Temp",
	///     None::<&w::IBindCtx>,
	/// )?;
	///
	/// op.CopyItem(&file, &dest, Some("new file.txt"), None)?;
	/// op.PerformOperations()?;
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
	/// [`IFileOperation::Advise`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-advise)
	/// method.
	fn Advise(&self, fops: &IFileOperationProgressSink) -> HrResult<u32> {
		let mut cookie = 0u32;
		HrRet(unsafe { (vt::<IFileOperationVT>(self).Advise)(self.ptr(), fops.ptr(), &mut cookie) })
			.to_hrresult()
			.map(|_| cookie)
	}

	/// [`IFileOperation::ApplyPropertiesToItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-applypropertiestoitem)
	/// method.
	fn ApplyPropertiesToItem(&self, item: &impl shell_IShellItem) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileOperationVT>(self).ApplyPropertiesToItem)(self.ptr(), item.ptr())
		})
		.to_hrresult()
	}

	/// [`IFileOperation::CopyItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-copyitem)
	/// method.
	fn CopyItem(
		&self,
		item: &impl shell_IShellItem,
		destination_folder: &impl shell_IShellItem,
		copy_name: Option<&str>,
		fops: Option<&IFileOperationProgressSink>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileOperationVT>(self).CopyItem)(
				self.ptr(),
				item.ptr(),
				destination_folder.ptr(),
				WString::from_opt_str(copy_name).as_ptr(),
				fops.map_or(std::ptr::null_mut(), |p| p.ptr()),
			)
		})
		.to_hrresult()
	}

	/// [`IFileOperation::DeleteItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-deleteitem)
	/// method.
	fn DeleteItem(
		&self,
		item: &impl shell_IShellItem,
		fops: Option<&IFileOperationProgressSink>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileOperationVT>(self).DeleteItem)(
				self.ptr(),
				item.ptr(),
				fops.map_or(std::ptr::null_mut(), |p| p.ptr()),
			)
		})
		.to_hrresult()
	}

	/// [`IFileOperation::GetAnyOperationsAborted`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-getanyoperationsaborted)
	/// method.
	#[must_use]
	fn GetAnyOperationsAborted(&self) -> HrResult<bool> {
		let mut res = 0;
		HrRet(unsafe {
			(vt::<IFileOperationVT>(self).GetAnyOperationsAborted)(self.ptr(), &mut res)
		})
		.to_hrresult()
		.map(|_| res != 0)
	}

	/// [`IFileOperation::MoveItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-moveitem)
	/// method.
	fn MoveItem(
		&self,
		item: &impl shell_IShellItem,
		destination_folder: &impl shell_IShellItem,
		new_name: Option<&str>,
		fops: Option<&IFileOperationProgressSink>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileOperationVT>(self).MoveItem)(
				self.ptr(),
				item.ptr(),
				destination_folder.ptr(),
				WString::from_opt_str(new_name).as_ptr(),
				fops.map_or(std::ptr::null_mut(), |p| p.ptr()),
			)
		})
		.to_hrresult()
	}

	/// [`IFileOperation::NewItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-newitem)
	/// method.
	fn NewItem(
		&self,
		destination_folder: &impl shell_IShellItem,
		file_attributes: co::FILE_ATTRIBUTE,
		name: &str,
		template_name: Option<&str>,
		fops: Option<&IFileOperationProgressSink>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileOperationVT>(self).NewItem)(
				self.ptr(),
				destination_folder.ptr(),
				file_attributes.raw(),
				WString::from_str(name).as_ptr(),
				WString::from_opt_str(template_name).as_ptr(),
				fops.map_or(std::ptr::null_mut(), |p| p.ptr()),
			)
		})
		.to_hrresult()
	}

	fn_com_noparm! { PerformOperations: IFileOperationVT;
		/// [`IFileOperation::PerformOperations`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-performoperations)
		/// method.
	}

	/// [`IFileOperation::RenameItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-renameitem)
	/// method.
	fn RenameItem(
		&self,
		item: &impl shell_IShellItem,
		new_name: &str,
		fops: Option<&IFileOperationProgressSink>,
	) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileOperationVT>(self).RenameItem)(
				self.ptr(),
				item.ptr(),
				WString::from_str(new_name).as_ptr(),
				fops.map_or(std::ptr::null_mut(), |p| p.ptr()),
			)
		})
		.to_hrresult()
	}

	/// [`IFileOperation::SetOperationFlags`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-setoperationflags)
	/// method.
	fn SetOperationFlags(&self, flags: co::FOF) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileOperationVT>(self).SetOperationFlags)(self.ptr(), flags.raw() as _)
		})
		.to_hrresult()
	}

	/// [`IFileOperation::SetOwnerWindow`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-setownerwindow)
	/// method.
	fn SetOwnerWindow(&self, hwnd_owner: &HWND) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileOperationVT>(self).SetOwnerWindow)(self.ptr(), hwnd_owner.ptr())
		})
		.to_hrresult()
	}

	/// [`IFileOperation::SetProgressDialog`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-setprogressdialog)
	/// method.
	fn SetProgressDialog(&self, popd: &IOperationsProgressDialog) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFileOperationVT>(self).SetProgressDialog)(self.ptr(), popd.ptr()) })
			.to_hrresult()
	}

	/// [`IFileOperation::Unadvise`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-unadvise)
	/// method.
	fn Unadvise(&self, cookie: u32) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFileOperationVT>(self).Unadvise)(self.ptr(), cookie) }).to_hrresult()
	}
}
