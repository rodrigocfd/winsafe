#![allow(non_camel_case_types)]

use crate::kernel::decl::ErrResult;
use crate::oleaut::decl::IPicture;
use crate::prelude::{gdi_oleaut_Hdc, Handle, oleaut_IPicture, user_Hwnd};
use crate::user::decl::{HDC, HWND, SIZE};

impl gdi_oleaut_IPicture for IPicture {}

/// This trait is enabled with `gdi` and `oleaut` features, and provides methods
/// for [`IPicture`](crate::IPicture).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(all(feature = "gdi", feature = "oleaut"))))]
pub trait gdi_oleaut_IPicture: oleaut_IPicture {
	/// Calls
	/// [`IPicture::get_Width`](crate::prelude::oleaut_IPicture::get_Width) and
	/// [`IPicture::get_Height`](crate::prelude::oleaut_IPicture::get_Height),
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
