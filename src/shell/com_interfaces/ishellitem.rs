#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::vts::*;

com_interface! { IShellItem: "43826d1e-e718-42ee-bc55-a1e261c37bfe";
	/// [`IShellItem`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// Usually created with
	/// [`SHCreateItemFromParsingName`](crate::SHCreateItemFromParsingName)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let shi = w::SHCreateItemFromParsingName::<w::IShellItem>(
	///     "C:\\Temp\\foo.txt",
	///     None::<&w::IBindCtx>,
	/// )?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl shell_IShellItem for IShellItem {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellItem`](crate::IShellItem).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IShellItem: ole_IUnknown {
	/// [`IShellItem::BindToHandler`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-bindtohandler)
	/// method.
	///
	/// # Examples
	///
	/// Retrieving the items inside a directory:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let sh_folder: w::IShellItem; // initialized somewhere
	/// # let sh_folder = unsafe { w::IShellItem::null() };
	///
	/// let sh_items = sh_folder.BindToHandler::<w::IEnumShellItems>(
	///     None::<&w::IBindCtx>,
	///     &co::BHID::EnumItems,
	/// )?;
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	fn BindToHandler<T>(&self, bind_ctx: Option<&impl ole_IBindCtx>, bhid: &co::BHID) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		HrRet(unsafe {
			(vt::<IShellItemVT>(self).BindToHandler)(
				self.ptr(),
				bind_ctx.map_or(std::ptr::null_mut(), |p| p.ptr()),
				pcvoid(bhid),
				pcvoid(&T::IID),
				queried.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| queried)
	}

	/// [`IShellItem::Compare`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-compare)
	/// method.
	#[must_use]
	fn Compare(&self, other: &impl shell_IShellItem, hint: co::SICHINTF) -> HrResult<i32> {
		let mut order = 0i32;
		HrRet(unsafe {
			(vt::<IShellItemVT>(self).Compare)(self.ptr(), other.ptr(), hint.raw(), &mut order)
		})
		.to_hrresult()
		.map(|_| order)
	}

	/// [`IShellItem::GetAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getattributes)
	/// method.
	#[must_use]
	fn GetAttributes(&self, sfgao_mask: co::SFGAO) -> HrResult<co::SFGAO> {
		let mut attrs = co::SFGAO::default();
		match unsafe {
			co::HRESULT::from_raw((vt::<IShellItemVT>(self).GetAttributes)(
				self.ptr(),
				sfgao_mask.raw(),
				attrs.as_mut(),
			))
		} {
			co::HRESULT::S_OK | co::HRESULT::S_FALSE => Ok(attrs),
			hr => Err(hr),
		}
	}

	/// [`IShellItem::GetDisplayName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getdisplayname)
	/// method.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let shi = w::SHCreateItemFromParsingName::<w::IShellItem>(
	///     "C:\\Temp\\foo.txt",
	///     None::<&w::IBindCtx>,
	/// )?;
	///
	/// let full_path = shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
	///
	/// println!("{}", full_path);
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	fn GetDisplayName(&self, sigdn_name: co::SIGDN) -> HrResult<String> {
		let mut pstr = std::ptr::null_mut::<u16>();
		HrRet(unsafe {
			(vt::<IShellItemVT>(self).GetDisplayName)(self.ptr(), sigdn_name.raw(), &mut pstr)
		})
		.to_hrresult()
		.map(|_| htaskmem_ptr_to_str(pstr))
	}

	fn_com_interface_get! { GetParent: IShellItemVT => IShellItem;
		/// [`IShellItem::GetParent`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem-getparent)
		/// method.
		///
		/// # Examples
		///
		/// ```no_run
		/// use winsafe::{self as w, prelude::*, co};
		///
		/// let shi = w::SHCreateItemFromParsingName::<w::IShellItem>(
		///     "C:\\Temp\\foo.txt",
		///     None::<&w::IBindCtx>,
		/// )?;
		///
		/// let parent_shi = shi.GetParent()?;
		/// let full_path = parent_shi.GetDisplayName(co::SIGDN::FILESYSPATH)?;
		///
		/// println!("{}", full_path);
		/// # w::HrResult::Ok(())
		/// ```
	}
}
