#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::decl::WString;
use crate::kernel::ffi_types::{HRES, PCSTR, PCVOID, PSTR, PVOID};
use crate::ole::decl::{ComPtr, CoTaskMemFree, GUID, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{ole_IUnknown, shell_IModalWindow};
use crate::shell::decl::{COMDLG_FILTERSPEC, IShellItem};
use crate::vt::IModalWindowVT;

/// [`IFileDialog`](crate::IFileDialog) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct IFileDialogVT {
	pub IModalWindowVT: IModalWindowVT,
	pub SetFileTypes: fn(ComPtr, u32, PCVOID) -> HRES,
	pub SetFileTypeIndex: fn(ComPtr, u32) -> HRES,
	pub GetFileTypeIndex: fn(ComPtr, *mut u32) -> HRES,
	pub Advise: fn(ComPtr, PVOID, *mut u32) -> HRES,
	pub Unadvise: fn(ComPtr, u32) -> HRES,
	pub SetOptions: fn(ComPtr, u32) -> HRES,
	pub GetOptions: fn(ComPtr, *mut u32) -> HRES,
	pub SetDefaultFolder: fn(ComPtr, ComPtr) -> HRES,
	pub SetFolder: fn(ComPtr, ComPtr) -> HRES,
	pub GetFolder: fn(ComPtr, *mut ComPtr) -> HRES,
	pub GetCurrentSelection: fn(ComPtr, *mut ComPtr) -> HRES,
	pub SetFileName: fn(ComPtr, PCSTR) -> HRES,
	pub GetFileName: fn(ComPtr, *mut PSTR) -> HRES,
	pub SetTitle: fn(ComPtr, PCSTR) -> HRES,
	pub SetOkButtonLabel: fn(ComPtr, PCSTR) -> HRES,
	pub SetFileNameLabel: fn(ComPtr, PCSTR) -> HRES,
	pub GetResult: fn(ComPtr, *mut ComPtr) -> HRES,
	pub AddPlace: fn(ComPtr, ComPtr, u32) -> HRES,
	pub SetDefaultExtension: fn(ComPtr, PCSTR) -> HRES,
	pub Close: fn(ComPtr, HRES) -> HRES,
	pub SetClientGuid: fn(ComPtr, PCVOID) -> HRES,
	pub ClearClientData: fn(ComPtr) -> HRES,
	pub SetFilter: fn(ComPtr, PVOID) -> HRES,
}

com_interface! { IFileDialog: "shell";
	"42f85136-db7e-439c-85f1-e4075d135fc8";
	/// [`IFileDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialog)
	/// COM interface over [`IFileDialogVT`](crate::vt::IFileDialogVT).
	///
	/// Automatically calls
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_IModalWindow for IFileDialog {}
impl shell_IFileDialog for IFileDialog {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IFileDialog`](crate::IFileDialog).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_IFileDialog: shell_IModalWindow {
	/// [`IFileDialog::AddPlace`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-addplace)
	/// method.
	fn AddPlace(&self, si: &IShellItem, fdap: co::FDAP) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.AddPlace)(self.ptr(), si.ptr(), fdap.0))
		}
	}

	/// [`IFileDialog::ClearClientData`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-clearclientdata)
	/// method.
	fn ClearClientData(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.ClearClientData)(self.ptr()))
		}
	}

	/// [`IFileDialog::Close`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-close)
	/// method.
	fn Close(&self, hr: co::ERROR) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.Close)(self.ptr(), hr.0 as _))
		}
	}

	/// [`IFileDialog::GetCurrentSelection`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getcurrentselection)
	/// method.
	#[must_use]
	fn GetCurrentSelection(&self) -> HrResult<IShellItem> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.GetCurrentSelection)(self.ptr(), &mut ppv_queried))
				.map(|_| IShellItem::from(ppv_queried))
		}
	}

	/// [`IFileDialog::GetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfilename)
	/// method.
	#[must_use]
	fn GetFileName(&self) -> HrResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.GetFileName)(self.ptr(), &mut pstr))
		}.map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr as _);
			name.to_string()
		})
	}

	/// [`IFileDialog::GetFileTypeIndex`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfiletypeindex)
	/// method.
	#[must_use]
	fn GetFileTypeIndex(&self) -> HrResult<u32> {
		let mut index = u32::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.GetFileTypeIndex)(self.ptr(), &mut index))
		}.map(|_| index)
	}

	/// [`IFileDialog::GetFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfolder)
	/// method.
	#[must_use]
	fn GetFolder(&self) -> HrResult<IShellItem> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.GetFolder)(self.ptr(), &mut ppv_queried))
				.map(|_| IShellItem::from(ppv_queried))
		}
	}

	/// [`IFileDialog::GetOptions`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getoptions)
	/// method.
	#[must_use]
	fn GetOptions(&self) -> HrResult<co::FOS> {
		let mut opts = u32::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.GetOptions)(self.ptr(), &mut opts))
		}.map(|_| co::FOS(opts))
	}

	/// [`IFileDialog::GetResult`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getresult)
	/// method.
	#[must_use]
	fn GetResult(&self) -> HrResult<IShellItem> {
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.GetResult)(self.ptr(), &mut ppv_queried))
				.map(|_| IShellItem::from(ppv_queried))
		}
	}

	/// [`IFileDialog::SetClientGuid`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setclientguid)
	/// method.
	fn SetClientGuid(&self, guid: &GUID) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.SetClientGuid)(self.ptr(), guid as *const _ as _))
		}
	}

	/// [`IFileDialog::SetDefaultExtension`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultextension)
	/// method.
	fn SetDefaultExtension(&self, default_extension: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult(
				(vt.SetDefaultExtension)(
					self.ptr(),
					WString::from_str(default_extension).as_ptr(),
				),
			)
		}
	}

	/// [`IFileDialog::SetDefaultFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultfolder)
	/// method.
	fn SetDefaultFolder(&self, si: &IShellItem) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.SetDefaultFolder)(self.ptr(), si.ptr()))
		}
	}

	/// [`IFileDialog::SetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilename)
	/// method.
	fn SetFileName(&self, name: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult(
				(vt.SetFileName)(self.ptr(), WString::from_str(name).as_ptr()),
			)
		}
	}

	/// [`IFileDialog::SetFileNameLabel`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilenamelabel)
	/// method.
	fn SetFileNameLabel(&self, label: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult(
				(vt.SetFileNameLabel)(
					self.ptr(),
					WString::from_str(label).as_ptr(),
				),
			)
		}
	}

	/// [`IFileDialog::SetFileTypeIndex`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypeindex)
	/// method.
	///
	/// **Note:** The index is one-based.
	fn SetFileTypeIndex(&self, index: u32) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.SetFileTypeIndex)(self.ptr(), index))
		}
	}

	/// [`IFileDialog::SetFileTypes`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypes)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IFileDialog;
	///
	/// let file_dlg: IFileDialog; // initialized somewhere
	/// # let file_dlg = IFileDialog::from(unsafe { winsafe::ComPtr::null() });
	///
	/// file_dlg.SetFileTypes(&[
	///     ("Documents", "*.docx;*.txt"),
	///     ("Images", "*.jpg;*.png;*.bmp"),
	///     ("All files", "*.*"),
	/// ])?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	fn SetFileTypes<S: AsRef<str>>(&self,
		filter_spec: &[(S, S)]) -> HrResult<()>
	{
		let mut names_buf = Vec::with_capacity(filter_spec.len());
		let mut specs_buf = Vec::with_capacity(filter_spec.len());
		let mut com_dlgs = Vec::with_capacity(filter_spec.len());

		for (name, spec) in filter_spec.iter() {
			names_buf.push(WString::from_str(name.as_ref()));
			specs_buf.push(WString::from_str(spec.as_ref()));
			com_dlgs.push(COMDLG_FILTERSPEC::default());
		}

		names_buf.iter_mut().enumerate()
			.for_each(|(i, el)| com_dlgs[i].set_pszName(Some(el)));

		specs_buf.iter_mut().enumerate()
			.for_each(|(i, el)| com_dlgs[i].set_pszSpec(Some(el)));

		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult(
				(vt.SetFileTypes)(
					self.ptr(),
					filter_spec.len() as _,
					com_dlgs.as_ptr() as _,
				),
			)
		}
	}

	/// [`IFileDialog::SetFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfolder)
	/// method.
	fn SetFolder(&self, si: &IShellItem) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.SetFolder)(self.ptr(), si.ptr()))
		}
	}

	/// [`IFileDialog::SetOkButtonLabel`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setokbuttonlabel)
	/// method.
	fn SetOkButtonLabel(&self, text: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult(
				(vt.SetOkButtonLabel)(self.ptr(), WString::from_str(text).as_ptr()),
			)
		}
	}

	/// [`IFileDialog::SetOptions`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setoptions)
	/// method.
	fn SetOptions(&self, opts: co::FOS) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult((vt.SetOptions)(self.ptr(), opts.0))
		}
	}

	/// [`IFileDialog::SetTitle`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-settitle)
	/// method.
	fn SetTitle(&self, text: &str) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			ok_to_hrresult(
				(vt.SetTitle)(self.ptr(), WString::from_str(text).as_ptr()),
			)
		}
	}
}
