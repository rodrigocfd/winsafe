#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::funcs::{CoTaskMemFree, hr_to_winresult};
use crate::com::PPComVT;
use crate::com::shell::{COMDLG_FILTERSPEC, IModalWindow, IShellItem};
use crate::com::shell::vt::{IFileDialogVT, IModalWindowVT, IShellItemVT};
use crate::structs::GUID;
use crate::WString;

/// [`IFileDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialog)
/// interface. Backed by [`IFileDialogVT`](crate::shell::IFileDialogVT) virtual
/// table.
///
/// Inherits from:
/// * [`IModalWindow`](crate::shell::IModalWindow);
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IFileDialog {
	/// Methods of base interface [`IModalWindow`](crate::shell::IModalWindow).
	pub IModalWindow: IModalWindow,
}

impl From<PPComVT<IFileDialogVT>> for IFileDialog {
	fn from(ppv: PPComVT<IFileDialogVT>) -> Self {
		Self {
			IModalWindow: IModalWindow::from(ppv as PPComVT<IModalWindowVT>),
		}
	}
}

impl IFileDialog {
	unsafe fn ppv(&self) -> PPComVT<IFileDialogVT> {
		self.IModalWindow.IUnknown.ppv::<IFileDialogVT>()
	}

	/// [`IFileDialog::ClearClientData`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-clearclientdata)
	/// method.
	pub fn ClearClientData(&self) -> WinResult<()> {
		hr_to_winresult(unsafe { ((**self.ppv()).ClearClientData)(self.ppv()) })
	}

	/// [`IFileDialog::Close`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-close)
	/// method.
	pub fn Close(&self, hr: co::ERROR) -> WinResult<()> {
		hr_to_winresult(
			unsafe { ((**self.ppv()).Close)(self.ppv(), hr.0 as i32) },
		)
	}

	/// [`IFileDialog::GetCurrentSelection`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getcurrentselection)
	/// method.
	pub fn GetCurrentSelection(&self) -> WinResult<IShellItem> {
		let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();
		hr_to_winresult(
			unsafe {
				((**self.ppv()).GetCurrentSelection)(
					self.ppv(),
					&mut ppvQueried as *mut _ as *mut _,
				)
			},
		).map(|_| IShellItem::from(ppvQueried))
	}

	/// [`IFileDialog::GetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfilename)
	/// method.
	pub fn GetFileName(&self) -> WinResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		hr_to_winresult(
			unsafe { ((**self.ppv()).GetFileName)(self.ppv(), &mut pstr) },
		).map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr);
			name.to_string()
		})
	}

	/// [`IFileDialog::GetFileTypeIndex`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfiletypeindex)
	/// method.
	pub fn GetFileTypeIndex(&self) -> WinResult<u32> {
		let mut index: u32 = 0;
		hr_to_winresult(
			unsafe { ((**self.ppv()).GetFileTypeIndex)(self.ppv(), &mut index) },
		).map(|_| index)
	}

	/// [`IFileDialog::GetFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfolder)
	/// method.
	pub fn GetFolder(&self) -> WinResult<IShellItem> {
		let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();
		hr_to_winresult(
			unsafe {
				((**self.ppv()).GetFolder)(
					self.ppv(),
					&mut ppvQueried as *mut _ as *mut _,
				)
			},
		).map(|_| IShellItem::from(ppvQueried))
	}

	/// [`IFileDialog::GetOptions`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getoptions)
	/// method.
	pub fn GetOptions(&self) -> WinResult<co::FOS> {
		let mut opts: u32 = 0;
		hr_to_winresult(
			unsafe { ((**self.ppv()).GetOptions)(self.ppv(), &mut opts) },
		).map(|_| co::FOS(opts))
	}

	/// [`IFileDialog::GetResult`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getresult)
	/// method.
	pub fn GetResult(&self) -> WinResult<IShellItem> {
		let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();
		hr_to_winresult(
			unsafe {
				((**self.ppv()).GetResult)(
					self.ppv(),
					&mut ppvQueried as *mut _ as *mut _,
				)
			},
		).map(|_| IShellItem::from(ppvQueried))
	}

	/// [`IFileDialog::SetClientGuid`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setclientguid)
	/// method.
	pub fn SetClientGuid(&self, guid: &GUID) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetClientGuid)(
					self.ppv(),
					guid as *const _ as *const _,
				)
			},
		)
	}

	/// [`IFileDialog::SetDefaultExtension`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultextension)
	/// method.
	pub fn SetDefaultExtension(&self, defaultExtension: &str) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetDefaultExtension)(
					self.ppv(),
					WString::from_str(defaultExtension).as_ptr(),
				)
			},
		)
	}

	/// [`IFileDialog::SetDefaultFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultfolder)
	/// method.
	pub fn SetDefaultFolder(&self, si: &IShellItem) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetDefaultFolder)(self.ppv(), si.IUnknown.ppv())
			},
		)
	}

	/// [`IFileDialog::SetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilename)
	/// method.
	pub fn SetFileName(&self, name: &str) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetFileName)(
					self.ppv(),
					WString::from_str(name).as_ptr(),
				)
			},
		)
	}

	/// [`IFileDialog::SetFileNameLabel`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilenamelabel)
	/// method.
	pub fn SetFileNameLabel(&self, label: &str) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetFileNameLabel)(
					self.ppv(),
					WString::from_str(label).as_ptr(),
				)
			},
		)
	}

	/// [`IFileDialog::SetFileTypeIndex`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypeindex)
	/// method.
	pub fn SetFileTypeIndex(&self, iFileType: u32) -> WinResult<()> {
		hr_to_winresult(
			unsafe { ((**self.ppv()).SetFileTypeIndex)(self.ppv(), iFileType) },
		)
	}

	/// [`IFileDialog::SetFileTypes`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypes)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::shell::IFileDialog;
	///
	/// let file_dlg: IFileDialog; // initialize it somewhere
	///
	/// file_dlg.SetFileTypes(&[
	///     ("Documents", "*.docx;*.txt"),
	///     ("Images", "*.jpg;*.png;*.bmp"),
	///     ("All files", "*.*"),
	/// ]).unwrap();
	/// ```
	pub fn SetFileTypes(&self, filterSpec: &[(&str, &str)]) -> WinResult<()> {
		let mut namesBuf = Vec::with_capacity(filterSpec.len());
		let mut specsBuf = Vec::with_capacity(filterSpec.len());
		let mut comDlgs = Vec::with_capacity(filterSpec.len());

		for (name, spec) in filterSpec.iter() {
			namesBuf.push(WString::from_str(name));
			specsBuf.push(WString::from_str(spec));
			comDlgs.push(COMDLG_FILTERSPEC::default());
		}

		for i in 0..filterSpec.len() {
			comDlgs[i].set_pszName(&namesBuf[i]);
			comDlgs[i].set_pszSpec(&specsBuf[i]);
		}

		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetFileTypes)(
					self.ppv(),
					filterSpec.len() as u32,
					comDlgs.as_ptr() as *mut _,
				)
			},
		)
	}

	/// [`IFileDialog::SetFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfolder)
	/// method.
	pub fn SetFolder(&self, si: &IShellItem) -> WinResult<()> {
		hr_to_winresult(
			unsafe { ((**self.ppv()).SetFolder)(self.ppv(), si.IUnknown.ppv()) },
		)
	}

	/// [`IFileDialog::SetOkButtonLabel`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setokbuttonlabel)
	/// method.
	pub fn SetOkButtonLabel(&self, text: &str) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetOkButtonLabel)(
					self.ppv(),
					WString::from_str(text).as_ptr(),
				)
			},
		)
	}

	/// [`IFileDialog::SetOptions`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setoptions)
	/// method.
	pub fn SetOptions(&self, opts: co::FOS) -> WinResult<()> {
		hr_to_winresult(
			unsafe { ((**self.ppv()).SetOptions)(self.ppv(), opts.0) },
		)
	}

	/// [`IFileDialog::SetTitle`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-settitle)
	/// method.
	pub fn SetTitle(&self, text: &str) -> WinResult<()> {
		hr_to_winresult(
			unsafe {
				((**self.ppv()).SetTitle)(
					self.ppv(),
					WString::from_str(text).as_ptr(),
				)
			},
		)
	}
}
