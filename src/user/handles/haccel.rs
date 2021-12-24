#![allow(non_snake_case)]

use crate::kernel::decl::{GetLastError, WinResult};
use crate::prelude::Handle;
use crate::user;
use crate::user::decl::ACCEL;

impl_handle! { HACCEL: "user";
	/// Handle to an
	/// [accelerator table](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#haccel).
}

impl UserHaccel for HACCEL {}

/// [`HACCEL`](crate::HACCEL) methods from `user` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub trait UserHaccel: Handle {
	/// [`CreateAcceleratorTable`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createacceleratortablew)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HACCEL::DestroyAcceleratorTable`](crate::prelude::UserHaccel::DestroyAcceleratorTable)
	/// call.
	fn CreateAcceleratorTable(accel: &mut [ACCEL]) -> WinResult<HACCEL> {
		unsafe {
			user::ffi::CreateAcceleratorTableW(
				accel.as_mut_ptr() as _,
				accel.len() as _,
			).as_mut()
		}.map(|ptr| HACCEL(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DestroyAcceleratorTable`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyacceleratortable)
	/// method.
	fn DestroyAcceleratorTable(self) -> bool {
		unsafe { user::ffi::DestroyAcceleratorTable(self.as_ptr()) != 0 }
	}
}
