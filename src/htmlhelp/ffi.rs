use crate::kernel::ffi_types::*;

// The htmlhelp declaration is completely different, thus extern_sys! macro is not used.
// https://github.com/rodrigocfd/winsafe/issues/197#issuecomment-4784295081

#[cfg_attr(
	not(feature = "raw-dylib"),
	link(name = "htmlhelp", kind = "static", modifiers = "-bundle")
)]
#[cfg_attr(
	feature = "raw-dylib",
	link(name = "hhctrl.ocx", kind = "raw-dylib", modifiers = "+verbatim")
)]
unsafe extern "system" {
	pub(crate) fn HtmlHelpW(_x: HANDLE, _x: PCSTR, _x: u32, _x: usize) -> HANDLE;
}
