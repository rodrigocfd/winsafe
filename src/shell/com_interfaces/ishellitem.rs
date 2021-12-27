#![allow(non_snake_case)]

use crate::{co, shell};
use crate::ffi_types::{HRES, PCVOID, PSTR, PVOID};
use crate::kernel::decl::WString;
use crate::ole::decl::{ComPtr, CoTaskMemFree, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::OleIUnknown;
use crate::vt::IUnknownVT;

/// [`IShellItem`](crate::IShellItem) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct IShellItemVT {
	pub IUnknownVT: IUnknownVT,
	pub BindToHandler: fn(ComPtr, PVOID, PCVOID, PCVOID, *mut ComPtr) -> HRES,
	pub GetParent: fn(ComPtr, *mut ComPtr) -> HRES,
	pub GetDisplayName: fn(ComPtr, u32, *mut PSTR) -> HRES,
	pub GetAttributes: fn(ComPtr, u32, *mut u32) -> HRES,
	pub Compare: fn(ComPtr, PVOID, u32, *mut i32) -> HRES,
}

/// [`IShellItem`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem)
/// COM interface over [`IShellItemVT`](crate::vt::IShellItemVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub struct IShellItem(ComPtr);

impl_iunknown!(IShellItem, 0x43826d1e, 0xe718, 0x42ee, 0xbc55, 0xa1e261c37bfe);
impl ShellIShellItem for IShellItem {}

/// [`IShellItem`](crate::IShellItem) methods from `shell` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait ShellIShellItem: OleIUnknown {
	/// Calls
	/// [`SHCreateItemFromParsingName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-shcreateitemfromparsingname)
	/// function to create a new shell item, using the given folder or file path.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IShellItem;
	///
	/// let shi = IShellItem::from_path("C:\\Temp\\test.txt")?;
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	fn from_path(file_or_folder_path: &str) -> HrResult<IShellItem> {
		let mut ppv_queried = ComPtr::null();
		ok_to_hrresult(
			unsafe {
				shell::ffi::SHCreateItemFromParsingName(
					WString::from_str(file_or_folder_path).as_ptr(),
					std::ptr::null_mut(),
					&Self::IID as *const _ as _,
					&mut ppv_queried as *mut _ as _,
				)
			},
		).map(|_| IShellItem::from(ppv_queried))
	}

	/// [`IShellItem::GetAttributes`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getattributes)
	/// method.
	fn GetAttributes(&self, sfgao_mask: co::SFGAO) -> HrResult<co::SFGAO> {
		let mut attrs = u32::default();
		match co::HRESULT(
			unsafe {
				let vt = &**(self.ptr().0 as *mut *mut IShellItemVT);
				(vt.GetAttributes)(self.ptr(), sfgao_mask.0, &mut attrs)
			},
		) {
			co::HRESULT::S_OK
			| co::HRESULT::S_FALSE => Ok(co::SFGAO(attrs)),
			hr => Err(hr),
		}
	}

	/// [`IShellItem::GetDisplayName`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getdisplayname)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IShellItem};
	///
	/// let shi = IShellItem::from_path("C:\\Temp\\test.txt")?;
	/// let full_path = shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
	/// println!("{}", full_path);
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	fn GetDisplayName(&self, sigdn_name: co::SIGDN) -> HrResult<String> {
		let mut pstr: *mut u16 = std::ptr::null_mut();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellItemVT);
			ok_to_hrresult(
				(vt.GetDisplayName)(self.ptr(), sigdn_name.0, &mut pstr),
			)
		}.map(|_| {
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
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IShellItem};
	///
	/// let shi = IShellItem::from_path("C:\\Temp\\test.txt")?;
	/// let parent_shi = shi.GetParent()?;
	/// let full_path = parent_shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
	/// println!("{}", full_path);
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	fn GetParent(&self) -> HrResult<IShellItem> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellItemVT);
			ok_to_hrresult((vt.GetParent)(self.ptr(), &mut ppv_queried))
		}.map(|_| IShellItem::from(ppv_queried))
	}
}
