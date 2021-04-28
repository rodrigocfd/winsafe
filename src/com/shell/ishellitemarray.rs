#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknown, IUnknownVT, PPComVT};
use crate::com::shell::IShellItem;
use crate::com::shell::vt::{IShellItemArrayVT, IShellItemVT};
use crate::privs::hr_to_winresult;

/// [`IShellItemArray`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitemarray)
/// COM interface.
///
/// Virtual table: [`IShellItemArrayVT`](crate::shell::vt::IShellItemArrayVT).
///
/// Inherits from:
/// * [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
#[derive(Clone)]
pub struct IShellItemArray {
	/// Methods of base interface [`IUnknown`](crate::IUnknown).
	pub IUnknown: IUnknown,
}

impl From<PPComVT<IShellItemArrayVT>> for IShellItemArray {
	fn from(ppv: PPComVT<IShellItemArrayVT>) -> Self {
		Self {
			IUnknown: IUnknown::from(ppv as PPComVT<IUnknownVT>)
		}
	}
}

impl IShellItemArray {
	unsafe fn ppv(&self) -> PPComVT<IShellItemArrayVT> {
		self.IUnknown.ppv::<IShellItemArrayVT>()
	}

	/// [`IShellItemArray::GetCount`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getcount)
	/// method.
	pub fn GetCount(&self) -> WinResult<u32> {
		let mut count: u32 = 0;
		hr_to_winresult(
			unsafe { ((**self.ppv()).GetCount)(self.ppv(), &mut count) },
		).map(|_| count)
	}

	/// [`IShellItemArray::GetItemAt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getitemat)
	/// method.
	pub fn GetItemAt(&self, dwIndex: u32) -> WinResult<IShellItem> {
		let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();
		hr_to_winresult(
			unsafe {
				((**self.ppv()).GetItemAt)(
					self.ppv(),
					dwIndex,
					&mut ppvQueried as *mut _ as _,
				)
			},
		).map(|_| IShellItem::from(ppvQueried))
	}
}
