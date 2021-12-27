#![allow(non_snake_case)]

use crate::kernel::decl::WinResult;
use crate::kernel::privs::bool_to_winresult;
use crate::msimg;
use crate::prelude::Handle;
use crate::user::decl::{COLORREF, HDC, POINT, SIZE};

impl MsimgHdc for HDC {}

/// [`HDC`](crate::HDC) methods from `msimg` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "msimg")))]
pub trait MsimgHdc: Handle {
	/// [`TransparentBlt`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-transparentblt)
	/// method.
	fn TransparentBlt(self,
		dest_top_left: POINT, dest_sz: SIZE,
		hdc_src: HDC,
		src_top_left: POINT, src_sz: SIZE,
		color_transparent: COLORREF) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				msimg::ffi::TransparentBlt(
					self.as_ptr(),
					dest_top_left.x, dest_top_left.y,
					dest_sz.cx, dest_sz.cy,
					hdc_src.0,
					src_top_left.x, src_top_left.y,
					src_sz.cx, src_sz.cy,
					color_transparent.0,
				)
			},
		)
	}
}
