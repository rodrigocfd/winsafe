//! Aliases to Win32 types.

use crate::co;
use crate::handles::HWND;

/// Type alias to callback function.
///
/// Used in:
/// * [`RemoveWindowSubclass`](crate::HWND::RemoveWindowSubclass) `pfnSubclass`;
/// * [`SetWindowSubclass`](crate::HWND::SetWindowSubclass) `pfnSubclass`.
pub type SUBCLASSPROC =
	unsafe extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
		uIdSubclass: usize,
		dwRefData: usize,
	) -> isize;

/// Type alias to callback function.
///
/// Used in:
/// * [`WNDCLASSEX`](crate::WNDCLASSEX) `lpfnWndProc`.
pub type WNDPROC =
	unsafe extern "system" fn(
		hWnd: HWND, uMsg: co::WM, wParam: usize, lParam: isize,
	) -> isize;