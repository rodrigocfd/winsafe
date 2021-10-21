#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::com::shell;
use crate::com::shell::ishellitem::{IShellItemT, IShellItem};
use crate::ffi::{HRESULT, PCVOID, PVOID};
use crate::privs::hr_to_winresult;

/// [`IShellItemArray`](crate::shell::IShellItemArray) virtual table.
pub struct IShellItemArrayVT {
	pub IUnknownVT: IUnknownVT,
	pub BindToHandler: fn(ComPtr, PVOID, PCVOID, PCVOID, *mut ComPtr) -> HRESULT,
	pub GetPropertyStore: fn(ComPtr, u32, PCVOID, *mut ComPtr) -> HRESULT,
	pub GetPropertyDescriptionList: fn(ComPtr, PVOID, PCVOID, *mut ComPtr) -> HRESULT,
	pub GetAttributes: fn(ComPtr, u32, u32, PVOID) -> HRESULT,
	pub GetCount: fn(ComPtr, *mut u32) -> HRESULT,
	pub GetItemAt: fn(ComPtr, u32, *mut ComPtr) -> HRESULT,
	pub EnumItems: fn(ComPtr, *mut PVOID) -> HRESULT,
}

/// [`IShellItemArray`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitemarray)
/// COM interface over
/// [`IShellItemArrayVT`](crate::shell::vt::IShellItemArrayVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
pub struct IShellItemArray(ComPtr);

impl_iunknown!(IShellItemArray, 0xb63ea76d, 0x1f85, 0x456f, 0xa19c, 0x48159efa858b);
impl IShellItemArrayT for IShellItemArray {}

/// Exposes the [`IShellItemArray`](crate::shell::IShellItemArray) methods.
pub trait IShellItemArrayT: IUnknownT {
	/// [`IShellItemArray::GetCount`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getcount)
	/// method.
	fn GetCount(&self) -> WinResult<u32> {
		let mut count = u32::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellItemArrayVT);
			hr_to_winresult((vt.GetCount)(self.ptr(), &mut count))
		}.map(|_| count)
	}

	/// Iterates through all items with
	/// [`GetCount`](crate::prelude::IShellItemArrayT::GetCount) and
	/// [`GetItemAt`](crate::prelude::IShellItemArrayT::GetItemAt), then calls
	/// [`GetDisplayName`](crate::prelude::IShellItemT::GetDisplayName) on each
	/// one of them.
	fn GetDisplayNames(&self,
		sigdn_name: shell::co::SIGDN) -> WinResult<Vec<String>>
	{
		let mut names = Vec::default();
		for i in 0..self.GetCount()? {
			let shell_item = self.GetItemAt(i)?;
			names.push(shell_item.GetDisplayName(sigdn_name)?);
		}
		Ok(names)
	}

	/// [`IShellItemArray::GetItemAt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getitemat)
	/// method.
	fn GetItemAt(&self, index: u32) -> WinResult<IShellItem> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellItemArrayVT);
			hr_to_winresult(
				(vt.GetItemAt)(self.ptr(), index, &mut ppv_queried as *mut _ as _),
			)
		}.map(|_| IShellItem::from(ppv_queried))
	}
}
