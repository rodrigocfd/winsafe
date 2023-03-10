#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::ffi_types::HRES;
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::{ok_to_hrresult, okfalse_to_hrresult};
use crate::prelude::ole_IUnknown;
use crate::shell::decl::IShellItem;
use crate::vt::IUnknownVT;

/// [`IEnumShellItems`](crate::IEnumShellItems) virtual table.
#[repr(C)]
pub struct IEnumShellItemsVT {
	pub IUnknownVT: IUnknownVT,
	pub Next: fn(ComPtr, u32, *mut ComPtr, *mut u32) -> HRES,
	pub Skip: fn(ComPtr, u32) -> HRES,
	pub Reset: fn(ComPtr) -> HRES,
	pub Clone: fn(ComPtr, *mut ComPtr) -> HRES,
}

com_interface! { IEnumShellItems: "70629033-e363-4a28-a567-0db78006e6d7";
	/// [`IEnumShellItems`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ienumshellitems)
	/// COM interface over [`IEnumShellItemsVT`](crate::vt::IEnumShellItemsVT).
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_IEnumShellItems for IEnumShellItems {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IEnumShellItems`](crate::IEnumShellItems).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IEnumShellItems: ole_IUnknown {
	/// Returns an iterator over the [`IShellItem`](crate::IShellItem) elements
	/// which calls
	/// [`IEnumShellItems::Next`](crate::prelude::shell_IEnumShellItems::Next)
	/// internally.
	///
	/// # Examples
	///
	/// Enumerating the items in a folder by iterating over the
	/// [`IShellItem`](crate::IShellItem) objects:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IBindCtx, IEnumShellItems, IShellItem, SHCreateItemFromParsingName};
	///
	/// let folder = SHCreateItemFromParsingName::<IShellItem>(
	///     "C:\\Temp",
	///     None::<&IBindCtx>,
	/// )?;
	///
	/// let items = folder.BindToHandler::<IEnumShellItems>(
	///     None::<&IBindCtx>,
	///     &co::BHID::EnumItems,
	/// )?;
	///
	/// for item in items.iter() {
	///     let item = item?;
	///     println!("{}", item.GetDisplayName(co::SIGDN::FILESYSPATH)?);
	/// }
	/// # Ok::<_, co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter(&self) -> Box<dyn Iterator<Item = HrResult<IShellItem>> + '_> {
		Box::new(EnumShellItemsIter::new(self))
	}

	/// [`IEnumShellItems::Next`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ienumshellitems-next)
	/// method.
	///
	/// Prefer using
	/// [`IEnumShellItems::iter`](crate::prelude::shell_IEnumShellItems::iter),
	/// which is simpler.
	#[must_use]
	fn Next(&self) -> HrResult<Option<IShellItem>> {
		let mut fetched = u32::default();
		unsafe {
			let mut ppv_queried = ComPtr::null();
			let vt = self.vt_ref::<IEnumShellItemsVT>();
			match ok_to_hrresult(
				(vt.Next)(self.ptr(), 1, &mut ppv_queried, &mut fetched), // retrieve only 1
			) {
				Ok(_) => Ok(Some(IShellItem::from(ppv_queried))),
				Err(hr) => match hr {
					co::HRESULT::S_FALSE => Ok(None), // no item found
				hr => Err(hr), // actual error
				},
			}
		}
	}

	/// [`IEnumShellItems::Reset`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ienumshellitems-reset)
	/// method.
	fn Reset(&self) -> HrResult<()> {
		unsafe {
			let vt = self.vt_ref::<IEnumShellItemsVT>();
			ok_to_hrresult((vt.Reset)(self.ptr()))
		}
	}

	/// [`IEnumShellItems::Skip`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ienumshellitems-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		unsafe {
			let vt = self.vt_ref::<IEnumShellItemsVT>();
			okfalse_to_hrresult((vt.Skip)(self.ptr(), count))
		}
	}
}

//------------------------------------------------------------------------------

struct EnumShellItemsIter<'a, I>
	where I: shell_IEnumShellItems,
{
	enum_shi: &'a I,
}

impl<'a, I> Iterator for EnumShellItemsIter<'a, I>
	where I: shell_IEnumShellItems,
{
	type Item = HrResult<IShellItem>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.enum_shi.Next() {
			Err(err) => Some(Err(err)),
			Ok(maybe_item) => maybe_item.map(|item| Ok(item)),
		}
	}
}

impl<'a, I> EnumShellItemsIter<'a, I>
	where I: shell_IEnumShellItems,
{
	fn new(enum_shi: &'a I) -> Self {
		Self { enum_shi }
	}
}
