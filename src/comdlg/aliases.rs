use crate::user::decl::HWND;

/// Type alias to
/// [`CCHOOKPROC`](https://learn.microsoft.com/en-us/windows/win32/api/commdlg/nc-commdlg-lpcchookproc)
/// callback function.
#[cfg_attr(docsrs, doc(cfg(feature = "comdlg")))]
pub type CCHOOKPROC =
	extern "system" fn(
		hWnd: HWND,
		uMsg: u32,
		wParam: usize,
		lParam: isize,
	) -> usize;
