#![allow(non_snake_case)]

macro_rules! IShellItemArray_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::com::shell::IShellItem;
		use crate::com::shell::vt::{IShellItemVT, IShellItemArrayVT};

		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			ppvt_conv!(ishellitemarray_vt, IShellItemArrayVT);

			/// [`IShellItemArray::GetCount`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getcount)
			/// method.
			pub fn GetCount(&self) -> WinResult<u32> {
				let mut count: u32 = 0;
				hr_to_winresult(
					(self.ishellitemarray_vt().GetCount)(
						self.ppvt,
						&mut count,
					),
				).map(|_| count)
			}

			/// [`IShellItemArray::GetItemAt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getitemat)
			/// method.
			pub fn GetItemAt(&self, dwIndex: u32) -> WinResult<IShellItem> {
				let mut ppvQueried: PPComVT<IShellItemVT> = std::ptr::null_mut();
				hr_to_winresult(
					(self.ishellitemarray_vt().GetItemAt)(
						self.ppvt,
						dwIndex,
						&mut ppvQueried as *mut _ as _,
					),
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
	IShellItemArray, crate::com::shell::vt::IShellItemArrayVT
}
