#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::user32;
use crate::funcs::GetLastError;
use crate::structs::ACCEL;

pub_struct_handle! {
	/// Handle to an
	/// [accelerator table](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#haccel).
	HACCEL
}

impl HACCEL {
	/// [`CreateAcceleratorTable`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createacceleratortablew)
	/// static method.
	pub fn CreateAcceleratorTable(paccel: &mut [ACCEL]) -> WinResult<HACCEL> {
		unsafe {
			user32::CreateAcceleratorTableW(
				paccel.as_mut_ptr() as _,
				paccel.len() as _,
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`DestroyAcceleratorTable`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyacceleratortable)
	/// method.
	pub fn DestroyAcceleratorTable(self) -> bool {
		unsafe { user32::DestroyAcceleratorTable(self.ptr) != 0 }
	}
}
