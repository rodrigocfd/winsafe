//! Aliases to Win32 types.

use crate::co;
use crate::handles::HWND;

/// Type alias to
/// [`HOOKPROC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-hookproc)
/// callback function.
pub type HOOKPROC =
	unsafe extern "system" fn(
		code: i32,
		wParam: usize,
		lParam: isize,
	) -> isize;

/// Type alias to
/// [`TIMERPROC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-timerproc)
/// callback function.
pub type TIMERPROC =
	unsafe extern "system" fn(
		hWnd: HWND,
		msg: co::WM,
		timerId: usize,
		nSeconds: u32,
	);

/// Type alias to
/// [`SUBCLASSPROC`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-subclassproc)
/// callback function.
pub type SUBCLASSPROC =
	unsafe extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
		uIdSubclass: usize,
		dwRefData: usize,
	) -> isize;

/// Type alias to
/// [`WNDPROC`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms633573(v=vs.85))
/// callback function.
pub type WNDPROC =
	unsafe extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
	) -> isize;