#![allow(non_snake_case)]

use crate::co;
use crate::ffi_types::BOOL;
use crate::kernel::decl::{GetLastError, HINSTANCE, WinResult, WString};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::MsgSend;
use crate::user;
use crate::user::decl::{ATOM, COLORREF, DEVMODE, GUITHREADINFO, HWND, MSG,
	POINT, RECT, SIZE, TRACKMOUSEEVENT, WNDCLASSEX};

/// [`AdjustWindowRectEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn AdjustWindowRectEx(
	rc: &mut RECT, style: co::WS,
	has_menu: bool, ex_style: co::WS_EX) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			user::ffi::AdjustWindowRectEx(
				rc as *mut _ as _,
				style.0,
				has_menu as _,
				ex_style.0,
			)
		},
	)
}

/// [`AnyPopup`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-anypopup)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn AnyPopup() -> bool {
	unsafe { user::ffi::AnyPopup() != 0 }
}

/// [`AttachThreadInput`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-attachthreadinput)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn AttachThreadInput(
	attach_id: u32, attach_to_id: u32, do_attach: bool) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			user::ffi::AttachThreadInput(attach_id, attach_to_id, do_attach as _)
		},
	)
}

/// [`ChangeDisplaySettings`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-changedisplaysettingsw)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn ChangeDisplaySettings(
	dev_mode: &mut DEVMODE,
	flags: co::CDS) -> Result<co::DISP_CHANGE, co::DISP_CHANGE>
{
	let ret = unsafe {
		user::ffi::ChangeDisplaySettingsW(dev_mode as *mut _ as _, flags.0)
	};

	if ret < 0 {
		Err(co::DISP_CHANGE(ret))
	} else {
		Ok(co::DISP_CHANGE(ret))
	}
}

/// [`ClipCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clipcursor)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn ClipCursor(rc: Option<&RECT>) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			user::ffi::ClipCursor(
				rc.map_or(std::ptr::null(), |lp| lp as *const _ as _),
			)
		},
	)
}

/// [`CloseClipboard`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn CloseClipboard() -> WinResult<()> {
	bool_to_winresult(unsafe { user::ffi::CloseClipboard() })
}

/// [`DispatchMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn DispatchMessage(msg: &MSG) -> isize {
	unsafe { user::ffi::DispatchMessageW(msg as *const _ as _) }
}

/// [`EmptyClipboard`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-emptyclipboard)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn EmptyClipboard() -> WinResult<()> {
	bool_to_winresult(unsafe { user::ffi::EmptyClipboard() })
}

/// [`EndMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endmenu)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn EndMenu() -> WinResult<()> {
	bool_to_winresult(unsafe { user::ffi::EndMenu() })
}

/// [`EnumDisplaySettingsEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaysettingsexw)
/// function
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn EnumDisplaySettingsEx(
	device_name: Option<&str>,
	mode_num: co::ENUM_SETTINGS,
	dev_mode: &mut DEVMODE,
	flags: co::EDS) -> WinResult<()>
{
	bool_to_winresult(
		unsafe {
			user::ffi::EnumDisplaySettingsExW(
				device_name.map_or(std::ptr::null(), |lp| WString::from_str(lp).as_ptr()),
				mode_num.0,
				dev_mode as *mut _ as _,
				flags.0
			)
		},
	)
}

/// [`EnumWindows`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumwindows)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{EnumWindows, HWND};
///
/// EnumWindows(|hwnd: HWND| -> bool {
///     println!("HWND: {}", hwnd);
///     true
/// })?;
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn EnumWindows<F>(func: F) -> WinResult<()>
	where F: Fn(HWND) -> bool,
{
	bool_to_winresult(
		unsafe {
			user::ffi::EnumWindows(
				enum_windows_proc::<F> as _,
				&func as *const _ as _,
			)
		},
	)
}
extern "system" fn enum_windows_proc<F>(hwnd: HWND, lparam: isize) -> BOOL
	where F: Fn(HWND) -> bool,
{
	let func = unsafe { &*(lparam as *const F) };
	func(hwnd) as _
}

/// [`GetAsyncKeyState`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getasynckeystate)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetAsyncKeyState(virt_key: co::VK) -> bool {
	unsafe { user::ffi::GetAsyncKeyState(virt_key.0 as _) != 0 }
}

/// [`GetClipCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclipcursor)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetClipCursor() -> WinResult<RECT> {
	let mut rc = RECT::default();
	bool_to_winresult(
		unsafe { user::ffi::GetClipCursor(&mut rc as *mut _ as _) },
	).map(|_| rc)
}

/// [`GetCursorPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetCursorPos() -> WinResult<POINT> {
	let mut pt = POINT::default();
	bool_to_winresult(
		unsafe { user::ffi::GetCursorPos(&mut pt as *mut _ as _) },
	).map(|_| pt)
}

/// [`GetDialogBaseUnits`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdialogbaseunits)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetDialogBaseUnits() -> i32 {
	unsafe { user::ffi::GetDialogBaseUnits() }
}

/// [`GetDoubleClickTime`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdoubleclicktime)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetDoubleClickTime() -> u32 {
	unsafe { user::ffi::GetDoubleClickTime() }
}

/// [`GetGUIThreadInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getguithreadinfo)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{GetGUIThreadInfo, GUITHREADINFO, HWND};
///
/// let hwnd: HWND; // initialized somewhere
/// # let hwnd = HWND::NULL;
///
/// let (thread_id, _) = hwnd.GetWindowThreadProcessId();
///
/// let mut gti = GUITHREADINFO::default();
/// GetGUIThreadInfo(thread_id, &mut gti)?;
///
/// println!("Caret rect: {}", gti.rcCaret);
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetGUIThreadInfo(
	thread_id: u32, gti: &mut GUITHREADINFO) -> WinResult<()>
{
	bool_to_winresult(
		unsafe { user::ffi::GetGUIThreadInfo(thread_id, gti as *mut _ as _) }
	)
}

/// [`GetMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetMessage(
	msg: &mut MSG, hwnd: Option<HWND>,
	msg_filter_min: u32, msg_filter_max: u32) -> WinResult<bool>
{
	match unsafe {
		user::ffi::GetMessageW(
			msg as *mut _ as _,
			hwnd.map_or(std::ptr::null_mut(), |h| h.0),
			msg_filter_min, msg_filter_max,
		)
	} {
		-1 => Err(GetLastError()),
		0 => Ok(false),
		_ => Ok(true),
	}
}

/// [`GetMenuCheckMarkDimensions`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenucheckmarkdimensions)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetMenuCheckMarkDimensions() -> SIZE {
	SIZE::from_u32(unsafe { user::ffi::GetMenuCheckMarkDimensions() } as _)
}

/// [`GetMessagePos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagepos)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetMessagePos() -> POINT {
	POINT::from_u32(unsafe { user::ffi::GetMessagePos() })
}

/// [`GetQueueStatus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getqueuestatus)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetQueueStatus(flags: co::QS) -> u32 {
	unsafe { user::ffi::GetQueueStatus(flags.0) }
}

/// [`GetSysColor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetSysColor(index: co::COLOR) -> COLORREF {
	COLORREF(unsafe { user::ffi::GetSysColor(index.0) })
}

/// [`GetSystemMetrics`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn GetSystemMetrics(index: co::SM) -> i32 {
	unsafe { user::ffi::GetSystemMetrics(index.0) }
}

/// [`InSendMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessage)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn InSendMessage() -> bool {
	unsafe { user::ffi::InSendMessage() != 0 }
}

/// [`InSendMessageEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessageex)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn InSendMessageEx() -> co::ISMEX {
	co::ISMEX(unsafe { user::ffi::InSendMessageEx()})
}

/// [`IsGUIThread`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isguithread)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn IsGUIThread(convert_to_gui_thread: bool) -> WinResult<bool> {
	let r = unsafe { user::ffi::IsGUIThread(convert_to_gui_thread as _) };
	if convert_to_gui_thread {
		match r {
			0 => Ok(false),
			1 => Ok(true),
			err => Err(co::ERROR(err as _)),
		}
	} else {
		Ok(r != 0)
	}
}

/// [`IsWow64Message`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswow64message)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn IsWow64Message() -> bool {
	return unsafe { user::ffi::IsWow64Message() != 0}
}

/// [`LockSetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-locksetforegroundwindow)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn LockSetForegroundWindow(lock_code: co::LSFW) -> WinResult<()> {
	bool_to_winresult(
		unsafe { user::ffi::LockSetForegroundWindow(lock_code.0) },
	)
}

/// [`PeekMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn PeekMessage(
	msg: &mut MSG, hwnd: Option<HWND>,
	msg_filter_min: u32, msg_filter_max: u32, remove_msg: co::PM) -> bool
{
	unsafe {
		user::ffi::PeekMessageW(
			msg as *mut _ as _,
			hwnd.map_or(std::ptr::null_mut(), |h| h.0),
			msg_filter_min,
			msg_filter_max,
			remove_msg.0,
		) != 0
	}
}

/// [`PostQuitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn PostQuitMessage(exit_code: i32) {
	unsafe { user::ffi::PostQuitMessage(exit_code) }
}

/// [`PostThreadMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postthreadmessagew)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn PostThreadMessage<M: MsgSend>(thread_id: u32, msg: M) -> WinResult<()> {
	let mut msg = msg;
	let wm_any = msg.as_generic_wm();
	bool_to_winresult(
		unsafe {
			user::ffi::PostThreadMessageW(
				thread_id, wm_any.msg_id.0, wm_any.wparam, wm_any.lparam,
			)
		}
	)
}

/// [`RegisterClassEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn RegisterClassEx(wcx: &WNDCLASSEX) -> WinResult<ATOM> {
	match unsafe { user::ffi::RegisterClassExW(wcx as *const _ as _) } {
		0 => Err(GetLastError()),
		atom => Ok(ATOM(atom)),
	}
}

/// [`ReleaseCapture`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasecapture)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn ReleaseCapture() -> WinResult<()> {
	bool_to_winresult(unsafe { user::ffi::ReleaseCapture() })
}

/// [`SetCaretBlinkTime`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcaretblinktime)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn SetCaretBlinkTime(milliseconds: u32) -> WinResult<()> {
	bool_to_winresult(
		unsafe { user::ffi::SetCaretBlinkTime(milliseconds) },
	)
}

/// [`SetCaretPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcaretpos)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn SetCaretPos(x: i32, y: i32) -> WinResult<()> {
	bool_to_winresult(unsafe { user::ffi::SetCaretPos(x, y) })
}

/// [`SetClipboardData`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setclipboarddata)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn SetClipboardData(format: co::CF, hmem: *mut u8) -> WinResult<*mut u8> {
	unsafe { user::ffi::SetClipboardData(format.0, hmem as _).as_mut() }
		.map(|hmem| hmem as *mut _ as _)
		.ok_or_else(|| GetLastError())
}

/// [`SetCursorPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursorpos)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn SetCursorPos(x: i32, y: i32) -> WinResult<()> {
	bool_to_winresult(unsafe { user::ffi::SetCursorPos(x, y) })
}

/// [`SetProcessDPIAware`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocessdpiaware)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn SetProcessDPIAware() -> WinResult<()> {
	bool_to_winresult(unsafe { user::ffi::SetProcessDPIAware() })
}

/// [`ShowCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcursor)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn ShowCursor(show: bool) -> i32 {
	unsafe { user::ffi::ShowCursor(show as _) }
}

/// [`SoundSentry`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-soundsentry)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn SoundSentry() -> bool {
	unsafe { user::ffi::SoundSentry() != 0 }
}

/// [`SystemParametersInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow)
/// function.
///
/// **Note:** The `pv_param` type varies according to `action`. If you set it
/// wrong, you're likely to cause a buffer overrun.
pub unsafe fn SystemParametersInfo<T>(
	action: co::SPI,
	ui_param: u32,
	pv_param: &mut T,
	win_ini: co::SPIF) -> WinResult<()>
{
	bool_to_winresult(
		user::ffi::SystemParametersInfoW(
			action.0,
			ui_param,
			pv_param as *mut _ as _,
			win_ini.0,
		),
	)
}

/// [`TrackMouseEvent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackmouseevent)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn TrackMouseEvent(tme: &mut TRACKMOUSEEVENT) -> WinResult<()> {
	bool_to_winresult(
		unsafe { user::ffi::TrackMouseEvent(tme as *mut _ as _) },
	)
}

/// [`TranslateMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn TranslateMessage(msg: &MSG) -> bool {
	unsafe { user::ffi::TranslateMessage(msg as *const _ as _) != 0 }
}

/// [`UnregisterClass`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn UnregisterClass(class_name: &str, hinst: HINSTANCE) -> WinResult<()> {
	bool_to_winresult(
		unsafe {
			user::ffi::UnregisterClassW(
				WString::from_str(class_name).as_ptr(),
				hinst.0,
			)
		},
	)
}

/// [`WaitMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-waitmessage)
/// function.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub fn WaitMessage() -> WinResult<()> {
	bool_to_winresult(unsafe { user::ffi::WaitMessage() })
}
