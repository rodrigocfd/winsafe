#![allow(non_snake_case)]

use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use crate::aliases::WinResult;
use crate::com::iunknown::{ComPtr, IUnknownT, IUnknownVT};
use crate::com::shell::ishellitem::IShellItem;
use crate::ffi::{HRESULT, PCVOID, PVOID};
use crate::privs::hr_to_winresult;

/// [`IShellItemArray`](crate::shell::IShellItemArray) virtual table.
#[repr(C)]
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

	/// [`IShellItemArray::GetItemAt`](https://docs.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getitemat)
	/// method.
	///
	/// Prefer using
	/// [`IShellItemArrayT::iter`](crate::prelude::IShellItemArrayT::iter).
	fn GetItemAt(&self, index: u32) -> WinResult<IShellItem> {
		let mut ppv_queried = ComPtr::null();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IShellItemArrayVT);
			hr_to_winresult((vt.GetItemAt)(self.ptr(), index, &mut ppv_queried))
		}.map(|_| IShellItem::from(ppv_queried))
	}

	/// Returns an iterator over the [`IShellItem`](crate::shell::IShellItem)
	/// elements by calling
	/// [`IShellItemArrayT::GetCount`](crate::prelude::IShellItemArrayT::GetCount)
	/// and
	/// [`IShellItemArray::GetItemAt`](crate::prelude::IShellItemArrayT::GetItemAt)
	/// consecutively.
	///
	/// # Examples
	///
	/// Iterating over the [`IShellItem`](crate::shell::IShellItem) objects:
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe:shell;
	///
	/// let ish_arr: shell::IShellItemArray; // initialized somewhere
	///
	/// for ish_item in ish_arr.iter()? {
	///     let ish_item = ish_item?;
	///     println!("Path: {}",
	///         ish_item.GetDisplayName(shell::co::SIGDN::FILESYSPATH)?);
	/// }
	/// ```
	///
	/// Collecting the file paths into a
	/// [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html):
	///
	/// ```rust,ignore
	/// use winsafe::prelude::*;
	/// use winsafe:{shell, WinResult};
	///
	/// let ish_arr: shell::IShellItemArray; // initialized somewhere
	///
	/// let paths = ish_arr.iter()?
	///     .map(|shi|
	///         shi.and_then(|shi|
	///             shi.GetDisplayName(shell::co::SIGDN::FILESYSPATH),
	///         ),
	///     )
	///     .collect::<WinResult<Vec<_>>>()?;
	/// ```
	fn iter<'a>(&'a self) -> WinResult<Box<dyn Iterator<Item = WinResult<IShellItem>> + 'a>> {
		Ok(Box::new(ShellItemIter::new(unsafe { self.ptr() })?))
	}
}

//------------------------------------------------------------------------------

struct ShellItemIter<'a> {
	array: ManuallyDrop<IShellItemArray>,
	count: u32,
	current: u32,
	owner_: PhantomData<&'a ()>,
}

impl<'a> Iterator for ShellItemIter<'a> {
	type Item = WinResult<IShellItem>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match self.array.GetItemAt(self.current) {
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
			Ok(shell_item) => {
				self.current += 1;
				Some(Ok(shell_item))
			},
		}
	}
}

impl<'a> ShellItemIter<'a> {
	fn new(com_ptr: ComPtr) -> WinResult<Self> {
		let array = ManuallyDrop::new(IShellItemArray(com_ptr));
		let count = array.GetCount()?;

		Ok(Self {
			array,
			count,
			current: 0,
			owner_: PhantomData,
		})
	}
}
