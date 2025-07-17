#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::{iterators::*, vts::*};

com_interface! { IShellItemArray: "b63ea76d-1f85-456f-a19c-48159efa858b";
	/// [`IShellItemArray`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitemarray)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
}

impl shell_IShellItemArray for IShellItemArray {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellItemArray`](crate::IShellItemArray).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IShellItemArray: ole_IUnknown {
	/// Returns an iterator over the [`IShellItem`](crate::IShellItem) elements
	/// by calling
	/// [`IShellItemArray::GetCount`](crate::prelude::shell_IShellItemArray::GetCount)
	/// and
	/// [`IShellItemArray::GetItemAt`](crate::prelude::shell_IShellItemArray::GetItemAt)
	/// consecutively.
	///
	/// # Examples
	///
	/// Iterating over the [`IShellItem`](crate::IShellItem) objects:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let ish_arr: w::IShellItemArray; // initialized somewhere
	/// # let ish_arr = unsafe { w::IShellItemArray::null() };
	///
	/// for ish_item in ish_arr.iter()? {
	///     let ish_item = ish_item?;
	///     println!("Path: {}",
	///         ish_item.GetDisplayName(co::SIGDN::FILESYSPATH)?);
	/// }
	/// # w::HrResult::Ok(())
	/// ```
	///
	/// Collecting the file paths into a [`Vec`](std::vec::Vec):
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let ish_arr: w::IShellItemArray; // initialized somewhere
	/// # let ish_arr = unsafe { w::IShellItemArray::null() };
	///
	/// let paths = ish_arr.iter()?
	///     .map(|shi| {
	///         let shi = shi?;
	///         let name = shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
	///         Ok(name)
	///     })
	///     .collect::<w::HrResult<Vec<_>>>()?;
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	fn iter(&self) -> HrResult<impl DoubleEndedIterator<Item = HrResult<IShellItem>> + '_> {
		Ok(IshellitemarrayIter::new(self)?)
	}

	/// [`IShellItemArray::GetCount`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getcount)
	/// method.
	#[must_use]
	fn GetCount(&self) -> HrResult<u32> {
		let mut count = 0u32;
		HrRet(unsafe { (vt::<IShellItemArrayVT>(self).GetCount)(self.ptr(), &mut count) })
			.to_hrresult()
			.map(|_| count)
	}

	/// [`IShellItemArray::GetItemAt`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitemarray-getitemat)
	/// method.
	///
	/// Prefer using
	/// [`IShellItemArrayT::iter`](crate::prelude::shell_IShellItemArray::iter).
	#[must_use]
	fn GetItemAt(&self, index: u32) -> HrResult<IShellItem> {
		let mut queried = unsafe { IShellItem::null() };
		HrRet(unsafe {
			(vt::<IShellItemArrayVT>(self).GetItemAt)(self.ptr(), index, queried.as_mut())
		})
		.to_hrresult()
		.map(|_| queried)
	}
}
