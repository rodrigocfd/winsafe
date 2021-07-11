#![allow(non_snake_case)]

use crate::com::iunknown::IUnknownVT;
use crate::com::traits::{ComInterface, PPComVT};
use crate::ffi::{HRESULT, PCVOID, PVOID};
use crate::structs::IID;

type PP = PPComVT<IUnknownVT>;

/// [`IShellItemArray`](crate::shell::IShellItemArray) virtual table.
pub struct IShellItemArrayVT {
	pub IUnknownVT: IUnknownVT,
	pub BindToHandler: fn(PP, PVOID, PCVOID, PCVOID, *mut PP) -> HRESULT,
	pub GetPropertyStore: fn(PP, u32, PCVOID, *mut PP) -> HRESULT,
	pub GetPropertyDescriptionList: fn(PP, PVOID, PCVOID, *mut PP) -> HRESULT,
	pub GetAttributes: fn(PP, u32, u32, PVOID) -> HRESULT,
	pub GetCount: fn(PP, *mut u32) -> HRESULT,
	pub GetItemAt: fn(PP, u32, *mut PP) -> HRESULT,
	pub EnumItems: fn(PP, *mut PVOID) -> HRESULT,
}

/// [`IShellItemArray`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitemarray)
/// COM interface over
/// [`IShellItemArrayVT`](crate::shell::vt::IShellItemArrayVT). Inherits
/// from [`IUnknown`](crate::IUnknown).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IShellItemArray {
	pub(crate) ppvt: PPComVT<IUnknownVT>,
}

impl_send_sync_fromppvt!(IShellItemArray);

impl ComInterface for IShellItemArray {
	const IID: IID = IID::new(0xb63ea76d, 0x1f85, 0x456f, 0xa19c, 0x48159efa858b);
}

macro_rules! impl_IShellItemArray {
	($name:ty, $vt:ty) => {
		use crate::com::shell::co as shellco;
		use crate::com::shell::IShellItem;

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
				sigdnName: shellco::SIGDN) -> WinResult<Vec<String>>
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
				let mut ppvQueried: PPComVT<IUnknownVT> = std::ptr::null_mut();
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

impl_IUnknown!(IShellItemArray, IShellItemArrayVT);
impl_IShellItemArray!(IShellItemArray, IShellItemArrayVT);
