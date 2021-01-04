use std::error::Error;

use crate::co;
use crate::ffi::kernel32;
use crate::handles::{HTHEME, HWND};
use crate::msg::WmNcPaint;
use crate::priv_funcs::str_dyn_error;
use crate::structs::{POINT, RECT, SIZE};

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

/// Paints the themed border of an user control, if it has the proper styles.
pub fn paint_control_borders(
	hwnd: HWND, wm_ncp: WmNcPaint) -> Result<(), Box<dyn Error>>
{
	hwnd.DefWindowProc(wm_ncp); // let the system draw the scrollbar for us

	let ex_style = co::WS_EX::from(hwnd.GetWindowLongPtr(co::GWLP::EXSTYLE) as u32);
	if !ex_style.has(co::WS_EX::CLIENTEDGE) // no border
		|| !HTHEME::IsThemeActive()
		|| !HTHEME::IsAppThemed()
	{
		return Ok(());
	}

	let mut rc = hwnd.GetWindowRect() // window outmost coordinates, including margins
		.map_err(|_| str_dyn_error("GetWindowRect failed."))?;
	hwnd.ScreenToClientRc(&mut rc)
		.map_err(|_| str_dyn_error("ScreenToClientRc failed."))?;
	rc.left += 2; rc.top += 2; rc.right += 2; rc.bottom += 2; // because it comes up anchored at -2,-2

	let hdc = hwnd.GetWindowDC()
		.map_err(|_| str_dyn_error("GetWindowDC failed."))?;

	if let Some(htheme) = hwnd.OpenThemeData("LISTVIEW") {
		// Draw only the borders to avoid flickering.
		htheme.DrawThemeBackground(hdc,
			co::VS_PART::LVP_LISTGROUP, co::VS_STATE::NONE, rc,
			RECT { left: rc.left, top: rc.top, right: rc.left + 2, bottom: rc.bottom })?;
		htheme.DrawThemeBackground(hdc,
			co::VS_PART::LVP_LISTGROUP, co::VS_STATE::NONE, rc,
			RECT { left: rc.left, top: rc.top, right: rc.right, bottom: rc.top + 2 })?;
		htheme.DrawThemeBackground(hdc,
			co::VS_PART::LVP_LISTGROUP, co::VS_STATE::NONE, rc,
			RECT { left: rc.right - 2, top: rc.top, right: rc.right, bottom: rc.bottom })?;
		htheme.DrawThemeBackground(hdc,
			co::VS_PART::LVP_LISTGROUP, co::VS_STATE::NONE, rc,
			RECT { left: rc.left, top: rc.bottom - 2, right: rc.right, bottom: rc.bottom })?;

		htheme.CloseThemeData()?;
	}

	hwnd.ReleaseDC(hdc);
	Ok(())
}
