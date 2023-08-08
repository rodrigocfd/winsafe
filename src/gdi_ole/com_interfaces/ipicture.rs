#![allow(non_camel_case_types)]

use crate::decl::*;
use crate::prelude::*;

impl gdi_ole_IPicture for IPicture {}

/// This trait is enabled with `gdi` and `ole` features, and provides methods
/// for [`IPicture`](crate::IPicture).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_ole_IPicture: ole_IPicture {
	/// Calls [`IPicture::get_Width`](crate::prelude::ole_IPicture::get_Width)
	/// and [`IPicture::get_Height`](crate::prelude::ole_IPicture::get_Height),
	/// then converts the HIMETRIC units to pixels.
	///
	/// If `hdc` is not provided, the screen DC, retrieved with
	/// `HWND::NULL.GetDC()`, will be used.
	#[must_use]
	fn size_px(&self, hdc: Option<&HDC>) -> HrResult<SIZE> {
		let (cx, cy) = match hdc {
			Some(hdc) => {
				hdc.HiMetricToPixel(
					self.get_Width()?, self.get_Height()?)
			},
			None => {
				let screen_dc = HWND::NULL.GetDC()
					.map_err(|e| e.to_hresult())?;
				let (cx, cy) = screen_dc.HiMetricToPixel(
					self.get_Width()?, self.get_Height()?);
				(cx, cy)
			},
		};
		Ok(SIZE::new(cx, cy))
	}
}
