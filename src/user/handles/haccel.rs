#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::SysResult;
use crate::kernel::privs::ptr_to_sysresult_handle;
use crate::prelude::Handle;
use crate::user;
use crate::user::decl::ACCEL;
use crate::user::guard::DestroyAcceleratorTableGuard;

impl_handle! { HACCEL;
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
pub trait user_Haccel: Handle {
	/// [`CreateAcceleratorTable`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createacceleratortablew)
	/// static method.
	#[must_use]
	fn CreateAcceleratorTable(
		accel: &mut [ACCEL]) -> SysResult<DestroyAcceleratorTableGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				user::ffi::CreateAcceleratorTableW(
					accel.as_mut_ptr() as _,
					accel.len() as _,
				),
			).map(|h| DestroyAcceleratorTableGuard::new(h))
		}
	}
}
