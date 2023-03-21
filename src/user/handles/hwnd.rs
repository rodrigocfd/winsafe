#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::{co, user};
use crate::kernel::decl::{
	GetLastError, HINSTANCE, HIWORD, LOWORD, SetLastError, SysResult, WString,
};
use crate::kernel::ffi_types::BOOL;
use crate::kernel::privs::{
	bool_to_sysresult, MAX_PATH, ptr_to_option_handle, ptr_to_sysresult_handle,
};
use crate::prelude::{Handle, MsgSend};
use crate::user::decl::{
	ALTTABINFO, AtomStr, HACCEL, HMENU, HMONITOR, HRGN, HwndPlace, IdMenu,
	IdPos, MENUBARINFO, MSG, PAINTSTRUCT, POINT, RECT, SCROLLINFO, SIZE,
	TIMERPROC, WINDOWINFO, WINDOWPLACEMENT,
};
use crate::user::guard::{
	CloseClipboardGuard, EndPaintGuard, ReleaseCaptureGuard, ReleaseDCGuard,
};
use crate::user::privs::zero_as_none;

impl_handle! { HWND;
	/// Handle to a
	/// [window](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
}

impl user_Hwnd for HWND {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hwnd: Handle {
	/// Represents all top-level windows in
	/// [`HWND::PostMessage`](crate::prelude::user_Hwnd::PostMessage) and
	/// [`HWND::SendMessage`](crate::prelude::user_Hwnd::SendMessage).
	const BROADCAST: HWND = HWND(0xffff as _);

	/// Represents the desktop window in
	/// [`HWND::GetDC`](crate::prelude::user_Hwnd::GetDC).
	const DESKTOP: HWND = HWND(std::ptr::null_mut());

	/// [`GetWindowLongPtr`](crate::prelude::user_Hwnd::GetWindowLongPtr)
	/// wrapper to retrieve the window [`HINSTANCE`](crate::HINSTANCE).
	#[must_use]
	fn hinstance(&self) -> HINSTANCE {
		unsafe {
			HINSTANCE::from_ptr(self.GetWindowLongPtr(co::GWLP::HINSTANCE) as _)
		}
	}

	/// [`ArrangeIconicWindows`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-arrangeiconicwindows)
	/// method.
	fn ArrangeIconicWindows(&self) -> SysResult<u32> {
		match unsafe { user::ffi::ArrangeIconicWindows(self.as_ptr()) } {
			0 => Err(GetLastError()),
			height => Ok(height),
		}
	}

	/// [`BeginPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
	/// method.
	///
	/// In the original C implementation, `BeginPaint` returns a handle which
	/// must be passed to
	/// [`EndPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint),
	/// as a cleanup operation. Also, you must allocate and pass a
	/// [`PAINTSTRUCT`](crate::PAINTSTRUCT) object.
	///
	/// Here, the cleanup is performed automatically, because `BeginPaint`
	/// returns an [`EndPaintGuard`](crate::guard::EndPaintGuard), which stores
	/// the `PAINTSTRUCT` and automatically calls `EndPaint` when the guard goes
	/// out of scope. You must, however, keep the guard alive, otherwise the
	/// cleanup will be performed right away.
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
	/// let hdc = hwnd.BeginPaint()?;
	///
	/// // hdc painting...
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	///
	/// If you don't use the returned device context handle, you must still keep
	/// the guard alive:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::HWND;
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// let _hdc = hwnd.BeginPaint()?; // keep guard alive
	///
	/// // hdc painting...
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn BeginPaint(&self) -> SysResult<EndPaintGuard<'_, Self>> {
		let mut ps = PAINTSTRUCT::default();
		unsafe {
			ptr_to_sysresult_handle(
				user::ffi::BeginPaint(self.as_ptr(), &mut ps as *mut _ as _),
			).map(|h| EndPaintGuard::new(self, h, ps))
		}
	}

	/// [`BringWindowToTop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-bringwindowtotop)
	/// method.
	fn BringWindowToTop(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { user::ffi::BringWindowToTop(self.as_ptr()) })
	}

	/// [`ChildWindowFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-childwindowfrompoint)
	/// method.
	#[must_use]
	fn ChildWindowFromPoint(&self, pt: POINT) -> Option<HWND> {
		ptr_to_option_handle(
			unsafe { user::ffi::ChildWindowFromPoint(self.as_ptr(), pt.x, pt.y) },
		)
	}

	/// [`ClientToScreen`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clienttoscreen)
	/// method.
	///
	/// If you need to convert a [`RECT`](crate::RECT), see the
	/// [`HWND::ClientToScreenRc`](crate::prelude::user_Hwnd::ClientToScreenRc)
	/// method.
	fn ClientToScreen(&self, pt: &mut POINT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::ClientToScreen(self.as_ptr(), pt as *mut _ as _) },
		)
	}

	/// [`ClientToScreen`](crate::prelude::user_Hwnd::ClientToScreen) method for
	/// a [`RECT`](crate::RECT).
	fn ClientToScreenRc(&self, rc: &mut RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::ClientToScreen(
					self.as_ptr(),
					&mut rc.left as *mut _ as _,
				)
			},
		).and_then(|_| bool_to_sysresult(
			unsafe {
				user::ffi::ClientToScreen(
					self.as_ptr(),
					&mut rc.right as *mut _ as _,
				)
			},
		))
	}

	/// [`CloseWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindow)
	/// method.
	///
	/// Note that this method will actually minimize the window, not destroy it.
	fn CloseWindow(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { user::ffi::CloseWindow(self.as_ptr()) })
	}

	/// [`CreateWindowEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
	/// static method.
	///
	/// # Safety
	///
	/// This method will create raw dynamic windows and controls outside the
	/// library safety – it's up to you to handle all the messages. You must use
	/// a properly registered class name and, if creating a custom window,
	/// provide its own window procedure.
	///
	/// The usable ID range for dynamic child controls goes from 1 to 19,999.
	/// IDs starting from 20,000 are used internally by the library, do not use
	/// them.
	unsafe fn CreateWindowEx(
		ex_style: co::WS_EX,
		class_name: AtomStr,
		title: Option<&str>,
		style: co::WS,
		pos: POINT,
		size: SIZE,
		hwnd_parent: Option<&HWND>,
		hmenu: IdMenu,
		hinstance: &HINSTANCE,
		lparam: Option<isize>,
	) -> SysResult<HWND>
	{
		ptr_to_sysresult_handle(
			user::ffi::CreateWindowExW(
				ex_style.0,
				class_name.as_ptr(),
				WString::from_opt_str(title).as_ptr(),
				style.0,
				pos.x, pos.y,
				size.cx, size.cy,
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.as_ptr()),
				hmenu.as_ptr(),
				hinstance.as_ptr(),
				lparam.unwrap_or_default() as _,
			),
		)
	}

	/// [`DefWindowProc`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
	/// method.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::prelude::MsgSend) trait. That means each
	/// message can define its own return type.
	fn DefWindowProc<M>(&self, msg: M) -> M::RetType
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

	/// [`DestroyWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
	/// method.
	///
	/// Usually you don't need to call this method directly, since it's
	/// automatically called inside the internal message loop. The ordinary way
	/// to close a window is sending a [`wm::Close`](crate::msg::wm::Close)
	/// message.
	fn DestroyWindow(&self) -> SysResult<()> {
		bool_to_sysresult( unsafe { user::ffi::DestroyWindow(self.as_ptr()) })
	}

	/// [`DrawMenuBar`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawmenubar)
	/// method.
	fn DrawMenuBar(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { user::ffi::DrawMenuBar(self.as_ptr()) })
	}

	/// [`EnableScrollBar`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablescrollbar)
	/// method.
	fn EnableScrollBar(&self,
		sb_flags: co::SBB, arrows: co::ESB) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				user::ffi::EnableScrollBar(self.as_ptr(), sb_flags.0 as _, arrows.0)
			},
		)
	}

	/// [`EnableWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablewindow)
	/// method.
	fn EnableWindow(&self, enable: bool) -> bool {
		unsafe { user::ffi::EnableWindow(self.as_ptr(), enable as _) != 0 }
	}

	/// [`EndDialog`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddialog)
	/// method.
	fn EndDialog(&self, result: isize) -> SysResult<()> {
		bool_to_sysresult(unsafe { user::ffi::EndDialog(self.as_ptr(), result) })
	}

	/// [`EnumChildWindows`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumchildwindows)
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
	fn EnumChildWindows<F>(&self, func: F)
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

	/// [`FindWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindoww)
	/// static method.
	#[must_use]
	fn FindWindow(
		class_name: Option<AtomStr>, title: Option<&str>) -> SysResult<HWND>
	{
		ptr_to_sysresult_handle(
			unsafe {
				user::ffi::FindWindowW(
					class_name.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
					WString::from_opt_str(title).as_ptr(),
				)
			},
		)
	}

	/// [`FindWindowEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindowexw)
	/// method.
	#[must_use]
	fn FindWindowEx(&self,
		hwnd_child_after: Option<&HWND>,
		class_name: AtomStr,
		title: Option<&str>,
	) -> SysResult<HWND>
	{
		ptr_to_sysresult_handle(
			unsafe {
				user::ffi::FindWindowExW(
					self.as_ptr(),
					hwnd_child_after.map_or(std::ptr::null_mut(), |h| h.as_ptr()),
					class_name.as_ptr(),
					WString::from_opt_str(title).as_ptr(),
				)
			},
		)
	}

	/// [`GetActiveWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getactivewindow)
	/// static method.
	#[must_use]
	fn GetActiveWindow() -> Option<HWND> {
		ptr_to_option_handle(
			unsafe { user::ffi::GetActiveWindow() },
		)
	}

	/// [`GetAltTabInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getalttabinfow)
	/// method.
	///
	/// If `item` is `None`, the item text is not retrieved.
	///
	/// The `sz_item_text` is the maximum number of expected chars for the item
	/// text. If `None`, defaults to 100.
	fn GetAltTabInfo(&self,
		item: Option<u32>,
		ati: &mut ALTTABINFO,
		sz_item_text: Option<u32>,
	) -> SysResult<String>
	{
		let buf_sz = sz_item_text.unwrap_or(100) + 1;
		let mut buf = match item {
			None => WString::default(),
			Some(_) => WString::new_alloc_buf(buf_sz as _), // room for terminating null
		};

		bool_to_sysresult(
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

	/// [`GetAncestor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getancestor)
	/// method.
	#[must_use]
	fn GetAncestor(&self, flags: co::GA) -> Option<HWND> {
		ptr_to_option_handle(
			unsafe { user::ffi::GetAncestor(self.as_ptr(), flags.0) },
		)
	}

	/// [`GetCapture`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcapture)
	/// static method.
	#[must_use]
	fn GetCapture() -> Option<HWND> {
		ptr_to_option_handle(unsafe { user::ffi::GetCapture() })
	}

	/// [`GetClassLongPtr`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclasslongptrw)
	/// method.
	#[must_use]
	fn GetClassLongPtr(&self, index: co::GCLP) -> usize {
		unsafe { user::ffi::GetClassLongPtrW(self.as_ptr(), index.0) }
	}

	/// [`GetClassName`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclassnamew)
	/// method.
	#[must_use]
	fn GetClassName(&self) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(256 + 1); // according to WNDCLASSEX docs
		match unsafe {
			user::ffi::GetClassNameW(
				self.as_ptr(),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(buf.to_string()),
		}
	}

	/// [`GetClientRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)
	/// method.
	#[must_use]
	fn GetClientRect(&self) -> SysResult<RECT> {
		let mut rc = RECT::default();
		match unsafe {
			user::ffi::GetClientRect(self.as_ptr(), &mut rc as *mut _ as _)
		} {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
	/// method.
	///
	/// To get the device context of the desktop window, use the predefined
	/// [`HWND::DESKTOP`](crate::prelude::user_Hwnd::DESKTOP).
	#[must_use]
	fn GetDC(&self) -> SysResult<ReleaseDCGuard<'_, Self>> {
		unsafe {
			ptr_to_sysresult_handle(user::ffi::GetDC(self.as_ptr()))
				.map(|h| ReleaseDCGuard::new(self, h))
		}
	}

	/// [`GetDesktopWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow)
	/// static method.
	#[must_use]
	fn GetDesktopWindow() -> HWND {
		unsafe { HWND::from_ptr(user::ffi::GetDesktopWindow()) }
	}

	/// [`GetDlgCtrlID`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgctrlid)
	/// method.
	#[must_use]
	fn GetDlgCtrlID(&self) -> SysResult<u16> {
		SetLastError(co::ERROR::SUCCESS);
		match unsafe { user::ffi::GetDlgCtrlID(self.as_ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual ID is zero
				err => Err(err),
			},
			id => Ok(id as _),
		}
	}

	/// [`GetDlgItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgitem)
	/// method.
	#[must_use]
	fn GetDlgItem(&self, ctrl_id: u16) -> SysResult<HWND> {
		ptr_to_sysresult_handle(
			unsafe { user::ffi::GetDlgItem(self.as_ptr(), ctrl_id as _) },
		)
	}

	/// [`GetFocus`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getfocus)
	/// static method.
	#[must_use]
	fn GetFocus() -> Option<HWND> {
		ptr_to_option_handle(unsafe { user::ffi::GetFocus() })
	}

	/// [`GetForegroundWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
	/// static method.
	#[must_use]
	fn GetForegroundWindow() -> Option<HWND> {
		ptr_to_option_handle(unsafe { user::ffi::GetForegroundWindow() })
	}

	/// [`GetLastActivePopup`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getlastactivepopup)
	/// method.
	#[must_use]
	fn GetLastActivePopup(&self) -> Option<HWND> {
		ptr_to_option_handle(
			unsafe { user::ffi::GetLastActivePopup(self.as_ptr()) },
		)
	}

	/// [`GetMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenu)
	/// method.
	#[must_use]
	fn GetMenu(&self) -> Option<HMENU> {
		ptr_to_option_handle(unsafe { user::ffi::GetMenu(self.as_ptr()) })
	}

	/// [`GetMenuBarInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenubarinfo)
	/// method.
	fn GetMenuBarInfo(&self,
		obj_id: co::OBJID, item_id: u32, mbi: &mut MENUBARINFO) -> SysResult<()>
	{
		bool_to_sysresult(
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

	/// [`GetMenuItemRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemrect)
	/// method.
	fn GetMenuItemRect(&self,
		hmenu: &HMENU, item_pos: u32, rc_item: &mut RECT) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				user::ffi::GetMenuItemRect(
					self.as_ptr(),
					hmenu.as_ptr(),
					item_pos,
					rc_item as *mut _ as _,
				)
			},
		)
	}

	/// [`GetNextDlgGroupItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlggroupitem)
	/// method.
	#[must_use]
	fn GetNextDlgGroupItem(&self,
		hwnd_ctrl: &HWND, previous: bool) -> SysResult<HWND>
	{
		ptr_to_sysresult_handle(
			unsafe {
				user::ffi::GetNextDlgGroupItem(
					self.as_ptr(),
					hwnd_ctrl.as_ptr(), previous as _,
				)
			},
		)
	}

	/// [`GetNextDlgTabItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlgtabitem)
	/// method.
	#[must_use]
	fn GetNextDlgTabItem(&self,
		hwnd_ctrl: &HWND, previous: bool) -> SysResult<HWND>
	{
		ptr_to_sysresult_handle(
			unsafe {
				user::ffi::GetNextDlgTabItem(
					self.as_ptr(),
					hwnd_ctrl.as_ptr(),
					previous as _,
				)
			},
		)
	}

	/// [`GetParent`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getparent)
	/// method.
	#[must_use]
	fn GetParent(&self) -> SysResult<HWND> {
		ptr_to_sysresult_handle(unsafe { user::ffi::GetParent(self.as_ptr()) })
	}

	/// [`GetScrollInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getscrollinfo)
	/// method.
	fn GetScrollInfo(&self,
		bar: co::SBB, si: &mut SCROLLINFO) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				user::ffi::GetScrollInfo(self.as_ptr(), bar.0, si as *mut _ as _)
			},
		)
	}

	/// [`GetScrollPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getscrollpos)
	/// method.
	#[must_use]
	fn GetScrollPos(&self, bar: co::SBB) -> SysResult<i32> {
		match unsafe { user::ffi::GetScrollPos(self.as_ptr(), bar.0) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero position
				err => Err(err),
			},
			pos => Ok(pos),
		}
	}

	/// [`GetShellWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getshellwindow)
	/// static method.
	#[must_use]
	fn GetShellWindow() -> Option<HWND> {
		ptr_to_option_handle(unsafe { user::ffi::GetShellWindow() })
	}

	/// [`GetSystemMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmenu)
	/// method.
	#[must_use]
	fn GetSystemMenu(&self, revert: bool) -> Option<HMENU> {
		ptr_to_option_handle(
			unsafe { user::ffi::GetSystemMenu(self.as_ptr(), revert as _) },
		)
	}

	/// [`GetTopWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-gettopwindow)
	/// method.
	#[must_use]
	fn GetTopWindow(&self) -> SysResult<Option<HWND>> {
		match ptr_to_option_handle(
			unsafe { user::ffi::GetTopWindow(self.as_ptr()) },
		) {
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no child window
				err => Err(err),
			},
			Some(h) => Ok(Some(h)),
		}
	}

	/// [`GetUpdateRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getupdaterect)
	/// method.
	#[must_use]
	fn GetUpdateRect(&self, erase: bool) -> Option<RECT> {
		let mut rc = RECT::default();
		zero_as_none(
			unsafe {
				user::ffi::GetUpdateRect(
					self.as_ptr(),
					&mut rc as *mut _ as _,
					erase as _,
				)
			} as _,
		).map(|_| rc)
	}

	/// [`GetUpdateRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getupdatergn)
	/// method.
	#[must_use]
	fn GetUpdateRgn(&self, hrgn: &HRGN, erase: bool) -> SysResult<co::REGION> {
		match unsafe {
			user::ffi::GetUpdateRgn(self.as_ptr(), hrgn.as_ptr(), erase as _) }
		{
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	/// method.
	#[must_use]
	fn GetWindow(&self, cmd: co::GW) -> SysResult<HWND> {
		ptr_to_sysresult_handle(
			unsafe { user::ffi::GetWindow(self.as_ptr(), cmd.0) },
		)
	}

	/// [`GetWindowDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdc)
	/// method.
	#[must_use]
	fn GetWindowDC(&self) -> SysResult<ReleaseDCGuard<'_, Self>> {
		unsafe {
			ptr_to_sysresult_handle(user::ffi::GetWindowDC(self.as_ptr()))
				.map(|h| ReleaseDCGuard::new(self, h))
		}
	}

	/// [`GetWindowDisplayAffinity`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdisplayaffinity)
	/// method.
	#[must_use]
	fn GetWindowDisplayAffinity(&self) -> SysResult<co::WDA> {
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

	/// [`GetWindowInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowinfo)
	/// method.
	fn GetWindowInfo(&self, wi: &mut WINDOWINFO) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::GetWindowInfo(self.as_ptr(), wi as *mut _ as _) },
		)
	}

	/// [`GetWindowLongPtr`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
	/// method.
	#[must_use]
	fn GetWindowLongPtr(&self, index: co::GWLP) -> isize {
		#[cfg(target_pointer_width = "32")]
		unsafe { user::ffi::GetWindowLongW(self.as_ptr(), index.0) }

		#[cfg(target_pointer_width = "64")]
		unsafe { user::ffi::GetWindowLongPtrW(self.as_ptr(), index.0) }
	}

	/// [`GetWindowModuleFileName`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowmodulefilenamew)
	/// method.
	#[must_use]
	fn GetWindowModuleFileName(&self) -> String {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		unsafe {
			user::ffi::GetWindowModuleFileNameW(
				self.as_ptr(),
				buf.as_mut_ptr(),
				buf.buf_len() as u32 - 1,
			);
		}
		buf.to_string()
	}

	/// [`GetWindowPlacement`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowplacement)
	/// method.
	fn GetWindowPlacement(&self, wp: &mut WINDOWPLACEMENT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::GetWindowPlacement(self.as_ptr(), wp as *mut _ as _)
			},
		)
	}

	/// [`GetWindowRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)
	/// method.
	#[must_use]
	fn GetWindowRect(&self) -> SysResult<RECT> {
		let mut rc = RECT::default();
		match unsafe {
			user::ffi::GetWindowRect(self.as_ptr(), &mut rc as *mut _ as _)
		} {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetWindowRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgn)
	/// method.
	#[must_use]
	fn GetWindowRgn(&self, hrgn: &HRGN) -> SysResult<co::REGION> {
		match unsafe { user::ffi::GetWindowRgn(self.as_ptr(), hrgn.as_ptr()) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindowRgnBox`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgnbox)
	/// method.
	#[must_use]
	fn GetWindowRgnBox(&self, rc: &mut RECT) -> SysResult<co::REGION> {
		match unsafe {
			user::ffi::GetWindowRgnBox(self.as_ptr(), rc as *mut _ as _)
		} {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindowText`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw)
	/// method.
	///
	/// Returns a
	/// [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
	/// performing all necessary allocations.
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
	#[must_use]
	fn GetWindowText(&self) -> SysResult<String> {
		let len = self.GetWindowTextLength()?;
		if len == 0 {
			return Ok(String::default()); // window has no text
		}

		let mut buf = WString::new_alloc_buf(len as usize + 1); // plus terminating null
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

	/// [`GetWindowTextLength`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextlengthw)
	/// method. Does not count the terminating null.
	///
	/// You should rarely use this method since
	/// [`GetWindowText`](crate::prelude::user_Hwnd::GetWindowText) returns a
	/// [`String`](https://doc.rust-lang.org/std/string/struct.String.html),
	/// performing all necessary allocations.
	#[must_use]
	fn GetWindowTextLength(&self) -> SysResult<i32> {
		SetLastError(co::ERROR::SUCCESS);
		match unsafe { user::ffi::GetWindowTextLengthW(self.as_ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero length
				err => Err(err),
			},
			len => Ok(len),
		}
	}

	/// [`GetWindowThreadProcessId`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid)
	/// method.
	///
	/// Returns thread ID and process ID, respectively.
	#[must_use]
	fn GetWindowThreadProcessId(&self) -> (u32, u32) {
		let mut proc_id = u32::default();
		let thread_id = unsafe {
			user::ffi::GetWindowThreadProcessId(self.as_ptr(), &mut proc_id)
		};
		(thread_id, proc_id)
	}

	/// [`HiliteMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-hilitemenuitem)
	/// method.
	fn HiliteMenuItem(&self,
		hmenu: &HMENU, id_or_pos: IdPos, hilite: co::MF) -> bool
	{
		unsafe {
			user::ffi::HiliteMenuItem(
				self.as_ptr(), hmenu.as_ptr(), id_or_pos.id_or_pos_u32(), hilite.0,
			) != 0
		}
	}

	/// [`InvalidateRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidaterect)
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
	fn InvalidateRect(&self, rc: Option<&RECT>, erase: bool) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::InvalidateRect(
					self.as_ptr(),
					rc.map_or(std::ptr::null(), |lp| lp as *const _ as _),
					erase as _,
				)
			},
		)
	}

	/// [`InvalidateRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidatergn)
	/// method.
	fn InvalidateRgn(&self, hrgn: &HRGN, erase: bool) {
		unsafe {
			user::ffi::InvalidateRgn(self.as_ptr(), hrgn.as_ptr(), erase as _);
		}
	}

	/// [`IsChild`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ischild)
	/// method.
	#[must_use]
	fn IsChild(&self, hwnd_possible_child: &HWND) -> bool {
		unsafe {
			user::ffi::IsChild(self.as_ptr(), hwnd_possible_child.as_ptr()) != 0
		}
	}

	/// [`IsDialogMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isdialogmessagew)
	/// method.
	#[must_use]
	fn IsDialogMessage(&self, msg: &mut MSG) -> bool {
		unsafe {
			user::ffi::IsDialogMessageW(self.as_ptr(), msg as *mut _ as _) != 0
		}
	}

	/// [`IsIconic`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isiconic)
	/// method.
	#[must_use]
	fn IsIconic(&self) -> bool {
		unsafe { user::ffi::IsIconic(self.as_ptr()) != 0 }
	}

	/// [`IsWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)
	/// method.
	#[must_use]
	fn IsWindow(&self) -> bool {
		unsafe { user::ffi::IsWindow(self.as_ptr()) != 0 }
	}

	/// [`IsWindowEnabled`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowenabled)
	/// method.
	#[must_use]
	fn IsWindowEnabled(&self) -> bool {
		unsafe { user::ffi::IsWindowEnabled(self.as_ptr()) != 0 }
	}

	/// [`IsWindowUnicode`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowunicode)
	/// method.
	#[must_use]
	fn IsWindowUnicode(&self) -> bool {
		unsafe { user::ffi::IsWindowUnicode(self.as_ptr()) != 0 }
	}

	/// [`IsWindowVisible`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowvisible)
	/// method.
	#[must_use]
	fn IsWindowVisible(&self) -> bool {
		unsafe { user::ffi::IsWindowVisible(self.as_ptr()) != 0 }
	}

	/// [`IsZoomed`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iszoomed)
	/// method.
	#[must_use]
	fn IsZoomed(&self) -> bool {
		unsafe { user::ffi::IsZoomed(self.as_ptr()) != 0 }
	}

	/// [`KillTimer`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-killtimer)
	/// method.
	fn KillTimer(&self, event_id: usize) -> SysResult<()> {
		match unsafe { user::ffi::KillTimer(self.as_ptr(), event_id) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(()),
				e => Err(e),
			}
			_ => Ok(()),
		}
	}

	/// [`LockWindowUpdate`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-lockwindowupdate)
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
	/// // Lock the window – only one window can be locked at a time.
	/// hwnd.LockWindowUpdate()?;
	///
	/// // After all operations, unlock the currently locked window.
	/// HWND::NULL.LockWindowUpdate()?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn LockWindowUpdate(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { user::ffi::LockWindowUpdate(self.as_ptr()) })
	}

	/// [`LogicalToPhysicalPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-logicaltophysicalpoint)
	/// method.
	fn LogicalToPhysicalPoint(&self, pt: *mut POINT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::LogicalToPhysicalPoint(self.as_ptr(), pt as _) },
		)
	}

	/// [`MapDialogRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapdialogrect)
	/// method.
	fn MapDialogRect(&self, rc: &mut RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::MapDialogRect(self.as_ptr(), rc as *mut _ as _) },
		)
	}

	/// [`MapWindowPoints`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapwindowpoints)
	/// method.
	fn MapWindowPoints(&self,
		hdest: &HWND, points: &mut [POINT]) -> SysResult<(i16, i16)>
	{
		SetLastError(co::ERROR::SUCCESS);
		match unsafe {
			user::ffi::MapWindowPoints(
				self.as_ptr(),
				hdest.as_ptr(),
				points.as_mut_ptr() as _,
				points.len() as _,
			)
		} {
			0 => Err(GetLastError()),
			n => Ok((LOWORD(n as _) as _, HIWORD(n as _) as _)),
		}
	}

	/// [`MessageBox`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// method.
	///
	/// Consider using the more modern
	/// [`TaskDialog`](crate::prelude::comctl_ole_Hwnd::TaskDialog) method.
	///
	/// # Examples
	///
	/// A modal message box, which blocks its parent:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HWND};
	///
	/// let hwnd: HWND; // initialized somewhere
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
	fn MessageBox(&self,
		text: &str, caption: &str, flags: co::MB) -> SysResult<co::DLGID>
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

	/// [`MonitorFromWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromwindow)
	/// method.
	#[must_use]
	fn MonitorFromWindow(&self, flags: co::MONITOR) -> HMONITOR {
		unsafe {
			HMONITOR::from_ptr(
				user::ffi::MonitorFromWindow(self.as_ptr(), flags.0),
			)
		}
	}

	/// [`MoveWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-movewindow)
	/// method.
	fn MoveWindow(&self,
		pos: POINT, size: SIZE, repaint: bool) -> SysResult<()>
	{
		bool_to_sysresult(
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

	/// [`OpenClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openclipboard)
	/// method.
	///
	/// In the original C implementation, you must call
	/// [`CloseClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closeclipboard)
	/// as a cleanup operation.
	///
	/// Here, the cleanup is performed automatically, because `OpenClipboard`
	/// returns a [`CloseClipboardGuard`](crate::guard::CloseClipboardGuard),
	/// which automatically calls `CloseClipboard` when the guard goes out of
	/// scope. You must, however, keep the guard alive, otherwise the cleanup
	/// will be performed right away.
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
	/// let _hclip = hwnd.OpenClipboard()?; // keep guard alive
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	///
	/// You can also open the clipboard without an `HWND` owner:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::HWND;
	///
	/// let _hclip = HWND::NULL.OpenClipboard()?; // keep guard alive
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn OpenClipboard(&self) -> SysResult<CloseClipboardGuard<'_>> {
		unsafe {
			bool_to_sysresult(user::ffi::OpenClipboard(self.as_ptr()))
				.map(|_| CloseClipboardGuard::new(PhantomData))
		}
	}

	/// [`PostMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postmessagew)
	/// method. Note that this method is asychronous.
	fn PostMessage<M>(&self, msg: M) -> SysResult<()>
		where M: MsgSend + Send + Copy + 'static,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		bool_to_sysresult(
			unsafe {
				user::ffi::PostMessageW(
					self.as_ptr(), wm_any.msg_id.0, wm_any.wparam, wm_any.lparam,
				)
			},
		)
	}

	/// [`RealChildWindowFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realchildwindowfrompoint)
	/// method.
	#[must_use]
	fn RealChildWindowFromPoint(&self,
		pt_parent_client_coords: POINT) -> Option<HWND>
	{
		ptr_to_option_handle(
			unsafe {
				user::ffi::RealChildWindowFromPoint(
					self.as_ptr(),
					pt_parent_client_coords.x,
					pt_parent_client_coords.y,
				)
			},
		)
	}

	/// [`RealGetWindowClassW`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realgetwindowclassw)
	/// method.
	#[must_use]
	fn RealGetWindowClass(&self) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(256 + 1); // according to WNDCLASSEX docs
		match unsafe {
			user::ffi::RealGetWindowClassW(
				self.as_ptr(),
				buf.as_mut_ptr(),
				buf.buf_len() as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(buf.to_string()),
		}
	}

	/// [`RedrawWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow)
	/// method.
	fn RedrawWindow(&self,
		rc_update: &RECT, hrgn_update: &HRGN, flags: co::RDW) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				user::ffi::RedrawWindow(
					self.as_ptr(),
					rc_update as *const _ as _,
					hrgn_update.as_ptr(),
					flags.0,
				)
			},
		)
	}

	/// [`ScreenToClient`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-screentoclient)
	/// method.
	///
	/// If you need to convert a [`RECT`](crate::RECT), see the
	/// [`HWND::ScreenToClientRc`](crate::prelude::user_Hwnd::ScreenToClientRc)
	/// method.
	fn ScreenToClient(&self, pt: &mut POINT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::ScreenToClient(self.as_ptr(), pt as *mut _ as _)
			},
		)
	}

	/// [`ScreenToClient`](crate::prelude::user_Hwnd::ScreenToClient) method for
	/// a [`RECT`](crate::RECT).
	fn ScreenToClientRc(&self, rc: &mut RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::ScreenToClient(
					self.as_ptr(),
					&mut rc.left as *mut _ as _,
				)
			},
		)?;
		bool_to_sysresult(
			unsafe {
				user::ffi::ScreenToClient(
					self.as_ptr(),
					&mut rc.right as *mut _ as _,
				)
			},
		)
	}

	/// [`ScrollWindowEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-scrollwindowex)
	/// method.
	fn ScrollWindowEx(&self,
		dx: i32,
		dy: i32,
		client_area_portion: Option<&RECT>,
		clipping_rect: Option<&RECT>,
		hrgn_update: Option<&HRGN>,
		updated_boundaries: Option<&mut RECT>,
		flags: co::SCROLLW,
	) -> SysResult<co::REGION>
	{
		match unsafe {
			user::ffi::ScrollWindowEx(
				self.as_ptr(),
				dx, dy,
				client_area_portion.map_or(std::ptr::null(), |rc| rc as *const _ as _),
				clipping_rect.map_or(std::ptr::null(), |rc| rc as *const _ as _),
				hrgn_update.map_or(std::ptr::null_mut(), |h| h.as_ptr()),
				updated_boundaries.map_or(std::ptr::null_mut(), |rc| rc as *mut _ as _),
				flags.0
			)
		} {
			0 => Err(GetLastError()),
			v => Ok(co::REGION(v)),
		}
	}

	/// [`SendMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)
	/// method.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::prelude::MsgSend) trait. That means each
	/// message can define its own return type.
	///
	/// # Examples
	///
	/// Sending a [`bm::GetImage`](crate::msg::bm::GetImage) button message,
	/// which demands an image type parameter. Note that this specific message
	/// can also return an error, which is handled with `?`:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HWND, msg::bm};
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// let bmp = hwnd.SendMessage(bm::GetImage {
	///     img_type: co::IMAGE_TYPE::BITMAP,
	/// })?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	///
	/// Sending an [`em::CharFromPos`](crate::msg::em::CharFromPos) edit message,
	/// which receives point coordinates and returns two values:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HWND, msg::em, POINT};
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// let (char_pos, line_pos) = hwnd.SendMessage(
	///     em::CharFromPos {
	///         coords: POINT::new(12, 20),
	///     },
	/// );
	/// ```
	fn SendMessage<M>(&self, msg: M) -> M::RetType
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

	/// [`SendMessageTimeout`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeoutw)
	/// method.
	fn SendMessageTimeout<M>(&self,
		msg: M,
		flags: co::SMTO,
		timeout_ms: u32,
	) -> SysResult<M::RetType>
		where M: MsgSend,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		let mut result = isize::default();

		bool_to_sysresult(
			unsafe {
				user::ffi::SendMessageTimeoutW(
					self.as_ptr(),
					wm_any.msg_id.0,
					wm_any.wparam,
					wm_any.lparam,
					flags.0,
					timeout_ms,
					&mut result,
				)
			} as _,
		).map(|_| msg.convert_ret(result))
	}

	/// [`SetActiveWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setactivewindow)
	/// method.
	fn SetActiveWindow(&self) -> SysResult<HWND> {
		ptr_to_sysresult_handle(
			unsafe { user::ffi::SetActiveWindow(self.as_ptr()) },
		)
	}

	/// [`SetCapture`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcapture)
	/// method.
	fn SetCapture(&self) -> ReleaseCaptureGuard<'_, Self> {
		unsafe {
			ReleaseCaptureGuard::new(
				self,
				user::ffi::SetCapture(self.as_ptr())
					.as_mut()
					.map(|ptr| HWND::from_ptr(ptr)),
			)
		}
	}

	/// [`SetFocus`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setfocus)
	/// method.
	fn SetFocus(&self) -> Option<HWND> {
		ptr_to_option_handle(unsafe { user::ffi::SetFocus(self.as_ptr()) })
	}

	/// [`SetForegroundWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow)
	/// method.
	fn SetForegroundWindow(&self) -> bool {
		unsafe { user::ffi::SetForegroundWindow(self.as_ptr()) != 0 }
	}

	/// [`SetMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenu)
	/// method.
	fn SetMenu(&self, hmenu: &HMENU) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::SetMenu(self.as_ptr(), hmenu.as_ptr()) },
		)
	}

	/// [`SetParent`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setparent)
	/// method.
	fn SetParent(&self, hwnd_new_parent: &HWND) -> SysResult<Option<HWND>> {
		match ptr_to_option_handle(
			unsafe {
				user::ffi::SetParent(self.as_ptr(), hwnd_new_parent.as_ptr())
			},
		) {	
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no previous parent
				err => Err(err),
			},
			Some(h) => Ok(Some(h)),
		}
	}

	/// [`SetScrollInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollinfo)
	/// method.
	fn SetScrollInfo(&self, bar: co::SBB, si: &SCROLLINFO, redraw: bool) -> i32 {
		unsafe {
			user::ffi::SetScrollInfo(
				self.as_ptr(),
				bar.0,
				si as *const _ as _,
				redraw as _,
			)
		}
	}

	/// [`SetScrollPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollpos)
	/// method.
	fn SetScrollPos(&self,
		b: co::SBB, pos: i32, redraw: bool) -> SysResult<i32>
	{
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

	/// [`SetScrollRange`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollrange)
	/// method.
	fn SetScrollRange(&self,
		bar: co::SBB, min_pos: i32, max_pos: i32, redraw: bool) -> SysResult<()>
	{
		bool_to_sysresult(
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

	/// [`SetTimer`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-settimer)
	/// method.
	fn SetTimer(&self,
		event_id: usize, elapse_ms: u32,
		timer_func: Option<TIMERPROC>,
	) -> SysResult<usize>
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

	/// [`SetWindowDisplayAffinity`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity)
	/// method.
	fn SetWindowDisplayAffinity(&self, affinity: co::WDA) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::SetWindowDisplayAffinity(self.as_ptr(), affinity.0)
			},
		)
	}

	/// [`SetWindowLongPtr`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
	/// method.
	fn SetWindowLongPtr(&self, index: co::GWLP, new_long: isize) -> isize {
		#[cfg(target_pointer_width = "32")]
		unsafe { user::ffi::SetWindowLongW(self.as_ptr(), index.0, new_long) }

		#[cfg(target_pointer_width = "64")]
		unsafe { user::ffi::SetWindowLongPtrW(self.as_ptr(), index.0, new_long) }
	}

	/// [`SetWindowPlacement`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowplacement)
	/// method.
	fn SetWindowPlacement(&self, wp: &WINDOWPLACEMENT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::SetWindowPlacement(self.as_ptr(), wp as *const _ as _)
			},
		)
	}

	/// [`SetWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HWND, HwndPlace, POINT, SIZE};
	///
	/// let hwnd: HWND; // initialized somewhere
	/// # let hwnd = HWND::NULL;
	///
	/// hwnd.SetWindowPos(
	///     HwndPlace::None,
	///     POINT::new(10, 10),
	///     SIZE::default(),
	///     co::SWP::NOZORDER | co::SWP::NOSIZE,
	/// )?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn SetWindowPos(&self,
		hwnd_insert_after: HwndPlace,
		pos: POINT,
		size: SIZE,
		flags: co::SWP,
	) -> SysResult<()>
	{
		bool_to_sysresult(
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

	/// [`SetWindowRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowrgn)
	/// method.
	fn SetWindowRgn(&self, hrgn: &HRGN, redraw: bool) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::SetWindowRgn(self.as_ptr(), hrgn.as_ptr(), redraw as _)
			},
		)
	}

	/// [`SetWindowText`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
	/// method.
	fn SetWindowText(&self, text: &str) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::SetWindowTextW(
					self.as_ptr(),
					WString::from_str(text).as_ptr(),
				)
			},
		)
	}

	/// [`ShowCaret`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcaret)
	/// method.
	fn ShowCaret(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { user::ffi::ShowCaret(self.as_ptr()) })
	}

	/// [`ShowWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	/// method.
	fn ShowWindow(&self, show_cmd: co::SW) -> bool {
		unsafe { user::ffi::ShowWindow(self.as_ptr(), show_cmd.0) != 0 }
	}

	/// [`ShowWindowAsync`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindowasync)
	/// method.
	fn ShowWindowAsync(&self, show_cmd: co::SW) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::ShowWindowAsync(self.as_ptr(), show_cmd.0) }
		)
	}

	/// [`TranslateAccelerator`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translateacceleratorw)
	/// method.
	fn TranslateAccelerator(&self,
		haccel_table: &HACCEL, msg: &mut MSG) -> SysResult<()>
	{
		match unsafe {
			user::ffi::TranslateAcceleratorW(
				self.as_ptr(),
				haccel_table.as_ptr(),
				msg as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`UpdateWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatewindow)
	/// method.
	fn UpdateWindow(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { user::ffi::UpdateWindow(self.as_ptr()) })
	}

	/// [`ValidateRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validaterect)
	/// method.
	fn ValidateRect(&self, rc: &RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::ValidateRect(self.as_ptr(), rc as *const _ as _)
			},
		)
	}

	/// [`ValidateRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validatergn)
	/// method.
	fn ValidateRgn(&self, hrgn: &HRGN) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::ValidateRgn(self.as_ptr(), hrgn.as_ptr()) },
		)
	}

	/// [`WindowFromPhysicalPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfromphysicalpoint)
	/// static method.
	#[must_use]
	fn WindowFromPhysicalPoint(pt: POINT) -> Option<HWND> {
		ptr_to_option_handle(
			unsafe { user::ffi::WindowFromPhysicalPoint(pt.x, pt.y) },
		)
	}

	/// [`WindowFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfrompoint)
	/// static method.
	#[must_use]
	fn WindowFromPoint(pt: POINT) -> Option<HWND> {
		ptr_to_option_handle(unsafe { user::ffi::WindowFromPoint(pt.x, pt.y) })
	}

	/// [`WinHelp`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-winhelpw)
	/// method.
	fn WinHelp(&self,
		help_file: &str, cmd: co::HELPW, data: usize) -> SysResult<()>
	{
		bool_to_sysresult(
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

//------------------------------------------------------------------------------

extern "system" fn enum_child_windows_proc<F>(
	hwnd: HWND, lparam: isize) -> BOOL
	where F: Fn(HWND) -> bool,
{
	let func = unsafe { &*(lparam as *const F) };
	func(hwnd) as _
}
