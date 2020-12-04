use crate::ffi::Void;

#[link(name = "user32")]
extern "system" {
	pub fn GetForegroundWindow() -> *const Void;
	pub fn GetParent(hWnd: *const Void) -> *const Void;
	pub fn GetWindow(hWnd: *const Void, uCmd: u32) -> *const Void;
	pub fn MessageBoxW(hWnd: *const Void, lpText: *const u16,
		lpCaption: *const u16, uType: u32) -> u32;
}