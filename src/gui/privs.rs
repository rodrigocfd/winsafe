//! Global objects used within `gui` module.

use std::error::Error;

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::{GetSystemMetrics, PostQuitMessage, SystemParametersInfo};
use crate::handles::{HFONT, HTHEME, HWND};
use crate::msg::wm;
use crate::structs::{NONCLIENTMETRICS, POINT, RECT, SIZE};

/// Global return error, will be taken in main loop.
pub(in crate::gui) static mut QUIT_ERROR: Option<Box<dyn Error>> = None;

/// Terminates the program with the given error.
pub(in crate::gui) fn post_quit_error(err: Box<dyn Error>) {
	unsafe { QUIT_ERROR = Some(err); } // store the error, so the main window/dialog can grab it
	PostQuitMessage(-1); // this -1 will be discarded in the main loop, anyway
}

//------------------------------------------------------------------------------

/// Global UI font object.
static mut UI_HFONT: Option<HFONT> = None;

/// Creates the global UI font object.
pub(in crate::gui) fn create_ui_font() -> WinResult<()> {
	let mut ncm = NONCLIENTMETRICS::default();
	unsafe {
		SystemParametersInfo(
			co::SPI::GETNONCLIENTMETRICS,
			std::mem::size_of::<NONCLIENTMETRICS>() as _,
			&mut ncm, co::SPIF::NoValue,
		)?;
		UI_HFONT = Some(HFONT::CreateFontIndirect(&ncm.lfMenuFont)?);
	}
	Ok(())
}

/// Frees the global UI font object.
pub(in crate::gui) fn delete_ui_font() -> WinResult<()> {
	unsafe {
		if let Some(hfont) = UI_HFONT {
			hfont.DeleteObject()?;
			UI_HFONT = None;
		}
	}
	Ok(())
}

/// Retrieves the global UI font object, or panics if not created yet.
pub(in crate::gui) fn ui_font() -> HFONT {
	unsafe {
		match UI_HFONT {
			Some(hfont) => hfont,
			None => panic!("Global UI font not created."),
		}
	}
}

//------------------------------------------------------------------------------

static mut BASE_CTRL_ID: u16 = 20_000; // in-between Visual Studio Resource Editor values

/// Returns the next sequential control ID.
pub(in crate::gui) fn auto_ctrl_id() -> u16 {
	unsafe {
		let new_id = BASE_CTRL_ID;
		BASE_CTRL_ID += 1;
		new_id
	}
}

//------------------------------------------------------------------------------

static mut DPI: POINT = POINT { x: 0, y: 0 };

/// Multiplies the given coordinates by current system DPI.
pub(in crate::gui) fn multiply_dpi(
	pt: Option<&mut POINT>, sz: Option<&mut SIZE>) -> WinResult<()>
{
	unsafe {
		if (pt.is_some() || sz.is_some()) && DPI.x == 0 { // DPI not cached yet?
			let screen_dc = HWND::NULL.GetDC()?;
			DPI.x = screen_dc.GetDeviceCaps(co::GDC::LOGPIXELSX); // cache
			DPI.y = screen_dc.GetDeviceCaps(co::GDC::LOGPIXELSY);
		}

		if let Some(pt) = pt {
			pt.x = kernel32::MulDiv(pt.x, DPI.x, 96);
			pt.y = kernel32::MulDiv(pt.y, DPI.y, 96);
		}

		if let Some(sz) = sz {
			sz.cx = kernel32::MulDiv(sz.cx, DPI.x, 96);
			sz.cy = kernel32::MulDiv(sz.cy, DPI.y, 96);
		}
	}
	Ok(())
}

//------------------------------------------------------------------------------

/// Calculates the bound rectangle to fit the text with current system font.
pub(in crate::gui) fn calc_text_bound_box(text: &str) -> WinResult<SIZE> {
	let desktop_hwnd = HWND::GetDesktopWindow();
	let desktop_hdc = desktop_hwnd.GetDC()?;
	let clone_dc = desktop_hdc.CreateCompatibleDC()?;
	let prev_hfont = clone_dc.SelectObjectFont(ui_font())?;

	let mut bounds = if text.is_empty() {
		clone_dc.GetTextExtentPoint32("Pj")? // just a placeholder to get the text height
	} else {
		clone_dc.GetTextExtentPoint32(&remove_accelerator_ampersands(text))?
	};

	if text.is_empty() {
		bounds.cx = 0; // if no text was given, return just the height
	}

	clone_dc.SelectObjectFont(prev_hfont)?;
	clone_dc.DeleteDC()?;
	desktop_hwnd.ReleaseDC(desktop_hdc)?;
	Ok(bounds)
}

/// Calculates the bound rectangle to fit the text with current system font,
/// adding a check box.
pub(in crate::gui) fn calc_text_bound_box_check(text: &str) -> WinResult<SIZE> {
	let mut bound_box = calc_text_bound_box(text)?;
	bound_box.cx += GetSystemMetrics(co::SM::CXMENUCHECK) // https://stackoverflow.com/a/1165052/6923555
		+ GetSystemMetrics(co::SM::CXEDGE);

	let cy_check = GetSystemMetrics(co::SM::CYMENUCHECK);
	if cy_check > bound_box.cy {
		bound_box.cy = cy_check; // if the check is taller than the font, use its height
	}

	Ok(bound_box)
}

fn remove_accelerator_ampersands(text: &str) -> String {
	let mut txt_no_ampersands = String::with_capacity(text.len());
	let mut last_ch = 'a'; // initial value will be skipped

	for (idx, ch) in text.char_indices() {
		if idx == 0 { // first char
			if ch != '&' {
				txt_no_ampersands.push(ch);
			}
		} else if ch != '&' || (ch == '&' && last_ch == '&') {
			txt_no_ampersands.push(ch);
		}
		last_ch = ch;
	}

	txt_no_ampersands
}

//------------------------------------------------------------------------------

/// Paints the themed border of an user control, if it has the proper styles.
pub(in crate::gui) fn paint_control_borders(
	hwnd: HWND, wm_ncp: wm::NcPaint) -> WinResult<()>
{
	hwnd.DefWindowProc(wm_ncp); // let the system draw the scrollbar for us

	let ex_style = co::WS_EX(hwnd.GetWindowLongPtr(co::GWLP::EXSTYLE) as _);
	if !ex_style.has(co::WS_EX::CLIENTEDGE) // no border
		|| !HTHEME::IsThemeActive()
		|| !HTHEME::IsAppThemed()
	{
		return Ok(());
	}

	let mut rc = hwnd.GetWindowRect()?; // window outmost coordinates, including margins
	hwnd.ScreenToClientRc(&mut rc)?;
	rc.left += 2; rc.top += 2; rc.right += 2; rc.bottom += 2; // because it comes up anchored at -2,-2

	let hdc = hwnd.GetWindowDC()?;

	if let Some(htheme) = hwnd.OpenThemeData("LISTVIEW") {
		// Draw only the borders to avoid flickering.
		htheme.DrawThemeBackground(hdc,
			co::VS::LISTVIEW_LISTGROUP, rc,
			RECT { left: rc.left, top: rc.top, right: rc.left + 2, bottom: rc.bottom })?;
		htheme.DrawThemeBackground(hdc,
			co::VS::LISTVIEW_LISTGROUP, rc,
			RECT { left: rc.left, top: rc.top, right: rc.right, bottom: rc.top + 2 })?;
		htheme.DrawThemeBackground(hdc,
			co::VS::LISTVIEW_LISTGROUP, rc,
			RECT { left: rc.right - 2, top: rc.top, right: rc.right, bottom: rc.bottom })?;
		htheme.DrawThemeBackground(hdc,
			co::VS::LISTVIEW_LISTGROUP, rc,
			RECT { left: rc.left, top: rc.bottom - 2, right: rc.right, bottom: rc.bottom })?;

		htheme.CloseThemeData()?;
	}

	hwnd.ReleaseDC(hdc)
}
