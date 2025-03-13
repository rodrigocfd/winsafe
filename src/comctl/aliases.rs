use crate::co;
use crate::decl::*;

/// Type alias to
/// [`LPFNPSPCALLBACK`](https://learn.microsoft.com/en-us/windows/win32/api/prsht/nc-prsht-lpfnpspcallbackw)
/// callback function.
pub type LPFNPSPCALLBACK = extern "system" fn(hwnd: HWND, uMsg: u32) -> u32;

/// Type alias to
/// [`PFNLVCOMPARE`](https://learn.microsoft.com/en-us/windows/win32/controls/lvm-sortitems)
/// callback function.
pub type PFNLVCOMPARE =
	extern "system" fn(lParam1: isize, lParam2: isize, lParamSort: isize) -> i32;

/// Type alias to
/// [`PFNLVGROUPCOMPARE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-pfnlvgroupcompare)
/// callback function.
pub type PFNLVGROUPCOMPARE =
	extern "system" fn(groupId1: i32, groupId2: i32, lpRefData: isize) -> i32;

/// Type alias to
/// [`PFNPROPSHEETCALLBACK`](https://learn.microsoft.com/en-us/windows/win32/api/prsht/nc-prsht-pfnpropsheetcallback)
/// callback function.
pub type PFNPROPSHEETCALLBACK = extern "system" fn(hwnd: HWND, uMsg: u32, lParam: isize) -> i32;

/// Type alias to
/// [`PFNTVCOMPARE`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-tvsortcb)
/// callback function.
pub type PFNTVCOMPARE =
	extern "system" fn(lParam1: isize, lParam2: isize, lParamSort: isize) -> i32;

/// Type alias to
/// [`PFTASKDIALOGCALLBACK`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-pftaskdialogcallback)
/// calback function.
pub type PFTASKDIALOGCALLBACK = extern "system" fn(
	hWnd: HWND,
	msg: co::TDN,
	wParam: usize,
	lParam: isize,
	lpRefData: isize,
) -> co::HRESULT;

/// Type alias to
/// [`SUBCLASSPROC`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nc-commctrl-subclassproc)
/// callback function.
pub type SUBCLASSPROC = extern "system" fn(
	hWnd: HWND,
	uMsg: co::WM,
	wParam: usize,
	lParam: isize,
	uIdSubclass: usize,
	dwRefData: usize,
) -> isize;
