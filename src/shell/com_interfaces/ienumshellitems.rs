#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::ffi_types::HRES;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::ole_IUnknown;
use crate::shell::decl::IShellItem;
use crate::vt::IUnknownVT;

/// [`IEnumShellItems`](crate::IEnumShellItems) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct IEnumShellItemsVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(ComPtr, u32, *mut ComPtr, *mut u32) -> HRES,
	pub Skip: fn(ComPtr, u32) -> HRES,
	pub Reset: fn(ComPtr) -> HRES,
	pub Clone: fn(ComPtr, *mut ComPtr) -> HRES,
}

/// [`IEnumShellItems`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ienumshellitems)
/// COM interface over [`IEnumShellItemsVT`](crate::vt::IEnumShellItemsVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub struct IEnumShellItems(ComPtr);

impl_iunknown!(IEnumShellItems, "70629033-e363-4a28-a567-0db78006e6d7");
impl shell_IEnumShellItems for IEnumShellItems {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IEnumShellItems`](crate::IEnumShellItems).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_IEnumShellItems: ole_IUnknown {
	/// [`IEnumShellItems::Next`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ienumshellitems-next)
	/// method.
	#[must_use]
	fn Next(&self) -> HrResult<Option<IShellItem>> {
		let mut ppv_queried = ComPtr::null();
		let mut fetched = u32::default();

		match unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IEnumShellItemsVT);
			ok_to_hrresult(
				(vt.Next)(self.ptr(), 1, &mut ppv_queried, &mut fetched),
			)
		}.map(|_| IShellItem::from(ppv_queried)) {
			Ok(filter) => Ok(Some(filter)),
			Err(hr) => match hr {
				co::HRESULT::S_FALSE => Ok(None), // no filter found
				hr => Err(hr), // actual error
			},
		}
	}

	/// [`IEnumShellItems::Reset`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ienumshellitems-reset)
	/// method.
	fn Reset(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IEnumShellItemsVT);
			ok_to_hrresult((vt.Reset)(self.ptr()))
		}
	}

	/// [`IEnumShellItems::Skip`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ienumshellitems-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IEnumShellItemsVT);
			okfalse_to_hrresult((vt.Skip)(self.ptr(), count))
		}
	}
}
