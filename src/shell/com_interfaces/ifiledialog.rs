#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::vts::*;

com_interface! { IFileDialog: "42f85136-db7e-439c-85f1-e4075d135fc8";
	/// [`IFileDialog`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialog)
	/// COM interface.
	///
	/// Automatically calls
	/// [`Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_IModalWindow for IFileDialog {}
impl shell_IFileDialog for IFileDialog {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IFileDialog`](crate::IFileDialog).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IFileDialog: shell_IModalWindow {
	/// [`IFileDialog::AddPlace`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-addplace)
	/// method.
	fn AddPlace(&self, si: &impl shell_IShellItem, fdap: co::FDAP) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFileDialogVT>(self).AddPlace)(self.ptr(), si.ptr(), fdap.raw()) })
			.to_hrresult()
	}

	/// [`IFileDialog::Advise`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-advise)
	/// method.
	fn Advise(&self, fde: &IFileDialogEvents) -> HrResult<u32> {
		let mut cookie = 0u32;
		HrRet(unsafe { (vt::<IFileDialogVT>(self).Advise)(self.ptr(), fde.ptr(), &mut cookie) })
			.to_hrresult()
			.map(|_| cookie)
	}

	fn_com_noparm! { ClearClientData: IFileDialogVT;
		/// [`IFileDialog::ClearClientData`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-clearclientdata)
		/// method.
	}

	/// [`IFileDialog::Close`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-close)
	/// method.
	fn Close(&self, hr: co::ERROR) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFileDialogVT>(self).Close)(self.ptr(), hr.raw() as _) }).to_hrresult()
	}

	fn_com_interface_get! { GetCurrentSelection: IFileDialogVT => IShellItem;
		/// [`IFileDialog::GetCurrentSelection`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getcurrentselection)
		/// method.
	}

	/// [`IFileDialog::GetFileName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfilename)
	/// method.
	#[must_use]
	fn GetFileName(&self) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		HrRet(unsafe { (vt::<IFileDialogVT>(self).GetFileName)(self.ptr(), &mut pstr) })
			.to_hrresult()
			.map(|_| htaskmem_ptr_to_str(pstr))
	}

	/// [`IFileDialog::GetFileTypeIndex`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfiletypeindex)
	/// method.
	#[must_use]
	fn GetFileTypeIndex(&self) -> HrResult<u32> {
		let mut index = 0u32;
		HrRet(unsafe { (vt::<IFileDialogVT>(self).GetFileTypeIndex)(self.ptr(), &mut index) })
			.to_hrresult()
			.map(|_| index)
	}

	fn_com_interface_get! { GetFolder: IFileDialogVT => IShellItem;
		/// [`IFileDialog::GetFolder`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfolder)
		/// method.
	}

	/// [`IFileDialog::GetOptions`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getoptions)
	/// method.
	#[must_use]
	fn GetOptions(&self) -> HrResult<co::FOS> {
		let mut opts = co::FOS::default();
		HrRet(unsafe { (vt::<IFileDialogVT>(self).GetOptions)(self.ptr(), opts.as_mut()) })
			.to_hrresult()
			.map(|_| opts)
	}

	fn_com_interface_get! { GetResult: IFileDialogVT => IShellItem;
		/// [`IFileDialog::GetResult`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getresult)
		/// method.
		///
		/// If you chose a single file, this is the method to retrieve its path.
	}

	/// [`IFileDialog::SetClientGuid`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setclientguid)
	/// method.
	fn SetClientGuid(&self, guid: &GUID) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFileDialogVT>(self).SetClientGuid)(self.ptr(), pcvoid(guid)) })
			.to_hrresult()
	}

	/// [`IFileDialog::SetDefaultExtension`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultextension)
	/// method.
	fn SetDefaultExtension(&self, default_extension: &str) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileDialogVT>(self).SetDefaultExtension)(
				self.ptr(),
				WString::from_str(default_extension).as_ptr(),
			)
		})
		.to_hrresult()
	}

	/// [`IFileDialog::SetDefaultFolder`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultfolder)
	/// method.
	fn SetDefaultFolder(&self, si: &impl shell_IShellItem) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFileDialogVT>(self).SetDefaultFolder)(self.ptr(), si.ptr()) })
			.to_hrresult()
	}

	/// [`IFileDialog::SetFileName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilename)
	/// method.
	fn SetFileName(&self, name: &str) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileDialogVT>(self).SetFileName)(self.ptr(), WString::from_str(name).as_ptr())
		})
		.to_hrresult()
	}

	/// [`IFileDialog::SetFileNameLabel`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilenamelabel)
	/// method.
	fn SetFileNameLabel(&self, label: &str) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileDialogVT>(self).SetFileNameLabel)(
				self.ptr(),
				WString::from_str(label).as_ptr(),
			)
		})
		.to_hrresult()
	}

	/// [`IFileDialog::SetFileTypeIndex`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypeindex)
	/// method.
	///
	/// **Note:** The index is one-based.
	fn SetFileTypeIndex(&self, index: u32) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFileDialogVT>(self).SetFileTypeIndex)(self.ptr(), index) })
			.to_hrresult()
	}

	/// [`IFileDialog::SetFileTypes`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypes)
	/// method.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let file_dlg: w::IFileDialog; // initialized somewhere
	/// # let file_dlg = unsafe { w::IFileDialog::null() };
	///
	/// file_dlg.SetFileTypes(&[
	///     ("Documents", "*.docx;*.txt"),
	///     ("Images", "*.jpg;*.png;*.bmp"),
	///     ("All files", "*.*"),
	/// ])?;
	/// # w::HrResult::Ok(())
	/// ```
	fn SetFileTypes<S: AsRef<str>>(&self, filter_spec: &[(S, S)]) -> HrResult<()> {
		let (mut strs_buf, mut com_dlgs): (Vec<_>, Vec<_>) = filter_spec
			.iter()
			.map(|(name, spec)| {
				let wname = WString::from_str(name.as_ref());
				let wspec = WString::from_str(spec.as_ref());
				((wname, wspec), COMDLG_FILTERSPEC::default())
			})
			.unzip();

		strs_buf
			.iter_mut()
			.zip(com_dlgs.iter_mut())
			.for_each(|((name, spec), com_dlg)| {
				com_dlg.set_pszName(Some(name));
				com_dlg.set_pszSpec(Some(spec));
			});

		HrRet(unsafe {
			(vt::<IFileDialogVT>(self).SetFileTypes)(
				self.ptr(),
				filter_spec.len() as _,
				com_dlgs.as_ptr() as _,
			)
		})
		.to_hrresult()
	}

	/// [`IFileDialog::SetFilter`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilter)
	/// method.
	fn SetFilter(&self, filter: &IShellItemFilter) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFileDialogVT>(self).SetFilter)(self.ptr(), filter.ptr()) })
			.to_hrresult()
	}

	/// [`IFileDialog::SetFolder`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfolder)
	/// method.
	fn SetFolder(&self, si: &impl shell_IShellItem) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFileDialogVT>(self).SetFolder)(self.ptr(), si.ptr()) }).to_hrresult()
	}

	/// [`IFileDialog::SetOkButtonLabel`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setokbuttonlabel)
	/// method.
	fn SetOkButtonLabel(&self, text: &str) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileDialogVT>(self).SetOkButtonLabel)(
				self.ptr(),
				WString::from_str(text).as_ptr(),
			)
		})
		.to_hrresult()
	}

	/// [`IFileDialog::SetOptions`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setoptions)
	/// method.
	fn SetOptions(&self, opts: co::FOS) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFileDialogVT>(self).SetOptions)(self.ptr(), opts.raw()) })
			.to_hrresult()
	}

	/// [`IFileDialog::SetTitle`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-settitle)
	/// method.
	fn SetTitle(&self, text: &str) -> HrResult<()> {
		HrRet(unsafe {
			(vt::<IFileDialogVT>(self).SetTitle)(self.ptr(), WString::from_str(text).as_ptr())
		})
		.to_hrresult()
	}

	/// [`IFileDialog::Unadvise`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-unadvise)
	/// method.
	fn Unadvise(&self, cookie: u32) -> HrResult<()> {
		HrRet(unsafe { (vt::<IFileDialogVT>(self).Unadvise)(self.ptr(), cookie) }).to_hrresult()
	}
}
