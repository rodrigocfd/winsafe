use crate::ffi::Void;

#[link(name = "user32")]
extern "system" {
	pub fn GetForegroundWindow() -> *mut Void;
	pub fn GetParent(hWnd: *mut Void) -> *mut Void;
	pub fn GetWindow(hWnd: *mut Void, uCmd: u32) -> *mut Void;
	pub fn MessageBoxW(hWnd: *mut Void, lpText: *const u16,
		lpCaption: *const u16, uType: u32) -> u32;
}