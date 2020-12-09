//! Raw bindings to user32.lib functions.

use std::ffi::c_void;

use crate::ffi::HANDLE;

#[link(name = "user32")]
extern "system" {
	pub fn AdjustWindowRectEx(lpRect: *mut c_void, dwStyle: u32, bMenu: u32, dwExStyle: u32) -> u32;
	pub fn AppendMenuW(hMenu: HANDLE, uFlags: u32, uIDNewItem: *const c_void, lpNewItem: *const u16) -> u32;
	pub fn BeginPaint(hWnd: HANDLE, lpPaint: *mut c_void) -> HANDLE;
	pub fn CreateMenu() -> HANDLE;
	pub fn CreatePopupMenu() -> HANDLE;
	pub fn CreateWindowExW(dwExStyle: u32, lpClassName: *const u16, lpWindowName: *const u16, dwStyle: u32, X: i32, Y: i32, nWidth: i32, nHeight: i32, hWndParent: *const c_void, hMenu: *const c_void, hInstance: *const c_void, lpParam: *const c_void) -> HANDLE;
	pub fn DefWindowProcW(hWnd: HANDLE, Msg: u32, wParam: usize, lParam: isize) -> isize;
	pub fn DestroyIcon(hIcon: HANDLE) -> u32;
	pub fn DestroyWindow(hWnd: HANDLE) -> u32;
	pub fn DispatchMessageW(lpMsg: *const c_void) -> isize;
	pub fn EnableWindow(hWnd: HANDLE, bEnable: u32) -> u32;
	pub fn EndPaint(hWnd: HANDLE, lpPaint: *const c_void) -> u32;
	pub fn FindWindowW(lpClassName: *const u16, lpWindowName: *const u16) -> HANDLE;
	pub fn GetAncestor(hwnd: HANDLE, gaFlags: u32) -> HANDLE;
	pub fn GetClassInfoExW(hInstance: HANDLE, lpszClass: *const u16, lpwcx: *mut c_void) -> u32;
	pub fn GetDlgCtrlID(hWnd: HANDLE) -> i32;
	pub fn GetDlgItem(hDlg: HANDLE, nIDDlgItem: i32) -> HANDLE;
	pub fn GetFocus() -> HANDLE;
	pub fn GetForegroundWindow() -> HANDLE;
	pub fn GetMenuItemCount(hMenu: HANDLE) -> i32;
	pub fn GetMenuItemID(hMenu: HANDLE, nPos: i32) -> i32;
	pub fn GetMessageW(lpMsg: *const c_void, hWnd: *const c_void, wMsgFilterMin: u32, wMsgFilterMax: u32) -> i32;
	pub fn GetNextDlgGroupItem(hDlg: HANDLE, hCtl: HANDLE, bPrevious: u32) -> HANDLE;
	pub fn GetNextDlgTabItem(hDlg: HANDLE, hCtl: HANDLE, bPrevious: u32) -> HANDLE;
	pub fn GetParent(hWnd: HANDLE) -> HANDLE;
	pub fn GetQueueStatus(flags: u32) -> u32;
	pub fn GetSubMenu(hMenu: HANDLE, nPos: i32) -> HANDLE;
	pub fn GetSystemMetrics(nIndex: i32) -> i32;
	pub fn GetWindow(hWnd: HANDLE, uCmd: u32) -> HANDLE;
	pub fn GetWindowLongPtrW(hWnd: HANDLE, nIndex: i32) -> isize;
	pub fn GetWindowTextLengthW(hWnd: HANDLE) -> i32;
	pub fn GetWindowTextW(hWnd: HANDLE, lpString: *const u16, nMaxCount: i32) -> i32;
	pub fn InsertMenuW(hMenu: HANDLE, uPosition: u32, uFlags: u32, uIDNewItem: *const c_void, lpNewItem: *const u16) -> u32;
	pub fn InvalidateRect(hWnd: HANDLE, lpRect: *const c_void, bErase: u32) -> u32;
	pub fn IsChild(hWndParent: HANDLE, hWnd: HANDLE) -> u32;
	pub fn IsDialogMessageW(hDlg: HANDLE, lpMsg: *mut c_void) -> u32;
	pub fn IsWindow(hWnd: HANDLE) -> u32;
	pub fn IsWindowEnabled(hWnd: HANDLE) -> u32;
	pub fn IsWindowVisible(hWnd: HANDLE) -> u32;
	pub fn LoadCursorW(hInstance: HANDLE, lpCursorName: *const u16) -> HANDLE;
	pub fn LoadIconW(hInstance: HANDLE, lpIconName: *const u16) -> HANDLE;
	pub fn MessageBoxW(hWnd: HANDLE, lpText: *const u16, lpCaption: *const u16, uType: u32) -> u32;
	pub fn PeekMessageW(lpMsg: *mut c_void, hWnd: HANDLE, wMsgFilterMin: u32, wMsgFilterMax: u32, wRemoveMsg: u32) -> u32;
	pub fn PostMessageW(hWnd: HANDLE, Msg: u32, wParam: usize, lParam: isize) -> isize;
	pub fn PostQuitMessage(nExitCode: i32);
	pub fn RegisterClassExW(lpwcx: *const c_void) -> u16;
	pub fn SendMessageW(hWnd: HANDLE, Msg: u32, wParam: usize, lParam: isize) -> isize;
	pub fn SetFocus(hWnd: HANDLE) -> HANDLE;
	pub fn SetParent(hWndChild: HANDLE, hWndNewParent: HANDLE) -> HANDLE;
	pub fn SetWindowLongPtrW(hWnd: HANDLE, nIndex: i32, dwNewLong: isize) -> isize;
	pub fn SetWindowTextW(hWnd: HANDLE, lpString: *const u16) -> u32;
	pub fn ShowWindow(hWnd: HANDLE, nCmdShow: i32) -> u32;
	pub fn TranslateAcceleratorW(hWnd: HANDLE, hAccTable: HANDLE, lpMsg: *mut c_void) -> i32;
	pub fn TranslateMessage(lpMsg: *const c_void) -> u32;
	pub fn UnregisterClassW(lpClassName: *const u16, hInstance: HANDLE) -> u32;
	pub fn UpdateWindow(hWnd: HANDLE) -> u32;
}