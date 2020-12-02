use crate::ffi::Void;

#[link(name = "user32")]
extern "system" {
	pub fn MessageBoxW(hWnd: *mut Void, lpText: *const u16,
		lpCaption: *const u16, uType: u32) -> u32;
}