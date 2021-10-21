#![allow(non_snake_case)]

use crate::aliases::{SUBCLASSPROC, TIMERPROC, WinResult};
use crate::co;
use crate::enums::{AtomStr, HwndPlace, IdMenu, IdPos, IdTdiconStr};
use crate::ffi::{BOOL, comctl32, shell32, user32, uxtheme};
use crate::funcs::{GetLastError, SetLastError};
use crate::handles::{HACCEL, HDC, HINSTANCE, HMENU, HMONITOR, HRGN, HTHEME};
use crate::msg::MsgSend;
use crate::privs::{bool_to_winresult, hr_to_winresult};
use crate::structs::{
	ALTTABINFO,
	MENUBARINFO,
	MSG,
	PAINTSTRUCT,
	POINT,
	RECT,
	SCROLLINFO,
	SIZE,
	WINDOWINFO,
	WINDOWPLACEMENT,
};
use crate::various::WString;

pub_struct_handle! {
	/// Handle to a
	/// [window](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
	HWND
}

impl HWND {
	/// Represents all top-level windows in
	/// [`HWND::PostMessage`](crate::HWND::PostMessage) and
	/// [`HWND::SendMessage`](crate::HWND::SendMessage).
	pub const BROADCAST: Self = Self { ptr: 0xffff as _ };

	/// [`GetWindowLongPtr`](crate::HWND::GetWindowLongPtr) wrapper to retrieve
	/// the window [`HINSTANCE`](crate::HINSTANCE).
	pub fn hinstance(self) -> HINSTANCE {
		HINSTANCE { ptr: self.GetWindowLongPtr(co::GWLP::HINSTANCE) as _ }
	}

	/// [`ArrangeIconicWindows`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-arrangeiconicwindows)
	/// method.
	pub fn ArrangeIconicWindows(self) -> WinResult<u32> {
		match unsafe { user32::ArrangeIconicWindows(self.ptr) } {
			0 => Err(GetLastError()),
			height => Ok(height),
		}
	}

	/// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HWND::EndPaint`](crate::HWND::EndPaint) call.
	///
	/// # Examples
	///
	/// `BeginPaint` is usually called inside a
	/// [`WM_PAINT`](crate::gui::events::prelude::EventsView::wm_paint) event to
	/// paint the window client area:
	///
	/// ```rust,ignore
	/// use winsafe::{gui, PAINTSTRUCT};
	///
	/// let my_main: gui::WindowMain; // initialized somewhere
	///
	/// my_main.on().wm_paint({
	///     let my_main = my_main.clone();
	///     move || {
	///         let mut ps = PAINTSTRUCT::default();
	///         let hdc = my_main.hwnd().BeginPaint(&mut ps)?;
	///
	///         // paint the HDC...
	///
	///         my_main.hwnd().EndPaint(&ps);
	///         Ok(())
	///     }
	/// });
	/// ```
	pub fn BeginPaint(self, ps: &mut PAINTSTRUCT) -> WinResult<HDC> {
		unsafe { user32::BeginPaint(self.ptr, ps as *mut _ as _).as_mut() }
			.map(|ptr| HDC { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`BringWindowToTop`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-bringwindowtotop)
	/// method.
	pub fn BringWindowToTop(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::BringWindowToTop(self.ptr) })
	}

	/// [`ChildWindowFromPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-childwindowfrompoint)
	/// method.
	pub fn ChildWindowFromPoint(self, pt: POINT) -> Option<HWND> {
		unsafe { user32::ChildWindowFromPoint(self.ptr, pt.x, pt.y).as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`ClientToScreen`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clienttoscreen)
	/// method.
	pub fn ClientToScreen(self, pt: &mut POINT) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::ClientToScreen(self.ptr, pt as *mut _ as _) },
		)
	}

	/// [`ClientToScreen`](crate::HWND::ClientToScreen) method for a
	/// [`RECT`](crate::RECT).
	pub fn ClientToScreenRc(self, rc: &mut RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::ClientToScreen(
					self.ptr,
					&mut rc.left as *mut _ as _,
				)
			},
		).and_then(|_| bool_to_winresult(
			unsafe {
				user32::ClientToScreen(
					self.ptr,
					&mut rc.right as *mut _ as _,
				)
			},
		))
	}

	/// [`CloseWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindow)
	/// method.
	///
	/// Note that this method will actually minimize the window, not destroy it.
	pub fn CloseWindow(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::CloseWindow(self.ptr) })
	}

	/// [`CreateWindowEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
	/// static method.
	pub fn CreateWindowEx(
		ex_style: co::WS_EX,
		class_name: AtomStr,
		title: Option<&str>,
		style: co::WS,
		pos: POINT,
		size: SIZE,
		hwnd_parent: Option<HWND>,
		hmenu: IdMenu,
		hinstance: HINSTANCE,
		lparam: Option<isize>) -> WinResult<HWND>
	{
		unsafe {
			user32::CreateWindowExW(
				ex_style.0,
				class_name.as_ptr(),
				WString::from_opt_str(title).as_ptr(),
				style.0,
				pos.x, pos.y,
				size.cx, size.cy,
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.ptr),
				hmenu.as_ptr(),
				hinstance.ptr,
				lparam.unwrap_or_default() as _,
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`DefSubclassProc`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-defsubclassproc)
	/// method.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::msg::MsgSend) trait. That means each
	/// message can define its own return type.
	pub fn DefSubclassProc<M: MsgSend>(self, msg: M) -> M::RetType {
		let wm_any = msg.as_generic_wm();
		msg.convert_ret(
			unsafe {
				comctl32::DefSubclassProc(
					self.ptr, wm_any.msg_id.0, wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`DefWindowProc`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
	/// method.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::msg::MsgSend) trait. That means each
	/// message can define its own return type.
	pub fn DefWindowProc<M: MsgSend>(self, msg: M) -> M::RetType {
		let wm_any = msg.as_generic_wm();
		msg.convert_ret(
			unsafe {
				user32::DefWindowProcW(
					self.ptr, wm_any.msg_id.0, wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
	/// method.
	pub fn DestroyWindow(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::DestroyWindow(self.ptr) })
	}

	/// [`DrawMenuBar`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawmenubar)
	/// method.
	pub fn DrawMenuBar(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::DrawMenuBar(self.ptr) })
	}

	/// [`EnableWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablewindow)
	/// method.
	pub fn EnableWindow(self, enable: bool) -> bool {
		unsafe { user32::EnableWindow(self.ptr, enable as _) != 0 }
	}

	/// [`EndDialog`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddialog)
	/// method.
	pub fn EndDialog(self, result: isize) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::EndDialog(self.ptr, result) })
	}

	/// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
	/// method.
	pub fn EndPaint(self, ps: &PAINTSTRUCT) {
		unsafe { user32::EndPaint(self.ptr, ps as *const _ as _); }
	}

	/// [`EnumChildWindows`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumchildwindows)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialized somewhere
	///
	/// my_hwnd.EnumChildWindows(|hchild: HWND| -> bool {
	///     println!("Child HWND: {}", hchild);
	///     true
	/// });
	/// ```
	pub fn EnumChildWindows<F>(self, func: F)
		where F: Fn(HWND) -> bool,
	{
		unsafe {
			user32::EnumChildWindows(
				self.ptr,
				Self::enum_child_windows_proc::<F> as _, // https://redd.it/npehj9
				&func as *const _ as _,
			);
		}
	}
	extern "system" fn enum_child_windows_proc<F>(
		hwnd: HWND, lparam: isize) -> BOOL
		where F: Fn(HWND) -> bool,
	{
		let func = unsafe { &*(lparam as *const F) };
		func(hwnd) as _
	}

	/// [`FindWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindoww)
	/// static method.
	pub fn FindWindow(
		class_name: Option<AtomStr>,
		title: Option<&str>) -> WinResult<HWND>
	{
		unsafe {
			user32::FindWindowW(
				class_name.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
				WString::from_opt_str(title).as_ptr(),
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`FindWindowEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindowexw)
	/// method.
	pub fn FindWindowEx(self,
		hwnd_child_after: Option<HWND>,
		class_name: AtomStr,
		title: Option<&str>) -> WinResult<HWND>
	{
		unsafe {
			user32::FindWindowExW(
				self.ptr,
				hwnd_child_after.map_or(std::ptr::null_mut(), |h| h.ptr),
				class_name.as_ptr(),
				WString::from_opt_str(title).as_ptr(),
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetActiveWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getactivewindow)
	/// static method.
	pub fn GetActiveWindow() -> Option<HWND> {
		unsafe { user32::GetActiveWindow().as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`GetAltTabInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getalttabinfow)
	/// method.
	///
	/// If `item` is `None`, the item text is not retrieved.
	///
	/// The `sz_item_text` is the maximum number of expected chars for the item
	/// text. If `None`, defaults to 100.
	pub fn GetAltTabInfo(self,
		item: Option<u32>,
		ati: &mut ALTTABINFO,
		sz_item_text: Option<u32>) -> WinResult<String>
	{
		let buf_sz = sz_item_text.unwrap_or(100) + 1;
		let mut buf = item.map_or(
			WString::default(),
			|_| WString::new_alloc_buffer(buf_sz as _), // room for terminating null
		);

		bool_to_winresult(
			unsafe {
				user32::GetAltTabInfoW(
					self.ptr,
					item.map_or(-1, |item| item as i32),
					ati as *mut _ as _,
					item.map_or(std::ptr::null_mut(), |_| buf.as_mut_ptr()),
					buf_sz,
				)
			},
		).map(|_| buf.to_string())
	}

	/// [`GetAncestor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getancestor)
	/// method.
	pub fn GetAncestor(self, flags: co::GA) -> Option<HWND> {
		unsafe { user32::GetAncestor(self.ptr, flags.0).as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`GetCapture`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcapture)
	/// static method.
	pub fn GetCapture() -> Option<HWND> {
		unsafe { user32::GetCapture().as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`GetClassLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclasslongptrw)
	/// method.
	pub fn GetClassLongPtr(self, index: co::GCLP) -> usize {
		unsafe { user32::GetClassLongPtrW(self.ptr, index.0) }
	}

	/// [`GetClassName`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclassnamew)
	/// method.
	pub fn GetClassName(self) -> WinResult<String> {
		let mut buf = WString::new_alloc_buffer(256 + 1); // according to WNDCLASSEX docs
		match unsafe {
			user32::GetClassNameW(
				self.ptr,
				buf.as_mut_ptr(),
				buf.buffer_size() as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(buf.to_string()),
		}
	}

	/// [`GetClientRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)
	/// method.
	pub fn GetClientRect(self) -> WinResult<RECT> {
		let mut rc = RECT::default();
		match unsafe {
			user32::GetClientRect(self.ptr, &mut rc as *mut _ as _)
		} {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HWND::ReleaseDC`](crate::HWND::ReleaseDC) call.
	pub fn GetDC(self) -> WinResult<HDC> {
		unsafe { user32::GetDC(self.ptr).as_mut() }
			.map(|ptr| HDC { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetDesktopWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow)
	/// static method.
	pub fn GetDesktopWindow() -> HWND {
		Self {
			ptr: unsafe { user32::GetDesktopWindow() }
		}
	}

	/// [`GetDlgCtrlID`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgctrlid)
	/// method.
	pub fn GetDlgCtrlID(self) -> WinResult<u16> {
		match unsafe { user32::GetDlgCtrlID(self.ptr) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual ID is zero
				err => Err(err),
			},
			id => Ok(id as _),
		}
	}

	/// [`GetDlgItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgitem)
	/// method.
	pub fn GetDlgItem(self, ctrl_id: u16) -> WinResult<HWND> {
		unsafe { user32::GetDlgItem(self.ptr, ctrl_id as _).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetFocus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getfocus)
	/// static method.
	pub fn GetFocus() -> Option<HWND> {
		unsafe { user32::GetFocus().as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`GetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
	/// static method.
	pub fn GetForegroundWindow() -> Option<HWND> {
		unsafe { user32::GetForegroundWindow().as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`GetLastActivePopup`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getlastactivepopup)
	/// method.
	pub fn GetLastActivePopup(self) -> Option<HWND> {
		unsafe { user32::GetLastActivePopup(self.ptr).as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`GetMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenu)
	/// method.
	pub fn GetMenu(self) -> Option<HMENU> {
		unsafe { user32::GetMenu(self.ptr).as_mut() }
			.map(|ptr| HMENU { ptr })
	}

	/// [`GetMenuBarInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenubarinfo)
	/// method.
	pub fn GetMenuBarInfo(self,
		obj_id: co::OBJID, item_id: u32, mbi: &mut MENUBARINFO) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::GetMenuBarInfo(
					self.ptr,
					obj_id.0 as _,
					item_id as _,
					mbi as *mut _ as _,
				)
			},
		)
	}

	/// [`GetMenuItemRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemrect)
	/// method.
	pub fn GetMenuItemRect(self,
		hmenu: HMENU, item_pos: u32, rc_item: &mut RECT) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::GetMenuItemRect(
					self.ptr,
					hmenu.ptr,
					item_pos,
					rc_item as *mut _ as _,
				)
			},
		)
	}

	/// [`GetNextDlgGroupItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlggroupitem)
	/// method.
	pub fn GetNextDlgGroupItem(self,
		hwnd_ctrl: HWND, previous: bool) -> WinResult<HWND>
	{
		unsafe {
			user32::GetNextDlgGroupItem(self.ptr, hwnd_ctrl.ptr, previous as _)
				.as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetNextDlgTabItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlgtabitem)
	/// method.
	pub fn GetNextDlgTabItem(self,
		hwnd_ctrl: HWND, previous: bool) -> WinResult<HWND>
	{
		unsafe {
			user32::GetNextDlgTabItem(self.ptr, hwnd_ctrl.ptr, previous as _).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetParent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getparent)
	/// method.
	pub fn GetParent(self) -> WinResult<HWND> {
		unsafe { user32::GetParent(self.ptr).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetScrollInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getscrollinfo)
	/// method.
	pub fn GetScrollInfo(self,
		bar: co::SBB, si: &mut SCROLLINFO) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::GetScrollInfo(self.ptr, bar.0, si as *mut _ as _)
			},
		)
	}

	/// [`GetScrollPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getscrollpos)
	/// method.
	pub fn GetScrollPos(self, bar: co::SBB) -> WinResult<i32> {
		match unsafe { user32::GetScrollPos(self.ptr, bar.0) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero position
				err => Err(err),
			},
			pos => Ok(pos),
		}
	}

	/// [`GetShellWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getshellwindow)
	/// static method.
	pub fn GetShellWindow() -> Option<HWND> {
		unsafe { user32::GetShellWindow().as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`GetSystemMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmenu)
	/// method.
	pub fn GetSystemMenu(self, revert: bool) -> Option<HMENU> {
		unsafe { user32::GetSystemMenu(self.ptr, revert as _).as_mut() }
			.map(|ptr| HMENU { ptr })
	}

	/// [`GetTopWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-gettopwindow)
	/// method.
	pub fn GetTopWindow(self) -> WinResult<Option<HWND>> {
		match unsafe { user32::GetTopWindow(self.ptr).as_mut() } {
			Some(h) => Ok(Some(Self { ptr: h })),
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no child window
				err => Err(err),
			},
		}
	}

	/// [`GetUpdateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getupdatergn)
	/// method.
	pub fn GetUpdateRgn(self, hrgn: HRGN, erase: bool) -> WinResult<co::REGION> {
		match unsafe { user32::GetUpdateRgn(self.ptr, hrgn.ptr, erase as _) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	/// method.
	pub fn GetWindow(self, cmd: co::GW) -> WinResult<HWND> {
		unsafe { user32::GetWindow(self.ptr, cmd.0).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetWindowDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdc)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HWND::ReleaseDC`](crate::HWND::ReleaseDC) call.
	pub fn GetWindowDC(self) -> WinResult<HDC> {
		unsafe { user32::GetWindowDC(self.ptr).as_mut() }
			.map(|ptr| HDC { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetWindowDisplayAffinity`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdisplayaffinity)
	/// method.
	pub fn GetWindowDisplayAffinity(self) -> WinResult<co::WDA> {
		let mut affinity = co::WDA::default();
		match unsafe {
			user32::GetWindowDisplayAffinity(
				self.ptr,
				&mut affinity as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(affinity),
		}
	}

	/// [`GetWindowInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowinfo)
	/// method.
	pub fn GetWindowInfo(self, wi: &mut WINDOWINFO) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::GetWindowInfo(self.ptr, wi as *mut _ as _) },
		)
	}

	/// [`GetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
	/// method.
	pub fn GetWindowLongPtr(self, index: co::GWLP) -> isize {
		unsafe { user32::GetWindowLongPtrW(self.ptr, index.0) }
	}

	/// [`GetWindowPlacement`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowplacement)
	/// method.
	pub fn GetWindowPlacement(self, wp: &mut WINDOWPLACEMENT) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::GetWindowPlacement(self.ptr, wp as *mut _ as _) },
		)
	}

	/// [`GetWindowRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)
	/// method.
	pub fn GetWindowRect(self) -> WinResult<RECT> {
		let mut rc = RECT::default();
		match unsafe {
			user32::GetWindowRect(self.ptr, &mut rc as *mut _ as _)
		} {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetWindowRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgn)
	/// method.
	pub fn GetWindowRgn(self, hrgn: HRGN) -> WinResult<co::REGION> {
		match unsafe { user32::GetWindowRgn(self.ptr, hrgn.ptr) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindowRgnBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgnbox)
	/// method.
	pub fn GetWindowRgnBox(self, rc: &mut RECT) -> WinResult<co::REGION> {
		match unsafe {
			user32::GetWindowRgnBox(self.ptr, rc as *mut _ as _)
		} {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindowText`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw)
	/// method.
	///
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialized somewhere
	///
	/// let text = my_hwnd.GetWindowText()?;
	/// println!("Text: {}", text);
	/// ```
	pub fn GetWindowText(self) -> WinResult<String> {
		let len = self.GetWindowTextLength()?;
		if len == 0 {
			return Ok(String::default()); // window has no text
		}

		let mut buf = WString::new_alloc_buffer(len as usize + 1); // plus terminating null
		match unsafe {
			user32::GetWindowTextW(self.ptr, buf.as_mut_ptr(), len + 1)
		} {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(String::default()), // no chars copied for some reason
				err => Err(err),
			},
			_ => Ok(buf.to_string()),
		}
	}

	/// [`GetWindowTextLength`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextlengthw)
	/// method. Does not count the terminating null.
	pub fn GetWindowTextLength(self) -> WinResult<i32> {
		SetLastError(co::ERROR::SUCCESS);
		match unsafe { user32::GetWindowTextLengthW(self.ptr) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero length
				err => Err(err),
			},
			len => Ok(len),
		}
	}

	/// [`GetWindowThreadProcessId`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid)
	/// method.
	pub fn GetWindowThreadProcessId(self) -> u32 {
		unsafe { user32::GetWindowThreadProcessId(self.ptr, std::ptr::null_mut()) }
	}

	/// [`HiliteMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-hilitemenuitem)
	/// method.
	pub fn HiliteMenuItem(self,
		hmenu: HMENU, id_or_pos: IdPos, hilite: co::MF) -> bool
	{
		unsafe {
			user32::HiliteMenuItem(
				self.ptr, hmenu.ptr, id_or_pos.id_or_pos_u32(), hilite.0,
			) != 0
		}
	}

	/// [`InvalidateRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidaterect)
	/// method.
	///
	/// # Examples
	///
	/// Most of the time you'll just want update the entire client area:
	///
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialized somewhere
	///
	/// my_hwnd.InvalidateRect(None, true)?;
	/// ```
	pub fn InvalidateRect(self,
		rc: Option<&RECT>, erase: bool) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::InvalidateRect(
					self.ptr,
					rc.map_or(std::ptr::null(), |lp| lp as *const _ as _),
					erase as _,
				)
			},
		)
	}

	/// [`InvalidateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidatergn)
	/// method.
	pub fn InvalidateRgn(self, hrgn: HRGN, erase: bool) {
		unsafe { user32::InvalidateRgn(self.ptr, hrgn.ptr, erase as _); }
	}

	/// [`IsChild`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ischild)
	/// method.
	pub fn IsChild(self, hwnd_possible_child: HWND) -> bool {
		unsafe { user32::IsChild(self.ptr, hwnd_possible_child.ptr) != 0 }
	}

	/// [`IsDialogMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isdialogmessagew)
	/// method.
	pub fn IsDialogMessage(self, msg: &mut MSG) -> bool {
		unsafe {
			user32::IsDialogMessageW(self.ptr, msg as *mut _ as _) != 0
		}
	}

	/// [`IsIconic`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isiconic)
	/// method.
	pub fn IsIconic(self) -> bool {
		unsafe { user32::IsIconic(self.ptr) != 0 }
	}

	/// [`IsWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)
	/// method.
	pub fn IsWindow(self) -> bool {
		unsafe { user32::IsWindow(self.ptr) != 0 }
	}

	/// [`IsWindowEnabled`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowenabled)
	/// method.
	pub fn IsWindowEnabled(self) -> bool {
		unsafe { user32::IsWindowEnabled(self.ptr) != 0 }
	}

	/// [`IsWindowUnicode`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowunicode)
	/// method.
	pub fn IsWindowUnicode(self) -> bool {
		unsafe { user32::IsWindowUnicode(self.ptr) != 0 }
	}

	/// [`IsWindowVisible`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowvisible)
	/// method.
	pub fn IsWindowVisible(self) -> bool {
		unsafe { user32::IsWindowVisible(self.ptr) != 0 }
	}

	/// [`IsZoomed`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iszoomed)
	/// method.
	pub fn IsZoomed(self) -> bool {
		unsafe { user32::IsZoomed(self.ptr) != 0 }
	}

	/// [`KillTimer`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-killtimer)
	/// method.
	pub fn KillTimer(self, event_id: usize) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::KillTimer(self.ptr, event_id) })
	}

	/// [`LogicalToPhysicalPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-logicaltophysicalpoint)
	/// method.
	pub fn LogicalToPhysicalPoint(self, pt: *mut POINT) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::LogicalToPhysicalPoint(self.ptr, pt as _) },
		)
	}

	/// [`MapDialogRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapdialogrect)
	/// method.
	pub fn MapDialogRect(self, rc: &mut RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::MapDialogRect(self.ptr, rc as *mut _ as _) },
		)
	}

	/// [`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// method.
	///
	/// Consider using the more modern [`TaskDialog`](crate::HWND::TaskDialog)
	/// method.
	///
	/// # Examples
	///
	/// A modal message box, which blocks its parent:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HWND};
	///
	/// let my_hwnd: HWND; // initialized somewhere
	///
	/// my_hwnd.MessageBox("Hello, world", "title",
	///     co::MB::OKCANCEL | co::MB::ICONINFORMATION)?
	/// ```
	///
	/// Usually the message box has a valid parent window, however, if for some
	/// reason you don't have a window to serve as parent, you still can show a
	/// non-modal, parent-less message box by retrieving the desktop handle:
	///
	/// ```rust,ignore
	/// HWND::GetDesktopWindow()
	///     .MessageBox("Hello, world", "Title", co::MB::ICONEXCLAMATION)?;
	/// ```
	pub fn MessageBox(self,
		text: &str, caption: &str, flags: co::MB) -> WinResult<co::DLGID>
	{
		match unsafe {
			user32::MessageBoxW(
				self.ptr,
				WString::from_str(text).as_ptr(),
				WString::from_str(caption).as_ptr(),
				flags.0,
			)
		} {
			0 => Err(GetLastError()),
			ret => Ok(co::DLGID(ret as _)),
		}
	}

	/// [`MonitorFromWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromwindow)
	/// method.
	pub fn MonitorFromWindow(self, flags: co::MONITOR) -> HMONITOR {
		HMONITOR {
			ptr: unsafe { user32::MonitorFromWindow(self.ptr, flags.0) },
		}
	}

	/// [`MoveWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-movewindow)
	/// method.
	pub fn MoveWindow(self,
		pos: POINT, size: SIZE, repaint: bool) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::MoveWindow(
					self.ptr,
					pos.x, pos.y,
					size.cx, size.cy,
					repaint as _,
				)
			},
		)
	}

	/// [`OpenClipboard`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openclipboard)
	/// method.
	///
	/// **Note:** Must be paired with a
	/// [`CloseClipboard`](crate::CloseClipboard) call.
	pub fn OpenClipboard(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::OpenClipboard(self.ptr) })
	}

	/// [`OpenThemeData`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-openthemedata)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HTHEME::CloseThemeData`](crate::HTHEME::CloseThemeData) call.
	pub fn OpenThemeData(self, class_list: &str) -> Option<HTHEME> {
		unsafe {
			uxtheme::OpenThemeData(
				self.ptr,
				WString::from_str(class_list).as_ptr(),
			).as_mut()
		}.map(|ptr| HTHEME { ptr })
	}

	/// [`PostMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postmessagew)
	/// method. Note that this method is asychronous.
	///
	/// # Examples
	///
	/// Programatically closing a window:
	///
	/// ```rust,ignore
	/// use winsafe::{HWND, msg::wm};
	///
	/// let my_hwnd: HWND; // initialized somewhere
	///
	/// my_hwnd.PostMessage(wm::Close {})?;
	/// ```
	///
	/// Sending a message to all top-level windows:
	///
	/// ```rust,ignore
	/// use winsafe::{HWND, msg::wm};
	///
	/// HWND::BROADCAST.PostMessage(
	///     wm::ExitMenuLoop { is_shortcut: false },
	/// )?;
	/// ```
	pub fn PostMessage<M: MsgSend>(self, msg: M) -> WinResult<()> {
		let wm_any = msg.as_generic_wm();
		bool_to_winresult(
			unsafe {
				user32::PostMessageW(
					self.ptr, wm_any.msg_id.0, wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`RealChildWindowFromPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realchildwindowfrompoint)
	/// method.
	pub fn RealChildWindowFromPoint(self,
		pt_parent_client_coords: POINT) -> Option<HWND>
	{
		unsafe {
			user32::RealChildWindowFromPoint(
				self.ptr,
				pt_parent_client_coords.x,
				pt_parent_client_coords.y,
			).as_mut()
		}.map(|ptr| Self { ptr })
	}

	/// [`RealGetWindowClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realgetwindowclassw)
	/// method.
	pub fn RealGetWindowClass(self) -> WinResult<String> {
		let mut buf = WString::new_alloc_buffer(256 + 1); // according to WNDCLASSEX docs
		match unsafe {
			user32::RealGetWindowClassW(
				self.ptr,
				buf.as_mut_ptr(),
				buf.buffer_size() as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(buf.to_string()),
		}
	}

	/// [`RedrawWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow)
	/// method.
	pub fn RedrawWindow(self,
		rc_update: &RECT, hrgn_update: HRGN, flags: co::RDW) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::RedrawWindow(
					self.ptr,
					rc_update as *const _ as _,
					hrgn_update.ptr,
					flags.0,
				)
			},
		)
	}

	/// [`ReleaseDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
	/// method.
	pub fn ReleaseDC(self, hdc: HDC) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::ReleaseDC(self.ptr, hdc.ptr) },
		)
	}

	/// [`RemoveWindowSubclass`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-removewindowsubclass)
	/// method.
	pub fn RemoveWindowSubclass(self,
		subclass_func: SUBCLASSPROC, subclass_id: usize) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				comctl32::RemoveWindowSubclass(
					self.ptr,
					subclass_func as _,
					subclass_id,
				)
			},
		)
	}

	/// [`ScreenToClient`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-screentoclient)
	/// method.
	pub fn ScreenToClient(self, pt: &mut POINT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::ScreenToClient(self.ptr, pt as *mut _ as _)
			},
		)
	}

	/// [`ScreenToClient`](crate::HWND::ScreenToClient) method for a
	/// [`RECT`](crate::RECT).
	pub fn ScreenToClientRc(self, rc: &mut RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::ScreenToClient(
					self.ptr,
					&mut rc.left as *mut _ as _,
				)
			},
		).and_then(|_| bool_to_winresult(
			unsafe {
				user32::ScreenToClient(
					self.ptr,
					&mut rc.right as *mut _ as _,
				)
			},
		))
	}

	/// [`SendMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)
	/// method.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::msg::MsgSend) trait. That means each
	/// message can define its own return type.
	///
	/// # Examples
	///
	/// Sending a [`lvm::SetItem`](crate::msg::lvm::SetItem) list view message,
	/// which demands a reference to an [`LVITEM`](crate::LVITEM) object:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HWND, LVITEM, msg::lvm};
	///
	/// let my_hwnd: HWND; // initialized somewhere
	///
	/// let mut lvi = LVITEM::default(); // object to be sent
	/// lvi.mask = co::LVIF::IMAGE;
	/// lvi.iImage = 3;
	///
	/// my_hwnd.SendMessage(lvm::SetItem {
	///     lvitem: &lvi,
	/// });
	/// ```
	pub fn SendMessage<M: MsgSend>(self, msg: M) -> M::RetType {
		let wm_any = msg.as_generic_wm();
		msg.convert_ret(
			unsafe {
				user32::SendMessageW(
					self.ptr, wm_any.msg_id.0, wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`SetCapture`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcapture)
	/// method.
	///
	/// **Note:** Must be paired with a
	/// [`ReleaseCapture`](crate::ReleaseCapture) call.
	pub fn SetCapture(&self) -> Option<HWND> {
		unsafe { user32::SetCapture(self.ptr).as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`SetFocus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setfocus)
	/// method.
	pub fn SetFocus(self) -> Option<HWND> {
		unsafe { user32::SetFocus(self.ptr).as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`SetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow)
	/// method.
	pub fn SetForegroundWindow(self) -> bool {
		unsafe { user32::SetForegroundWindow(self.ptr) != 0 }
	}

	/// [`SetMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenu)
	/// method.
	pub fn SetMenu(self, hmenu: HMENU) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::SetMenu(self.ptr, hmenu.ptr) })
	}

	/// [`SetParent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setparent)
	/// method.
	pub fn SetParent(self, hwnd_new_parent: HWND) -> WinResult<Option<HWND>> {
		match unsafe {
			user32::SetParent(self.ptr, hwnd_new_parent.ptr).as_mut()
		} {
			Some(ptr) => Ok(Some(Self { ptr })),
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no previous parent
				err => Err(err),
			},
		}
	}

	/// [`SetScrollInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollinfo)
	/// method.
	pub fn SetScrollInfo(self,
		bar: co::SBB, si: &SCROLLINFO, redraw: bool) -> i32
	{
		unsafe {
			user32::SetScrollInfo(
				self.ptr,
				bar.0,
				si as *const _ as _,
				redraw as _,
			)
		}
	}

	/// [`SetScrollPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollpos)
	/// method.
	pub fn SetScrollPos(self,
		b: co::SBB, pos: i32, redraw: bool) -> WinResult<i32>
	{
		match unsafe {
			user32::SetScrollPos(self.ptr, b.0, pos, redraw as BOOL)
		} {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero position
				err => Err(err),
			},
			pos => Ok(pos),
		}
	}

	/// [`SetScrollRange`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollrange)
	/// method.
	pub fn SetScrollRange(self,
		bar: co::SBB, min_pos: i32, max_pos: i32, redraw: bool) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::SetScrollRange(
					self.ptr,
					bar.0,
					min_pos,
					max_pos,
					redraw as _,
				)
			},
		)
	}

	/// [`SetTimer`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-settimer)
	/// method.
	pub fn SetTimer(self,
		event_id: usize, elapse_ms: u32,
		timer_func: Option<TIMERPROC>) -> WinResult<usize>
	{
		match unsafe {
			user32::SetTimer(
				self.ptr,
				event_id,
				elapse_ms,
				timer_func.map_or(std::ptr::null(), |lp| lp as _),
			)
		} {
			0 => Err(GetLastError()),
			tid => Ok(tid),
		}
	}

	/// [`SetWindowDisplayAffinity`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity)
	/// method.
	pub fn SetWindowDisplayAffinity(self, affinity: co::WDA) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::SetWindowDisplayAffinity(self.ptr, affinity.0) },
		)
	}

	/// [`SetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
	/// method.
	pub fn SetWindowLongPtr(self, index: co::GWLP, new_long: isize) -> isize {
		unsafe { user32::SetWindowLongPtrW(self.ptr, index.0, new_long) }
	}

	/// [`SetWindowPlacement`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowplacement)
	/// method.
	pub fn SetWindowPlacement(self, wp: &WINDOWPLACEMENT) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::SetWindowPlacement(self.ptr, wp as *const _ as _) },
		)
	}

	/// [`SetWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)
	/// method.
	pub fn SetWindowPos(self,
		hwnd_insert_after: HwndPlace,
		pos: POINT, size: SIZE, flags: co::SWP) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::SetWindowPos(
					self.ptr,
					hwnd_insert_after.as_ptr(),
					pos.x, pos.y,
					size.cx, size.cy,
					flags.0,
				)
			},
		)
	}

	/// [`SetWindowRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowrgn)
	/// method.
	pub fn SetWindowRgn(self, hrgn: HRGN, redraw: bool) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::SetWindowRgn(self.ptr, hrgn.ptr, redraw as _) },
		)
	}

	/// [`SetWindowSubclass`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass)
	/// method.
	pub fn SetWindowSubclass(self,
		subclass_proc: SUBCLASSPROC,
		subclass_id: usize, ref_data: usize) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				comctl32::SetWindowSubclass(
					self.ptr,
					subclass_proc as _,
					subclass_id,
					ref_data,
				)
			},
		)
	}

	/// [`SetWindowText`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
	/// method.
	pub fn SetWindowText(self, text: &str) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::SetWindowTextW(
					self.ptr,
					WString::from_str(text).as_ptr(),
				)
			},
		)
	}

	/// [`ShellExecute`](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew)
	/// method.
	pub fn ShellExecute(self,
		operation: &str,
		file: &str,
		parameters: Option<&str>,
		directory: Option<&str>,
		show_cmd: co::SW) -> Result<HINSTANCE, co::SE_ERR>
	{
		let ret = unsafe {
			shell32::ShellExecuteW(
				self.ptr,
				WString::from_str(operation).as_ptr(),
				WString::from_str(file).as_ptr(),
				parameters.map_or(std::ptr::null(), |lp| WString::from_str(lp).as_ptr()),
				directory.map_or(std::ptr::null(), |lp| WString::from_str(lp).as_ptr()),
				show_cmd.0,
			)
		};

		if ret <= 32 as _ {
			Err(co::SE_ERR(ret as _))
		} else {
			Ok(HINSTANCE { ptr: ret as _ })
		}
	}

	/// [`ShowCaret`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcaret)
	/// method.
	pub fn ShowCaret(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::ShowCaret(self.ptr) })
	}

	/// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	/// method.
	pub fn ShowWindow(self, show_cmd: co::SW) -> bool {
		unsafe { user32::ShowWindow(self.ptr, show_cmd.0) != 0 }
	}

	/// [`TaskDialog`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-taskdialog)
	/// method.
	///
	/// If you need more customization, consider the
	/// [`TaskDialogIndirect`](crate::TaskDialogIndirect) function.
	///
	/// # Examples
	///
	/// An information message with just an OK button:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HWND, IdTdicon};
	///
	/// let my_hwnd: HWND; // initialized somewhere
	///
	/// my_hwnd.TaskDialog(
	///     None,
	///     Some("My app name"),
	///     Some("Operation successful"),
	///     Some("The operation completed successfully."),
	///     co::TDCBF::OK
	///     IdTdicon::Tdicon(co::TD_ICON::INFORMATION),
	/// )?;
	/// ```
	///
	/// Prompt the user to click OK or Cancel upon a question:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HWND, IdTdicon};
	///
	/// let my_hwnd: HWND; // initialized somewhere
	///
	/// let answer = my_hwnd.TaskDialog(
	///     None,
	///     Some("My app name"),
	///     Some("File modified"),
	///     Some("The file has been modified.\nProceed closing the application?"),
	///     co::TDCBF::OK | co::TDCBF::CANCEL,
	///     IdTdicon::Tdicon(co::TD_ICON::WARNING),
	/// )?;
	///
	/// if answer == co::DLGID::OK {
	///     println!("User clicked OK.");
	/// }
	/// ```
	pub fn TaskDialog(self,
		hinstance: Option<HINSTANCE>,
		window_title: Option<&str>,
		main_instruction: Option<&str>,
		content: Option<&str>,
		common_buttons: co::TDCBF,
		icon: IdTdiconStr) -> WinResult<co::DLGID>
	{
		// https://weblogs.asp.net/kennykerr/Windows-Vista-for-Developers-_1320_-Part-2-_1320_-Task-Dialogs-in-Depth
		let mut pn_button = i32::default();
		hr_to_winresult(
			unsafe {
				comctl32::TaskDialog(
					self.ptr,
					hinstance.map_or(std::ptr::null_mut(), |h| h.ptr),
					WString::from_opt_str(window_title).as_ptr(),
					WString::from_opt_str(main_instruction).as_ptr(),
					WString::from_opt_str(content).as_ptr(),
					common_buttons.0,
					icon.as_ptr(),
					&mut pn_button,
				)
			},
		).map(|_| co::DLGID(pn_button as _))
	}

	/// [`TranslateAccelerator`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translateacceleratorw)
	/// method.
	pub fn TranslateAccelerator(self,
		haccel_table: HACCEL, msg: &mut MSG) -> WinResult<()>
	{
		match unsafe {
			user32::TranslateAcceleratorW(
				self.ptr,
				haccel_table.ptr,
				msg as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`UpdateWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatewindow)
	/// method.
	pub fn UpdateWindow(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::UpdateWindow(self.ptr) })
	}

	/// [`ValidateRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validaterect)
	/// method.
	pub fn ValidateRect(self, rc: &RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::ValidateRect(self.ptr, rc as *const _ as _)
			},
		)
	}

	/// [`ValidateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validatergn)
	/// method.
	pub fn ValidateRgn(self, hrgn: HRGN) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::ValidateRgn(self.ptr, hrgn.ptr) })
	}

	/// [`WindowFromPhysicalPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfromphysicalpoint)
	/// static method.
	pub fn WindowFromPhysicalPoint(pt: POINT) -> Option<HWND> {
		unsafe { user32::WindowFromPhysicalPoint(pt.x, pt.y).as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`WindowFromPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfrompoint)
	/// static method.
	pub fn WindowFromPoint(pt: POINT) -> Option<HWND> {
		unsafe { user32::WindowFromPoint(pt.x, pt.y).as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`WinHelp`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-winhelpw)
	/// method.
	pub fn WinHelp(self,
		help_file: &str, cmd: co::HELPW, data: usize) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::WinHelpW(
					self.ptr,
					WString::from_str(help_file).as_ptr(),
					cmd.0,
					data,
				)
			},
		)
	}
}
