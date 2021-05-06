#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::funcs::CoTaskMemFree;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::shell::{COMDLG_FILTERSPEC, IShellItem};
use crate::com::shell::vt::{IFileDialogVT, IModalWindowVT, IShellItemVT};
use crate::funcs::HRESULT_FROM_WIN32;
use crate::handles::HWND;
use crate::privs::{hr_to_winresult, ref_as_pcvoid};
use crate::structs::GUID;
use crate::WString;

macro_rules! IFileDialog_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IModalWindow_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IFileDialog::ClearClientData`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-clearclientdata)
			/// method.
			pub fn ClearClientData(&self) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(unsafe { ((**ppvt).ClearClientData)(ppvt) })
			}

			/// [`IFileDialog::Close`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-close)
			/// method.
			pub fn Close(&self, hr: co::ERROR) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe { ((**ppvt).Close)(ppvt, hr.0 as _) },
				)
			}

			/// [`IFileDialog::GetCurrentSelection`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getcurrentselection)
			/// method.
			pub fn GetCurrentSelection(&self) -> WinResult<IShellItem> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						((**ppvt).GetCurrentSelection)(
							ppvt,
							&mut ppvQueried as *mut _ as _,
						)
					},
				).map(|_| IShellItem::from(ppvQueried))
			}

			/// [`IFileDialog::GetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfilename)
			/// method.
			pub fn GetFileName(&self) -> WinResult<String> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				let mut pstr: *mut u16 = std::ptr::null_mut();
				hr_to_winresult(
					unsafe { ((**ppvt).GetFileName)(ppvt, &mut pstr) },
				).map(|_| {
					let name = WString::from_wchars_nullt(pstr);
					CoTaskMemFree(pstr);
					name.to_string()
				})
			}

			/// [`IFileDialog::GetFileTypeIndex`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfiletypeindex)
			/// method.
			pub fn GetFileTypeIndex(&self) -> WinResult<u32> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				let mut index: u32 = 0;
				hr_to_winresult(
					unsafe { ((**ppvt).GetFileTypeIndex)(ppvt, &mut index) },
				).map(|_| index)
			}

			/// [`IFileDialog::GetFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getfolder)
			/// method.
			pub fn GetFolder(&self) -> WinResult<IShellItem> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						((**ppvt).GetFolder)(ppvt, &mut ppvQueried as *mut _ as _)
					},
				).map(|_| IShellItem::from(ppvQueried))
			}

			/// [`IFileDialog::GetOptions`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getoptions)
			/// method.
			pub fn GetOptions(&self) -> WinResult<co::FOS> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				let mut opts: u32 = 0;
				hr_to_winresult(
					unsafe { ((**ppvt).GetOptions)(ppvt, &mut opts) },
				).map(|_| co::FOS(opts))
			}

			/// [`IFileDialog::GetResult`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-getresult)
			/// method.
			pub fn GetResult(&self) -> WinResult<IShellItem> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						((**ppvt).GetResult)(ppvt, &mut ppvQueried as *mut _ as _)
					},
				).map(|_| IShellItem::from(ppvQueried))
			}

			/// [`IFileDialog::SetClientGuid`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setclientguid)
			/// method.
			pub fn SetClientGuid(&self, guid: &GUID) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetClientGuid)(ppvt, ref_as_pcvoid(guid))
					},
				)
			}

			/// [`IFileDialog::SetDefaultExtension`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultextension)
			/// method.
			pub fn SetDefaultExtension(&self,
				defaultExtension: &str) -> WinResult<()>
			{
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetDefaultExtension)(
							ppvt,
							WString::from_str(defaultExtension).as_ptr(),
						)
					},
				)
			}

			/// [`IFileDialog::SetDefaultFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setdefaultfolder)
			/// method.
			pub fn SetDefaultFolder(&self, si: &IShellItem) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetDefaultFolder)(ppvt, si.ppvt())
					},
				)
			}

			/// [`IFileDialog::SetFileName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilename)
			/// method.
			pub fn SetFileName(&self, name: &str) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetFileName)(
							ppvt,
							WString::from_str(name).as_ptr(),
						)
					},
				)
			}

			/// [`IFileDialog::SetFileNameLabel`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfilenamelabel)
			/// method.
			pub fn SetFileNameLabel(&self, label: &str) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetFileNameLabel)(
							ppvt,
							WString::from_str(label).as_ptr(),
						)
					},
				)
			}

			/// [`IFileDialog::SetFileTypeIndex`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfiletypeindex)
			/// method.
			pub fn SetFileTypeIndex(&self, iFileType: u32) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe { ((**ppvt).SetFileTypeIndex)(ppvt, iFileType) },
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

				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetFileTypes)(
							ppvt,
							filterSpec.len() as _,
							comDlgs.as_ptr() as _,
						)
					},
				)
			}

			/// [`IFileDialog::SetFolder`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setfolder)
			/// method.
			pub fn SetFolder(&self, si: &IShellItem) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe { ((**ppvt).SetFolder)(ppvt, si.ppvt()) },
				)
			}

			/// [`IFileDialog::SetOkButtonLabel`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setokbuttonlabel)
			/// method.
			pub fn SetOkButtonLabel(&self, text: &str) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetOkButtonLabel)(
							ppvt,
							WString::from_str(text).as_ptr(),
						)
					},
				)
			}

			/// [`IFileDialog::SetOptions`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-setoptions)
			/// method.
			pub fn SetOptions(&self, opts: co::FOS) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe { ((**ppvt).SetOptions)(ppvt, opts.0) },
				)
			}

			/// [`IFileDialog::SetTitle`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifiledialog-settitle)
			/// method.
			pub fn SetTitle(&self, text: &str) -> WinResult<()> {
				let ppvt = unsafe { self.ppvt::<IFileDialogVT>() };
				hr_to_winresult(
					unsafe {
						((**ppvt).SetTitle)(
							ppvt,
							WString::from_str(text).as_ptr(),
						)
					},
				)
			}
		}
	};
}

IFileDialog_impl! {
	/// [`IFileDialog`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ifiledialog)
	/// COM interface over [`IFileDialogVT`](crate::shell::vt::IFileDialogVT).
	/// Inherits from [`IModalWindow`](crate::shell::IModalWindow),
	/// [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IFileDialog, IFileDialogVT
}
