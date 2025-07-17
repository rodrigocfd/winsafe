#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::{iterators::*, vts::*};

com_interface! { IEnumShellItems: "70629033-e363-4a28-a567-0db78006e6d7";
	/// [`IEnumShellItems`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ienumshellitems)
	/// COM interface.
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
/// ```no_run
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
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let folder = w::SHCreateItemFromParsingName::<w::IShellItem>(
	///     "C:\\Temp",
	///     None::<&w::IBindCtx>,
	/// )?;
	///
	/// let items = folder.BindToHandler::<w::IEnumShellItems>(
	///     None::<&w::IBindCtx>,
	///     &co::BHID::EnumItems,
	/// )?;
	///
	/// for item in items.iter() {
	///     let item = item?;
	///     println!("{}", item.GetDisplayName(co::SIGDN::FILESYSPATH)?);
	/// }
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	fn iter(&self) -> impl Iterator<Item = HrResult<IShellItem>> + '_ {
		IenumshellitemsIter::new(self)
	}

	/// [`IEnumShellItems::Next`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ienumshellitems-next)
	/// method.
	///
	/// Prefer using
	/// [`IEnumShellItems::iter`](crate::prelude::shell_IEnumShellItems::iter),
	/// which is simpler.
	#[must_use]
	fn Next(&self) -> HrResult<Option<IShellItem>> {
		let mut queried = unsafe { IShellItem::null() };
		let mut fetched = 0u32;

		match HrRet(unsafe {
			(vt::<IEnumShellItemsVT>(self).Next)(
				self.ptr(),
				1, // retrieve only 1
				queried.as_mut(),
				&mut fetched,
			)
		})
		.to_hrresult()
		{
			Ok(_) => Ok(Some(queried)),
			Err(hr) => match hr {
				co::HRESULT::S_FALSE => Ok(None), // no item found
				hr => Err(hr),                    // actual error
			},
		}
	}

	fn_com_noparm! { Reset: IEnumShellItemsVT;
		/// [`IEnumShellItems::Reset`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ienumshellitems-reset)
		/// method.
	}

	/// [`IEnumShellItems::Skip`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ienumshellitems-skip)
	/// method.
	fn Skip(&self, count: u32) -> HrResult<bool> {
		HrRet(unsafe { (vt::<IEnumShellItemsVT>(self).Skip)(self.ptr(), count) }).to_bool_hrresult()
	}
}
