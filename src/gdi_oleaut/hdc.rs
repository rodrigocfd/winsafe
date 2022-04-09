#![allow(non_snake_case)]

use crate::co;
use crate::gdi_oleaut::privs::HIMETRIC_PER_INCH;
use crate::kernel::decl::MulDiv;
use crate::prelude::GdiHdc;
use crate::user::decl::HDC;

impl GdiOleautHdc for HDC {}

/// [`HDC`](crate::HDC) methods from `gdi`+`oleaut` feature.
pub trait GdiOleautHdc: GdiHdc {
	/// Converts HIMETRIC units to pixels.
	///
	/// Equivalent to
	/// [`AtlHiMetricToPixel`](https://docs.microsoft.com/en-us/cpp/atl/reference/pixel-himetric-conversion-global-functions?view=msvc-170#atlhimetrictopixel)
	/// ATL function.
	#[must_use]
	fn HiMetricToPixel(self, x: i32, y: i32) -> (i32, i32) {
		// http://www.verycomputer.com/5_5f2f75dc2d090ee8_1.htm
		// https://forums.codeguru.com/showthread.php?109554-Unresizable-activeX-control
		(
			MulDiv(x, self.GetDeviceCaps(co::GDC::LOGPIXELSX), HIMETRIC_PER_INCH),
			MulDiv(y, self.GetDeviceCaps(co::GDC::LOGPIXELSY), HIMETRIC_PER_INCH),
		)
	}

	/// Converts pixels to HIMETRIC units.
	///
	/// Equivalent to
	/// [`AtlPixelToHiMetric`](https://docs.microsoft.com/en-us/cpp/atl/reference/pixel-himetric-conversion-global-functions?view=msvc-170#atlpixeltohimetric)
	/// ATL function.
	#[must_use]
	fn PixelToHiMetric(self, x: i32, y: i32) -> (i32, i32) {
		(
			MulDiv(x, HIMETRIC_PER_INCH, self.GetDeviceCaps(co::GDC::LOGPIXELSX)),
			MulDiv(y, HIMETRIC_PER_INCH, self.GetDeviceCaps(co::GDC::LOGPIXELSY)),
		)
	}
}
