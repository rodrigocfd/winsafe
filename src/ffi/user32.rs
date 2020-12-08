//! Raw bindings to user32.lib functions.

use crate::ffi::Void;

#[link(name = "user32")]
extern "system" {
	pub fn CreateMenu() -> *const Void;
	pub fn CreatePopupMenu() -> *const Void;
	pub fn CreateWindowExW(dwExStyle: u32, lpClassName: *const u16,
		lpWindowName: *const u16, dwStyle: u32, X: i32, Y: i32,
		nWidth: i32, nHeight: i32, hWndParent: *const Void, hMenu: *const Void,
		hInstance: *const Void, lpParam: *const Void) -> *const Void;
	pub fn DestroyIcon(hIcon: *const Void) -> u32;
	pub fn DestroyWindow(hIcon: *const Void) -> u32;
	pub fn FindWindowW(
		lpClassName: *const u16, lpWindowName: *const u16) -> *const Void;
	pub fn GetForegroundWindow() -> *const Void;
	pub fn GetMessageW(lpMsg: *const Void, hWnd: *const Void,
		wMsgFilterMin: u32, wMsgFilterMax: u32) -> i32;
	pub fn GetParent(hWnd: *const Void) -> *const Void;
	pub fn GetWindow(hWnd: *const Void, uCmd: u32) -> *const Void;
	pub fn MessageBoxW(hWnd: *const Void, lpText: *const u16,
		lpCaption: *const u16, uType: u32) -> u32;
	pub fn RegisterClassExW(Arg1: *const Void) -> u16;
	pub fn ShowWindow(hWnd: *const Void, nCmdShow: i32) -> u32;
	pub fn TranslateMessage(lpMsg: *const Void) -> u32;
	pub fn UnregisterClassW(
		lpClassName: *const u16, hInstance: *const Void) -> u32;
}