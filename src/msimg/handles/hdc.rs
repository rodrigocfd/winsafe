#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::SysResult;
use crate::kernel::privs::bool_to_sysresult;
use crate::msimg;
use crate::prelude::Handle;
use crate::user::decl::{COLORREF, HDC, POINT, SIZE};

impl msimg_Hdc for HDC {}

/// This trait is enabled with the `msimg` feature, and provides methods for
/// [`HDC`](crate::HDC).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait msimg_Hdc: Handle {
	/// [`TransparentBlt`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-transparentblt)
	/// function.
	fn TransparentBlt(&self,
		dest_top_left: POINT,
		dest_sz: SIZE,
		hdc_src: HDC,
		src_top_left: POINT,
		src_sz: SIZE,
		color_transparent: COLORREF,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				msimg::ffi::TransparentBlt(
					self.ptr(),
					dest_top_left.x, dest_top_left.y,
					dest_sz.cx, dest_sz.cy,
					hdc_src.ptr(),
					src_top_left.x, src_top_left.y,
					src_sz.cx, src_sz.cy,
					color_transparent.into(),
				)
			},
		)
	}
}
