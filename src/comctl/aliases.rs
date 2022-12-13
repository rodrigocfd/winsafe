use crate::co;
use crate::user::decl::HWND;

/// Type alias to
/// [`PFNLVCOMPARE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-listview_sortitems)
/// callback function.
pub type PFNLVCOMPARE =
	extern "system" fn(
		lParam1: isize,
		lParam2: isize,
		lParamSort: isize,
	) -> i32;

/// Type alias to
/// [`PFNLVGROUPCOMPARE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-pfnlvgroupcompare)
/// callback function.
pub type PFNLVGROUPCOMPARE =
	extern "system" fn(
		groupId1: i32,
		groupId2: i32,
		lpRefData: isize,
	) -> i32;

/// Type alias to
/// [`SUBCLASSPROC`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-subclassproc)
/// callback function.
pub type SUBCLASSPROC =
	extern "system" fn(
		hWnd: HWND,
		uMsg: co::WM,
		wParam: usize,
		lParam: isize,
		uIdSubclass: usize,
		dwRefData: usize,
	) -> isize;
