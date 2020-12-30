//! Global objects used within `gui` module.

use std::error::Error;

use crate::co;
use crate::funcs::SystemParametersInfo;
use crate::handles::HFONT;
use crate::priv_funcs::str_dyn_error;
use crate::structs::NONCLIENTMETRICS;

/// Global UI font object.
static mut UI_HFONT: Option<HFONT> = None;

/// Creates the global UI font object.
pub fn create_ui_font() -> Result<(), Box<dyn Error>> {
	let mut ncm = NONCLIENTMETRICS::default();
	unsafe {
		SystemParametersInfo(co::SPI::GETNONCLIENTMETRICS,
			std::mem::size_of::<NONCLIENTMETRICS>() as u32,
			&mut ncm, co::SPIF::ZERO)?;

		UI_HFONT = Some(
			HFONT::CreateFontIndirect(&ncm.lfMenuFont)
				.map_err(|_| str_dyn_error(""))?
		);
	}
	Ok(())
}

/// Frees the global UI font object.
pub fn delete_ui_font() {
	unsafe {
		if let Some(hfont) = UI_HFONT {
			hfont.DeleteObject().unwrap();
			UI_HFONT = None;
		}
	}
}

/// Retrieves the global UI font object, or panics if not created yet.
pub fn ui_font() -> HFONT {
	unsafe {
		match UI_HFONT {
			Some(hfont) => hfont,
			None => panic!("Global UI font not created."),
		}
	}
}
