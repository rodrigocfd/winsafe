use crate::co;
use crate::decl::*;
use crate::prelude::*;

/// Global horizontal and vertical system DPI factor.
static mut DPI: (i32, i32) = (0, 0);

fn cache_dpi() {
	if unsafe { DPI } == (0, 0) {
		let hdc_screen = HWND::NULL.GetDC().unwrap(); // should really never fail
		let x = hdc_screen.GetDeviceCaps(co::GDC::LOGPIXELSX);
		let y = hdc_screen.GetDeviceCaps(co::GDC::LOGPIXELSY);
		unsafe {
			DPI = (x, y);
		}
	}
}

/// Returns the value adjusted according to the current horizontal system DPI,
/// retrieved with [`HDC::GetDeviceCaps`](crate::HDC::GetDeviceCaps).
pub fn dpi_x(x_val: i32) -> i32 {
	cache_dpi();
	MulDiv(x_val.into(), unsafe { DPI }.0, 96)
}

/// Returns the value adjusted according to the current vertical system DPI,
/// retrieved with [`HDC::GetDeviceCaps`](crate::HDC::GetDeviceCaps).
pub fn dpi_y(x_val: i32) -> i32 {
	cache_dpi();
	MulDiv(x_val.into(), unsafe { DPI }.0, 96)
}

/// Returns the values adjusted according to the current horizontal and vertical
/// system DPI, retrieved with
/// [`HDC::GetDeviceCaps`](crate::HDC::GetDeviceCaps).
pub fn dpi(x_val: i32, y_val: i32) -> (i32, i32) {
	(dpi_x(x_val), dpi_y(y_val))
}
