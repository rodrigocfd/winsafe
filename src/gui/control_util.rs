use std::error::Error;

use crate::co;
use crate::ffi::kernel32;
use crate::handles::HWND;
use crate::priv_funcs::str_dyn_error;
use crate::structs::{POINT, SIZE};

static mut DPI: POINT = POINT{ x: 0, y: 0 };

/// Multiplies the given coordinates by current system DPI.
pub fn multiply_dpi(
	pt: Option<&mut POINT>, sz: Option<&mut SIZE>) -> Result<(), Box<dyn Error>>
{
	if (pt.is_some() || sz.is_some())
		&& unsafe { DPI.x } == 0
	{
		let screen_dc = unsafe { HWND::null_handle() }
			.GetDC()
			.map_err(|_| str_dyn_error("GetDC failed."))?;
		unsafe {
			DPI.x = screen_dc.GetDeviceCaps(co::GDC::LOGPIXELSX); // cache
			DPI.y = screen_dc.GetDeviceCaps(co::GDC::LOGPIXELSY);
		}
	}

	if let Some(pt) = pt {
		unsafe {
			pt.x = kernel32::MulDiv(pt.x, DPI.x, 96);
			pt.y = kernel32::MulDiv(pt.y, DPI.y, 96);
		}
	}

	if let Some(sz) = sz {
		unsafe {
			sz.cx = kernel32::MulDiv(sz.cx, DPI.x, 96);
			sz.cy = kernel32::MulDiv(sz.cy, DPI.y, 96);
		}
	}

	Ok(())
}
