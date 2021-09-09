#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;

pub_struct_handle_gdi! {
	/// Handle to a
	/// [bitmap](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbitmap).
	HBITMAP
}

impl HBITMAP {
	/// [`CreateBitmap`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbitmap)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HBITMAP::DeleteObject`](crate::HBITMAP::DeleteObject) call.
	pub fn CreateBitmap(
		nWidth: i32, nHeight: i32,
		nPlanes: u32, nBitCount: u32, lpBits: *mut u8) -> WinResult<HBITMAP>
	{
		unsafe {
			gdi32::CreateBitmap(nWidth, nHeight, nPlanes, nBitCount, lpBits as _)
				.as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}
}
