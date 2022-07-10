#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::gdi_oleaut::privs::HIMETRIC_PER_INCH;
use crate::kernel::decl::MulDiv;
use crate::prelude::gdi_Hdc;
use crate::user::decl::HDC;

impl gdi_oleaut_Hdc for HDC {}

/// This trait is enabled with `gdi` and `oleaut` features, and provides methods
/// for [`HDC`](crate::HDC).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(all(feature = "gdi", feature = "oleaut"))))]
pub trait gdi_oleaut_Hdc: gdi_Hdc {
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
