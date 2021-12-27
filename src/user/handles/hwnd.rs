#![allow(non_snake_case)]

use crate::{co, user};
use crate::ffi_types::BOOL;
use crate::kernel::decl::{GetLastError, HINSTANCE, SetLastError, WinResult,
	WString};
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::{Handle, MsgSend};
use crate::user::decl::{ALTTABINFO, AtomStr, HACCEL, HDC, HMENU, HMONITOR, HRGN,
	HwndPlace, IdMenu, IdPos, MENUBARINFO, MSG, PAINTSTRUCT, POINT, RECT,
	SCROLLINFO, SIZE, TIMERPROC, WINDOWINFO, WINDOWPLACEMENT};

impl_handle! { HWND: "user";
	/// Handle to a
	/// [window](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
}

impl UserHwnd for HWND {}

/// [`HWND`](crate::HWND) methods from `user` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub trait UserHwnd: Handle {
	/// Represents all top-level windows in
	/// [`HWND::PostMessage`](crate::prelude::UserHwnd::PostMessage) and
	/// [`HWND::SendMessage`](crate::prelude::UserHwnd::SendMessage).
	const BROADCAST: HWND = HWND(0xffff as _);

	/// [`GetWindowLongPtr`](crate::prelude::UserHwnd::GetWindowLongPtr) wrapper
	/// to retrieve the window [`HINSTANCE`](crate::HINSTANCE).
	fn hinstance(self) -> HINSTANCE {
		HINSTANCE(self.GetWindowLongPtr(co::GWLP::HINSTANCE) as _)
	}

	/// [`ArrangeIconicWindows`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-arrangeiconicwindows)
	/// method.
	fn ArrangeIconicWindows(self) -> WinResult<u32> {
		match unsafe { user::ffi::ArrangeIconicWindows(self.as_ptr()) } {
			0 => Err(GetLastError()),
			height => Ok(height),
		}
	}

	/// [`BeginPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HWND::EndPaint`](crate::prelude::UserHwnd::EndPaint) call.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HWND, PAINTSTRUCT};
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// let mut ps = PAINTSTRUCT::default();
	/// let hdc = hwnd.BeginPaint(&mut ps)?;
	///
	/// // hdc painting...
	///
	/// hwnd.EndPaint(&ps);
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn BeginPaint(self, ps: &mut PAINTSTRUCT) -> WinResult<HDC> {
		unsafe {
			user::ffi::BeginPaint(self.as_ptr(), ps as *mut _ as _).as_mut()
		}.map(|ptr| HDC(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`BringWindowToTop`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-bringwindowtotop)
	/// method.
	fn BringWindowToTop(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::BringWindowToTop(self.as_ptr()) })
	}

	/// [`ChildWindowFromPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-childwindowfrompoint)
	/// method.
	fn ChildWindowFromPoint(self, pt: POINT) -> Option<HWND> {
		unsafe {
			user::ffi::ChildWindowFromPoint(self.as_ptr(), pt.x, pt.y).as_mut()
		}.map(|ptr| HWND(ptr))
	}

	/// [`ClientToScreen`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clienttoscreen)
	/// method.
	fn ClientToScreen(self, pt: &mut POINT) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user::ffi::ClientToScreen(self.as_ptr(), pt as *mut _ as _) },
		)
	}

	/// [`ClientToScreen`](crate::prelude::UserHwnd::ClientToScreen) method for
	/// a [`RECT`](crate::RECT).
	fn ClientToScreenRc(self, rc: &mut RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user::ffi::ClientToScreen(
					self.as_ptr(),
					&mut rc.left as *mut _ as _,
				)
			},
		).and_then(|_| bool_to_winresult(
			unsafe {
				user::ffi::ClientToScreen(
					self.as_ptr(),
					&mut rc.right as *mut _ as _,
				)
			},
		))
	}

	/// [`CloseWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindow)
	/// method.
	///
	/// Note that this method will actually minimize the window, not destroy it.
	fn CloseWindow(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::CloseWindow(self.as_ptr()) })
	}

	/// [`CreateWindowEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
	/// static method.
	fn CreateWindowEx(
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
			user::ffi::CreateWindowExW(
				ex_style.0,
				class_name.as_ptr(),
				WString::from_opt_str(title).as_ptr(),
				style.0,
				pos.x, pos.y,
				size.cx, size.cy,
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.0),
				hmenu.as_ptr(),
				hinstance.0,
				lparam.unwrap_or_default() as _,
			).as_mut()
		}.map(|ptr| HWND(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DefWindowProc`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
	/// method.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::prelude::MsgSend) trait. That means each
	/// message can define its own return type.
	fn DefWindowProc<M>(self, msg: M) -> M::RetType
		where M: MsgSend,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		msg.convert_ret(
			unsafe {
				user::ffi::DefWindowProcW(
					self.as_ptr(), wm_any.msg_id.0, wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
	/// method.
	fn DestroyWindow(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::DestroyWindow(self.as_ptr()) })
	}

	/// [`DrawMenuBar`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawmenubar)
	/// method.
	fn DrawMenuBar(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::DrawMenuBar(self.as_ptr()) })
	}

	/// [`EnableWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablewindow)
	/// method.
	fn EnableWindow(self, enable: bool) -> bool {
		unsafe { user::ffi::EnableWindow(self.as_ptr(), enable as _) != 0 }
	}

	/// [`EndDialog`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddialog)
	/// method.
	fn EndDialog(self, result: isize) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::EndDialog(self.as_ptr(), result) })
	}

	/// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
	/// method.
	fn EndPaint(self, ps: &PAINTSTRUCT) {
		unsafe { user::ffi::EndPaint(self.as_ptr(), ps as *const _ as _); }
	}

	/// [`EnumChildWindows`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumchildwindows)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::HWND;
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// hwnd.EnumChildWindows(|hchild: HWND| -> bool {
	///     println!("Child HWND: {}", hchild);
	///     true
	/// });
	/// ```
	fn EnumChildWindows<F>(self, func: F)
		where F: Fn(HWND) -> bool,
	{
		unsafe {
			user::ffi::EnumChildWindows(
				self.as_ptr(),
				enum_child_windows_proc::<F> as _, // https://redd.it/npehj9
				&func as *const _ as _,
			);
		}
	}

	/// [`FindWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindoww)
	/// static method.
	fn FindWindow(
		class_name: Option<AtomStr>,
		title: Option<&str>) -> WinResult<HWND>
	{
		unsafe {
			user::ffi::FindWindowW(
				class_name.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
				WString::from_opt_str(title).as_ptr(),
			).as_mut()
		}.map(|ptr| HWND(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`FindWindowEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindowexw)
	/// method.
	fn FindWindowEx(self,
		hwnd_child_after: Option<HWND>,
		class_name: AtomStr,
		title: Option<&str>) -> WinResult<HWND>
	{
		unsafe {
			user::ffi::FindWindowExW(
				self.as_ptr(),
				hwnd_child_after.map_or(std::ptr::null_mut(), |h| h.0),
				class_name.as_ptr(),
				WString::from_opt_str(title).as_ptr(),
			).as_mut()
		}.map(|ptr| HWND(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetActiveWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getactivewindow)
	/// static method.
	fn GetActiveWindow() -> Option<HWND> {
		unsafe { user::ffi::GetActiveWindow().as_mut() }
			.map(|ptr| HWND(ptr))
	}

	/// [`GetAltTabInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getalttabinfow)
	/// method.
	///
	/// If `item` is `None`, the item text is not retrieved.
	///
	/// The `sz_item_text` is the maximum number of expected chars for the item
	/// text. If `None`, defaults to 100.
	fn GetAltTabInfo(self,
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
				user::ffi::GetAltTabInfoW(
					self.as_ptr(),
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
	fn GetAncestor(self, flags: co::GA) -> Option<HWND> {
		unsafe { user::ffi::GetAncestor(self.as_ptr(), flags.0).as_mut() }
			.map(|ptr| HWND(ptr))
	}

	/// [`GetCapture`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcapture)
	/// static method.
	fn GetCapture() -> Option<HWND> {
		unsafe { user::ffi::GetCapture().as_mut() }
			.map(|ptr| HWND(ptr))
	}

	/// [`GetClassLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclasslongptrw)
	/// method.
	fn GetClassLongPtr(self, index: co::GCLP) -> usize {
		unsafe { user::ffi::GetClassLongPtrW(self.as_ptr(), index.0) }
	}

	/// [`GetClassName`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclassnamew)
	/// method.
	fn GetClassName(self) -> WinResult<String> {
		let mut buf = WString::new_alloc_buffer(256 + 1); // according to WNDCLASSEX docs
		match unsafe {
			user::ffi::GetClassNameW(
				self.as_ptr(),
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
	fn GetClientRect(self) -> WinResult<RECT> {
		let mut rc = RECT::default();
		match unsafe {
			user::ffi::GetClientRect(self.as_ptr(), &mut rc as *mut _ as _)
		} {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HWND::ReleaseDC`](crate::prelude::UserHwnd::ReleaseDC) call.
	fn GetDC(self) -> WinResult<HDC> {
		unsafe { user::ffi::GetDC(self.as_ptr()).as_mut() }
			.map(|ptr| HDC(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetDesktopWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow)
	/// static method.
	fn GetDesktopWindow() -> HWND {
		HWND(unsafe { user::ffi::GetDesktopWindow() })
	}

	/// [`GetDlgCtrlID`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgctrlid)
	/// method.
	fn GetDlgCtrlID(self) -> WinResult<u16> {
		match unsafe { user::ffi::GetDlgCtrlID(self.as_ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual ID is zero
				err => Err(err),
			},
			id => Ok(id as _),
		}
	}

	/// [`GetDlgItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgitem)
	/// method.
	fn GetDlgItem(self, ctrl_id: u16) -> WinResult<HWND> {
		unsafe { user::ffi::GetDlgItem(self.as_ptr(), ctrl_id as _).as_mut() }
			.map(|ptr| HWND(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetFocus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getfocus)
	/// static method.
	fn GetFocus() -> Option<HWND> {
		unsafe { user::ffi::GetFocus().as_mut() }
			.map(|ptr| HWND(ptr))
	}

	/// [`GetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
	/// static method.
	fn GetForegroundWindow() -> Option<HWND> {
		unsafe { user::ffi::GetForegroundWindow().as_mut() }
			.map(|ptr| HWND(ptr))
	}

	/// [`GetLastActivePopup`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getlastactivepopup)
	/// method.
	fn GetLastActivePopup(self) -> Option<HWND> {
		unsafe { user::ffi::GetLastActivePopup(self.as_ptr()).as_mut() }
			.map(|ptr| HWND(ptr))
	}

	/// [`GetMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenu)
	/// method.
	fn GetMenu(self) -> Option<HMENU> {
		unsafe { user::ffi::GetMenu(self.as_ptr()).as_mut() }
			.map(|ptr| HMENU(ptr))
	}

	/// [`GetMenuBarInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenubarinfo)
	/// method.
	fn GetMenuBarInfo(self,
		obj_id: co::OBJID, item_id: u32, mbi: &mut MENUBARINFO) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user::ffi::GetMenuBarInfo(
					self.as_ptr(),
					obj_id.0 as _,
					item_id as _,
					mbi as *mut _ as _,
				)
			},
		)
	}

	/// [`GetMenuItemRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemrect)
	/// method.
	fn GetMenuItemRect(self,
		hmenu: HMENU, item_pos: u32, rc_item: &mut RECT) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user::ffi::GetMenuItemRect(
					self.as_ptr(),
					hmenu.0,
					item_pos,
					rc_item as *mut _ as _,
				)
			},
		)
	}

	/// [`GetNextDlgGroupItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlggroupitem)
	/// method.
	fn GetNextDlgGroupItem(self,
		hwnd_ctrl: HWND, previous: bool) -> WinResult<HWND>
	{
		unsafe {
			user::ffi::GetNextDlgGroupItem(
				self.as_ptr(), hwnd_ctrl.0, previous as _,
			).as_mut()
		}.map(|ptr| HWND(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetNextDlgTabItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlgtabitem)
	/// method.
	fn GetNextDlgTabItem(self,
		hwnd_ctrl: HWND, previous: bool) -> WinResult<HWND>
	{
		unsafe {
			user::ffi::GetNextDlgTabItem(self.as_ptr(), hwnd_ctrl.0, previous as _)
				.as_mut()
		}.map(|ptr| HWND(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetParent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getparent)
	/// method.
	fn GetParent(self) -> WinResult<HWND> {
		unsafe { user::ffi::GetParent(self.as_ptr()).as_mut() }
			.map(|ptr| HWND(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetScrollInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getscrollinfo)
	/// method.
	fn GetScrollInfo(self,
		bar: co::SBB, si: &mut SCROLLINFO) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user::ffi::GetScrollInfo(self.as_ptr(), bar.0, si as *mut _ as _)
			},
		)
	}

	/// [`GetScrollPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getscrollpos)
	/// method.
	fn GetScrollPos(self, bar: co::SBB) -> WinResult<i32> {
		match unsafe { user::ffi::GetScrollPos(self.as_ptr(), bar.0) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero position
				err => Err(err),
			},
			pos => Ok(pos),
		}
	}

	/// [`GetShellWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getshellwindow)
	/// static method.
	fn GetShellWindow() -> Option<HWND> {
		unsafe { user::ffi::GetShellWindow().as_mut() }
			.map(|ptr| HWND(ptr))
	}

	/// [`GetSystemMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmenu)
	/// method.
	fn GetSystemMenu(self, revert: bool) -> Option<HMENU> {
		unsafe { user::ffi::GetSystemMenu(self.as_ptr(), revert as _).as_mut() }
			.map(|ptr| HMENU(ptr))
	}

	/// [`GetTopWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-gettopwindow)
	/// method.
	fn GetTopWindow(self) -> WinResult<Option<HWND>> {
		match unsafe { user::ffi::GetTopWindow(self.as_ptr()).as_mut() } {
			Some(ptr) => Ok(Some(HWND(ptr))),
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no child window
				err => Err(err),
			},
		}
	}

	/// [`GetUpdateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getupdatergn)
	/// method.
	fn GetUpdateRgn(self, hrgn: HRGN, erase: bool) -> WinResult<co::REGION> {
		match unsafe {
			user::ffi::GetUpdateRgn(self.as_ptr(), hrgn.0, erase as _) }
		{
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	/// method.
	fn GetWindow(self, cmd: co::GW) -> WinResult<HWND> {
		unsafe { user::ffi::GetWindow(self.as_ptr(), cmd.0).as_mut() }
			.map(|ptr| HWND(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetWindowDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdc)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HWND::ReleaseDC`](crate::prelude::UserHwnd::ReleaseDC) call.
	fn GetWindowDC(self) -> WinResult<HDC> {
		unsafe { user::ffi::GetWindowDC(self.as_ptr()).as_mut() }
			.map(|ptr| HDC(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`GetWindowDisplayAffinity`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdisplayaffinity)
	/// method.
	fn GetWindowDisplayAffinity(self) -> WinResult<co::WDA> {
		let mut affinity = co::WDA::default();
		match unsafe {
			user::ffi::GetWindowDisplayAffinity(
				self.as_ptr(),
				&mut affinity as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(affinity),
		}
	}

	/// [`GetWindowInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowinfo)
	/// method.
	fn GetWindowInfo(self, wi: &mut WINDOWINFO) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user::ffi::GetWindowInfo(self.as_ptr(), wi as *mut _ as _) },
		)
	}

	/// [`GetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
	/// method.
	fn GetWindowLongPtr(self, index: co::GWLP) -> isize {
		unsafe { user::ffi::GetWindowLongPtrW(self.as_ptr(), index.0) }
	}

	/// [`GetWindowPlacement`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowplacement)
	/// method.
	fn GetWindowPlacement(self, wp: &mut WINDOWPLACEMENT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user::ffi::GetWindowPlacement(self.as_ptr(), wp as *mut _ as _)
			},
		)
	}

	/// [`GetWindowRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)
	/// method.
	fn GetWindowRect(self) -> WinResult<RECT> {
		let mut rc = RECT::default();
		match unsafe {
			user::ffi::GetWindowRect(self.as_ptr(), &mut rc as *mut _ as _)
		} {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetWindowRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgn)
	/// method.
	fn GetWindowRgn(self, hrgn: HRGN) -> WinResult<co::REGION> {
		match unsafe { user::ffi::GetWindowRgn(self.as_ptr(), hrgn.0) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindowRgnBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgnbox)
	/// method.
	fn GetWindowRgnBox(self, rc: &mut RECT) -> WinResult<co::REGION> {
		match unsafe {
			user::ffi::GetWindowRgnBox(self.as_ptr(), rc as *mut _ as _)
		} {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindowText`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw)
	/// method.
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::HWND;
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// let text = hwnd.GetWindowText()?;
	/// println!("Text: {}", text);
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn GetWindowText(self) -> WinResult<String> {
		let len = self.GetWindowTextLength()?;
		if len == 0 {
			return Ok(String::default()); // window has no text
		}

		let mut buf = WString::new_alloc_buffer(len as usize + 1); // plus terminating null
		match unsafe {
			user::ffi::GetWindowTextW(self.as_ptr(), buf.as_mut_ptr(), len + 1)
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
	fn GetWindowTextLength(self) -> WinResult<i32> {
		SetLastError(co::ERROR::SUCCESS);
		match unsafe { user::ffi::GetWindowTextLengthW(self.as_ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero length
				err => Err(err),
			},
			len => Ok(len),
		}
	}

	/// [`GetWindowThreadProcessId`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid)
	/// method.
	///
	/// Returns thread ID and process ID, respectively.
	fn GetWindowThreadProcessId(self) -> (u32, u32) {
		let mut proc_id = u32::default();
		let thread_id = unsafe {
			user::ffi::GetWindowThreadProcessId(self.as_ptr(), &mut proc_id)
		};
		(thread_id, proc_id)
	}

	/// [`HiliteMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-hilitemenuitem)
	/// method.
	fn HiliteMenuItem(self,
		hmenu: HMENU, id_or_pos: IdPos, hilite: co::MF) -> bool
	{
		unsafe {
			user::ffi::HiliteMenuItem(
				self.as_ptr(), hmenu.0, id_or_pos.id_or_pos_u32(), hilite.0,
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
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::HWND;
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// hwnd.InvalidateRect(None, true)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn InvalidateRect(self, rc: Option<&RECT>, erase: bool) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user::ffi::InvalidateRect(
					self.as_ptr(),
					rc.map_or(std::ptr::null(), |lp| lp as *const _ as _),
					erase as _,
				)
			},
		)
	}

	/// [`InvalidateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidatergn)
	/// method.
	fn InvalidateRgn(self, hrgn: HRGN, erase: bool) {
		unsafe { user::ffi::InvalidateRgn(self.as_ptr(), hrgn.0, erase as _); }
	}

	/// [`IsChild`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ischild)
	/// method.
	fn IsChild(self, hwnd_possible_child: HWND) -> bool {
		unsafe { user::ffi::IsChild(self.as_ptr(), hwnd_possible_child.0) != 0 }
	}

	/// [`IsDialogMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isdialogmessagew)
	/// method.
	fn IsDialogMessage(self, msg: &mut MSG) -> bool {
		unsafe {
			user::ffi::IsDialogMessageW(self.as_ptr(), msg as *mut _ as _) != 0
		}
	}

	/// [`IsIconic`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isiconic)
	/// method.
	fn IsIconic(self) -> bool {
		unsafe { user::ffi::IsIconic(self.as_ptr()) != 0 }
	}

	/// [`IsWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)
	/// method.
	fn IsWindow(self) -> bool {
		unsafe { user::ffi::IsWindow(self.as_ptr()) != 0 }
	}

	/// [`IsWindowEnabled`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowenabled)
	/// method.
	fn IsWindowEnabled(self) -> bool {
		unsafe { user::ffi::IsWindowEnabled(self.as_ptr()) != 0 }
	}

	/// [`IsWindowUnicode`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowunicode)
	/// method.
	fn IsWindowUnicode(self) -> bool {
		unsafe { user::ffi::IsWindowUnicode(self.as_ptr()) != 0 }
	}

	/// [`IsWindowVisible`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowvisible)
	/// method.
	fn IsWindowVisible(self) -> bool {
		unsafe { user::ffi::IsWindowVisible(self.as_ptr()) != 0 }
	}

	/// [`IsZoomed`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iszoomed)
	/// method.
	fn IsZoomed(self) -> bool {
		unsafe { user::ffi::IsZoomed(self.as_ptr()) != 0 }
	}

	/// [`KillTimer`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-killtimer)
	/// method.
	fn KillTimer(self, event_id: usize) -> WinResult<()> {
		match unsafe { user::ffi::KillTimer(self.as_ptr(), event_id) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(()),
				e => Err(e),
			}
			_ => Ok(()),
		}
	}

	/// [`LogicalToPhysicalPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-logicaltophysicalpoint)
	/// method.
	fn LogicalToPhysicalPoint(self, pt: *mut POINT) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user::ffi::LogicalToPhysicalPoint(self.as_ptr(), pt as _) },
		)
	}

	/// [`MapDialogRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapdialogrect)
	/// method.
	fn MapDialogRect(self, rc: &mut RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user::ffi::MapDialogRect(self.as_ptr(), rc as *mut _ as _) },
		)
	}

	/// [`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// method.
	///
	/// Consider using the more modern
	/// [`TaskDialog`](crate::prelude::ComctlOleHwnd::TaskDialog) method.
	///
	/// # Examples
	///
	/// A modal message box, which blocks its parent:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HWND};
	///
	/// let my_hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// hwnd.MessageBox("Hello, world", "title",
	///     co::MB::OKCANCEL | co::MB::ICONINFORMATION)?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	///
	/// Usually the message box has a valid parent window, however, if for some
	/// reason you don't have a window to serve as parent, you still can show a
	/// non-modal, parent-less message box by retrieving the desktop handle:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HWND};
	///
	/// HWND::GetDesktopWindow()
	///     .MessageBox("Hello, world", "Title", co::MB::ICONEXCLAMATION)?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn MessageBox(self,
		text: &str, caption: &str, flags: co::MB) -> WinResult<co::DLGID>
	{
		match unsafe {
			user::ffi::MessageBoxW(
				self.as_ptr(),
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
	fn MonitorFromWindow(self, flags: co::MONITOR) -> HMONITOR {
		HMONITOR(unsafe { user::ffi::MonitorFromWindow(self.as_ptr(), flags.0) })
	}

	/// [`MoveWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-movewindow)
	/// method.
	fn MoveWindow(self, pos: POINT, size: SIZE, repaint: bool) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user::ffi::MoveWindow(
					self.as_ptr(),
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
	fn OpenClipboard(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::OpenClipboard(self.as_ptr()) })
	}

	/// [`PostMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postmessagew)
	/// method. Note that this method is asychronous.
	///
	/// # Examples
	///
	/// Programatically closing a window:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HWND, msg::wm};
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// hwnd.PostMessage(wm::Close {})?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	///
	/// Sending a message to all top-level windows:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HWND, msg::wm};
	///
	/// HWND::BROADCAST.PostMessage(
	///     wm::ExitMenuLoop { is_shortcut: false },
	/// )?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn PostMessage<M>(self, msg: M) -> WinResult<()>
		where M: MsgSend,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		bool_to_winresult(
			unsafe {
				user::ffi::PostMessageW(
					self.as_ptr(), wm_any.msg_id.0, wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`RealChildWindowFromPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realchildwindowfrompoint)
	/// method.
	fn RealChildWindowFromPoint(self,
		pt_parent_client_coords: POINT) -> Option<HWND>
	{
		unsafe {
			user::ffi::RealChildWindowFromPoint(
				self.as_ptr(),
				pt_parent_client_coords.x,
				pt_parent_client_coords.y,
			).as_mut()
		}.map(|ptr| HWND(ptr))
	}

	/// [`RealGetWindowClassW`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realgetwindowclassw)
	/// method.
	fn RealGetWindowClass(self) -> WinResult<String> {
		let mut buf = WString::new_alloc_buffer(256 + 1); // according to WNDCLASSEX docs
		match unsafe {
			user::ffi::RealGetWindowClassW(
				self.as_ptr(),
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
	fn RedrawWindow(self,
		rc_update: &RECT, hrgn_update: HRGN, flags: co::RDW) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user::ffi::RedrawWindow(
					self.as_ptr(),
					rc_update as *const _ as _,
					hrgn_update.0,
					flags.0,
				)
			},
		)
	}

	/// [`ReleaseDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
	/// method.
	fn ReleaseDC(self, hdc: HDC) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user::ffi::ReleaseDC(self.as_ptr(), hdc.0) },
		)
	}

	/// [`ScreenToClient`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-screentoclient)
	/// method.
	fn ScreenToClient(self, pt: &mut POINT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user::ffi::ScreenToClient(self.as_ptr(), pt as *mut _ as _)
			},
		)
	}

	/// [`ScreenToClient`](crate::prelude::UserHwnd::ScreenToClient) method for
	/// a [`RECT`](crate::RECT).
	fn ScreenToClientRc(self, rc: &mut RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user::ffi::ScreenToClient(
					self.as_ptr(),
					&mut rc.left as *mut _ as _,
				)
			},
		).and_then(|_| bool_to_winresult(
			unsafe {
				user::ffi::ScreenToClient(
					self.as_ptr(),
					&mut rc.right as *mut _ as _,
				)
			},
		))
	}

	/// [`SendMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)
	/// method.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::prelude::MsgSend) trait. That means each
	/// message can define its own return type.
	///
	/// # Examples
	///
	/// Sending a [`lvm::SetItem`](crate::msg::lvm::SetItem) list view message,
	/// which demands a reference to an [`LVITEM`](crate::LVITEM) object:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HWND, LVITEM, msg::lvm};
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// let mut lvi = LVITEM::default(); // object to be sent
	/// lvi.mask = co::LVIF::IMAGE;
	/// lvi.iImage = 3;
	///
	/// hwnd.SendMessage(lvm::SetItem {
	///     lvitem: &lvi,
	/// });
	/// ```
	fn SendMessage<M>(self, msg: M) -> M::RetType
		where M: MsgSend,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		msg.convert_ret(
			unsafe {
				user::ffi::SendMessageW(
					self.as_ptr(), wm_any.msg_id.0, wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`SetCapture`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcapture)
	/// method.
	///
	/// **Note:** Must be paired with a
	/// [`ReleaseCapture`](crate::ReleaseCapture) call.
	fn SetCapture(&self) -> Option<HWND> {
		unsafe { user::ffi::SetCapture(self.as_ptr()).as_mut() }
			.map(|ptr| HWND(ptr))
	}

	/// [`SetFocus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setfocus)
	/// method.
	fn SetFocus(self) -> Option<HWND> {
		unsafe { user::ffi::SetFocus(self.as_ptr()).as_mut() }
			.map(|ptr| HWND(ptr))
	}

	/// [`SetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow)
	/// method.
	fn SetForegroundWindow(self) -> bool {
		unsafe { user::ffi::SetForegroundWindow(self.as_ptr()) != 0 }
	}

	/// [`SetMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenu)
	/// method.
	fn SetMenu(self, hmenu: HMENU) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::SetMenu(self.as_ptr(), hmenu.0) })
	}

	/// [`SetParent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setparent)
	/// method.
	fn SetParent(self, hwnd_new_parent: HWND) -> WinResult<Option<HWND>> {
		match unsafe {
			user::ffi::SetParent(self.as_ptr(), hwnd_new_parent.0).as_mut()
		} {
			Some(ptr) => Ok(Some(HWND(ptr))),
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no previous parent
				err => Err(err),
			},
		}
	}

	/// [`SetScrollInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollinfo)
	/// method.
	fn SetScrollInfo(self, bar: co::SBB, si: &SCROLLINFO, redraw: bool) -> i32 {
		unsafe {
			user::ffi::SetScrollInfo(
				self.as_ptr(),
				bar.0,
				si as *const _ as _,
				redraw as _,
			)
		}
	}

	/// [`SetScrollPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollpos)
	/// method.
	fn SetScrollPos(self, b: co::SBB, pos: i32, redraw: bool) -> WinResult<i32> {
		match unsafe {
			user::ffi::SetScrollPos(self.as_ptr(), b.0, pos, redraw as BOOL)
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
	fn SetScrollRange(self,
		bar: co::SBB, min_pos: i32, max_pos: i32, redraw: bool) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user::ffi::SetScrollRange(
					self.as_ptr(),
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
	fn SetTimer(self,
		event_id: usize, elapse_ms: u32,
		timer_func: Option<TIMERPROC>) -> WinResult<usize>
	{
		match unsafe {
			user::ffi::SetTimer(
				self.as_ptr(),
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
	fn SetWindowDisplayAffinity(self, affinity: co::WDA) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user::ffi::SetWindowDisplayAffinity(self.as_ptr(), affinity.0)
			},
		)
	}

	/// [`SetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
	/// method.
	fn SetWindowLongPtr(self, index: co::GWLP, new_long: isize) -> isize {
		unsafe { user::ffi::SetWindowLongPtrW(self.as_ptr(), index.0, new_long) }
	}

	/// [`SetWindowPlacement`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowplacement)
	/// method.
	fn SetWindowPlacement(self, wp: &WINDOWPLACEMENT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user::ffi::SetWindowPlacement(self.as_ptr(), wp as *const _ as _)
			},
		)
	}

	/// [`SetWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)
	/// method.
	fn SetWindowPos(self,
		hwnd_insert_after: HwndPlace,
		pos: POINT, size: SIZE, flags: co::SWP) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user::ffi::SetWindowPos(
					self.as_ptr(),
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
	fn SetWindowRgn(self, hrgn: HRGN, redraw: bool) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user::ffi::SetWindowRgn(self.as_ptr(), hrgn.0, redraw as _) },
		)
	}

	/// [`SetWindowText`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
	/// method.
	fn SetWindowText(self, text: &str) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user::ffi::SetWindowTextW(
					self.as_ptr(),
					WString::from_str(text).as_ptr(),
				)
			},
		)
	}

	/// [`ShowCaret`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcaret)
	/// method.
	fn ShowCaret(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::ShowCaret(self.as_ptr()) })
	}

	/// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	/// method.
	fn ShowWindow(self, show_cmd: co::SW) -> bool {
		unsafe { user::ffi::ShowWindow(self.as_ptr(), show_cmd.0) != 0 }
	}

	/// [`TranslateAccelerator`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translateacceleratorw)
	/// method.
	fn TranslateAccelerator(self,
		haccel_table: HACCEL, msg: &mut MSG) -> WinResult<()>
	{
		match unsafe {
			user::ffi::TranslateAcceleratorW(
				self.as_ptr(),
				haccel_table.0,
				msg as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`UpdateWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatewindow)
	/// method.
	fn UpdateWindow(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user::ffi::UpdateWindow(self.as_ptr()) })
	}

	/// [`ValidateRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validaterect)
	/// method.
	fn ValidateRect(self, rc: &RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user::ffi::ValidateRect(self.as_ptr(), rc as *const _ as _)
			},
		)
	}

	/// [`ValidateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validatergn)
	/// method.
	fn ValidateRgn(self, hrgn: HRGN) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user::ffi::ValidateRgn(self.as_ptr(), hrgn.0) },
		)
	}

	/// [`WindowFromPhysicalPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfromphysicalpoint)
	/// static method.
	fn WindowFromPhysicalPoint(pt: POINT) -> Option<HWND> {
		unsafe { user::ffi::WindowFromPhysicalPoint(pt.x, pt.y).as_mut() }
			.map(|ptr| HWND(ptr))
	}

	/// [`WindowFromPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfrompoint)
	/// static method.
	fn WindowFromPoint(pt: POINT) -> Option<HWND> {
		unsafe { user::ffi::WindowFromPoint(pt.x, pt.y).as_mut() }
			.map(|ptr| HWND(ptr))
	}

	/// [`WinHelp`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-winhelpw)
	/// method.
	fn WinHelp(self,
		help_file: &str, cmd: co::HELPW, data: usize) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user::ffi::WinHelpW(
					self.as_ptr(),
					WString::from_str(help_file).as_ptr(),
					cmd.0,
					data,
				)
			},
		)
	}
}

extern "system" fn enum_child_windows_proc<F>(
	hwnd: HWND, lparam: isize) -> BOOL
	where F: Fn(HWND) -> bool,
{
	let func = unsafe { &*(lparam as *const F) };
	func(hwnd) as _
}
