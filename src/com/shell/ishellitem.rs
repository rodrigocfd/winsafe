#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::{ComVT, IUnknown, IUnknownVT, PPComVT};
use crate::com::funcs::{CoTaskMemFree, hr_to_winresult};
use crate::com::shell::vt::IShellItemVT;
use crate::ffi::shell32;
use crate::funcs::HRESULT_FROM_WIN32;
use crate::structs::GUID;
use crate::WString;

/// [`IShellItem`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem)
/// COM interface. Backed by [`IShellItemVT`](crate::shell::IShellItemVT)
/// virtual table.
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IShellItem {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<IShellItemVT>> for IShellItem {
	fn from(ppv: PPComVT<IShellItemVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl IShellItem {
	unsafe fn ppv(&self) -> PPComVT<IShellItemVT> {
		self.IUnknown.ppv::<IShellItemVT>()
	}

	/// Calls
	/// [`SHCreateItemFromParsingName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromparsingname)
	/// function to create a new shell item, using the given folder or file path.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::shell;
	///
	/// let shi = shell::IShellItem::from_path("C:\\Temp\\test.txt").unwrap();
	/// ```
	pub fn from_path(file_or_folder_path: &str) -> WinResult<IShellItem> {
		let mut ppv: PPComVT<IShellItemVT> = std::ptr::null_mut();

		let hr = unsafe {
			shell32::SHCreateItemFromParsingName(
				WString::from_str(file_or_folder_path).as_ptr(),
				std::ptr::null_mut(),
				IShellItemVT::IID().as_ref() as *const GUID as *const _,
				&mut ppv
					as *mut PPComVT<IShellItemVT>
					as *mut *mut _,
			)
		};

		match HRESULT_FROM_WIN32(hr) {
			co::ERROR::S_OK => Ok(IShellItem::from(ppv)),
			_ => Err(co::ERROR(hr as u32)),
		}
	}

	/// [`IShellItem::GetAttributes`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getattributes)
	/// method.
	pub fn GetAttributes(&self, sfgaoMask: co::SFGAO) -> WinResult<co::SFGAO> {
		let mut attribs: u32 = 0;

		let hr = unsafe {
			((**self.ppv()).GetAttributes)(self.ppv(), sfgaoMask.0, &mut attribs)
		};

		match HRESULT_FROM_WIN32(hr) {
			co::ERROR::S_OK | co::ERROR::S_FALSE => Ok(co::SFGAO(attribs)),
			_ => Err(co::ERROR(hr as u32)),
		}
	}

	/// [`IShellItem::GetDisplayName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getdisplayname)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, shell};
	///
	/// let shi = shell::IShellItem::new("C:\\Temp\\test.txt").unwrap();
	/// let full_path = shi.GetDisplayName(co::SIGDN::FILESYSPATH).unwrap();
	/// println!("{}", full_path);
	/// ```
	pub fn GetDisplayName(&self, sigdnName: co::SIGDN) -> WinResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();

		hr_to_winresult(
			unsafe {
				((**self.ppv()).GetDisplayName)(self.ppv(), sigdnName.0, &mut pstr)
			},
		).map(|_| {
			let name = WString::from_wchars_nullt(pstr);
			CoTaskMemFree(pstr);
			name.to_string()
		})
	}

	/// [`IShellItem::GetParent`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getparent)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, shell};
	///
	/// let shi = shell::IShellItem::new("C:\\Temp\\test.txt").unwrap();
	/// let parent_shi = shi.GetParent().unwrap();
	/// let full_path = parent_shi.GetDisplayName(co::SIGDN::FILESYSPATH).unwrap();
	/// println!("{}", full_path);
	/// ```
	pub fn GetParent(&self) -> WinResult<IShellItem> {
		let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();

		hr_to_winresult(
			unsafe {
				((**self.ppv()).GetParent)(
					self.ppv(),
					&mut ppvQueried
						as *mut PPComVT<IShellItemVT>
						as *mut *mut _,
				)
			},
		).map(|_| IShellItem::from(ppvQueried))
	}
}
