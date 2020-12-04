use crate::ffi::Void;

#[link(name = "user32")]
extern "system" {
	pub fn CreateMenu() -> *const Void;
	pub fn CreatePopupMenu() -> *const Void;
	pub fn CreateWindowExW(dwExStyle: u32, lpClassName: *const u16,
		lpWindowName: *const u16, dwStyle: u32, X: i32, Y: i32,
		nWidth: i32, nHeight: i32, hWndParent: *const Void, hMenu: *const Void,
		hInstance: *const Void, lpParam: *const Void) -> *const Void;
	pub fn GetForegroundWindow() -> *const Void;
	pub fn GetParent(hWnd: *const Void) -> *const Void;
	pub fn GetWindow(hWnd: *const Void, uCmd: u32) -> *const Void;
	pub fn MessageBoxW(hWnd: *const Void, lpText: *const u16,
		lpCaption: *const u16, uType: u32) -> u32;
}