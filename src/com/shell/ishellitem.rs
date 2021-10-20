#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPVT};
use crate::ffi::{HRESULT, PCSTR, PCVOID, PSTR, PVOID};
use crate::structs::IID;

/// [`IShellItem`](crate::shell::IShellItem) virtual table.
pub struct IShellItemVT {
	pub IUnknownVT: IUnknownVT,
	pub BindToHandler: fn(PPVT, PVOID, PCVOID, PCVOID, *mut PPVT) -> HRESULT,
	pub GetParent: fn(PPVT, *mut PPVT) -> HRESULT,
	pub GetDisplayName: fn(PPVT, u32, *mut PSTR) -> HRESULT,
	pub GetAttributes: fn(PPVT, u32, *mut u32) -> HRESULT,
	pub Compare: fn(PPVT, PVOID, u32, *mut i32) -> HRESULT,
}

#[link(name = "shell32")]
extern "system" {
	pub(crate) fn SHCreateItemFromParsingName(_: PCSTR, _: PVOID, _: PCVOID, _: *mut PVOID) -> HRESULT;
}

/// [`IShellItem`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem)
/// COM interface over [`IShellItemVT`](crate::shell::vt::IShellItemVT).
/// Inherits from [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IShellItem {
	pub(crate) ppvt: PPVT,
}

impl ComInterface for IShellItem {
	const IID: IID = IID::new(0x43826d1e, 0xe718, 0x42ee, 0xbc55, 0xa1e261c37bfe);
}
macro_rules! impl_IShellItem {
	($name:ty, $vt:ty) => {
		use crate::co;
		use crate::com::CoTaskMemFree;
		use crate::com::shell::co as shellco;
		use crate::various::WString;

		impl $name {
			fn ishellitem_vt(&self) -> &IShellItemVT {
				unsafe { &**(self.ppvt as *mut *mut _) }
			}

			/// Calls
			/// [`SHCreateItemFromParsingName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromparsingname)
			/// function to create a new shell item, using the given folder or file path.
			///
			/// # Examples
			///
			/// ```rust,ignore
			/// use winsafe::prelude::*;
			/// use winsafe::shell;
			///
			/// let shi = shell::IShellItem::from_path("C:\\Temp\\test.txt")?;
			/// ```
			pub fn from_path(file_or_folder_path: &str) -> WinResult<IShellItem> {
				let mut ppv_queried: PPVT = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						SHCreateItemFromParsingName(
							WString::from_str(file_or_folder_path).as_ptr(),
							std::ptr::null_mut(),
							&IShellItem::IID as *const _ as _,
							&mut ppv_queried as *mut _ as _,
						)
					},
				).map(|_| IShellItem::from(ppv_queried))
			}

			/// [`IShellItem::GetAttributes`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getattributes)
			/// method.
			pub fn GetAttributes(&self,
				sfgao_mask: shellco::SFGAO) -> WinResult<shellco::SFGAO>
			{
				let mut attrs: u32 = 0;
				match co::ERROR(
					(self.ishellitem_vt().GetAttributes)(
						self.ppvt,
						sfgao_mask.0,
						&mut attrs,
					) as _,
				) {
					co::ERROR::S_OK | co::ERROR::S_FALSE => Ok(shellco::SFGAO(attrs)),
					err => Err(err),
				}
			}

			/// [`IShellItem::GetDisplayName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getdisplayname)
			/// method.
			///
			/// # Examples
			///
			/// ```rust,ignore
			/// use winsafe::prelude::*;
			/// use winsafe::{co, shell};
			///
			/// let shi = shell::IShellItem::from_path("C:\\Temp\\test.txt")?;
			/// let full_path = shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
			/// println!("{}", full_path);
			/// ```
			pub fn GetDisplayName(&self,
				sigdn_name: shellco::SIGDN) -> WinResult<String>
			{
				let mut pstr: *mut u16 = std::ptr::null_mut();
				hr_to_winresult(
					(self.ishellitem_vt().GetDisplayName)(
						self.ppvt,
						sigdn_name.0,
						&mut pstr,
					),
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
			/// use winsafe::prelude::*;
			/// use winsafe::{co, shell};
			///
			/// let shi = shell::IShellItem::from_path("C:\\Temp\\test.txt")?;
			/// let parent_shi = shi.GetParent()?;
			/// let full_path = parent_shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
			/// println!("{}", full_path);
			/// ```
			pub fn GetParent(&self) -> WinResult<IShellItem> {
				let mut ppv_queried: PPVT = std::ptr::null_mut();
				hr_to_winresult(
					(self.ishellitem_vt().GetParent)(
						self.ppvt,
						&mut ppv_queried as *mut _ as _,
					),
				).map(|_| IShellItem::from(ppv_queried))
			}
		}
	};
}

impl_IUnknown!(IShellItem, IShellItemVT);
impl_IShellItem!(IShellItem, IShellItemVT);
