use crate::co;
use crate::user::decl::HWND;

/// Type alias to
/// [`DLGPROC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-dlgproc)
/// callback function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub type DLGPROC =
	extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
	) -> isize;

/// Type alias to
/// [`EDITWORDBREAKPROC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-editwordbreakprocw)
/// callback function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub type EDITWORDBREAKPROC =
	extern "system" fn(
		lpch: *mut u16,
		ichCurrent: i32,
		cch: i32,
		code: i32,
	) -> i32;

/// Type alias to
/// [`HOOKPROC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-hookproc)
/// callback function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub type HOOKPROC =
	extern "system" fn(
		code: i32,
		wParam: usize,
		lParam: isize,
	) -> isize;

/// Type alias to
/// [`TIMERPROC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-timerproc)
/// callback function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub type TIMERPROC =
	extern "system" fn(
		hWnd: HWND,
		msg: co::WM,
		timerId: usize,
		nSeconds: u32,
	);

/// Type alias to
/// [`WNDPROC`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms633573(v=vs.85))
/// callback function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub type WNDPROC =
	extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
	) -> isize;
