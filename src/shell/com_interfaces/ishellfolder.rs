#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::vts::*;

com_interface! { IShellFolder: "000214e6-0000-0000-c000-000000000046";
	/// [`IShellFolder`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellfolder)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let _com_guard = w::CoInitializeEx(
	///     co::COINIT::APARTMENTTHREADED | co::COINIT::DISABLE_OLE1DDE)?;
	///
	/// let f = w::SHCreateItemFromParsingName::<w::IShellItem>(
	///     "C:\\Temp",
	///     None::<&w::IBindCtx>,
	/// )?;
	///
	/// let f2 = f.BindToHandler::<w::IShellFolder>(
	///     None::<&w::IBindCtx>,
	///     &co::BHID::SFObject,
	/// )?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl shell_IShellFolder for IShellFolder {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellFolder`](crate::IShellFolder).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IShellFolder: ole_IUnknown {
	/// [`IShellFolder::BindToObject`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellfolder-bindtoobject)
	/// method.
	#[must_use]
	fn BindToObject<T>(&self, pidl: &PIDL, bind_ctx: Option<&impl ole_IBindCtx>) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(unsafe {
			(vt::<IShellFolderVT>(self).BindToObject)(
				self.ptr(),
				pidl.ptr() as _,
				bind_ctx.map_or(std::ptr::null_mut(), |p| p.ptr()),
				pcvoid(&T::IID),
				queried.as_mut(),
			)
		})
		.map(|_| queried)
	}

	/// [`IShellFolder::BindToStorage`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellfolder-bindtostorage)
	/// method.
	#[must_use]
	fn BindToStorage<T>(&self, pidl: &PIDL, bind_ctx: Option<&impl ole_IBindCtx>) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(unsafe {
			(vt::<IShellFolderVT>(self).BindToStorage)(
				self.ptr(),
				pidl.ptr() as _,
				bind_ctx.map_or(std::ptr::null_mut(), |p| p.ptr()),
				pcvoid(&T::IID),
				queried.as_mut(),
			)
		})
		.map(|_| queried)
	}

	/// [`IShellFolder::CompareIDs`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellfolder-compareids)
	/// method.
	fn CompareIDs(
		&self,
		sorting_rule: u16,
		sorting_flags: co::SHCIDS,
		pidl1: &PIDL,
		pidl2: &PIDL,
	) -> HrResult<i32> {
		let hr = unsafe {
			co::HRESULT::from_raw({
				(vt::<IShellFolderVT>(self).CompareIDs)(
					self.ptr(),
					(sorting_rule as u32 | sorting_flags.raw()) as _,
					pidl1.ptr() as _,
					pidl2.ptr() as _,
				)
			})
		};
		if hr.SUCCEEDED() { Ok(hr.code() as _) } else { Err(hr) }
	}

	/// [`IShellFolder::CreateViewObject`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellfolder-createviewobject)
	/// method.
	#[must_use]
	fn CreateViewObject<T>(&self, hwnd_owner: &HWND) -> HrResult<T>
	where
		T: ole_IUnknown,
	{
		let mut queried = unsafe { T::null() };
		ok_to_hrresult(unsafe {
			(vt::<IShellFolderVT>(self).CreateViewObject)(
				self.ptr(),
				hwnd_owner.ptr(),
				pcvoid(&T::IID),
				queried.as_mut(),
			)
		})
		.map(|_| queried)
	}

	/// [`IShellFolder::ParseDisplayName`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellfolder-parsedisplayname)
	/// method.
	#[must_use]
	fn ParseDisplayName(
		&self,
		hwnd: Option<&HWND>,
		bind_ctx: Option<&impl ole_IBindCtx>,
		display_name: &str,
		attributes: Option<&mut co::SFGAO>,
	) -> HrResult<CoTaskMemFreePidlGuard> {
		let mut pidl = unsafe { PIDL::from_ptr(std::ptr::null_mut()) };
		let mut ch_eaten = 0u32;

		unsafe {
			ok_to_hrresult((vt::<IShellFolderVT>(self).ParseDisplayName)(
				self.ptr(),
				hwnd.map_or(std::ptr::null_mut(), |h| h.ptr()),
				bind_ctx.map_or(std::ptr::null_mut(), |p| p.ptr()),
				WString::from_str(display_name).as_ptr(),
				&mut ch_eaten,
				pvoid(&mut pidl),
				attributes.map_or(std::ptr::null_mut(), |a| a.as_mut()),
			))
			.map(|_| CoTaskMemFreePidlGuard::new(pidl))
		}
	}
}
