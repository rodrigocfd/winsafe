#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::kernel::decl::FILETIME;
use crate::kernel::ffi_types::BOOL;
use crate::ole::decl::HrResult;
use crate::ole::privs::{ok_to_hrresult, vt};
use crate::oleaut::decl::{IPropertyStore, PROPERTYKEY};
use crate::prelude::{ole_IUnknown, shell_IShellItem2};
use crate::shell::decl::IShellItem2;
use crate::vt::IShellItem2VT;

impl oleaut_shell_IShellItem2 for IShellItem2 {}

/// This trait is enabled with `oleaut` and `shell` features, and provides
/// methods for [`IShellItem2`](crate::IShellItem2).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait oleaut_shell_IShellItem2: shell_IShellItem2 {
	/// [`IShellItem2::GetBool`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getbool)
	/// method.
	#[must_use]
	fn GetBool(&self, key: &PROPERTYKEY) -> HrResult<bool> {
		let mut f: BOOL = 0;
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetBool)(
					self.ptr(),
					key as *const _ as _,
					&mut f,
				)
			},
		).map(|_| f != 0)
	}

	/// [`IShellItem2::GetFileTime`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getfiletime)
	/// method.
	#[must_use]
	fn GetFileTime(&self, key: &PROPERTYKEY) -> HrResult<FILETIME> {
		let mut ft = FILETIME::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetFileTime)(
					self.ptr(),
					key as *const _ as _,
					&mut ft as *mut _ as _,
				)
			},
		).map(|_| ft)
	}

	/// [`IShellItem2::GetInt32`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getint32)
	/// method.
	#[must_use]
	fn GetInt32(&self, key: &PROPERTYKEY) -> HrResult<i32> {
		let mut i = i32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetInt32)(
					self.ptr(),
					key as *const _ as _,
					&mut i,
				)
			},
		).map(|_| i)
	}

	/// [`IShellItem2::GetPropertyStore`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getpropertystore)
	/// method.
	#[must_use]
	fn GetPropertyStore(&self, flags: co::GPS) -> HrResult<IPropertyStore> {
		let mut queried = unsafe { IPropertyStore::null() };
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetPropertyStore)(
					self.ptr(),
					flags.raw(),
					&IPropertyStore::IID as *const _ as _,
					queried.as_mut(),
				)
			},
		).map(|_| queried)
	}

	/// [`IShellItem2::GetUInt32`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getuint32)
	/// method.
	#[must_use]
	fn GetUInt32(&self, key: &PROPERTYKEY) -> HrResult<u32> {
		let mut ui = u32::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetUInt32)(
					self.ptr(),
					key as *const _ as _,
					&mut ui,
				)
			},
		).map(|_| ui)
	}

	/// [`IShellItem2::GetUInt64`](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ishellitem2-getuint64)
	/// method.
	#[must_use]
	fn GetUInt64(&self, key: &PROPERTYKEY) -> HrResult<u64> {
		let mut ull = u64::default();
		ok_to_hrresult(
			unsafe {
				(vt::<IShellItem2VT>(self).GetUInt64)(
					self.ptr(),
					key as *const _ as _,
					&mut ull,
				)
			},
		).map(|_| ull)
	}
}
