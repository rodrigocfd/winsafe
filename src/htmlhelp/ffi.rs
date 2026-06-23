use crate::kernel::ffi_types::*;
use crate::macros::*;

extern_sys! { "htmlhelp";
	HtmlHelpW(HANDLE, PCSTR, u32, usize) -> HANDLE
}
