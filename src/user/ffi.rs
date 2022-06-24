use crate::ffi_types::{BOOL, HANDLE, PCSTR, PCVOID, PFUNC, PSTR, PVOID};

#[cfg(target_pointer_width = "32")]
extern_sys! { "user32";
	GetWindowLongW(HANDLE, i32) -> isize
	SetWindowLongW(HANDLE, i32, isize) -> isize
}

#[cfg(target_pointer_width = "64")]
extern_sys! { "user32";
	GetWindowLongPtrW(HANDLE, i32) -> isize
	SetWindowLongPtrW(HANDLE, i32, isize) -> isize
	InSendMessageEx() -> u32
}

extern_sys! { "user32";
	AdjustWindowRectEx(PVOID, u32, BOOL, u32) -> BOOL
	AllowSetForegroundWindow(u32) -> BOOL
	AnyPopup() -> BOOL
	AppendMenuW(HANDLE, u32, usize, PCSTR) -> BOOL
	ArrangeIconicWindows(HANDLE) -> u32
	AttachThreadInput(u32, u32, BOOL) -> BOOL
	BeginDeferWindowPos(i32) -> HANDLE
	BeginPaint(HANDLE, PVOID) -> HANDLE
	BringWindowToTop(HANDLE) -> BOOL
	CallNextHookEx(HANDLE, i32, usize, isize) -> isize
	ChangeDisplaySettingsExW(PCSTR, PVOID, PVOID, u32, PVOID) -> i32
	ChangeDisplaySettingsW(PVOID, u32) -> i32
	CheckMenuItem(HANDLE, u32, u32) -> i32
	CheckMenuRadioItem(HANDLE, u32, u32, u32, u32) -> BOOL
	ChildWindowFromPoint(HANDLE, i32, i32) -> HANDLE
	ClientToScreen(HANDLE, PVOID) -> BOOL
	ClipCursor(PCVOID) -> BOOL
	CloseClipboard() -> BOOL
	CloseWindow(HANDLE) -> BOOL
	CopyIcon(HANDLE) -> HANDLE
	CreateAcceleratorTableW(PVOID, i32) -> HANDLE
	CreateDialogParamW(HANDLE, PCSTR, HANDLE, PFUNC, isize) -> HANDLE
	CreateMenu() -> HANDLE
	CreatePopupMenu() -> HANDLE
	CreateWindowExW(u32, PCSTR, PCSTR, u32, i32, i32, i32, i32, HANDLE, HANDLE, HANDLE, PVOID) -> HANDLE
	DeferWindowPos(HANDLE, HANDLE, HANDLE, i32, i32, i32, i32, u32) -> HANDLE
	DefWindowProcW(HANDLE, u32, usize, isize) -> isize
	DeleteMenu(HANDLE, u32, u32) -> BOOL
	DestroyAcceleratorTable(HANDLE) -> BOOL
	DestroyCursor(HANDLE) -> BOOL
	DestroyIcon(HANDLE) -> BOOL
	DestroyMenu(HANDLE) -> BOOL
	DestroyWindow(HANDLE) -> BOOL
	DialogBoxParamW(HANDLE, PCSTR, HANDLE, PFUNC, isize) -> isize
	DispatchMessageW(PCVOID) -> isize
	DrawMenuBar(HANDLE) -> BOOL
	EmptyClipboard() -> BOOL
	EnableMenuItem(HANDLE, u32, u32) -> BOOL
	EnableWindow(HANDLE, BOOL) -> BOOL
	EndDeferWindowPos(HANDLE) -> BOOL
	EndDialog(HANDLE, isize) -> BOOL
	EndMenu() -> BOOL
	EndPaint(HANDLE, PCVOID) -> BOOL
	EnumChildWindows(HANDLE, PFUNC, isize) -> BOOL
	EnumDisplayDevicesW(PCSTR, u32, PVOID, u32) -> BOOL
	EnumDisplayMonitors(HANDLE, PCVOID, PFUNC, isize) -> BOOL
	EnumDisplaySettingsExW(PCSTR, u32, PVOID, u32) -> BOOL
	EnumDisplaySettingsW(PCSTR, u32, PVOID) -> BOOL
	EnumWindows(PFUNC, isize) -> BOOL
	FindWindowExW(HANDLE, HANDLE, PCSTR, PCSTR) -> HANDLE
	FindWindowW(PCSTR, PCSTR) -> HANDLE
	GetActiveWindow() -> HANDLE
	GetAltTabInfoW(HANDLE, i32, PVOID, PSTR, u32) -> BOOL
	GetAncestor(HANDLE, u32) -> HANDLE
	GetAsyncKeyState(i32) -> i16
	GetCapture() -> HANDLE
	GetClassInfoExW(HANDLE, PCSTR, PVOID) -> BOOL
	GetClassLongPtrW(HANDLE, i32) -> usize
	GetClassNameW(HANDLE, PSTR, i32) -> i32
	GetClientRect(HANDLE, PVOID) -> BOOL
	GetClipboardSequenceNumber() -> u32
	GetClipCursor(PVOID) -> BOOL
	GetCursorPos(PVOID) -> BOOL
	GetDC(HANDLE) -> HANDLE
	GetDesktopWindow() -> HANDLE
	GetDialogBaseUnits() -> i32
	GetDlgCtrlID(HANDLE) -> i32
	GetDlgItem(HANDLE, i32) -> HANDLE
	GetDoubleClickTime() -> u32
	GetFocus() -> HANDLE
	GetForegroundWindow() -> HANDLE
	GetGUIThreadInfo(u32, PVOID) -> BOOL
	GetLastActivePopup(HANDLE) -> HANDLE
	GetMenu(HANDLE) -> HANDLE
	GetMenuBarInfo(HANDLE, i32, i32, PVOID) -> BOOL
	GetMenuCheckMarkDimensions() -> i32
	GetMenuDefaultItem(HANDLE, u32, u32) -> u32
	GetMenuInfo(HANDLE, PVOID) -> BOOL
	GetMenuItemCount(HANDLE) -> i32
	GetMenuItemID(HANDLE, i32) -> i32
	GetMenuItemInfoW(HANDLE, u32, BOOL, PVOID) -> BOOL
	GetMenuItemRect(HANDLE, HANDLE, u32, PVOID) -> BOOL
	GetMenuState(HANDLE, u32, u32) -> u32
	GetMenuStringW(HANDLE, u32, PSTR, i32, u32) -> i32
	GetMessagePos() -> u32
	GetMessageW(PVOID, HANDLE, u32, u32) -> BOOL
	GetMonitorInfoW(HANDLE, PVOID) -> BOOL
	GetNextDlgGroupItem(HANDLE, HANDLE, BOOL) -> HANDLE
	GetNextDlgTabItem(HANDLE, HANDLE, BOOL) -> HANDLE
	GetParent(HANDLE) -> HANDLE
	GetQueueStatus(u32) -> u32
	GetScrollInfo(HANDLE, i32, PVOID) -> BOOL
	GetScrollPos(HANDLE, i32) -> i32
	GetShellWindow() -> HANDLE
	GetSubMenu(HANDLE, i32) -> HANDLE
	GetSysColor(i32) -> u32
	GetSystemMenu(HANDLE, BOOL) -> HANDLE
	GetSystemMetrics(i32) -> i32
	GetTopWindow(HANDLE) -> HANDLE
	GetUpdateRgn(HANDLE, HANDLE, BOOL) -> i32
	GetWindow(HANDLE, u32) -> HANDLE
	GetWindowDC(HANDLE) -> HANDLE
	GetWindowDisplayAffinity(HANDLE, PVOID) -> BOOL
	GetWindowInfo(HANDLE, PVOID) -> BOOL
	GetWindowPlacement(HANDLE, PVOID) -> BOOL
	GetWindowRect(HANDLE, PVOID) -> BOOL
	GetWindowRgn(HANDLE, HANDLE) -> i32
	GetWindowRgnBox(HANDLE, PVOID) -> i32
	GetWindowTextLengthW(HANDLE) -> i32
	GetWindowTextW(HANDLE, PSTR, i32) -> i32
	GetWindowThreadProcessId(HANDLE, *mut u32) -> u32
	HiliteMenuItem(HANDLE, HANDLE, u32, u32) -> BOOL
	InSendMessage() -> BOOL
	InsertMenuItemW(HANDLE, u32, BOOL, PCVOID) -> BOOL
	InvalidateRect(HANDLE, PCVOID, BOOL) -> BOOL
	InvalidateRgn(HANDLE, HANDLE, BOOL) -> BOOL
	IsChild(HANDLE, HANDLE) -> BOOL
	IsDialogMessageW(HANDLE, PVOID) -> BOOL
	IsGUIThread(BOOL) -> BOOL
	IsIconic(HANDLE) -> BOOL
	IsMenu(HANDLE) -> BOOL
	IsWindow(HANDLE) -> BOOL
	IsWindowEnabled(HANDLE) -> BOOL
	IsWindowUnicode(HANDLE) -> BOOL
	IsWindowVisible(HANDLE) -> BOOL
	IsWow64Message() -> BOOL
	IsZoomed(HANDLE) -> BOOL
	KillTimer(HANDLE, usize) -> BOOL
	LoadAcceleratorsW(HANDLE, PCSTR) -> HANDLE
	LoadCursorW(HANDLE, PCSTR) -> HANDLE
	LoadIconW(HANDLE, PCSTR) -> HANDLE
	LoadImageW(HANDLE, PCSTR, u32, i32, i32, u32) -> HANDLE
	LoadMenuW(HANDLE, PCSTR) -> HANDLE
	LoadStringW(HANDLE, u32, PSTR, i32) -> i32
	LockSetForegroundWindow(u32) -> BOOL
	LogicalToPhysicalPoint(HANDLE, PVOID) -> BOOL
	MapDialogRect(HANDLE, PVOID) -> BOOL
	MessageBoxW(HANDLE, PCSTR, PCSTR, u32) -> i32
	MonitorFromPoint(i32, i32, u32) -> HANDLE
	MonitorFromRect(PCVOID, u32) -> HANDLE
	MonitorFromWindow(HANDLE, u32) -> HANDLE
	MoveWindow(HANDLE, i32, i32, i32, i32, BOOL) -> BOOL
	OpenClipboard(HANDLE) -> BOOL
	PeekMessageW(PVOID, HANDLE, u32, u32, u32) -> BOOL
	PostMessageW(HANDLE, u32, usize, isize) -> BOOL
	PostQuitMessage(i32)
	PostThreadMessageW(u32, u32, usize, isize) -> BOOL
	RealChildWindowFromPoint(HANDLE, i32, i32) -> HANDLE
	RealGetWindowClassW(HANDLE, PSTR, i32) -> u32
	RedrawWindow(HANDLE, PCVOID, HANDLE, u32) -> BOOL
	RegisterClassExW(PCVOID) -> u16
	ReleaseCapture() -> BOOL
	ReleaseDC(HANDLE, HANDLE) -> i32
	RemoveMenu(HANDLE, u32, u32) -> BOOL
	ScreenToClient(HANDLE, PVOID) -> BOOL
	SendMessageTimeoutW(HANDLE, u32, usize, isize, u32, u32, *mut isize) -> isize
	SendMessageW(HANDLE, u32, usize, isize) -> isize
	SetCapture(HANDLE) -> HANDLE
	SetCaretBlinkTime(u32) -> BOOL
	SetCaretPos(i32, i32) -> BOOL
	SetClipboardData(u32, HANDLE) -> HANDLE
	SetCursorPos(i32, i32) -> BOOL
	SetFocus(HANDLE) -> HANDLE
	SetForegroundWindow(HANDLE) -> BOOL
	SetMenu(HANDLE, HANDLE) -> BOOL
	SetMenuDefaultItem(HANDLE, u32, u32) -> BOOL
	SetMenuInfo(HANDLE, PCVOID) -> BOOL
	SetMenuItemBitmaps(HANDLE, u32, u32, HANDLE, HANDLE) -> BOOL
	SetMenuItemInfoW(HANDLE, u32, BOOL, PCVOID) -> BOOL
	SetParent(HANDLE, HANDLE) -> HANDLE
	SetProcessDPIAware() -> BOOL
	SetScrollInfo(HANDLE, i32, PCVOID, BOOL) -> i32
	SetScrollPos(HANDLE, i32, i32, BOOL) -> i32
	SetScrollRange(HANDLE, i32, i32, i32, BOOL) -> BOOL
	SetSystemCursor(HANDLE, u32) -> BOOL
	SetTimer(HANDLE, usize, u32, PFUNC) -> usize
	SetUserObjectInformationW(HANDLE, i32, PVOID, u32) -> BOOL
	SetWindowDisplayAffinity(HANDLE, u32) -> BOOL
	SetWindowPlacement(HANDLE, PCVOID) -> BOOL
	SetWindowPos(HANDLE, HANDLE, i32, i32, i32, i32, u32) -> BOOL
	SetWindowRgn(HANDLE, HANDLE, BOOL) -> i32
	SetWindowsHookExW(i32, PFUNC, HANDLE, u32) -> HANDLE
	SetWindowTextW(HANDLE, PCSTR) -> BOOL
	ShowCaret(HANDLE) -> BOOL
	ShowCursor(BOOL) -> i32
	ShowWindow(HANDLE, i32) -> BOOL
	ShowWindowAsync(HANDLE, i32) -> BOOL
	SoundSentry() -> BOOL
	SystemParametersInfoW(u32, u32, PVOID, u32) -> BOOL
	TrackMouseEvent(PVOID) -> BOOL
	TrackPopupMenu(HANDLE, u32, i32, i32, i32, HANDLE, PCVOID) -> BOOL
	TranslateAcceleratorW(HANDLE, HANDLE, PVOID) -> i32
	TranslateMessage(PCVOID) -> BOOL
	UnhookWindowsHookEx(HANDLE) -> BOOL
	UnregisterClassW(PCSTR, HANDLE) -> BOOL
	UpdateWindow(HANDLE) -> BOOL
	ValidateRect(HANDLE, PCVOID) -> BOOL
	ValidateRgn(HANDLE, HANDLE) -> BOOL
	WaitMessage() -> BOOL
	WindowFromPhysicalPoint(i32, i32) -> HANDLE
	WindowFromPoint(i32, i32) -> HANDLE
	WinHelpW(HANDLE, PCSTR, u32, usize) -> BOOL
}
