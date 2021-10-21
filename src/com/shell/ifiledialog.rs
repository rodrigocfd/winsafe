#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::funcs::CoTaskMemFree;
use crate::com::iunknown::{ComPtr, IUnknownT};
use crate::com::shell;
use crate::com::shell::any_structs::COMDLG_FILTERSPEC;
use crate::com::shell::imodalwindow::{IModalWindowT, IModalWindowVT};
use crate::com::shell::ishellitem::IShellItem;
use crate::ffi::{HRESULT, PCSTR, PCVOID, PSTR, PVOID};
use crate::privs::hr_to_winresult;
use crate::structs::GUID;
use crate::various::WString;

/// [`IFileDialog`](crate::shell::IFileDialog) virtual table.
pub struct IFileDialogVT {
	pub IModalWindowVT: IModalWindowVT,
	pub SetFileTypes: fn(ComPtr, u32, PCVOID) -> HRESULT,
	pub SetFileTypeIndex: fn(ComPtr, u32) -> HRESULT,
	pub GetFileTypeIndex: fn(ComPtr, *mut u32) -> HRESULT,
	pub Advise: fn(ComPtr, PVOID, *mut u32) -> HRESULT,
	pub Unadvise: fn(ComPtr, u32) -> HRESULT,
	pub SetOptions: fn(ComPtr, u32) -> HRESULT,
	pub GetOptions: fn(ComPtr, *mut u32) -> HRESULT,
	pub SetDefaultFolder: fn(ComPtr, ComPtr) -> HRESULT,
	pub SetFolder: fn(ComPtr, ComPtr) -> HRESULT,
	pub GetFolder: fn(ComPtr, *mut PVOID) -> HRESULT,
	pub GetCurrentSelection: fn(ComPtr, *mut ComPtr) -> HRESULT,
	pub SetFileName: fn(ComPtr, PCSTR) -> HRESULT,
	pub GetFileName: fn(ComPtr, *mut PSTR) -> HRESULT,
	pub SetTitle: fn(ComPtr, PCSTR) -> HRESULT,
	pub SetOkButtonLabel: fn(ComPtr, PCSTR) -> HRESULT,
	pub SetFileNameLabel: fn(ComPtr, PCSTR) -> HRESULT,
	pub GetResult: fn(ComPtr, *mut ComPtr) -> HRESULT,
	pub AddPlace: fn(ComPtr, ComPtr, u32) -> HRESULT,
	pub SetDefaultExtension: fn(ComPtr, PCSTR) -> HRESULT,
	pub Close: fn(ComPtr, HRESULT) -> HRESULT,
	pub SetClientGuid: fn(ComPtr, PCVOID) -> HRESULT,
	pub ClearClientData: fn(ComPtr) -> HRESULT,
	pub SetFilter: fn(ComPtr, PVOID) -> HRESULT,
}

/// [`IFileDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialog)
/// COM interface over [`IFileDialogVT`](crate::shell::vt::IFileDialogVT).
///
/// Automatically calls
/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IFileDialog(ComPtr);

impl_iunknown!(IFileDialog, 0x42f85136, 0xdb7e, 0x439c, 0x85f1, 0xe4075d135fc8);
impl IModalWindowT for IFileDialog {}
impl IFileDialogT for IFileDialog {}

/// Exposes the [`IFileDialog`](crate::shell::IFileDialog) methods.
pub trait IFileDialogT: IModalWindowT {
	/// [`IFileDialog::AddPlace`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-addplace)
	/// method.
	fn AddPlace(&self, si: &IShellItem, fdap: shell::co::FDAP) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult((vt.AddPlace)(self.ptr(), si.ptr(), fdap.0))
		}
	}

	/// [`IFileDialog::ClearClientData`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-clearclientdata)
	/// method.
	fn ClearClientData(&self) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult((vt.ClearClientData)(self.ptr()))
		}
	}

	/// [`IFileDialog::Close`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-close)
	/// method.
	fn Close(&self, hr: co::ERROR) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult((vt.Close)(self.ptr(), hr.0 as _))
		}
	}

	/// [`IFileDialog::GetCurrentSelection`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getcurrentselection)
	/// method.
	fn GetCurrentSelection(&self) -> WinResult<IShellItem> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult(
				(vt.GetCurrentSelection)(
					self.ptr(),
					&mut ppv_queried as *mut _ as _,
				),
			)
		}.map(|_| IShellItem::from(ppv_queried))
	}

	/// [`IFileDialog::GetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfilename)
	/// method.
	fn GetFileName(&self) -> WinResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult((vt.GetFileName)(self.ptr(), &mut pstr))
		}.map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr);
			name.to_string()
		})
	}

	/// [`IFileDialog::GetFileTypeIndex`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfiletypeindex)
	/// method.
	fn GetFileTypeIndex(&self) -> WinResult<u32> {
		let mut index = u32::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult((vt.GetFileTypeIndex)(self.ptr(), &mut index))
		}.map(|_| index)
	}

	/// [`IFileDialog::GetFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfolder)
	/// method.
	fn GetFolder(&self) -> WinResult<IShellItem> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult(
				(vt.GetFolder)(self.ptr(), &mut ppv_queried as *mut _ as _),
			)
		}.map(|_| IShellItem::from(ppv_queried))
	}

	/// [`IFileDialog::GetOptions`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getoptions)
	/// method.
	fn GetOptions(&self) -> WinResult<shell::co::FOS> {
		let mut opts = u32::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult((vt.GetOptions)(self.ptr(), &mut opts))
		}.map(|_| shell::co::FOS(opts))
	}

	/// [`IFileDialog::GetResult`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getresult)
	/// method.
	fn GetResult(&self) -> WinResult<IShellItem> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult(
				(vt.GetResult)(self.ptr(), &mut ppv_queried as *mut _ as _),
			)
		}.map(|_| IShellItem::from(ppv_queried))
	}

	/// [`IFileDialog::SetClientGuid`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setclientguid)
	/// method.
	fn SetClientGuid(&self, guid: &GUID) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult((vt.SetClientGuid)(self.ptr(), guid as *const _ as _))
		}
	}

	/// [`IFileDialog::SetDefaultExtension`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultextension)
	/// method.
	fn SetDefaultExtension(&self, default_extension: &str) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult(
				(vt.SetDefaultExtension)(
					self.ptr(),
					WString::from_str(default_extension).as_ptr(),
				),
			)
		}
	}

	/// [`IFileDialog::SetDefaultFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultfolder)
	/// method.
	fn SetDefaultFolder(&self, si: &IShellItem) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult((vt.SetDefaultFolder)(self.ptr(), si.ptr()))
		}
	}

	/// [`IFileDialog::SetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilename)
	/// method.
	fn SetFileName(&self, name: &str) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult(
				(vt.SetFileName)(self.ptr(), WString::from_str(name).as_ptr()),
			)
		}
	}

	/// [`IFileDialog::SetFileNameLabel`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilenamelabel)
	/// method.
	fn SetFileNameLabel(&self, label: &str) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult(
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
	fn SetFileTypeIndex(&self, index: u32) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult((vt.SetFileTypeIndex)(self.ptr(), index))
		}
	}

	/// [`IFileDialog::SetFileTypes`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypes)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe::shell::IFileDialog;
	///
	/// let file_dlg: IFileDialog; // initialized somewhere
	///
	/// file_dlg.SetFileTypes(&[
	///     ("Documents", "*.docx;*.txt"),
	///     ("Images", "*.jpg;*.png;*.bmp"),
	///     ("All files", "*.*"),
	/// ])?;
	/// ```
	fn SetFileTypes<S: AsRef<str>>(&self,
		filter_spec: &[(S, S)]) -> WinResult<()>
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
			hr_to_winresult(
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
	fn SetFolder(&self, si: &IShellItem) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult((vt.SetFolder)(self.ptr(), si.ptr()))
		}
	}

	/// [`IFileDialog::SetOkButtonLabel`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setokbuttonlabel)
	/// method.
	fn SetOkButtonLabel(&self, text: &str) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult(
				(vt.SetOkButtonLabel)(self.ptr(), WString::from_str(text).as_ptr()),
			)
		}
	}

	/// [`IFileDialog::SetOptions`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setoptions)
	/// method.
	fn SetOptions(&self, opts: shell::co::FOS) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult((vt.SetOptions)(self.ptr(), opts.0))
		}
	}

	/// [`IFileDialog::SetTitle`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-settitle)
	/// method.
	fn SetTitle(&self, text: &str) -> WinResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IFileDialogVT);
			hr_to_winresult(
				(vt.SetTitle)(self.ptr(), WString::from_str(text).as_ptr()),
			)
		}
	}
}
