#![allow(non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::{GetLastError, HINSTANCE, SysResult, WString};
use crate::kernel::ffi_types::BOOL;
use crate::kernel::privs::{bool_to_sysresult, ptr_to_sysresult};
use crate::prelude::MsgSend;
use crate::user::decl::{
	ATOM, COLORREF, DEVMODE, DISPLAY_DEVICE, GmidxEnum, GUITHREADINFO, HWND,
	MSG, POINT, RECT, SIZE, TRACKMOUSEEVENT, WNDCLASSEX,
};
use crate::user::privs::ASFW_ANY;

/// [`AdjustWindowRectEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex)
/// function.
pub fn AdjustWindowRectEx(
	rc: &mut RECT,
	style: co::WS,
	has_menu: bool,
	ex_style: co::WS_EX) -> SysResult<()>
{
	bool_to_sysresult(
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

/// [`AllowSetForegroundWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-allowsetforegroundwindow)
/// function
pub fn AllowSetForegroundWindow(process_id: Option<u32>) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			user::ffi::AllowSetForegroundWindow(process_id.unwrap_or(ASFW_ANY))
		},
	)
}

/// [`AnyPopup`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-anypopup)
/// function.
#[must_use]
pub fn AnyPopup() -> bool {
	unsafe { user::ffi::AnyPopup() != 0 }
}

/// [`AttachThreadInput`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-attachthreadinput)
/// function.
pub fn AttachThreadInput(
	attach_id: u32, attach_to_id: u32, do_attach: bool) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			user::ffi::AttachThreadInput(attach_id, attach_to_id, do_attach as _)
		},
	)
}

/// [`BroadcastSystemMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-broadcastsystemmessage)
/// function.
pub fn BroadcastSystemMessage<M>(
	flags: co::BSF, info: co::BSM, msg: M) -> SysResult<co::BSM>
	where M: MsgSend,
{
	let mut msg = msg;
	let wm_any = msg.as_generic_wm();

	let mut info_ret = info;

	if unsafe {
		user::ffi::BroadcastSystemMessageW(
			flags.0,
			&mut info_ret.0 as _,
			wm_any.msg_id.0,
			wm_any.wparam,
			wm_any.lparam,
		)
	} > 0 {
		Ok(info_ret)
	} else {
		Err(GetLastError())
	}
}

/// [`ChangeDisplaySettings`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-changedisplaysettingsw)
/// function.
pub fn ChangeDisplaySettings(
	dev_mode: Option<&mut DEVMODE>,
	flags: co::CDS) -> Result<co::DISP_CHANGE, co::DISP_CHANGE>
{
	let ret = unsafe {
		user::ffi::ChangeDisplaySettingsW(
			dev_mode.map_or(std::ptr::null_mut(), |dm| dm as *mut _ as _),
			flags.0
		)
	};

	if ret < 0 {
		Err(co::DISP_CHANGE(ret))
	} else {
		Ok(co::DISP_CHANGE(ret))
	}
}

/// [`ChangeDisplaySettingsEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-changedisplaysettingsexw)
/// function.
pub fn ChangeDisplaySettingsEx(
	device_name: Option<&str>,
	dev_mode: Option<&mut DEVMODE>,
	flags: co::CDS) -> Result<co::DISP_CHANGE, co::DISP_CHANGE>
{
	let ret = unsafe {
		user::ffi::ChangeDisplaySettingsExW(
			WString::from_opt_str(device_name).as_ptr(),
			dev_mode.map_or(std::ptr::null_mut(), |dm| dm as *mut _ as _),
			std::ptr::null_mut(),
			flags.0,
			std::ptr::null_mut(),
		)
	};

	if ret < 0 {
		Err(co::DISP_CHANGE(ret))
	} else {
		Ok(co::DISP_CHANGE(ret))
	}
}

/// [`ClipCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clipcursor)
/// function.
pub fn ClipCursor(rc: Option<&RECT>) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			user::ffi::ClipCursor(
				rc.map_or(std::ptr::null(), |lp| lp as *const _ as _),
			)
		},
	)
}

/// [`DispatchMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
/// function.
///
/// # Safety
///
/// This function is used internally in window loops. Avoid using it in other
/// situations.
pub unsafe fn DispatchMessage(msg: &MSG) -> isize {
	user::ffi::DispatchMessageW(msg as *const _ as _)
}

/// [`EmptyClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-emptyclipboard)
/// function.
pub fn EmptyClipboard() -> SysResult<()> {
	bool_to_sysresult(unsafe { user::ffi::EmptyClipboard() })
}

/// [`EndMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endmenu)
/// function.
pub fn EndMenu() -> SysResult<()> {
	bool_to_sysresult(unsafe { user::ffi::EndMenu() })
}

/// [`EnumDisplayDevices`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaydevicesw)
/// function.
///
/// # Examples
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, DISPLAY_DEVICE, EnumDisplayDevices};
///
/// let mut dide = DISPLAY_DEVICE::default();
/// let mut dev_num: u32 = 0;
///
/// loop {
///     let is_good = EnumDisplayDevices(
///         None, dev_num, &mut dide, co::EDD::NoValue)?;
///
///     if !is_good {
///         break;
///     }
///
///     println!("{}: {} - {}",
///         dev_num, dide.DeviceName(), dide.DeviceString());
///
///     dev_num += 1; // advance to next display device
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
pub fn EnumDisplayDevices(
	device_name: Option<&str>,
	device_num: u32,
	display_device: &mut DISPLAY_DEVICE,
	flags: co::EDD) -> SysResult<bool>
{
	match unsafe {
		user::ffi::EnumDisplayDevicesW(
			WString::from_opt_str(device_name).as_ptr(),
			device_num,
			display_device as *mut _ as _,
			flags.0,
		)
	} {
		// Empirical tests have shown that two different error codes can be
		// returned to signal the end of the loop, so we consider both.
		// https://github.com/rodrigocfd/winsafe/issues/36
		0 => match GetLastError() {
			co::ERROR::PROC_NOT_FOUND | co::ERROR::ENVVAR_NOT_FOUND => Ok(false),
			err => Err(err), // actual error
		},
		_ => Ok(true),
	}
}

/// [`EnumDisplaySettings`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaysettingsw)
/// function.
///
/// # Examples
///
/// Iterating graphics modes.
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{DEVMODE, EnumDisplaySettings, GmidxEnum};
///
/// let mut dm = DEVMODE::default();
/// let mut graphics_mode_idx: u32 = 0;
///
/// loop {
///     let is_good = EnumDisplaySettings(
///         None,
///         GmidxEnum::Gmidx(graphics_mode_idx),
///         &mut dm,
///     )?;
///
///     if !is_good {
///         break;
///     }
///
///     println!("{}: {}, {}, {}",
///         graphics_mode_idx,
///         dm.dmDeviceName(), dm.dmDisplayFrequency, dm.dmBitsPerPel);
///
///     graphics_mode_idx += 1;
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
///
/// Retrieving from the predefined enum.
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, DEVMODE, EnumDisplaySettings, GmidxEnum};
///
/// let mut dm = DEVMODE::default();
///
/// EnumDisplaySettings(
///     None,
///     GmidxEnum::Enum(co::ENUM_SETTINGS::CURRENT),
///     &mut dm,
/// )?;
///
/// println!("{}, {}, {}",
///     dm.dmDeviceName(), dm.dmDisplayFrequency, dm.dmBitsPerPel);
/// # Ok::<_, co::ERROR>(())
/// ```
pub fn EnumDisplaySettings(
	device_name: Option<&str>,
	mode_num: GmidxEnum,
	dev_mode: &mut DEVMODE) -> SysResult<bool>
{
	match unsafe {
		user::ffi::EnumDisplaySettingsW(
			WString::from_opt_str(device_name).as_ptr(),
			mode_num.into(),
			dev_mode as *mut _ as _,
		)
	} {
		0 => match GetLastError() {
			co::ERROR::PROC_NOT_FOUND => Ok(false), // actual false
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`EnumDisplaySettingsEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaysettingsexw)
/// function.
pub fn EnumDisplaySettingsEx(
	device_name: Option<&str>,
	mode_num: GmidxEnum,
	dev_mode: &mut DEVMODE,
	flags: co::EDS) -> SysResult<bool>
{
	match unsafe {
		user::ffi::EnumDisplaySettingsExW(
			WString::from_opt_str(device_name).as_ptr(),
			mode_num.into(),
			dev_mode as *mut _ as _,
			flags.0,
		)
	} {
		0 => match GetLastError() {
			co::ERROR::PROC_NOT_FOUND => Ok(false), // actual false
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`EnumWindows`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumwindows)
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
pub fn EnumWindows<F>(func: F) -> SysResult<()>
	where F: Fn(HWND) -> bool,
{
	bool_to_sysresult(
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

/// [`GetAsyncKeyState`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getasynckeystate)
/// function.
#[must_use]
pub fn GetAsyncKeyState(virt_key: co::VK) -> bool {
	unsafe { user::ffi::GetAsyncKeyState(virt_key.0 as _) != 0 }
}

/// [`GetClipboardData`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclipboarddata)
/// function.
///
/// # Safety
///
/// The returned pointer must be correctly cast to the memory block specified by
/// `format`.
#[must_use]
pub unsafe fn GetClipboardData(format: co::CF) -> SysResult<*mut u8> {
	ptr_to_sysresult(
		user::ffi::GetClipboardData(format.0),
		|hmem| hmem as *mut _ as _,
	)
}

/// [`GetClipboardSequenceNumber`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclipboardsequencenumber)
/// function.
#[must_use]
pub fn GetClipboardSequenceNumber() -> u32 {
	unsafe { user::ffi::GetClipboardSequenceNumber() }
}

/// [`GetClipCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclipcursor)
/// function.
#[must_use]
pub fn GetClipCursor() -> SysResult<RECT> {
	let mut rc = RECT::default();
	bool_to_sysresult(
		unsafe { user::ffi::GetClipCursor(&mut rc as *mut _ as _) },
	).map(|_| rc)
}

/// [`GetCursorPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos)
/// function.
#[must_use]
pub fn GetCursorPos() -> SysResult<POINT> {
	let mut pt = POINT::default();
	bool_to_sysresult(
		unsafe { user::ffi::GetCursorPos(&mut pt as *mut _ as _) },
	).map(|_| pt)
}

/// [`GetDialogBaseUnits`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdialogbaseunits)
/// function.
#[must_use]
pub fn GetDialogBaseUnits() -> i32 {
	unsafe { user::ffi::GetDialogBaseUnits() }
}

/// [`GetDoubleClickTime`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdoubleclicktime)
/// function.
#[must_use]
pub fn GetDoubleClickTime() -> u32 {
	unsafe { user::ffi::GetDoubleClickTime() }
}

/// [`GetGUIThreadInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getguithreadinfo)
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
pub fn GetGUIThreadInfo(
	thread_id: u32, gti: &mut GUITHREADINFO) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe { user::ffi::GetGUIThreadInfo(thread_id, gti as *mut _ as _) },
	)
}

/// [`GetMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
/// function.
pub fn GetMessage(
	msg: &mut MSG, hwnd: Option<&HWND>,
	msg_filter_min: u32, msg_filter_max: u32) -> SysResult<bool>
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

/// [`GetMenuCheckMarkDimensions`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenucheckmarkdimensions)
/// function.
#[must_use]
pub fn GetMenuCheckMarkDimensions() -> SIZE {
	SIZE::from(unsafe { user::ffi::GetMenuCheckMarkDimensions() })
}

/// [`GetMessagePos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagepos)
/// function.
#[must_use]
pub fn GetMessagePos() -> POINT {
	POINT::from(unsafe { user::ffi::GetMessagePos() })
}

/// [`GetQueueStatus`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getqueuestatus)
/// function.
#[must_use]
pub fn GetQueueStatus(flags: co::QS) -> u32 {
	unsafe { user::ffi::GetQueueStatus(flags.0) }
}

/// [`GetSysColor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
/// function.
#[must_use]
pub fn GetSysColor(index: co::COLOR) -> COLORREF {
	COLORREF(unsafe { user::ffi::GetSysColor(index.0) })
}

/// [`GetSystemMetrics`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics)
/// function.
#[must_use]
pub fn GetSystemMetrics(index: co::SM) -> i32 {
	unsafe { user::ffi::GetSystemMetrics(index.0) }
}

/// [`GetSystemMetricsForDpi`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetricsfordpi)
/// function.
#[must_use]
pub fn GetSystemMetricsForDpi(index: co::SM, dpi: u32) -> SysResult<i32> {
	match unsafe { user::ffi::GetSystemMetricsForDpi(index.0, dpi) } {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(0), // actual value is zero
			err => Err(err),
		},
		val => Ok(val),
	}
}

/// [`InSendMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessage)
/// function.
#[must_use]
pub fn InSendMessage() -> bool {
	unsafe { user::ffi::InSendMessage() != 0 }
}

/// [`InflateRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-inflaterect)
/// function.
pub fn InflateRect(rc: &mut RECT, dx: i32, dy: i32) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { user::ffi::InflateRect(rc as *mut _ as _, dx, dy) },
	)
}

/// [`InSendMessageEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessageex)
/// function.
///
/// **Note:** This function doesn't exist in x32.
#[cfg(target_pointer_width = "64")]
#[must_use]
pub fn InSendMessageEx() -> co::ISMEX {
	co::ISMEX(unsafe { user::ffi::InSendMessageEx()})
}

/// [`IntersectRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-intersectrect)
/// function.
pub fn IntersectRect(
	dest: &mut RECT, src1: &RECT, src2: &RECT) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			user::ffi::IntersectRect(
				dest as *mut _ as _,
				src1 as *const _ as _,
				src2 as *const _ as _,
			)
		},
	)
}

/// [`IsGUIThread`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isguithread)
/// function.
pub fn IsGUIThread(convert_to_gui_thread: bool) -> SysResult<bool> {
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

/// [`IsRectEmpty`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isrectempty)
/// function.
#[must_use]
pub fn IsRectEmpty(rc: &RECT) -> bool {
	unsafe { user::ffi::IsRectEmpty(rc as *const _ as _) != 0 }
}

/// [`IsWow64Message`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswow64message)
/// function.
#[must_use]
pub fn IsWow64Message() -> bool {
	return unsafe { user::ffi::IsWow64Message() != 0}
}

/// [`LockSetForegroundWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-locksetforegroundwindow)
/// function.
pub fn LockSetForegroundWindow(lock_code: co::LSFW) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { user::ffi::LockSetForegroundWindow(lock_code.0) },
	)
}

/// [`OffsetRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-offsetrect)
/// function.
pub fn OffsetRect(rc: &mut RECT, dx: i32, dy: i32) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { user::ffi::OffsetRect(rc as *mut _ as _, dx, dy) },
	)
}

/// [`PeekMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
/// function.
pub fn PeekMessage(
	msg: &mut MSG,
	hwnd: Option<&HWND>,
	msg_filter_min: u32,
	msg_filter_max: u32,
	remove_msg: co::PM) -> bool
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

/// [`PostQuitMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
/// function.
pub fn PostQuitMessage(exit_code: i32) {
	unsafe { user::ffi::PostQuitMessage(exit_code) }
}

/// [`PostThreadMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postthreadmessagew)
/// function.
pub fn PostThreadMessage<M>(thread_id: u32, msg: M) -> SysResult<()>
	where M: MsgSend + Send + Copy + 'static,
{
	let mut msg = msg;
	let wm_any = msg.as_generic_wm();
	bool_to_sysresult(
		unsafe {
			user::ffi::PostThreadMessageW(
				thread_id, wm_any.msg_id.0, wm_any.wparam, wm_any.lparam)
		}
	)
}

/// [`PtInRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ptinrect)
/// function.
pub fn PtInRect(rc: &RECT, pt: POINT) -> bool {
	unsafe { user::ffi::PtInRect(rc as *const _ as _, pt.x, pt.y) != 0 }
}

/// [`RegisterClassEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw)
/// function.
///
/// # Safety
///
/// In order to register a window class name, you must reset the global error
/// with [`SetLastError`](crate::SetLastError) and provide a window procedure.
pub unsafe fn RegisterClassEx(wcx: &WNDCLASSEX) -> SysResult<ATOM> {
	match unsafe { user::ffi::RegisterClassExW(wcx as *const _ as _) } {
		0 => Err(GetLastError()),
		atom => Ok(ATOM(atom)),
	}
}

/// [`RegisterWindowMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerwindowmessagew)
/// function.
#[must_use]
pub fn RegisterWindowMessage(s: &str) -> SysResult<u32> {
	match unsafe {
			user::ffi::RegisterWindowMessageW(WString::from_str(s).as_ptr())
	} {
		0 => Err(GetLastError()),
		id => Ok(id),
	}
}

/// [`SetCaretBlinkTime`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcaretblinktime)
/// function.
pub fn SetCaretBlinkTime(milliseconds: u32) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { user::ffi::SetCaretBlinkTime(milliseconds) },
	)
}

/// [`SetCaretPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcaretpos)
/// function.
pub fn SetCaretPos(x: i32, y: i32) -> SysResult<()> {
	bool_to_sysresult(unsafe { user::ffi::SetCaretPos(x, y) })
}

/// [`SetClipboardData`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setclipboarddata)
/// function.
///
/// # Safety
///
/// The `hmem` memory block must be correctly allocated and contain the type
/// specified by `format`.
pub unsafe fn SetClipboardData(
	format: co::CF, hmem: *mut u8) -> SysResult<*mut u8>
{
	ptr_to_sysresult(
		user::ffi::SetClipboardData(format.0, hmem as _),
		|hmem| hmem as *mut _ as _,
	)
}

/// [`SetCursorPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursorpos)
/// function.
pub fn SetCursorPos(x: i32, y: i32) -> SysResult<()> {
	bool_to_sysresult(unsafe { user::ffi::SetCursorPos(x, y) })
}

/// [`SetProcessDPIAware`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocessdpiaware)
/// function.
pub fn SetProcessDPIAware() -> SysResult<()> {
	bool_to_sysresult(unsafe { user::ffi::SetProcessDPIAware() })
}

/// [`ShowCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcursor)
/// function.
pub fn ShowCursor(show: bool) -> i32 {
	unsafe { user::ffi::ShowCursor(show as _) }
}

/// [`SoundSentry`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-soundsentry)
/// function.
pub fn SoundSentry() -> bool {
	unsafe { user::ffi::SoundSentry() != 0 }
}

/// [`SubtractRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-subtractrect)
/// function.
pub fn SubtractRect(
	dest: &mut RECT, src1: &RECT, src2: &RECT) -> SysResult<()>
{
	bool_to_sysresult(
		unsafe {
			user::ffi::SubtractRect(
				dest as *mut _ as _,
				src1 as *const _ as _,
				src2 as *const _ as _,
			)
		},
	)
}

/// [`SystemParametersInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow)
/// function.
///
/// # Safety
///
/// The `pv_param` type varies according to `action`. If you set it wrong,
/// you're likely to cause a buffer overrun.
pub unsafe fn SystemParametersInfo<T>(
	action: co::SPI,
	ui_param: u32,
	pv_param: &mut T,
	win_ini: co::SPIF) -> SysResult<()>
{
	bool_to_sysresult(
		user::ffi::SystemParametersInfoW(
			action.0,
			ui_param,
			pv_param as *mut _ as _,
			win_ini.0,
		),
	)
}

/// [`TrackMouseEvent`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackmouseevent)
/// function.
pub fn TrackMouseEvent(tme: &mut TRACKMOUSEEVENT) -> SysResult<()> {
	bool_to_sysresult(
		unsafe { user::ffi::TrackMouseEvent(tme as *mut _ as _) },
	)
}

/// [`TranslateMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
/// function.
pub fn TranslateMessage(msg: &MSG) -> bool {
	unsafe { user::ffi::TranslateMessage(msg as *const _ as _) != 0 }
}

/// [`UnionRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unionrect)
/// function.
pub fn UnionRect(dest: &mut RECT, src1: &RECT, src2: &RECT) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			user::ffi::UnionRect(
				dest as *mut _ as _,
				src1 as *const _ as _,
				src2 as *const _ as _,
			)
		},
	)
}

/// [`UnregisterClass`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
/// function.
pub fn UnregisterClass(class_name: &str, hinst: &HINSTANCE) -> SysResult<()> {
	bool_to_sysresult(
		unsafe {
			user::ffi::UnregisterClassW(
				WString::from_str(class_name).as_ptr(),
				hinst.0,
			)
		},
	)
}

/// [`WaitMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-waitmessage)
/// function.
pub fn WaitMessage() -> SysResult<()> {
	bool_to_sysresult(unsafe { user::ffi::WaitMessage() })
}
