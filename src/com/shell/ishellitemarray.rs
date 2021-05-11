#![allow(non_snake_case)]

macro_rules! IShellItemArray_impl {
	(
		$(#[$doc:meta])*
		$name:ident, $vt:ty
	) => {
		use crate::co;
		use crate::com::shell::IShellItem;
		use crate::com::shell::vt::{IShellItemVT, IShellItemArrayVT};

		IUnknown_impl! {
			$(#[$doc])*
			$name, $vt
		}

		impl $name {
			fn ishellitemarray_vt(&self) -> &IShellItemArrayVT {
				unsafe { &**(self.ppvt as PPComVT<_>) }
			}

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

			/// Iterates through all items with
			/// [`GetCount`](crate::shell::IShellItemArray::GetCount) and
			/// [`GetItemAt`](crate::shell::IShellItemArray::GetItemAt), then
			/// calls
			/// [`GetDisplayName`](crate::shell::IShellItem::GetDisplayName) on
			/// each one of them.
			pub fn GetDisplayNames(&self,
				sigdnName: co::SIGDN) -> WinResult<Vec<String>>
			{
				let mut names = Vec::default();
				for i in 0..self.GetCount()? {
					let shellItem = self.GetItemAt(i)?;
					names.push(shellItem.GetDisplayName(sigdnName)?);
				}
				Ok(names)
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
