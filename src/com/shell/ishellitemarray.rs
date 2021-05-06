#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::{IUnknownVT, PPComVT};
use crate::com::shell::IShellItem;
use crate::com::shell::vt::{IShellItemArrayVT, IShellItemVT};
use crate::privs::hr_to_winresult;

macro_rules! IShellItemArray_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ident
	) => {
		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			/// [`IShellItemArray::GetCount`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getcount)
			/// method.
			pub fn GetCount(&self) -> WinResult<u32> {
				let ppvt = unsafe { self.ppvt::<IShellItemArrayVT>() };
				let mut count: u32 = 0;
				hr_to_winresult(
					unsafe { ((**ppvt).GetCount)(ppvt, &mut count) },
				).map(|_| count)
			}

			/// [`IShellItemArray::GetItemAt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getitemat)
			/// method.
			pub fn GetItemAt(&self, dwIndex: u32) -> WinResult<IShellItem> {
				let ppvt = unsafe { self.ppvt::<IShellItemArrayVT>() };
				let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();
				hr_to_winresult(
					unsafe {
						((**ppvt).GetItemAt)(ppvt,
							dwIndex,
							&mut ppvQueried as *mut _ as _,
						)
					},
				).map(|_| IShellItem::from(ppvQueried))
			}
		}
	};
}

IShellItemArray_impl! {
	/// [`IShellItemArray`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitemarray)
	/// COM interface over
	/// [`IShellItemArrayVT`](crate::shell::vt::IShellItemArrayVT). Inherits
	/// from [`IUnknown`](crate::IUnknown).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	IShellItemArray, IShellItemArrayVT
}
