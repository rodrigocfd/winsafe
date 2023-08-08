use crate::co;
use crate::decl::*;

/// Type alias to
/// [`PFTASKDIALOGCALLBACK`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-pftaskdialogcallback)
/// calback function.
pub type PFTASKDIALOGCALLBACK =
	extern "system" fn(
		hWnd: HWND,
		msg: co::WM,
		wParam: usize,
		lParam: isize,
		lpRefData: isize,
	);
