#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::invalidate_handle;
use crate::prelude::Handle;
use crate::user;
use crate::user::decl::ACCEL;

impl_handle! { HACCEL: "user";
	/// Handle to an
	/// [accelerator table](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#haccel).
}

impl user_Haccel for HACCEL {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HACCEL`](crate::HACCEL).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub trait user_Haccel: Handle {
	/// [`CreateAcceleratorTable`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createacceleratortablew)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HACCEL::DestroyAcceleratorTable`](crate::prelude::user_Haccel::DestroyAcceleratorTable)
	/// call.
	#[must_use]
	fn CreateAcceleratorTable(accel: &mut [ACCEL]) -> SysResult<HACCEL> {
		unsafe {
			user::ffi::CreateAcceleratorTableW(
				accel.as_mut_ptr() as _,
				accel.len() as _,
			).as_mut()
		}.map(|ptr| HACCEL(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DestroyAcceleratorTable`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyacceleratortable)
	/// method.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	fn DestroyAcceleratorTable(&self) -> bool {
		let ret = unsafe {
			user::ffi::DestroyAcceleratorTable(self.as_ptr()) != 0
		};
		invalidate_handle(self);
		ret
	}
}
