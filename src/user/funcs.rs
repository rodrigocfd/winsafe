#![allow(non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::{ffi, iterators::*, privs::*, proc};

/// [`AdjustWindowRectEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectex)
/// function.
///
/// # Related functions
///
/// * [`AdjustWindowRectExForDpi`](crate::AdjustWindowRectExForDpi)
#[must_use]
pub fn AdjustWindowRectEx(
	rc: RECT,
	style: co::WS,
	has_menu: bool,
	ex_style: co::WS_EX,
) -> SysResult<RECT> {
	let mut buf = rc;
	bool_to_sysresult(unsafe {
		ffi::AdjustWindowRectEx(pvoid(&mut buf), style.raw(), has_menu as _, ex_style.raw())
	})
	.map(|_| buf)
}

/// [`AdjustWindowRectExForDpi`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-adjustwindowrectexfordpi)
/// function.
///
/// # Related functions
///
/// * [`AdjustWindowRectEx`](crate::AdjustWindowRectEx)
#[must_use]
pub fn AdjustWindowRectExForDpi(
	rc: RECT,
	style: co::WS,
	has_menu: bool,
	ex_style: co::WS_EX,
	dpi: u32,
) -> SysResult<RECT> {
	let mut buf = rc;
	bool_to_sysresult(unsafe {
		ffi::AdjustWindowRectExForDpi(
			pvoid(&mut buf),
			style.raw(),
			has_menu as _,
			ex_style.raw(),
			dpi,
		)
	})
	.map(|_| buf)
}

/// [`AllowSetForegroundWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-allowsetforegroundwindow)
/// function
pub fn AllowSetForegroundWindow(process_id: Option<u32>) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::AllowSetForegroundWindow(process_id.unwrap_or(ASFW_ANY)) })
}

/// [`AnyPopup`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-anypopup)
/// function.
#[must_use]
pub fn AnyPopup() -> bool {
	unsafe { ffi::AnyPopup() != 0 }
}

/// [`AttachThreadInput`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-attachthreadinput)
/// function.
pub fn AttachThreadInput(attach_id: u32, attach_to_id: u32, do_attach: bool) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::AttachThreadInput(attach_id, attach_to_id, do_attach as _) })
}

/// [`BlockInput`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-blockinput)
/// function.
pub fn BlockInput(block_it: bool) -> bool {
	unsafe { ffi::BlockInput(block_it as _) != 0 }
}

/// [`BroadcastSystemMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-broadcastsystemmessage)
/// function.
///
/// # Safety
///
/// Messages manipulate pointers, copies and window states. Improper use may
/// lead to undefined behavior.
pub unsafe fn BroadcastSystemMessage<M>(flags: co::BSF, info: co::BSM, msg: M) -> SysResult<co::BSM>
where
	M: MsgSend,
{
	let mut msg = msg;
	let wm_any = msg.as_generic_wm();

	let mut info_ret = info;

	if {
		unsafe {
			ffi::BroadcastSystemMessageW(
				flags.raw(),
				info_ret.as_mut(),
				wm_any.msg_id.raw(),
				wm_any.wparam,
				wm_any.lparam,
			)
		}
	} > 0
	{
		Ok(info_ret)
	} else {
		Err(GetLastError())
	}
}

/// [`ChangeDisplaySettings`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-changedisplaysettingsw)
/// function.
pub fn ChangeDisplaySettings(
	dev_mode: Option<&mut DEVMODE>,
	flags: co::CDS,
) -> Result<co::DISP_CHANGE, co::DISP_CHANGE> {
	let ret = unsafe { ffi::ChangeDisplaySettingsW(pvoid_or_null(dev_mode), flags.raw()) };
	unsafe {
		if ret < 0 {
			Err(co::DISP_CHANGE::from_raw(ret))
		} else {
			Ok(co::DISP_CHANGE::from_raw(ret))
		}
	}
}

/// [`ChangeDisplaySettingsEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-changedisplaysettingsexw)
/// function.
pub fn ChangeDisplaySettingsEx(
	device_name: Option<&str>,
	dev_mode: Option<&mut DEVMODE>,
	flags: co::CDS,
) -> Result<co::DISP_CHANGE, co::DISP_CHANGE> {
	let ret = unsafe {
		ffi::ChangeDisplaySettingsExW(
			WString::from_opt_str(device_name).as_ptr(),
			pvoid_or_null(dev_mode),
			std::ptr::null_mut(),
			flags.raw(),
			std::ptr::null_mut(),
		)
	};

	unsafe {
		if ret < 0 {
			Err(co::DISP_CHANGE::from_raw(ret))
		} else {
			Ok(co::DISP_CHANGE::from_raw(ret))
		}
	}
}

/// [`ChooseColor`](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms646912(v=vs.85))
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let parent_hwnd: w::HWND; // initialized somewhere
/// # let parent_hwnd = w::HWND::NULL;
///
/// let mut cc = w::CHOOSECOLOR::default();
/// let mut custom_colors = [w::COLORREF::new(255, 255, 255); 16];
///
/// cc.hwndOwner = parent_hwnd;
/// cc.Flags = co::CC::ANYCOLOR | co::CC::FULLOPEN | co::CC::RGBINIT;
/// cc.rgbResult = w::COLORREF::new(255, 0, 0); // color initially chosen
/// cc.set_lpCustColors(Some(&mut custom_colors));
///
/// if w::ChooseColor(&mut cc)? {
///     println!("The color: {} {} {}",
///         cc.rgbResult.GetRValue(),
///         cc.rgbResult.GetGValue(),
///         cc.rgbResult.GetBValue(),
///     );
/// }
/// # Ok::<_, co::CDERR>(())
/// ```
pub fn ChooseColor(cc: &mut CHOOSECOLOR) -> Result<bool, co::CDERR> {
	match unsafe { ffi::ChooseColorW(pvoid(cc)) } {
		0 => match CommDlgExtendedError() {
			co::CDERR::NoValue => Ok(false),
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`ClipCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clipcursor)
/// function.
pub fn ClipCursor(rc: Option<&RECT>) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::ClipCursor(pcvoid_or_null(rc)) })
}

/// [`CommDlgExtendedError`](https://learn.microsoft.com/en-us/windows/win32/api/commdlg/nf-commdlg-commdlgextendederror)
/// function.
#[must_use]
pub fn CommDlgExtendedError() -> co::CDERR {
	unsafe { co::CDERR::from_raw(ffi::CommDlgExtendedError()) }
}

/// [`DispatchMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)
/// function.
///
/// # Safety
///
/// This function is used internally in window loops. Avoid using it in other
/// situations.
pub unsafe fn DispatchMessage(msg: &MSG) -> isize {
	unsafe { ffi::DispatchMessageW(pcvoid(msg)) }
}

/// [`EndMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endmenu)
/// function.
pub fn EndMenu() -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::EndMenu() })
}

/// [`EnumDisplayDevices`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaydevicesw)
/// function.
///
/// Returns an iterator over [`DISPLAY_DEVICE`](crate::DISPLAY_DEVICE) elements.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// // Ordinary for loop
/// for displ_dev in w::EnumDisplayDevices(None, None) {
///     let displ_dev = displ_dev?;
///     println!("{} - {}",
///         displ_dev.DeviceName(), displ_dev.DeviceString());
/// }
///
/// // Closure with try_for_each
/// w::EnumDisplayDevices(None, None)
///     .try_for_each(|displ_dev| {
///         let displ_dev = displ_dev?;
///         println!("{} - {}",
///             displ_dev.DeviceName(), displ_dev.DeviceString());
///         Ok(())
///     })?;
///
/// // Collecting into a Vec
/// let all = w::EnumDisplayDevices(None, None)
///     .map(|displ_dev| {
///         let displ_dev = displ_dev?;
///         let name = format!("{} - {}",
///             displ_dev.DeviceName(), displ_dev.DeviceString());
///         Ok(name)
///     })
///     .collect::<w::SysResult<Vec<_>>>()?;
/// # w::SysResult::Ok(())
/// ```
#[must_use]
pub fn EnumDisplayDevices(
	device_name: Option<&str>,
	flags: Option<co::EDD>,
) -> impl Iterator<Item = SysResult<&'_ DISPLAY_DEVICE>> {
	EnumdisplaydevicesIter::new(device_name, flags)
}

/// [`EnumDisplaySettings`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaysettingsw)
/// function.
///
/// # Examples
///
/// Iterating graphics modes.
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let mut dm = w::DEVMODE::default();
/// let mut graphics_mode_idx = u32::default();
///
/// loop {
///     let is_good = w::EnumDisplaySettings(
///         None,
///         w::GmidxEnum::Gmidx(graphics_mode_idx),
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
/// # w::SysResult::Ok(())
/// ```
///
/// Retrieving from the predefined enum.
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let mut dm = w::DEVMODE::default();
///
/// w::EnumDisplaySettings(
///     None,
///     w::GmidxEnum::Enum(co::ENUM_SETTINGS::CURRENT),
///     &mut dm,
/// )?;
///
/// println!("{}, {}, {}",
///     dm.dmDeviceName(), dm.dmDisplayFrequency, dm.dmBitsPerPel);
/// # w::SysResult::Ok(())
/// ```
pub fn EnumDisplaySettings(
	device_name: Option<&str>,
	mode_num: GmidxEnum,
	dev_mode: &mut DEVMODE,
) -> SysResult<bool> {
	match unsafe {
		ffi::EnumDisplaySettingsW(
			WString::from_opt_str(device_name).as_ptr(),
			mode_num.into(),
			pvoid(dev_mode),
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
	flags: co::EDS,
) -> SysResult<bool> {
	match unsafe {
		ffi::EnumDisplaySettingsExW(
			WString::from_opt_str(device_name).as_ptr(),
			mode_num.into(),
			pvoid(dev_mode),
			flags.raw(),
		)
	} {
		0 => match GetLastError() {
			co::ERROR::PROC_NOT_FOUND => Ok(false), // actual false
			err => Err(err),
		},
		_ => Ok(true),
	}
}

/// [`EnumThreadWindows`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumthreadwindows)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// w::EnumThreadWindows(
///     w::GetCurrentThreadId(),
///     |hwnd: w::HWND| -> bool {
///         println!("HWND: {}", hwnd);
///         true
///     },
/// )?;
/// # w::SysResult::Ok(())
/// ```
pub fn EnumThreadWindows<F>(thread_id: u32, func: F) -> SysResult<()>
where
	F: FnMut(HWND) -> bool,
{
	bool_to_sysresult(unsafe {
		ffi::EnumThreadWindows(thread_id, proc::func_enum_thread_wnd::<F> as _, pcvoid(&func))
	})
}

/// [`EnumWindows`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumwindows)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// w::EnumWindows(|hwnd: w::HWND| -> bool {
///     println!("HWND: {}", hwnd);
///     true
/// })?;
/// # w::SysResult::Ok(())
/// ```
pub fn EnumWindows<F>(func: F) -> SysResult<()>
where
	F: FnMut(HWND) -> bool,
{
	bool_to_sysresult(unsafe {
		ffi::EnumWindows(proc::func_enum_windows::<F> as _, &func as *const _ as _)
	})
}

/// [`ExitWindowsEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-exitwindowsex)
/// function.
pub fn ExitWindowsEx(flags: co::EWX, reason: co::SHTDN_REASON) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::ExitWindowsEx(flags.raw(), reason.raw()) })
}

/// [`FlashWindowEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-flashwindowex)
/// function.
pub fn FlashWindowEx(fwi: &FLASHWINFO) -> u32 {
	unsafe { ffi::FlashWindowEx(pcvoid(fwi)) as _ }
}

/// [`GetAsyncKeyState`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getasynckeystate)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// let esc_is_down = w::GetAsyncKeyState(co::VK::ESCAPE);
/// ```
#[must_use]
pub fn GetAsyncKeyState(virt_key: co::VK) -> bool {
	unsafe { (ffi::GetAsyncKeyState(virt_key.raw() as _) as u16) & 0x8000 != 0 }
}

/// [`GetCaretBlinkTime`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcaretblinktime)
/// function.
#[must_use]
pub fn GetCaretBlinkTime() -> SysResult<u32> {
	match unsafe { ffi::GetCaretBlinkTime() } {
		INFINITE => Err(GetLastError()),
		n => Ok(n),
	}
}

/// [`GetCaretPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcaretpos)
/// function.
#[must_use]
pub fn GetCaretPos() -> SysResult<POINT> {
	let mut pt = POINT::default();
	bool_to_sysresult(unsafe { ffi::GetCaretPos(pvoid(&mut pt)) }).map(|_| pt)
}

/// [`GetClipCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclipcursor)
/// function.
#[must_use]
pub fn GetClipCursor() -> SysResult<RECT> {
	let mut rc = RECT::default();
	bool_to_sysresult(unsafe { ffi::GetClipCursor(pvoid(&mut rc)) }).map(|_| rc)
}

/// [`GetCursorInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorinfo)
/// function.
pub fn GetCursorInfo(ci: &mut CURSORINFO) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::GetCursorInfo(pvoid(ci)) })
}

/// [`GetCursorPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcursorpos)
/// function.
#[must_use]
pub fn GetCursorPos() -> SysResult<POINT> {
	let mut pt = POINT::default();
	bool_to_sysresult(unsafe { ffi::GetCursorPos(pvoid(&mut pt)) }).map(|_| pt)
}

/// [`GetDialogBaseUnits`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdialogbaseunits)
/// function.
#[must_use]
pub fn GetDialogBaseUnits() -> i32 {
	unsafe { ffi::GetDialogBaseUnits() }
}

/// [`GetDoubleClickTime`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdoubleclicktime)
/// function.
#[must_use]
pub fn GetDoubleClickTime() -> u32 {
	unsafe { ffi::GetDoubleClickTime() }
}

/// [`GetGUIThreadInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getguithreadinfo)
/// function.
///
/// # Examples
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let hwnd: w::HWND; // initialized somewhere
/// # let hwnd = w::HWND::NULL;
///
/// let (thread_id, _) = hwnd.GetWindowThreadProcessId();
/// let gti = w::GetGUIThreadInfo(thread_id)?;
///
/// println!("Caret rect: {}", gti.rcCaret);
/// # w::SysResult::Ok(())
/// ```
#[must_use]
pub fn GetGUIThreadInfo(thread_id: u32) -> SysResult<GUITHREADINFO> {
	let mut gti = GUITHREADINFO::default();
	bool_to_sysresult(unsafe { ffi::GetGUIThreadInfo(thread_id, pvoid(&mut gti)) }).map(|_| gti)
}

/// [`GetLastInputInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getlastinputinfo)
/// function.
#[must_use]
pub fn GetLastInputInfo() -> SysResult<LASTINPUTINFO> {
	let mut lii = LASTINPUTINFO::default();
	bool_to_sysresult(unsafe { ffi::GetLastInputInfo(pvoid(&mut lii)) }).map(|_| lii)
}

/// [`GetPhysicalCursorPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getphysicalcursorpos)
/// function.
///
/// # Related functions
///
/// * [`SetPhysicalCursorPos`](crate::SetPhysicalCursorPos)
#[must_use]
pub fn GetPhysicalCursorPos() -> SysResult<POINT> {
	let mut pt = POINT::default();
	bool_to_sysresult(unsafe { ffi::GetPhysicalCursorPos(pvoid(&mut pt)) }).map(|_| pt)
}

/// [`GetMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew)
/// function.
pub fn GetMessage(
	msg: &mut MSG,
	hwnd: Option<&HWND>,
	msg_filter_min: u32,
	msg_filter_max: u32,
) -> SysResult<bool> {
	match unsafe {
		ffi::GetMessageW(
			pvoid(msg),
			hwnd.map_or(std::ptr::null_mut(), |h| h.ptr()),
			msg_filter_min,
			msg_filter_max,
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
	SIZE::from(unsafe { ffi::GetMenuCheckMarkDimensions() })
}

/// [`GetMessagePos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagepos)
/// function.
#[must_use]
pub fn GetMessagePos() -> POINT {
	POINT::from(unsafe { ffi::GetMessagePos() })
}

/// [`GetProcessDefaultLayout`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getprocessdefaultlayout)
/// function.
///
/// # Related functions
///
/// * [`SetProcessDefaultLayout`](crate::SetProcessDefaultLayout)
#[must_use]
pub fn GetProcessDefaultLayout() -> SysResult<co::LAYOUT> {
	let mut dl = co::LAYOUT::default();
	unsafe { bool_to_sysresult(ffi::GetProcessDefaultLayout(dl.as_mut())).map(|_| dl) }
}

/// [`GetQueueStatus`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getqueuestatus)
/// function.
#[must_use]
pub fn GetQueueStatus(flags: co::QS) -> u32 {
	unsafe { ffi::GetQueueStatus(flags.raw()) }
}

/// [`GetSysColor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
/// function.
///
/// # Related functions
///
/// * [`SetSysColors`](crate::SetSysColors)
#[must_use]
pub fn GetSysColor(index: co::COLOR) -> COLORREF {
	unsafe { COLORREF::from_raw(ffi::GetSysColor(index.raw())) }
}

/// [`GetSystemMetrics`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics)
/// function.
#[must_use]
pub fn GetSystemMetrics(index: co::SM) -> i32 {
	unsafe { ffi::GetSystemMetrics(index.raw()) }
}

/// [`GetSystemMetricsForDpi`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetricsfordpi)
/// function.
#[must_use]
pub fn GetSystemMetricsForDpi(index: co::SM, dpi: u32) -> SysResult<i32> {
	match unsafe { ffi::GetSystemMetricsForDpi(index.raw(), dpi) } {
		0 => match GetLastError() {
			co::ERROR::SUCCESS => Ok(0), // actual value is zero
			err => Err(err),
		},
		val => Ok(val),
	}
}

/// [`GetThreadDpiHostingBehavior`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getthreaddpihostingbehavior)
/// function.
#[must_use]
pub fn GetThreadDpiHostingBehavior() -> co::DPI_HOSTING_BEHAVIOR {
	unsafe { co::DPI_HOSTING_BEHAVIOR::from_raw(ffi::GetThreadDpiHostingBehavior()) }
}

/// [`InSendMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessage)
/// function.
#[must_use]
pub fn InSendMessage() -> bool {
	unsafe { ffi::InSendMessage() != 0 }
}

/// [`InflateRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-inflaterect)
/// function.
#[must_use]
pub fn InflateRect(rc: RECT, dx: i32, dy: i32) -> SysResult<RECT> {
	let mut buf = rc;
	bool_to_sysresult(unsafe { ffi::InflateRect(pvoid(&mut buf), dx, dy) }).map(|_| buf)
}

/// [`InSendMessageEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessageex)
/// function.
///
/// **Note:** This function doesn't exist in x32.
#[cfg(target_pointer_width = "64")]
#[must_use]
pub fn InSendMessageEx() -> co::ISMEX {
	unsafe { co::ISMEX::from_raw(ffi::InSendMessageEx()) }
}

/// [`IntersectRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-intersectrect)
/// function.
#[must_use]
pub fn IntersectRect(src1: RECT, src2: RECT) -> SysResult<RECT> {
	let mut dest = RECT::default();
	bool_to_sysresult(unsafe { ffi::IntersectRect(pvoid(&mut dest), pcvoid(&src1), pcvoid(&src2)) })
		.map(|_| dest)
}

/// [`IsGUIThread`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isguithread)
/// function.
pub fn IsGUIThread(convert_to_gui_thread: bool) -> SysResult<bool> {
	let r = unsafe { ffi::IsGUIThread(convert_to_gui_thread as _) };
	if convert_to_gui_thread {
		match r {
			0 => Ok(false),
			1 => Ok(true),
			err => Err(unsafe { co::ERROR::from_raw(err as _) }),
		}
	} else {
		Ok(r != 0)
	}
}

/// [`IsRectEmpty`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isrectempty)
/// function.
#[must_use]
pub fn IsRectEmpty(rc: RECT) -> bool {
	unsafe { ffi::IsRectEmpty(pcvoid(&rc)) != 0 }
}

/// [`IsWow64Message`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswow64message)
/// function.
#[must_use]
pub fn IsWow64Message() -> bool {
	return unsafe { ffi::IsWow64Message() != 0 };
}

/// [`LockSetForegroundWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-locksetforegroundwindow)
/// function.
pub fn LockSetForegroundWindow(lock_code: co::LSFW) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::LockSetForegroundWindow(lock_code.raw()) })
}

/// [`LockWorkStation`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-lockworkstation)
/// function.
pub fn LockWorkStation() -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::LockWorkStation() })
}

/// [`MessageBeep`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebeep)
/// function.
pub fn MessageBeep(sound_type: co::MBP) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::MessageBeep(sound_type.raw()) })
}

/// [`OffsetRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-offsetrect)
/// function.
#[must_use]
pub fn OffsetRect(rc: RECT, dx: i32, dy: i32) -> SysResult<RECT> {
	let mut buf = rc;
	bool_to_sysresult(unsafe { ffi::OffsetRect(pvoid(&mut buf), dx, dy) }).map(|_| buf)
}

/// [`PeekMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)
/// function.
pub fn PeekMessage(
	msg: &mut MSG,
	hwnd: Option<&HWND>,
	msg_filter_min: u32,
	msg_filter_max: u32,
	remove_msg: co::PM,
) -> bool {
	unsafe {
		ffi::PeekMessageW(
			pvoid(msg),
			hwnd.map_or(std::ptr::null_mut(), |h| h.ptr()),
			msg_filter_min,
			msg_filter_max,
			remove_msg.raw(),
		) != 0
	}
}

/// [`PostQuitMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)
/// function.
pub fn PostQuitMessage(exit_code: i32) {
	unsafe { ffi::PostQuitMessage(exit_code) }
}

/// [`PostThreadMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postthreadmessagew)
/// function.
///
/// # Safety
///
/// Messages manipulate pointers, copies and window states. Improper use may
/// lead to undefined behavior.
pub unsafe fn PostThreadMessage<M>(thread_id: u32, msg: M) -> SysResult<()>
where
	M: MsgSend + Send + Copy + 'static,
{
	let mut msg = msg;
	let wm_any = msg.as_generic_wm();
	bool_to_sysresult(unsafe {
		ffi::PostThreadMessageW(thread_id, wm_any.msg_id.raw(), wm_any.wparam, wm_any.lparam)
	})
}

/// [`PtInRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ptinrect)
/// function.
#[must_use]
pub fn PtInRect(rc: RECT, pt: POINT) -> bool {
	unsafe { ffi::PtInRect(pcvoid(&rc), pt.x, pt.y) != 0 }
}

/// [`RegisterClassEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw)
/// function.
///
/// # Safety
///
/// In order to register a window class name, you must reset the global error
/// with [`SetLastError`](crate::SetLastError) and provide a window procedure.
pub unsafe fn RegisterClassEx(wcx: &WNDCLASSEX) -> SysResult<ATOM> {
	match unsafe { ffi::RegisterClassExW(pcvoid(wcx)) } {
		0 => Err(GetLastError()),
		atom => Ok(unsafe { ATOM::from_raw(atom) }),
	}
}

/// [`RegisterWindowMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerwindowmessagew)
/// function.
#[must_use]
pub fn RegisterWindowMessage(s: &str) -> SysResult<u32> {
	match unsafe { ffi::RegisterWindowMessageW(WString::from_str(s).as_ptr()) } {
		0 => Err(GetLastError()),
		id => Ok(id),
	}
}

/// [`SendInput`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput)
/// function.
///
/// # Examples
///
/// Sending Win+D to toggle the desktop:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*, co};
///
/// w::SendInput(&[
///     w::HwKbMouse::Kb(
///         w::KEYBDINPUT {
///             wVk: co::VK::LWIN,
///             ..Default::default()
///         },
///     ),
///     w::HwKbMouse::Kb(
///         w::KEYBDINPUT {
///             wVk: co::VK::CHAR_D,
///             ..Default::default()
///         },
///     ),
///     w::HwKbMouse::Kb(
///         w::KEYBDINPUT {
///             wVk: co::VK::CHAR_D,
///             dwFlags: co::KEYEVENTF::KEYUP,
///             ..Default::default()
///         },
///     ),
///     w::HwKbMouse::Kb(
///         w::KEYBDINPUT {
///             wVk: co::VK::LWIN,
///             dwFlags: co::KEYEVENTF::KEYUP,
///             ..Default::default()
///         },
///     ),
/// ])?;
/// # w::SysResult::Ok(())
/// ```
pub fn SendInput(inputs: &[HwKbMouse]) -> SysResult<u32> {
	let objs = inputs
		.iter()
		.map(|ipt| INPUT::new(*ipt))
		.collect::<Vec<_>>();

	match unsafe {
		ffi::SendInput(objs.len() as _, vec_ptr(&objs) as _, std::mem::size_of::<INPUT>() as _)
	} {
		0 => Err(GetLastError()),
		n => Ok(n),
	}
}

/// [`SetCaretBlinkTime`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcaretblinktime)
/// function.
pub fn SetCaretBlinkTime(milliseconds: u32) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::SetCaretBlinkTime(milliseconds) })
}

/// [`SetCaretPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcaretpos)
/// function.
pub fn SetCaretPos(x: i32, y: i32) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::SetCaretPos(x, y) })
}

/// [`SetCursorPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcursorpos)
/// function.
pub fn SetCursorPos(x: i32, y: i32) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::SetCursorPos(x, y) })
}

/// [`SetDoubleClickTime`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setdoubleclicktime)
/// function.
#[must_use]
pub fn SetDoubleClickTime(ms: u32) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::SetDoubleClickTime(ms) })
}

/// [`SetPhysicalCursorPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setphysicalcursorpos)
/// function.
///
/// # Related functions
///
/// * [`GetPhysicalCursorPos`](crate::GetPhysicalCursorPos)
pub fn SetPhysicalCursorPos(x: i32, y: i32) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::SetPhysicalCursorPos(x, y) })
}

/// [`SetProcessDefaultLayout`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocessdefaultlayout)
/// function.
///
/// # Related functions
///
/// * [`GetProcessDefaultLayout`](crate::GetProcessDefaultLayout)
pub fn SetProcessDefaultLayout(layout: co::LAYOUT) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::SetProcessDefaultLayout(layout.raw()) })
}

/// [`SetProcessDPIAware`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setprocessdpiaware)
/// function.
pub fn SetProcessDPIAware() -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::SetProcessDPIAware() })
}

/// [`SetSysColors`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setsyscolors)
/// function.
///
/// # Related functions
///
/// * [`GetSysColor`](crate::GetSysColor)
pub fn SetSysColors(elements_and_colors: &[(co::COLOR, COLORREF)]) -> SysResult<()> {
	let (elems, colors): (Vec<_>, Vec<_>) = elements_and_colors
		.iter()
		.map(|ec| (ec.0.raw(), ec.1.raw()))
		.unzip();

	bool_to_sysresult(unsafe {
		ffi::SetSysColors(elements_and_colors.len() as _, elems.as_ptr(), colors.as_ptr())
	})
}

/// [`SetThreadDpiHostingBehavior`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setthreaddpihostingbehavior)
/// function.
pub fn SetThreadDpiHostingBehavior(value: co::DPI_HOSTING_BEHAVIOR) -> co::DPI_HOSTING_BEHAVIOR {
	unsafe { co::DPI_HOSTING_BEHAVIOR::from_raw(ffi::SetThreadDpiHostingBehavior(value.raw())) }
}

/// [`ShowCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcursor)
/// function.
pub fn ShowCursor(show: bool) -> i32 {
	unsafe { ffi::ShowCursor(show as _) }
}

/// [`SoundSentry`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-soundsentry)
/// function.
pub fn SoundSentry() -> bool {
	unsafe { ffi::SoundSentry() != 0 }
}

/// [`SubtractRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-subtractrect)
/// function.
#[must_use]
pub fn SubtractRect(src1: RECT, src2: RECT) -> SysResult<RECT> {
	let mut dest = RECT::default();
	bool_to_sysresult(unsafe { ffi::SubtractRect(pvoid(&mut dest), pcvoid(&src1), pcvoid(&src2)) })
		.map(|_| dest)
}

/// [`SwapMouseButton`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-swapmousebutton)
/// function.
pub fn SwapMouseButton(swap: bool) -> bool {
	unsafe { ffi::SwapMouseButton(swap as _) != 0 }
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
	win_ini: co::SPIF,
) -> SysResult<()> {
	bool_to_sysresult(unsafe {
		ffi::SystemParametersInfoW(action.raw(), ui_param, pvoid(pv_param), win_ini.raw())
	})
}

/// [`TrackMouseEvent`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackmouseevent)
/// function.
pub fn TrackMouseEvent(tme: &mut TRACKMOUSEEVENT) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::TrackMouseEvent(pvoid(tme)) })
}

/// [`TranslateMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)
/// function.
pub fn TranslateMessage(msg: &MSG) -> bool {
	unsafe { ffi::TranslateMessage(pcvoid(msg)) != 0 }
}

/// [`UnionRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unionrect)
/// function.
#[must_use]
pub fn UnionRect(src1: RECT, src2: RECT) -> SysResult<RECT> {
	let mut dest = RECT::default();
	bool_to_sysresult(unsafe { ffi::UnionRect(pvoid(&mut dest), pcvoid(&src1), pcvoid(&src2)) })
		.map(|_| dest)
}

/// [`UnregisterClass`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)
/// function.
pub fn UnregisterClass(class_name: AtomStr, hinst: &HINSTANCE) -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::UnregisterClassW(class_name.as_ptr(), hinst.ptr()) })
}

/// [`WaitMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-waitmessage)
/// function.
pub fn WaitMessage() -> SysResult<()> {
	bool_to_sysresult(unsafe { ffi::WaitMessage() })
}
