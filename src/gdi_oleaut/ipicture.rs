use crate::kernel::decl::ErrResult;
use crate::oleaut::decl::IPicture;
use crate::prelude::{GdiOleautHdc, Handle, OleautIPicture, UserHwnd};
use crate::user::decl::{HDC, HWND, SIZE};

impl GdiOleautIPicture for IPicture {}

/// [`IPicture`](crate::IPicture) methods from `gdi`+`oleaut` feature.
#[cfg_attr(docsrs, doc(cfg(all(feature = "gdi", feature = "oleaut"))))]
pub trait GdiOleautIPicture: OleautIPicture {
	/// Calls
	/// [`IPicture::get_Width`](crate::prelude::OleautIPicture::get_Width) and
	/// [`IPicture::get_Height`](crate::prelude::OleautIPicture::get_Height),
	/// then converts the HIMETRIC units to pixels.
	///
	/// If `hdc` is not provided, `HWND::NULL.GetDC()` will be used.
	#[must_use]
	fn size_px(&self, hdc: Option<HDC>) -> ErrResult<SIZE> {
		let our_hdc = if let Some(hdc) = hdc { hdc } else { HWND::NULL.GetDC()? };
		let (cx, cy) = our_hdc.HiMetricToPixel(self.get_Width()?, self.get_Height()?);
		if hdc.is_none() { HWND::NULL.ReleaseDC(our_hdc)?; }
		Ok(SIZE::new(cx, cy))
	}
}
