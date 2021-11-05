#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::ffi::gdi32;
use crate::funcs::GetLastError;
use crate::handles::HandleGdi;
use crate::structs::{BITMAP, SIZE};

/// Handle to a
/// [bitmap](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hbitmap).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HBITMAP(pub(crate) *mut std::ffi::c_void);

impl_handle!(HBITMAP);
impl HandleGdi for HBITMAP {}

impl HBITMAP {
	/// [`CreateBitmap`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbitmap)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HBITMAP::DeleteObject`](crate::HBITMAP::DeleteObject) call.
	pub fn CreateBitmap(
		sz: SIZE, num_planes: u32,
		bit_count: u32, bits: *mut u8) -> WinResult<HBITMAP>
	{
		unsafe {
			gdi32::CreateBitmap(sz.cx, sz.cy, num_planes, bit_count, bits as _)
				.as_mut()
		}.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// method.
	pub fn GetObject(self, pv: &mut BITMAP) -> WinResult<()> {
		match unsafe {
			gdi32::GetObjectW(
				self.0,
				std::mem::size_of::<BITMAP>() as _,
				pv as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}
}
