#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::com::{ComVT, IUnknownVT, PPComVT};
use crate::com::funcs::CoTaskMemFree;
use crate::com::shell::vt::IShellItemVT;
use crate::ffi::shell32;
use crate::privs::{hr_to_winresult, ref_as_pcvoid};
use crate::WString;

macro_rules! IShellItem_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
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
				let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						shell32::SHCreateItemFromParsingName(
							WString::from_str(file_or_folder_path).as_ptr(),
							std::ptr::null_mut(),
							ref_as_pcvoid(&IShellItemVT::IID()),
							&mut ppvQueried as *mut _ as _,
						)
					},
				).map(|_| IShellItem::from(ppvQueried))
			}

			/// [`IShellItem::GetAttributes`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getattributes)
			/// method.
			pub fn GetAttributes(&self,
				sfgaoMask: co::SFGAO) -> WinResult<co::SFGAO>
			{
				let ppvt = unsafe { self.ppvt::<IShellItemVT>() };
				let mut attrs: u32 = 0;
				match co::ERROR(
					unsafe {
						((**ppvt).GetAttributes)(ppvt, sfgaoMask.0, &mut attrs)
					} as _,
				) {
					co::ERROR::S_OK | co::ERROR::S_FALSE => Ok(co::SFGAO(attrs)),
					err => Err(err),
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
				let ppvt = unsafe { self.ppvt::<IShellItemVT>() };
				let mut pstr: *mut u16 = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						((**ppvt).GetDisplayName)(ppvt, sigdnName.0, &mut pstr)
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
				let ppvt = unsafe { self.ppvt::<IShellItemVT>() };
				let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						((**ppvt).GetParent)(ppvt, &mut ppvQueried as *mut _ as _)
					},
				).map(|_| IShellItem::from(ppvQueried))
			}
		}
	};
}

IShellItem_impl! {
	/// [`IShellItem`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem)
	/// COM interface over [`IShellItemVT`](crate::shell::vt::IShellItemVT).
	/// Inherits from [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IShellItem, IShellItemVT
}
