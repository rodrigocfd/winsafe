//! Raw bindings to user32.lib functions.

use std::ffi::c_void;

#[link(name = "user32")]
extern "system" {
	pub fn AppendMenuW(hMenu: *const c_void, uFlags: u32,
		uIDNewItem: *const c_void, lpNewItem: *const u16) -> u32;
	pub fn BeginPaint(
		hWnd: *const c_void, lpPaint: *mut c_void) -> *const c_void;
	pub fn CreateMenu() -> *const c_void;
	pub fn CreatePopupMenu() -> *const c_void;
	pub fn CreateWindowExW(dwExStyle: u32, lpClassName: *const u16,
		lpWindowName: *const u16, dwStyle: u32, X: i32, Y: i32,
		nWidth: i32, nHeight: i32, hWndParent: *const c_void,
		hMenu: *const c_void, hInstance: *const c_void,
		lpParam: *const c_void) -> *const c_void;
	pub fn DefWindowProcW(hWnd: *const c_void, Msg: u32,
		wParam: *const c_void, lParam: *const c_void) -> isize;
	pub fn DestroyIcon(hIcon: *const c_void) -> u32;
	pub fn DestroyWindow(hIcon: *const c_void) -> u32;
	pub fn DispatchMessageW(lpMsg: *const c_void) -> isize;
	pub fn EnableWindow(hWnd: *const c_void, bEnable: u32) -> u32;
	pub fn EndPaint(hWnd: *const c_void, lpPaint: *const c_void) -> u32;
	pub fn FindWindowW(
		lpClassName: *const u16, lpWindowName: *const u16) -> *const c_void;
	pub fn GetAncestor(hwnd: *const c_void, gaFlags: u32) -> *const c_void;
	pub fn GetClassInfoExW(hInstance: *const c_void,
		lpszClass: *const u16, lpwcx: *mut c_void) -> u32;
	pub fn GetFocus() -> *const c_void;
	pub fn GetForegroundWindow() -> *const c_void;
	pub fn GetMenuItemCount(hMenu: *const c_void) -> i32;
	pub fn GetMenuItemID(hMenu: *const c_void, nPos: i32) -> i32;
	pub fn GetMessageW(lpMsg: *const c_void, hWnd: *const c_void,
		wMsgFilterMin: u32, wMsgFilterMax: u32) -> i32;
	pub fn GetParent(hWnd: *const c_void) -> *const c_void;
	pub fn GetSubMenu(hMenu: *const c_void, nPos: i32) -> *const c_void;
	pub fn GetWindow(hWnd: *const c_void, uCmd: u32) -> *const c_void;
	pub fn GetWindowLongPtrW(hWnd: *const c_void, nIndex: i32) -> *const c_void;
	pub fn InsertMenuW(hMenu: *const c_void, uPosition: u32, uFlags: u32,
		uIDNewItem: *const c_void, lpNewItem: *const u16) -> u32;
	pub fn InvalidateRect(
		hWnd: *const c_void, lpRect: *const c_void, bErase: u32) -> u32;
	pub fn IsWindowEnabled(hWnd: *const c_void) -> u32;
	pub fn LoadCursorW(
		hInstance: *const c_void, lpCursorName: *const u16) -> *const c_void;
	pub fn LoadIconW(
		hInstance: *const c_void, lpIconName: *const u16) -> *const c_void;
	pub fn MessageBoxW(hWnd: *const c_void, lpText: *const u16,
		lpCaption: *const u16, uType: u32) -> u32;
	pub fn RegisterClassExW(lpwcx: *const c_void) -> u16;
	pub fn SetWindowLongPtrW(hWnd: *const c_void,
		nIndex: i32, dwNewLong: *const c_void) -> *const c_void;
	pub fn ShowWindow(hWnd: *const c_void, nCmdShow: i32) -> u32;
	pub fn TranslateMessage(lpMsg: *const c_void) -> u32;
	pub fn UnregisterClassW(
		lpClassName: *const u16, hInstance: *const c_void) -> u32;
	pub fn UpdateWindow(hWnd: *const c_void) -> u32;
}