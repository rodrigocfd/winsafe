#![allow(non_snake_case)]

use crate::decl::*;
use crate::htmlhelp::ffi;

impl HWND {
	/// [`HtmllHelp`](https://learn.microsoft.com/en-us/windows/win32/api/htmlhelp/nf-htmlhelp-htmlhelpw)
	/// function.
	pub fn HtmlHelp(&self, chm_file: &str, command: HhCmd) -> HWND {
		let (hh, data) = command.as_data();
		unsafe {
			HWND::from_ptr(ffi::HtmlHelpW(
				self.ptr(),
				WString::from_str(chm_file).as_ptr(),
				hh.raw(),
				data.serialize(),
			))
		}
	}
}
