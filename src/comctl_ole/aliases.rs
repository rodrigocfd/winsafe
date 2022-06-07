use crate::co;
use crate::user::decl::HWND;

/// Type alias to
/// [`PFTASKDIALOGCALLBACK`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-pftaskdialogcallback)
/// calback function.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "ole"))))]
pub type PFTASKDIALOGCALLBACK =
	extern "system" fn(
		hWnd: HWND,
		msg: co::WM,
		wParam: usize,
		lParam: isize,
		lpRefData: isize,
	);
