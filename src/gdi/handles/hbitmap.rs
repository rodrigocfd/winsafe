#![allow(non_camel_case_types, non_snake_case)]

use crate::gdi;
use crate::gdi::decl::BITMAP;
use crate::gdi::guard::DeleteObjectGuard;
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::ptr_to_sysresult;
use crate::prelude::GdiObject;
use crate::user::decl::{HBITMAP, SIZE};

impl GdiObject for HBITMAP {}
impl gdi_Hbitmap for HBITMAP {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HBITMAP`](crate::HBITMAP).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hbitmap: GdiObject {
	/// [`CreateBitmap`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbitmap)
	/// static method.
	#[must_use]
	fn CreateBitmap(
		sz: SIZE,
		num_planes: u32,
		bit_count: u32,
		bits: *mut u8,
	) -> SysResult<DeleteObjectGuard<HBITMAP>>
	{
		ptr_to_sysresult(
			unsafe {
				gdi::ffi::CreateBitmap(
					sz.cx, sz.cy, num_planes, bit_count, bits as _)
			},
			|ptr| DeleteObjectGuard::new(HBITMAP(ptr)),
		)
	}

	/// [`GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// method.
	fn GetObject(&self, pv: &mut BITMAP) -> SysResult<()> {
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
