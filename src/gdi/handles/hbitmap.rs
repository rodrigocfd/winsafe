#![allow(non_snake_case)]

use crate::gdi;
use crate::gdi::decl::BITMAP;
use crate::kernel::decl::{GetLastError, WinResult};
use crate::prelude::{Handle, HandleGdi};
use crate::user::decl::{HBITMAP, SIZE};

impl HandleGdi for HBITMAP {}
impl GdiHbitmap for HBITMAP {}

/// [`HBITMAP`](crate::HBITMAP) methods from `gdi` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "gdi")))]
pub trait GdiHbitmap: Handle {
	/// [`CreateBitmap`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbitmap)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HBITMAP::DeleteObject`](crate::prelude::HandleGdi::DeleteObject) call.
	fn CreateBitmap(
		sz: SIZE, num_planes: u32,
		bit_count: u32, bits: *mut u8) -> WinResult<HBITMAP>
	{
		unsafe {
			gdi::ffi::CreateBitmap(
				sz.cx, sz.cy, num_planes, bit_count, bits as _,
			).as_mut()
		}.map(|ptr| HBITMAP(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// method.
	fn GetObject(self, pv: &mut BITMAP) -> WinResult<()> {
		match unsafe {
			gdi::ffi::GetObjectW(
				self.as_ptr(),
				std::mem::size_of::<BITMAP>() as _,
				pv as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}
}
