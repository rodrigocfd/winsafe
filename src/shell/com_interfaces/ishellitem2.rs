#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::shell::vts::*;

com_interface! { IShellItem2: "7e9fb0d3-919f-4307-ab2e-9b1860310c93";
	/// [`IShellItem2`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitem2)
	/// COM interface.
	///
	/// Automatically calls
	/// [`IUnknown::Release`](https://learn.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
	/// when the object goes out of scope.
	///
	/// # Examples
	///
	/// Direct creation with
	/// [`SHCreateItemFromParsingName`](crate::SHCreateItemFromParsingName):
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let item2 = w::SHCreateItemFromParsingName::<w::IShellItem2>(
	///     "C:\\Temp\\foo.txt",
	///     None::<&w::IBindCtx>,
	/// )?;
	/// # w::HrResult::Ok(())
	/// ```
	///
	/// Queried from [`IShellItem`](crate::IShellItem):
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let item = w::SHCreateItemFromParsingName::<w::IShellItem>(
	///     "C:\\Temp\\foo.txt",
	///     None::<&w::IBindCtx>,
	/// )?;
	///
	/// let item2 = item.QueryInterface::<w::IShellItem2>()?;
	/// # w::HrResult::Ok(())
	/// ```
}

impl shell_IShellItem for IShellItem2 {}
impl shell_IShellItem2 for IShellItem2 {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IShellItem2`](crate::IShellItem2).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait shell_IShellItem2: shell_IShellItem {
	/// [`IShellItem2::GetBool`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getbool)
	/// method.
	///
	/// Usually `key` is a [`co::PKEY`](crate::co::PKEY) constant.
	#[must_use]
	fn GetBool(&self, key: &impl AsRef<PROPERTYKEY>) -> HrResult<bool> {
		let mut f = 0;
		ok_to_hrresult(unsafe {
			(vt::<IShellItem2VT>(self).GetBool)(self.ptr(), pcvoid(key), &mut f)
		})
		.map(|_| f != 0)
	}

	/// [`IShellItem2::GetFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getfiletime)
	/// method.
	///
	/// Usually `key` is a [`co::PKEY`](crate::co::PKEY) constant.
	#[must_use]
	fn GetFileTime(&self, key: &impl AsRef<PROPERTYKEY>) -> HrResult<FILETIME> {
		let mut ft = FILETIME::default();
		ok_to_hrresult(unsafe {
			(vt::<IShellItem2VT>(self).GetFileTime)(self.ptr(), pcvoid(key), pvoid(&mut ft))
		})
		.map(|_| ft)
	}

	/// [`IShellItem2::GetInt32`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getint32)
	/// method.
	///
	/// Usually `key` is a [`co::PKEY`](crate::co::PKEY) constant.
	#[must_use]
	fn GetInt32(&self, key: &impl AsRef<PROPERTYKEY>) -> HrResult<i32> {
		let mut i = i32::default();
		ok_to_hrresult(unsafe {
			(vt::<IShellItem2VT>(self).GetInt32)(self.ptr(), pcvoid(key), &mut i)
		})
		.map(|_| i)
	}

	/// [`IShellItem2::GetPropertyStore`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getpropertystore)
	/// method.
	#[must_use]
	fn GetPropertyStore(&self, flags: co::GPS) -> HrResult<IPropertyStore> {
		let mut queried = unsafe { IPropertyStore::null() };
		ok_to_hrresult(unsafe {
			(vt::<IShellItem2VT>(self).GetPropertyStore)(
				self.ptr(),
				flags.raw(),
				pcvoid(&IPropertyStore::IID),
				queried.as_mut(),
			)
		})
		.map(|_| queried)
	}

	/// [`IShellItem2::GetUInt32`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getuint32)
	/// method.
	///
	/// Usually `key` is a [`co::PKEY`](crate::co::PKEY) constant.
	#[must_use]
	fn GetUInt32(&self, key: &impl AsRef<PROPERTYKEY>) -> HrResult<u32> {
		let mut ui = u32::default();
		ok_to_hrresult(unsafe {
			(vt::<IShellItem2VT>(self).GetUInt32)(self.ptr(), pcvoid(key), &mut ui)
		})
		.map(|_| ui)
	}

	/// [`IShellItem2::GetUInt64`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getuint64)
	/// method.
	///
	/// Usually `key` is a [`co::PKEY`](crate::co::PKEY) constant.
	#[must_use]
	fn GetUInt64(&self, key: &impl AsRef<PROPERTYKEY>) -> HrResult<u64> {
		let mut ull = u64::default();
		ok_to_hrresult(unsafe {
			(vt::<IShellItem2VT>(self).GetUInt64)(self.ptr(), pcvoid(key), &mut ull)
		})
		.map(|_| ull)
	}

	/// [`IShellItem2::Update`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-update)
	/// method.
	fn Update(&self, pbc: &impl ole_IBindCtx) -> HrResult<()> {
		ok_to_hrresult(unsafe { (vt::<IShellItem2VT>(self).Update)(self.ptr(), pbc.ptr()) })
	}
}
