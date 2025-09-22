use crate::co;
use crate::decl::*;
use crate::msg::*;
use crate::prelude::*;

/// Message to be used on `expect()` of internal calls, which are not supposed
/// to fail.
pub(in crate::gui) const DONTFAIL: &str = "GUI internals are not supposed to fail.";

/// Identifies whether a window is dialog-based.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::gui) enum WndTy {
	Raw,
	Dlg,
}

impl WndTy {
	/// `WM_CREATE` for ordinary windows, `WM_INITDIALOG` for dialogs.
	#[must_use]
	pub(in crate::gui) const fn creation_msg(&self) -> co::WM {
		match self {
			Self::Raw => co::WM::CREATE,
			Self::Dlg => co::WM::INITDIALOG,
		}
	}

	/// `0` for ordinary windows, `TRUE` for dialogs.
	#[must_use]
	pub(in crate::gui) const fn def_proc_val(&self) -> isize {
		match self {
			Self::Raw => 0,
			Self::Dlg => 1, // TRUE
		}
	}
}

/// Global horizontal and vertical system DPI factor, cached.
pub(in crate::gui) static mut DPI: (i32, i32) = (0, 0);

/// Stuff that should run before anything else.
pub(in crate::gui) fn initial_gui_setup() {
	if unsafe { DPI } == (0, 0) {
		if IsWindowsVistaOrGreater().expect(DONTFAIL) {
			SetProcessDPIAware().expect(DONTFAIL);
		}

		InitCommonControls();

		if IsWindows8OrGreater().expect(DONTFAIL) {
			// https://github.com/rodrigocfd/winsafe-examples/issues/6
			let mut b_val = 0; // FALSE
			match unsafe {
				HPROCESS::GetCurrentProcess().SetUserObjectInformation(
					co::UOI::TIMERPROC_EXCEPTION_SUPPRESSION, // SetTimer() safety
					&mut b_val,
				)
			} {
				Err(e) if e == co::ERROR::INVALID_PARAMETER => {
					// Do nothing: Wine doesn't support SetUserObjectInformation for now.
					// https://bugs.winehq.org/show_bug.cgi?id=54951
				},
				Err(e) => panic!("TIMERPROC_EXCEPTION_SUPPRESSION failed: {e:?}"), // should never happen
				_ => {},
			}
		}

		let hdc_screen = HWND::NULL.GetDC().expect(DONTFAIL);
		let x = hdc_screen.GetDeviceCaps(co::GDC::LOGPIXELSX);
		let y = hdc_screen.GetDeviceCaps(co::GDC::LOGPIXELSY);
		unsafe {
			DPI = (x, y);
		}
	}
}

pub(in crate::gui) mod quit_error {
	use std::error::Error;
	use std::sync::Mutex;

	use crate::decl::*;
	use crate::gui::*;
	use crate::msg::*;

	/// Global return error originated from an event handling closure; will be taken
	/// in main loop.
	pub(in crate::gui) static QUIT_ERROR: Mutex<Option<MsgError>> = Mutex::new(None);

	/// Calls `PostQuitMessage` to terminate the program with the given error.
	pub(in crate::gui) fn post_quit_error(src_msg: WndMsg, err: Box<dyn Error + Send + Sync>) {
		{
			let mut msg_error = QUIT_ERROR.lock().unwrap();
			*msg_error = Some(MsgError::new(src_msg, err)); // store the error, so the main window loop can grab it
		};
		PostQuitMessage(-1); // this -1 will be discarded in the main loop
	}
}

pub(in crate::gui) mod ui_font {
	use crate::co;
	use crate::decl::*;
	use crate::guard::*;
	use crate::gui::privs::*;
	use crate::msg::*;

	/// Global UI font object, cached.
	static mut UI_HFONT: Option<DeleteObjectGuard<HFONT>> = None;

	// Returns the global UI font, creating it of not yet.
	pub(in crate::gui) fn get() -> HFONT {
		unsafe {
			match &*&raw const UI_HFONT {
				None => {
					let mut ncm = NONCLIENTMETRICS::default(); // create the font once
					SystemParametersInfo(
						co::SPI::GETNONCLIENTMETRICS,
						std::mem::size_of::<NONCLIENTMETRICS>() as _,
						&mut ncm,
						co::SPIF::NoValue,
					)
					.expect(DONTFAIL);

					let font = HFONT::CreateFontIndirect(&ncm.lfMenuFont).expect(DONTFAIL);
					let ret_font = font.raw_copy();
					UI_HFONT = Some(font);
					ret_font
				},
				Some(font) => font.raw_copy(),
			}
		}
	}

	/// Sets the global UI font on the given window.
	pub(in crate::gui) fn set(hwnd: &HWND) {
		unsafe {
			hwnd.SendMessage(wm::SetFont { hfont: get(), redraw: true });
		}
	}

	/// Frees the global UI font object.
	pub(in crate::gui) fn delete() {
		unsafe {
			UI_HFONT = None; // https://users.rust-lang.org/t/why-drop-trait-not-called-when-use-global-static
		}
	}
}

pub(in crate::gui) mod auto_id {
	/// Next auto control ID to be assigned to controls without one.
	static mut BASE_CTRL_ID: u16 = 0xdfff; // https://stackoverflow.com/a/18192766/6923555

	/// Returns the next sequential control ID.
	#[must_use]
	pub(in crate::gui) const fn next() -> u16 {
		unsafe {
			let ret_id = BASE_CTRL_ID;
			BASE_CTRL_ID -= 1; // go down
			ret_id
		}
	}

	/// If the given ID is zero, returns the next sequential control ID.
	#[must_use]
	pub(in crate::gui) const fn set_if_zero(id: u16) -> u16 {
		if id == 0 { next() } else { id }
	}
}

pub(in crate::gui) mod text_calc {
	use crate::co;
	use crate::decl::*;
	use crate::gui::privs::*;

	/// "&He && she" becomes "He & she".
	#[must_use]
	pub(in crate::gui) fn remove_accel_ampersands(text: &str) -> String {
		let mut txt_no_ampersands = String::with_capacity(text.len());
		let mut last_ch = 'a'; // initial value will be skipped

		for (idx, ch) in text.char_indices() {
			if idx == 0 {
				// first char
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

	/// Calculates the bound rectangle to fit the text with current system font.
	#[must_use]
	pub(in crate::gui) fn bound_box(text: &str) -> SIZE {
		let desktop_hwnd = HWND::GetDesktopWindow();
		let desktop_hdc = desktop_hwnd.GetDC().expect(DONTFAIL);
		let clone_dc = desktop_hdc.CreateCompatibleDC().expect(DONTFAIL);
		let _prev_font = clone_dc.SelectObject(&ui_font::get()).expect(DONTFAIL);

		let mut bounds = clone_dc
			.GetTextExtentPoint32(if text.trim().is_empty() { "Pj" } else { text }) // "Pj" is a placeholder to get the height
			.expect(DONTFAIL);

		if text.is_empty() {
			bounds.cx = 0; // if no text was given, return just the height
		}
		bounds
	}

	/// Calculates the bound rectangle to fit the text with current system font,
	/// along with the system check/radio box.
	#[must_use]
	pub(in crate::gui) fn bound_box_with_check(text: &str) -> SIZE {
		let mut bounds = bound_box(text);
		bounds.cx += GetSystemMetrics(co::SM::CXMENUCHECK) // https://stackoverflow.com/a/1165052/6923555
			+ GetSystemMetrics(co::SM::CXEDGE);

		let cy_check = GetSystemMetrics(co::SM::CYMENUCHECK);
		if cy_check > bounds.cy {
			bounds.cy = cy_check; // if the check is taller than the font, use its height
		}
		bounds
	}
}

/// Paints the themed border of an user control, if it has the proper styles.
pub(in crate::gui) fn paint_control_borders(hwnd: &HWND, wm_ncp: wm::NcPaint) {
	unsafe {
		hwnd.DefWindowProc(wm_ncp); // let the system draw the scrollbar for us
	}

	if !hwnd.style_ex().has(co::WS_EX::CLIENTEDGE) // no border
		|| !IsThemeActive()
		|| !IsAppThemed()
	{
		return;
	}

	let mut rc = hwnd.GetWindowRect().expect(DONTFAIL); // window outmost coordinates, including margins
	rc = hwnd.ScreenToClientRc(rc).expect(DONTFAIL);
	rc = OffsetRect(rc, 2, 2).expect(DONTFAIL); // because it comes up anchored at -2,-2

	let hdc = hwnd.GetWindowDC().expect(DONTFAIL);

	// The HRGN which comes in WM_NCPAINT seems to be invalid, so we carve our own.
	let hrgn_hole =
		HRGN::CreateRectRgnIndirect(InflateRect(rc, -2, -2).expect(DONTFAIL)).expect(DONTFAIL);
	let hrgn_clip = HRGN::CreateRectRgnIndirect(rc).expect(DONTFAIL);
	hrgn_clip
		.CombineRgn(&hrgn_clip, &hrgn_hole, co::RGN::DIFF)
		.expect(DONTFAIL);
	hdc.SelectClipRgn(&hrgn_clip).expect(DONTFAIL);

	if let Some(htheme) = hwnd.OpenThemeData("EDIT") {
		htheme
			.DrawThemeBackground(&hdc, co::VS::EDIT_EDITTEXT_NORMAL, rc, None)
			.expect(DONTFAIL);
	}
}
