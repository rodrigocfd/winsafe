#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::structs::ACCEL;

/// Handle to an
/// [accelerator table](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#haccel).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HACCEL(pub(crate) *mut std::ffi::c_void);

impl_handle!(HACCEL);

impl HACCEL {
	/// [`CreateAcceleratorTable`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createacceleratortablew)
	/// static method.
	pub fn CreateAcceleratorTable(accel: &mut [ACCEL]) -> WinResult<HACCEL> {
		unsafe {
			user32::CreateAcceleratorTableW(
				accel.as_mut_ptr() as _,
				accel.len() as _,
			).as_mut()
		}.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DestroyAcceleratorTable`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyacceleratortable)
	/// method.
	pub fn DestroyAcceleratorTable(self) -> bool {
		unsafe { user32::DestroyAcceleratorTable(self.0) != 0 }
	}
}
