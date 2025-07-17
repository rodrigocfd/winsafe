#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::msg::*;
use crate::prelude::*;
use crate::user::{callbacks, ffi, privs::*};

handle! { HWND;
	/// Handle to a
	/// [window](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
}

impl HWND {
	/// Represents all top-level windows in
	/// [`HWND::PostMessage`](crate::HWND::PostMessage) and
	/// [`HWND::SendMessage`](crate::HWND::SendMessage).
	pub const BROADCAST: HWND = HWND(0xffff as _);

	/// Represents the desktop window in [`HWND::GetDC`](crate::HWND::GetDC).
	pub const DESKTOP: HWND = HWND(std::ptr::null_mut());

	/// Calls
	/// [`HWND::GetWindowLongPtr`](crate::HWND::GetWindowLongPtr) to retrieve
	/// the window [`HINSTANCE`](crate::HINSTANCE).
	#[must_use]
	pub fn hinstance(&self) -> HINSTANCE {
		unsafe { HINSTANCE::from_ptr(self.GetWindowLongPtr(co::GWLP::HINSTANCE) as _) }
	}

	/// Calls
	/// [`HWND::GetClassLongPtr`](crate::HWND::GetClassLongPtr) to retrieve the
	/// [class atom](https://stackoverflow.com/a/64437627/6923555) and check
	/// whether the window was created from a dialog resource.
	#[must_use]
	pub fn is_dialog(&self) -> bool {
		self.GetClassLongPtr(co::GCLP::ATOM) as u16 == WC_DIALOG
	}

	/// Calls
	/// [`HWND::SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) to set the
	/// window styles.
	pub fn set_style(&self, style: impl Into<co::WS>) {
		let style: co::WS = style.into();
		unsafe {
			self.SetWindowLongPtr(co::GWLP::STYLE, style.raw() as _);
		}
	}

	/// Calls [`HWND::SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) to set
	/// the extended window styles.
	pub fn set_style_ex(&self, ex_style: impl Into<co::WS_EX>) {
		let ex_style: co::WS_EX = ex_style.into();
		unsafe {
			self.SetWindowLongPtr(co::GWLP::EXSTYLE, ex_style.raw() as _);
		}
	}

	/// Calls [`HWND::GetWindowLongPtr`](crate::HWND::GetWindowLongPtr) to
	/// retrieve the window styles.
	#[must_use]
	pub fn style(&self) -> co::WS {
		unsafe { co::WS::from_raw(self.GetWindowLongPtr(co::GWLP::STYLE) as _) }
	}

	/// Calls [`HWND::GetWindowLongPtr`](crate::HWND::GetWindowLongPtr) to
	/// retrieve the extended window styles.
	#[must_use]
	pub fn style_ex(&self) -> co::WS_EX {
		unsafe { co::WS_EX::from_raw(self.GetWindowLongPtr(co::GWLP::EXSTYLE) as _) }
	}

	/// [`ArrangeIconicWindows`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-arrangeiconicwindows)
	/// function.
	pub fn ArrangeIconicWindows(&self) -> SysResult<u32> {
		match unsafe { ffi::ArrangeIconicWindows(self.ptr()) } {
			0 => Err(GetLastError()),
			height => Ok(height),
		}
	}

	/// [`BeginPaint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-beginpaint)
	/// function.
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
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let hdc = hwnd.BeginPaint()?;
	///
	/// // do your hdc painting...
	///
	/// // EndPaint() called automatically
	/// # w::SysResult::Ok(())
	/// ```
	///
	/// If you don't use the returned device context handle, you must still keep
	/// the guard alive:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let _hdc = hwnd.BeginPaint()?; // keep guard alive
	///
	/// // do your hdc painting...
	///
	/// // EndPaint() called automatically
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn BeginPaint(&self) -> SysResult<EndPaintGuard<'_>> {
		let mut ps = PAINTSTRUCT::default();
		unsafe {
			PtrRet(ffi::BeginPaint(self.ptr(), pvoid(&mut ps)))
				.to_sysresult_handle()
				.map(|h| EndPaintGuard::new(self, h, ps))
		}
	}

	/// [`BringWindowToTop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-bringwindowtotop)
	/// function.
	pub fn BringWindowToTop(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::BringWindowToTop(self.ptr()) }).to_sysresult()
	}

	/// [`ChildWindowFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-childwindowfrompoint)
	/// function.
	#[must_use]
	pub fn ChildWindowFromPoint(&self, pt: POINT) -> Option<HWND> {
		PtrRet(unsafe { ffi::ChildWindowFromPoint(self.ptr(), pt.x, pt.y) }).to_opt_handle()
	}

	/// [`ClientToScreen`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clienttoscreen)
	/// function.
	///
	/// If you need to convert a [`RECT`](crate::RECT), see the
	/// [`HWND::ClientToScreenRc`](crate::HWND::ClientToScreenRc) function.
	#[must_use]
	pub fn ClientToScreen(&self, pt: POINT) -> SysResult<POINT> {
		let mut buf = pt;
		BoolRet(unsafe { ffi::ClientToScreen(self.ptr(), pvoid(&mut buf)) })
			.to_sysresult()
			.map(|_| buf)
	}

	/// [`ClientToScreen`](crate::HWND::ClientToScreen) method for a
	/// [`RECT`](crate::RECT).
	#[must_use]
	pub fn ClientToScreenRc(&self, rc: RECT) -> SysResult<RECT> {
		let mut buf = rc;
		BoolRet(unsafe { ffi::ClientToScreen(self.ptr(), pvoid(&mut buf.left)) }).to_sysresult()?;
		BoolRet(unsafe { ffi::ClientToScreen(self.ptr(), pvoid(&mut buf.right)) })
			.to_sysresult()
			.map(|_| buf)
	}

	/// [`CloseWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindow)
	/// function.
	///
	/// Note that this method will actually minimize the window, not destroy it.
	pub fn CloseWindow(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::CloseWindow(self.ptr()) }).to_sysresult()
	}

	/// [`CreateWindowEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw)
	/// function.
	///
	/// # Safety
	///
	/// This method will create raw dynamic windows and controls outside the
	/// library safety – it's up to you to handle all the messages. You must use
	/// a properly registered class name and, if creating a custom window,
	/// provide its own window procedure.
	///
	/// The usable ID range for child controls is
	/// [8 to 57,343](https://stackoverflow.com/a/18192766/6923555).
	pub unsafe fn CreateWindowEx(
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
	) -> SysResult<HWND> {
		PtrRet(unsafe {
			ffi::CreateWindowExW(
				ex_style.raw(),
				class_name.as_ptr(),
				WString::from_opt_str(title).as_ptr(),
				style.raw(),
				pos.x,
				pos.y,
				size.cx,
				size.cy,
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.ptr()),
				hmenu.as_ptr(),
				hinstance.ptr(),
				lparam.unwrap_or_default() as _,
			)
		})
		.to_sysresult_handle()
	}

	/// [`DefWindowProc`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
	/// function.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::prelude::MsgSend) trait. That means each
	/// message can define its own return type.
	///
	/// # Safety
	///
	/// Messages manipulate pointers, copies and window states. Improper use may
	/// lead to undefined behavior.
	pub unsafe fn DefWindowProc<M>(&self, msg: M) -> M::RetType
	where
		M: MsgSend,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		unsafe {
			msg.isize_to_ret(ffi::DefWindowProcW(
				self.ptr(),
				wm_any.msg_id.raw(),
				wm_any.wparam,
				wm_any.lparam,
			))
		}
	}

	/// [`DestroyWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
	/// function.
	///
	/// Usually you don't need to call this method directly, since it's
	/// automatically called inside the internal message loop. The ordinary way
	/// to close a window is sending a [`wm::Close`](crate::msg::wm::Close)
	/// message.
	pub fn DestroyWindow(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::DestroyWindow(self.ptr()) }).to_sysresult()
	}

	/// [`DragDetect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dragdetect)
	/// function.
	#[must_use]
	pub fn DragDetect(&self, pt: POINT) -> bool {
		unsafe { ffi::DragDetect(self.ptr(), pt.x, pt.y) != 0 }
	}

	/// [`DrawCaption`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawcaption)
	/// function.
	pub fn DrawCaption(&self, hdc: &HDC, rect: RECT, flags: Option<co::DC>) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::DrawCaption(self.ptr(), hdc.ptr(), pcvoid(&rect), flags.unwrap_or_default().raw())
		})
		.to_sysresult()
	}

	/// [`DrawMenuBar`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawmenubar)
	/// function.
	pub fn DrawMenuBar(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::DrawMenuBar(self.ptr()) }).to_sysresult()
	}

	/// [`EnableScrollBar`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablescrollbar)
	/// function.
	pub fn EnableScrollBar(&self, sb_flags: co::SBB, arrows: co::ESB) -> SysResult<()> {
		BoolRet(unsafe { ffi::EnableScrollBar(self.ptr(), sb_flags.raw() as _, arrows.raw()) })
			.to_sysresult()
	}

	/// [`EnableWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablewindow)
	/// function.
	pub fn EnableWindow(&self, enable: bool) -> bool {
		unsafe { ffi::EnableWindow(self.ptr(), enable as _) != 0 }
	}

	/// [`EndDialog`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddialog)
	/// function.
	pub fn EndDialog(&self, result: isize) -> SysResult<()> {
		BoolRet(unsafe { ffi::EndDialog(self.ptr(), result) }).to_sysresult()
	}

	/// [`EnumChildWindows`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumchildwindows)
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
	/// hwnd.EnumChildWindows(|hchild: w::HWND| -> bool {
	///     println!("Child HWND: {}", hchild);
	///     true
	/// });
	/// ```
	pub fn EnumChildWindows<F>(&self, func: F)
	where
		F: FnMut(HWND) -> bool,
	{
		unsafe {
			ffi::EnumChildWindows(
				self.ptr(),
				callbacks::hwnd_enum_child_windows::<F> as _, // https://redd.it/npehj9
				pcvoid(&func),
			);
		}
	}

	/// [`FindWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindoww)
	/// function.
	#[must_use]
	pub fn FindWindow(class_name: Option<AtomStr>, title: Option<&str>) -> SysResult<Option<HWND>> {
		let ptr = unsafe {
			ffi::FindWindowW(
				class_name.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
				WString::from_opt_str(title).as_ptr(),
			)
		};

		if ptr.is_null() {
			match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no window found
				err => Err(err),                // actual error
			}
		} else {
			Ok(Some(unsafe { HWND::from_ptr(ptr) }))
		}
	}

	/// [`FindWindowEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindowexw)
	/// function.
	#[must_use]
	pub fn FindWindowEx(
		&self,
		hwnd_child_after: Option<&HWND>,
		class_name: AtomStr,
		title: Option<&str>,
	) -> SysResult<Option<HWND>> {
		let ptr = unsafe {
			ffi::FindWindowExW(
				self.ptr(),
				hwnd_child_after.map_or(std::ptr::null_mut(), |h| h.ptr()),
				class_name.as_ptr(),
				WString::from_opt_str(title).as_ptr(),
			)
		};

		if ptr.is_null() {
			match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no window found
				err => Err(err),                // actual error
			}
		} else {
			Ok(Some(unsafe { HWND::from_ptr(ptr) }))
		}
	}

	/// [`GetActiveWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getactivewindow)
	/// function.
	#[must_use]
	pub fn GetActiveWindow() -> Option<HWND> {
		PtrRet(unsafe { ffi::GetActiveWindow() }).to_opt_handle()
	}

	/// [`GetAltTabInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getalttabinfow)
	/// function.
	///
	/// If `item` is `None`, the item text is not retrieved.
	///
	/// The `sz_item_text` is the maximum number of expected chars for the item
	/// text. If `None`, defaults to 100.
	pub fn GetAltTabInfo(
		&self,
		item: Option<u32>,
		ati: &mut ALTTABINFO,
		sz_item_text: Option<u32>,
	) -> SysResult<String> {
		let buf_sz = sz_item_text.unwrap_or(100) + 1;
		let mut buf = match item {
			None => WString::new(),
			Some(_) => WString::new_alloc_buf(buf_sz as _), // room for terminating null
		};

		BoolRet(unsafe {
			ffi::GetAltTabInfoW(
				self.ptr(),
				item.map_or(-1, |item| item as i32),
				pvoid(ati),
				item.map_or(std::ptr::null_mut(), |_| buf.as_mut_ptr()),
				buf_sz,
			)
		})
		.to_sysresult()
		.map(|_| buf.to_string())
	}

	/// [`GetAncestor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getancestor)
	/// function.
	#[must_use]
	pub fn GetAncestor(&self, flags: co::GA) -> Option<HWND> {
		PtrRet(unsafe { ffi::GetAncestor(self.ptr(), flags.raw()) }).to_opt_handle()
	}

	/// [`GetCapture`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getcapture)
	/// function.
	#[must_use]
	pub fn GetCapture() -> Option<HWND> {
		PtrRet(unsafe { ffi::GetCapture() }).to_opt_handle()
	}

	/// [`GetClassLongPtr`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclasslongptrw)
	/// function.
	///
	/// If you just want to check whether the window is a dialog, prefer using
	/// [`HWND::is_dialog`](crate::HWND::is_dialog) method.
	#[must_use]
	pub fn GetClassLongPtr(&self, index: co::GCLP) -> usize {
		unsafe { ffi::GetClassLongPtrW(self.ptr(), index.raw()) }
	}

	/// [`GetClassName`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclassnamew)
	/// function.
	#[must_use]
	pub fn GetClassName(&self) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(256 + 1); // according to WNDCLASSEX docs
		BoolRet(unsafe { ffi::GetClassNameW(self.ptr(), buf.as_mut_ptr(), buf.buf_len() as _) })
			.to_sysresult()
			.map(|_| buf.to_string())
	}

	/// [`GetClientRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)
	/// function.
	#[must_use]
	pub fn GetClientRect(&self) -> SysResult<RECT> {
		let mut rc = RECT::default();
		BoolRet(unsafe { ffi::GetClientRect(self.ptr(), pvoid(&mut rc)) })
			.to_sysresult()
			.map(|_| rc)
	}

	/// [`GetDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
	/// function.
	///
	/// # Examples
	///
	/// Retrieving the device context of the desktop window:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hdc_desktop = w::HWND::DESKTOP.GetDC()?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn GetDC(&self) -> SysResult<ReleaseDCGuard<'_>> {
		unsafe {
			PtrRet(ffi::GetDC(self.ptr()))
				.to_sysresult_handle()
				.map(|h| ReleaseDCGuard::new(self, h))
		}
	}

	/// [`GetDCEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdcex)
	/// function.
	#[must_use]
	pub fn GetDCEx(&self, hrgn_clip: &HRGN, flags: co::DCX) -> SysResult<ReleaseDCGuard<'_>> {
		unsafe {
			PtrRet(ffi::GetDCEx(self.ptr(), hrgn_clip.ptr(), flags.raw()))
				.to_sysresult_handle()
				.map(|h| ReleaseDCGuard::new(self, h))
		}
	}

	/// [`GetDesktopWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow)
	/// function.
	#[must_use]
	pub fn GetDesktopWindow() -> HWND {
		unsafe { HWND::from_ptr(ffi::GetDesktopWindow()) }
	}

	/// [`GetDialogDpiChangeBehavior`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdialogdpichangebehavior)
	/// function.
	#[must_use]
	pub fn GetDialogDpiChangeBehavior(&self) -> SysResult<co::DDC> {
		match unsafe { co::DDC::from_raw(ffi::GetDialogDpiChangeBehavior(self.ptr())) } {
			co::DDC::DEFAULT => match GetLastError() {
				co::ERROR::SUCCESS => Ok(co::DDC::DEFAULT), // actual return value is zero
				err => Err(err),
			},
			ddc => Ok(ddc),
		}
	}

	/// [`GetDlgCtrlID`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgctrlid)
	/// function.
	#[must_use]
	pub fn GetDlgCtrlID(&self) -> SysResult<u16> {
		SetLastError(co::ERROR::SUCCESS);
		match unsafe { ffi::GetDlgCtrlID(self.ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual ID is zero
				err => Err(err),
			},
			id => Ok(id as _),
		}
	}

	/// [`GetDlgItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgitem)
	/// function.
	#[must_use]
	pub fn GetDlgItem(&self, ctrl_id: u16) -> SysResult<HWND> {
		PtrRet(unsafe { ffi::GetDlgItem(self.ptr(), ctrl_id as _) }).to_sysresult_handle()
	}

	/// [`GetDpiForWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdpiforwindow)
	/// function.
	#[must_use]
	pub fn GetDpiForWindow(&self) -> u32 {
		unsafe { ffi::GetDpiForWindow(self.ptr()) }
	}

	/// [`GetFocus`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getfocus)
	/// function.
	#[must_use]
	pub fn GetFocus() -> Option<HWND> {
		PtrRet(unsafe { ffi::GetFocus() }).to_opt_handle()
	}

	/// [`GetForegroundWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
	/// function.
	#[must_use]
	pub fn GetForegroundWindow() -> Option<HWND> {
		PtrRet(unsafe { ffi::GetForegroundWindow() }).to_opt_handle()
	}

	/// [`GetLastActivePopup`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getlastactivepopup)
	/// function.
	#[must_use]
	pub fn GetLastActivePopup(&self) -> Option<HWND> {
		PtrRet(unsafe { ffi::GetLastActivePopup(self.ptr()) }).to_opt_handle()
	}

	/// [`GetMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenu)
	/// function.
	#[must_use]
	pub fn GetMenu(&self) -> Option<HMENU> {
		PtrRet(unsafe { ffi::GetMenu(self.ptr()) }).to_opt_handle()
	}

	/// [`GetMenuBarInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenubarinfo)
	/// function.
	pub fn GetMenuBarInfo(
		&self,
		obj_id: co::OBJID,
		item_id: u32,
		mbi: &mut MENUBARINFO,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::GetMenuBarInfo(self.ptr(), obj_id.raw() as _, item_id as _, pvoid(mbi))
		})
		.to_sysresult()
	}

	/// [`GetMenuItemRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmenuitemrect)
	/// function.
	#[must_use]
	pub fn GetMenuItemRect(&self, hmenu: &HMENU, item_pos: u32) -> SysResult<RECT> {
		let mut rc = RECT::default();
		BoolRet(unsafe { ffi::GetMenuItemRect(self.ptr(), hmenu.ptr(), item_pos, pvoid(&mut rc)) })
			.to_sysresult()
			.map(|_| rc)
	}

	/// [`GetNextDlgGroupItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlggroupitem)
	/// function.
	#[must_use]
	pub fn GetNextDlgGroupItem(&self, hwnd_ctrl: &HWND, previous: bool) -> SysResult<HWND> {
		PtrRet(unsafe { ffi::GetNextDlgGroupItem(self.ptr(), hwnd_ctrl.ptr(), previous as _) })
			.to_sysresult_handle()
	}

	/// [`GetNextDlgTabItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlgtabitem)
	/// function.
	#[must_use]
	pub fn GetNextDlgTabItem(&self, hwnd_ctrl: &HWND, previous: bool) -> SysResult<HWND> {
		PtrRet(unsafe { ffi::GetNextDlgTabItem(self.ptr(), hwnd_ctrl.ptr(), previous as _) })
			.to_sysresult_handle()
	}

	/// [`GetParent`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getparent)
	/// function.
	#[must_use]
	pub fn GetParent(&self) -> SysResult<HWND> {
		PtrRet(unsafe { ffi::GetParent(self.ptr()) }).to_sysresult_handle()
	}

	/// [`GetScrollInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getscrollinfo)
	/// function.
	pub fn GetScrollInfo(&self, bar: co::SBB, si: &mut SCROLLINFO) -> SysResult<()> {
		BoolRet(unsafe { ffi::GetScrollInfo(self.ptr(), bar.raw(), pvoid(si)) }).to_sysresult()
	}

	/// [`GetScrollPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getscrollpos)
	/// function.
	#[must_use]
	pub fn GetScrollPos(&self, bar: co::SBB) -> SysResult<i32> {
		match unsafe { ffi::GetScrollPos(self.ptr(), bar.raw()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero position
				err => Err(err),
			},
			pos => Ok(pos),
		}
	}

	/// [`GetShellWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getshellwindow)
	/// function.
	#[must_use]
	pub fn GetShellWindow() -> Option<HWND> {
		PtrRet(unsafe { ffi::GetShellWindow() }).to_opt_handle()
	}

	/// [`GetSystemMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmenu)
	/// function.
	#[must_use]
	pub fn GetSystemMenu(&self, revert: bool) -> Option<HMENU> {
		PtrRet(unsafe { ffi::GetSystemMenu(self.ptr(), revert as _) }).to_opt_handle()
	}

	/// [`GetTopWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-gettopwindow)
	/// function.
	#[must_use]
	pub fn GetTopWindow(&self) -> SysResult<Option<HWND>> {
		match PtrRet(unsafe { ffi::GetTopWindow(self.ptr()) }).to_opt_handle() {
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no child window
				err => Err(err),
			},
			Some(h) => Ok(Some(h)),
		}
	}

	/// [`GetUpdateRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getupdaterect)
	/// function.
	#[must_use]
	pub fn GetUpdateRect(&self, erase: bool) -> Option<RECT> {
		let mut rc = RECT::default();
		zero_as_none(unsafe { ffi::GetUpdateRect(self.ptr(), pvoid(&mut rc), erase as _) } as _)
			.map(|_| rc)
	}

	/// [`GetUpdateRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getupdatergn)
	/// function.
	#[must_use]
	pub fn GetUpdateRgn(&self, hrgn: &HRGN, erase: bool) -> SysResult<co::REGION> {
		match unsafe { ffi::GetUpdateRgn(self.ptr(), hrgn.ptr(), erase as _) } {
			0 => Err(GetLastError()),
			ret => Ok(unsafe { co::REGION::from_raw(ret) }),
		}
	}

	/// [`GetWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	/// function.
	#[must_use]
	pub fn GetWindow(&self, cmd: co::GW) -> SysResult<HWND> {
		PtrRet(unsafe { ffi::GetWindow(self.ptr(), cmd.raw()) }).to_sysresult_handle()
	}

	/// [`GetWindowDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdc)
	/// function.
	#[must_use]
	pub fn GetWindowDC(&self) -> SysResult<ReleaseDCGuard<'_>> {
		unsafe {
			PtrRet(ffi::GetWindowDC(self.ptr()))
				.to_sysresult_handle()
				.map(|h| ReleaseDCGuard::new(self, h))
		}
	}

	/// [`GetWindowDisplayAffinity`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdisplayaffinity)
	/// function.
	#[must_use]
	pub fn GetWindowDisplayAffinity(&self) -> SysResult<co::WDA> {
		let mut affinity = co::WDA::default();
		BoolRet(unsafe { ffi::GetWindowDisplayAffinity(self.ptr(), pvoid(&mut affinity)) })
			.to_sysresult()
			.map(|_| affinity)
	}

	/// [`GetWindowDpiHostingBehavior`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdpihostingbehavior)
	/// function.
	#[must_use]
	pub fn GetWindowDpiHostingBehavior(&self) -> co::DPI_HOSTING_BEHAVIOR {
		unsafe { co::DPI_HOSTING_BEHAVIOR::from_raw(ffi::GetWindowDpiHostingBehavior(self.ptr())) }
	}

	/// [`GetWindowInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowinfo)
	/// function.
	pub fn GetWindowInfo(&self, wi: &mut WINDOWINFO) -> SysResult<()> {
		BoolRet(unsafe { ffi::GetWindowInfo(self.ptr(), pvoid(wi)) }).to_sysresult()
	}

	/// [`GetWindowLong`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongw)
	/// function (x32) or
	/// [`GetWindowLongPtr`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
	/// function (x64).
	///
	/// If you just want to retrieve the window [`HINSTANCE`](crate::HINSTANCE),
	/// prefer using [`HWND::hinstance`](crate::HWND::hinstance).
	///
	/// If you just want to retrieve the window styles, prefer using
	/// [`HWND::style`](crate::HWND::style) and
	/// [`HWND::style_ex`](crate::HWND::style_ex).
	#[must_use]
	pub fn GetWindowLongPtr(&self, index: co::GWLP) -> isize {
		#[cfg(target_pointer_width = "32")]
		unsafe {
			ffi::GetWindowLongW(self.ptr(), index.raw())
		}

		#[cfg(target_pointer_width = "64")]
		unsafe {
			ffi::GetWindowLongPtrW(self.ptr(), index.raw())
		}
	}

	/// [`GetWindowModuleFileName`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowmodulefilenamew)
	/// function.
	#[must_use]
	pub fn GetWindowModuleFileName(&self) -> String {
		let mut buf = WString::new_alloc_buf(MAX_PATH + 1);
		unsafe {
			ffi::GetWindowModuleFileNameW(self.ptr(), buf.as_mut_ptr(), buf.buf_len() as u32 - 1);
		}
		buf.to_string()
	}

	/// [`GetWindowPlacement`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowplacement)
	/// function.
	pub fn GetWindowPlacement(&self, wp: &mut WINDOWPLACEMENT) -> SysResult<()> {
		BoolRet(unsafe { ffi::GetWindowPlacement(self.ptr(), pvoid(wp)) }).to_sysresult()
	}

	/// [`GetWindowRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)
	/// function.
	#[must_use]
	pub fn GetWindowRect(&self) -> SysResult<RECT> {
		let mut rc = RECT::default();
		BoolRet(unsafe { ffi::GetWindowRect(self.ptr(), pvoid(&mut rc)) })
			.to_sysresult()
			.map(|_| rc)
	}

	/// [`GetWindowRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgn)
	/// function.
	#[must_use]
	pub fn GetWindowRgn(&self, hrgn: &HRGN) -> SysResult<co::REGION> {
		match unsafe { ffi::GetWindowRgn(self.ptr(), hrgn.ptr()) } {
			0 => Err(GetLastError()),
			ret => Ok(unsafe { co::REGION::from_raw(ret) }),
		}
	}

	/// [`GetWindowRgnBox`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgnbox)
	/// function.
	#[must_use]
	pub fn GetWindowRgnBox(&self) -> SysResult<(RECT, co::REGION)> {
		let mut rc = RECT::default();
		match unsafe { ffi::GetWindowRgnBox(self.ptr(), pvoid(&mut rc)) } {
			0 => Err(GetLastError()),
			ret => Ok((rc, unsafe { co::REGION::from_raw(ret) })),
		}
	}

	/// [`GetWindowText`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw)
	/// function.
	///
	/// Calls
	/// [`GetWindowTextLength`](crate::HWND::GetWindowTextLength) and performs
	/// all necessary allocations, returning an ordinary
	/// [`String`](std::string::String).
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let text = hwnd.GetWindowText()?;
	/// println!("Text: {}", text);
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn GetWindowText(&self) -> SysResult<String> {
		let len = self.GetWindowTextLength()?;
		if len == 0 {
			return Ok(String::new()); // window has no text
		}

		let mut buf = WString::new_alloc_buf(len as usize + 1); // plus terminating null
		match unsafe { ffi::GetWindowTextW(self.ptr(), buf.as_mut_ptr(), len + 1) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(String::new()), // no chars copied for some reason
				err => Err(err),
			},
			_ => Ok(buf.to_string()),
		}
	}

	/// [`GetWindowTextLength`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextlengthw)
	/// function.
	///
	/// Does not count the terminating null.
	///
	/// You usually don't need to call this method directly, since
	/// [`GetWindowText`](crate::HWND::GetWindowText) returns a
	/// [`String`](std::string::String), performing all necessary allocations.
	#[must_use]
	pub fn GetWindowTextLength(&self) -> SysResult<i32> {
		SetLastError(co::ERROR::SUCCESS);
		match unsafe { ffi::GetWindowTextLengthW(self.ptr()) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero length
				err => Err(err),
			},
			len => Ok(len),
		}
	}

	/// [`GetWindowThreadProcessId`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid)
	/// function.
	///
	/// Returns thread ID and process ID, respectively.
	#[must_use]
	pub fn GetWindowThreadProcessId(&self) -> (u32, u32) {
		let mut proc_id = 0u32;
		let thread_id = unsafe { ffi::GetWindowThreadProcessId(self.ptr(), &mut proc_id) };
		(thread_id, proc_id)
	}

	/// [`HideCaret`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-hidecaret)
	/// function.
	pub fn HideCaret(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::HideCaret(self.ptr()) }).to_sysresult()
	}

	/// [`HiliteMenuItem`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-hilitemenuitem)
	/// function.
	pub fn HiliteMenuItem(&self, hmenu: &HMENU, id_or_pos: IdPos, hilite: bool) -> bool {
		unsafe {
			ffi::HiliteMenuItem(
				self.ptr(),
				hmenu.ptr(),
				id_or_pos.id_or_pos_u32(),
				id_or_pos.mf_flag().raw()
					| if hilite { co::MF::HILITE } else { co::MF::UNHILITE }.raw(),
			) != 0
		}
	}

	/// [`InheritWindowMonitor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-inheritwindowmonitor)
	/// function.
	pub fn InheritWindowMonitor(&self, hwnd_inherit: &HWND) -> SysResult<()> {
		BoolRet(unsafe { ffi::InheritWindowMonitor(self.ptr(), hwnd_inherit.ptr()) }).to_sysresult()
	}

	/// [`InvalidateRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidaterect)
	/// function.
	///
	/// # Examples
	///
	/// Most of the time you'll just want update the entire client area:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// hwnd.InvalidateRect(None, true)?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn InvalidateRect(&self, rc: Option<&RECT>, erase: bool) -> SysResult<()> {
		BoolRet(unsafe { ffi::InvalidateRect(self.ptr(), pcvoid_or_null(rc), erase as _) })
			.to_sysresult()
	}

	/// [`InvalidateRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidatergn)
	/// function.
	pub fn InvalidateRgn(&self, hrgn: &HRGN, erase: bool) {
		unsafe {
			ffi::InvalidateRgn(self.ptr(), hrgn.ptr(), erase as _);
		}
	}

	/// [`IsChild`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ischild)
	/// function.
	#[must_use]
	pub fn IsChild(&self, hwnd_possible_child: &HWND) -> bool {
		unsafe { ffi::IsChild(self.ptr(), hwnd_possible_child.ptr()) != 0 }
	}

	/// [`IsDialogMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isdialogmessagew)
	/// function.
	#[must_use]
	pub fn IsDialogMessage(&self, msg: &mut MSG) -> bool {
		unsafe { ffi::IsDialogMessageW(self.ptr(), pvoid(msg)) != 0 }
	}

	/// [`IsIconic`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isiconic)
	/// function.
	#[must_use]
	pub fn IsIconic(&self) -> bool {
		unsafe { ffi::IsIconic(self.ptr()) != 0 }
	}

	/// [`IsWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)
	/// function.
	#[must_use]
	pub fn IsWindow(&self) -> bool {
		unsafe { ffi::IsWindow(self.ptr()) != 0 }
	}

	/// [`IsWindowEnabled`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowenabled)
	/// function.
	#[must_use]
	pub fn IsWindowEnabled(&self) -> bool {
		unsafe { ffi::IsWindowEnabled(self.ptr()) != 0 }
	}

	/// [`IsWindowUnicode`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowunicode)
	/// function.
	#[must_use]
	pub fn IsWindowUnicode(&self) -> bool {
		unsafe { ffi::IsWindowUnicode(self.ptr()) != 0 }
	}

	/// [`IsWindowVisible`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindowvisible)
	/// function.
	#[must_use]
	pub fn IsWindowVisible(&self) -> bool {
		unsafe { ffi::IsWindowVisible(self.ptr()) != 0 }
	}

	/// [`IsZoomed`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iszoomed)
	/// function.
	#[must_use]
	pub fn IsZoomed(&self) -> bool {
		unsafe { ffi::IsZoomed(self.ptr()) != 0 }
	}

	/// [`KillTimer`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-killtimer)
	/// function.
	///
	/// This function ends the timer calls for the given timer ID. If you don't
	/// call this function, the timer calls will continue until the window is
	/// destroyed – at this point, any remaining timers will be automatically
	/// cleared.
	pub fn KillTimer(&self, event_id: usize) -> SysResult<()> {
		match unsafe { ffi::KillTimer(self.ptr(), event_id) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(()),
				e => Err(e),
			},
			_ => Ok(()),
		}
	}

	/// [`LockWindowUpdate`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-lockwindowupdate)
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
	/// // Lock the window – only one window can be locked at a time.
	/// hwnd.LockWindowUpdate()?;
	///
	/// // After all operations, unlock the currently locked window.
	/// w::HWND::NULL.LockWindowUpdate()?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn LockWindowUpdate(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::LockWindowUpdate(self.ptr()) }).to_sysresult()
	}

	/// [`LogicalToPhysicalPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-logicaltophysicalpoint)
	/// function.
	pub fn LogicalToPhysicalPoint(&self, pt: *mut POINT) -> SysResult<()> {
		BoolRet(unsafe { ffi::LogicalToPhysicalPoint(self.ptr(), pt as _) }).to_sysresult()
	}

	/// [`MapDialogRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapdialogrect)
	/// function.
	#[must_use]
	pub fn MapDialogRect(&self, rc: RECT) -> SysResult<RECT> {
		let mut buf = rc;
		BoolRet(unsafe { ffi::MapDialogRect(self.ptr(), pvoid(&mut buf)) })
			.to_sysresult()
			.map(|_| buf)
	}

	/// [`MapWindowPoints`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapwindowpoints)
	/// function.
	///
	/// This method can convert either a series of [`POINT`](crate::POINT)
	/// structs or a single [`RECT`](crate::RECT).
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	/// let hwnd_dest: w::HWND;
	/// # let hwnd_dest = w::HWND::NULL;
	///
	/// let mut points = vec![w::POINT::default(), w::POINT::default()];
	///
	/// hwnd.MapWindowPoints(
	///     &hwnd_dest,
	///     w::PtsRc::Pts(&mut points),
	/// )?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn MapWindowPoints(&self, hdest: &HWND, points: PtsRc) -> SysResult<(i16, i16)> {
		let forced_pts = match points {
			PtsRc::Pts(pts) => pts,
			PtsRc::Rc(rc) => unsafe { std::slice::from_raw_parts_mut(rc as *mut _ as _, 2) },
		};

		SetLastError(co::ERROR::SUCCESS);
		match unsafe {
			ffi::MapWindowPoints(
				self.ptr(),
				hdest.ptr(),
				forced_pts.as_mut_ptr() as _,
				forced_pts.len() as _,
			)
		} {
			0 => Err(GetLastError()),
			n => Ok((LOWORD(n as _) as _, HIWORD(n as _) as _)),
		}
	}

	/// [`MessageBox`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// function.
	///
	/// Consider using the more modern
	/// [`HWND::TaskDialog`](crate::HWND::TaskDialog) method.
	///
	/// # Examples
	///
	/// A modal message box, which blocks its parent:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// hwnd.MessageBox("Hello, world", "title",
	///     co::MB::OKCANCEL | co::MB::ICONINFORMATION)?;
	/// # w::SysResult::Ok(())
	/// ```
	///
	/// Usually the message box has a valid parent window, however, if for some
	/// reason you don't have a window to serve as parent, you still can show a
	/// non-modal, parent-less message box by using the null window handle:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// w::HWND::NULL
	///     .MessageBox("Hello, world", "Title", co::MB::ICONEXCLAMATION)?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn MessageBox(&self, text: &str, caption: &str, flags: co::MB) -> SysResult<co::DLGID> {
		match unsafe {
			ffi::MessageBoxW(
				self.ptr(),
				WString::from_str(text).as_ptr(),
				WString::from_str(caption).as_ptr(),
				flags.raw(),
			)
		} {
			0 => Err(GetLastError()),
			ret => Ok(unsafe { co::DLGID::from_raw(ret as _) }),
		}
	}

	/// [`MonitorFromWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromwindow)
	/// function.
	#[must_use]
	pub fn MonitorFromWindow(&self, flags: co::MONITOR) -> HMONITOR {
		unsafe { HMONITOR::from_ptr(ffi::MonitorFromWindow(self.ptr(), flags.raw())) }
	}

	/// [`MoveWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-movewindow)
	/// function.
	pub fn MoveWindow(&self, pos: POINT, size: SIZE, repaint: bool) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::MoveWindow(self.ptr(), pos.x, pos.y, size.cx, size.cy, repaint as _)
		})
		.to_sysresult()
	}

	/// [`OpenClipboard`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openclipboard)
	/// function.
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
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let hclip = hwnd.OpenClipboard()?;
	/// let data = hclip.GetClipboardData(co::CF::TEXT)?;
	/// # w::SysResult::Ok(())
	/// ```
	///
	/// You can also open the clipboard without an `HWND` owner:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hclip = w::HWND::NULL.OpenClipboard()?;
	/// let data = hclip.GetClipboardData(co::CF::TEXT)?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn OpenClipboard(&self) -> SysResult<CloseClipboardGuard<'_>> {
		unsafe {
			BoolRet(ffi::OpenClipboard(self.ptr()))
				.to_sysresult()
				.map(|_| CloseClipboardGuard::new(self, HCLIPBOARD::INVALID))
		}
	}

	/// [`PostMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postmessagew)
	/// function.
	///
	/// Note that this method is asychronous.
	///
	/// # Safety
	///
	/// Messages manipulate pointers, copies and window states. Improper use may
	/// lead to undefined behavior.
	pub unsafe fn PostMessage<M>(&self, msg: M) -> SysResult<()>
	where
		M: MsgSend + Send + Copy + 'static,
	{
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		BoolRet(unsafe {
			ffi::PostMessageW(self.ptr(), wm_any.msg_id.raw(), wm_any.wparam, wm_any.lparam)
		})
		.to_sysresult()
	}

	/// [`RealChildWindowFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realchildwindowfrompoint)
	/// function.
	#[must_use]
	pub fn RealChildWindowFromPoint(&self, pt_parent_client_coords: POINT) -> Option<HWND> {
		PtrRet(unsafe {
			ffi::RealChildWindowFromPoint(
				self.ptr(),
				pt_parent_client_coords.x,
				pt_parent_client_coords.y,
			)
		})
		.to_opt_handle()
	}

	/// [`RealGetWindowClass`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realgetwindowclassw)
	/// function.
	#[must_use]
	pub fn RealGetWindowClass(&self) -> SysResult<String> {
		let mut buf = WString::new_alloc_buf(256 + 1); // according to WNDCLASSEX docs
		BoolRet(unsafe {
			ffi::RealGetWindowClassW(self.ptr(), buf.as_mut_ptr(), buf.buf_len() as _)
		} as _)
		.to_sysresult()
		.map(|_| buf.to_string())
	}

	/// [`RedrawWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow)
	/// function.
	pub fn RedrawWindow(
		&self,
		rc_update: RECT,
		hrgn_update: &HRGN,
		flags: co::RDW,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::RedrawWindow(self.ptr(), pcvoid(&rc_update), hrgn_update.ptr(), flags.raw())
		})
		.to_sysresult()
	}

	/// [`RegisterHotKey`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerhotkey)
	/// function.
	pub fn RegisterHotKey(&self, id: i32, modifiers: co::MOD, vkey_code: co::VK) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::RegisterHotKey(self.ptr(), id, modifiers.raw() as _, vkey_code.raw() as _)
		})
		.to_sysresult()
	}

	/// [`ScreenToClient`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-screentoclient)
	/// function.
	///
	/// If you need to convert a [`RECT`](crate::RECT), see the
	/// [`HWND::ScreenToClientRc`](crate::HWND::ScreenToClientRc) function.
	#[must_use]
	pub fn ScreenToClient(&self, pt: POINT) -> SysResult<POINT> {
		let mut buf = pt;
		BoolRet(unsafe { ffi::ScreenToClient(self.ptr(), pvoid(&mut buf)) })
			.to_sysresult()
			.map(|_| buf)
	}

	/// [`ScreenToClient`](crate::HWND::ScreenToClient) method for a
	/// [`RECT`](crate::RECT).
	#[must_use]
	pub fn ScreenToClientRc(&self, rc: RECT) -> SysResult<RECT> {
		let mut buf = rc;
		BoolRet(unsafe { ffi::ScreenToClient(self.ptr(), pvoid(&mut buf.left)) }).to_sysresult()?;
		BoolRet(unsafe { ffi::ScreenToClient(self.ptr(), pvoid(&mut buf.right)) })
			.to_sysresult()
			.map(|_| buf)
	}

	/// [`ScrollWindowEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-scrollwindowex)
	/// function.
	pub fn ScrollWindowEx(
		&self,
		dx: i32,
		dy: i32,
		client_area_portion: Option<&RECT>,
		clipping_rect: Option<&RECT>,
		hrgn_update: Option<&HRGN>,
		updated_boundaries: Option<&mut RECT>,
		flags: co::SCROLLW,
	) -> SysResult<co::REGION> {
		match unsafe {
			ffi::ScrollWindowEx(
				self.ptr(),
				dx,
				dy,
				pcvoid_or_null(client_area_portion),
				pcvoid_or_null(clipping_rect),
				hrgn_update.map_or(std::ptr::null_mut(), |h| h.ptr()),
				pvoid_or_null(updated_boundaries),
				flags.raw(),
			)
		} {
			0 => Err(GetLastError()),
			v => Ok(unsafe { co::REGION::from_raw(v) }),
		}
	}

	/// [`SendMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)
	/// function, specialized to send a [`wm::Command`](crate::msg::wm::Command)
	/// message.
	///
	/// Unlike the general [`SendMessage`](crate::HWND::SendMessage), sending a
	/// command is safe.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, msg};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// const ID_MENU_FILE_OPEN: u16 = 103;
	///
	/// hwnd.SendCommand(
	///     w::AccelMenuCtrl::Menu(ID_MENU_FILE_OPEN),
	/// );
	/// ```
	pub fn SendCommand(&self, cmd: AccelMenuCtrl) {
		unsafe {
			self.SendMessage(wm::Command { event: cmd });
		}
	}

	/// [`SendMessage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)
	/// function.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`MsgSend`](crate::prelude::MsgSend) trait. That means each
	/// message can define its own return type.
	///
	/// # Safety
	///
	/// Messages manipulate pointers, copies and window states. Improper use may
	/// lead to undefined behavior.
	///
	/// # Examples
	///
	/// Sending a [`bm::GetImage`](crate::msg::bm::GetImage) button message,
	/// which demands an image type parameter. Note that this specific message
	/// can also return an error, which is handled with
	/// [`?`](https://doc.rust-lang.org/std/result/index.html#the-question-mark-operator-):
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co, msg};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let bmp = unsafe {
	///     hwnd.SendMessage(
	///         msg::bm::GetImage {
	///             img_type: co::IMAGE_TYPE::BITMAP,
	///         },
	///     )
	/// }?;
	/// # w::SysResult::Ok(())
	/// ```
	///
	/// Sending an [`em::CharFromPos`](crate::msg::em::CharFromPos) edit message,
	/// which receives point coordinates and returns two values:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, msg};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// let (char_pos, line_pos) = unsafe {
	///     hwnd.SendMessage(
	///         msg::em::CharFromPos {
	///             coords: w::POINT::with(12, 20),
	///         },
	///     )
	/// };
	/// ```
	pub unsafe fn SendMessage<M: MsgSend>(&self, msg: M) -> M::RetType {
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		unsafe {
			msg.isize_to_ret(ffi::SendMessageW(
				self.ptr(),
				wm_any.msg_id.raw(),
				wm_any.wparam,
				wm_any.lparam,
			))
		}
	}

	/// [`SendMessageTimeout`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeoutw)
	/// function.
	///
	/// # Safety
	///
	/// Messages manipulate pointers, copies and window states. Improper use may
	/// lead to undefined behavior.
	pub unsafe fn SendMessageTimeout<M: MsgSend>(
		&self,
		msg: M,
		flags: co::SMTO,
		timeout_ms: u32,
	) -> SysResult<M::RetType> {
		let mut msg = msg;
		let wm_any = msg.as_generic_wm();
		let mut result = 0isize;

		unsafe {
			BoolRet(ffi::SendMessageTimeoutW(
				self.ptr(),
				wm_any.msg_id.raw(),
				wm_any.wparam,
				wm_any.lparam,
				flags.raw(),
				timeout_ms,
				&mut result,
			) as _)
			.to_sysresult()
			.map(|_| msg.isize_to_ret(result))
		}
	}

	/// [`SetActiveWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setactivewindow)
	/// function.
	pub fn SetActiveWindow(&self) -> SysResult<HWND> {
		PtrRet(unsafe { ffi::SetActiveWindow(self.ptr()) }).to_sysresult_handle()
	}

	/// [`SetCapture`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setcapture)
	/// function.
	pub fn SetCapture(&self) -> ReleaseCaptureGuard<'_> {
		unsafe {
			ReleaseCaptureGuard::new(
				self,
				ffi::SetCapture(self.ptr())
					.as_mut()
					.map(|ptr| HWND::from_ptr(ptr)),
			)
		}
	}

	/// [`SetDialogDpiChangeBehavior`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setdialogdpichangebehavior)
	/// function.
	pub fn SetDialogDpiChangeBehavior(&self, mask: co::DDC, values: co::DDC) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetDialogDpiChangeBehavior(self.ptr(), mask.raw(), values.raw()) })
			.to_sysresult()
	}

	/// [`SetFocus`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setfocus)
	/// function.
	pub fn SetFocus(&self) -> Option<HWND> {
		PtrRet(unsafe { ffi::SetFocus(self.ptr()) }).to_opt_handle()
	}

	/// [`SetForegroundWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow)
	/// function.
	pub fn SetForegroundWindow(&self) -> bool {
		unsafe { ffi::SetForegroundWindow(self.ptr()) != 0 }
	}

	/// [`SetLayeredWindowAttributes`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setlayeredwindowattributes)
	/// function.
	pub fn SetLayeredWindowAttributes(
		&self,
		transparency_color_key: COLORREF,
		alpha: u8,
		flags: co::LWA,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::SetLayeredWindowAttributes(
				self.ptr(),
				transparency_color_key.raw(),
				alpha,
				flags.raw(),
			)
		})
		.to_sysresult()
	}

	/// [`SetMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setmenu)
	/// function.
	pub fn SetMenu(&self, hmenu: &HMENU) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetMenu(self.ptr(), hmenu.ptr()) }).to_sysresult()
	}

	/// [`SetParent`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setparent)
	/// function.
	pub fn SetParent(&self, hwnd_new_parent: &HWND) -> SysResult<Option<HWND>> {
		match PtrRet(unsafe { ffi::SetParent(self.ptr(), hwnd_new_parent.ptr()) }).to_opt_handle() {
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no previous parent
				err => Err(err),
			},
			Some(h) => Ok(Some(h)),
		}
	}

	/// [`SetScrollInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollinfo)
	/// function.
	pub fn SetScrollInfo(&self, bar: co::SBB, si: &SCROLLINFO, redraw: bool) -> i32 {
		unsafe { ffi::SetScrollInfo(self.ptr(), bar.raw(), pcvoid(si), redraw as _) }
	}

	/// [`SetScrollPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollpos)
	/// function.
	pub fn SetScrollPos(&self, b: co::SBB, pos: i32, redraw: bool) -> SysResult<i32> {
		match unsafe { ffi::SetScrollPos(self.ptr(), b.raw(), pos, redraw as _) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero position
				err => Err(err),
			},
			pos => Ok(pos),
		}
	}

	/// [`SetScrollRange`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollrange)
	/// function.
	pub fn SetScrollRange(
		&self,
		bar: co::SBB,
		min_pos: i32,
		max_pos: i32,
		redraw: bool,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::SetScrollRange(self.ptr(), bar.raw(), min_pos, max_pos, redraw as _)
		})
		.to_sysresult()
	}

	/// This method returns the timer ID, to be passed to
	/// [`HWND::KillTimer`](crate::HWND::KillTimer).
	///
	/// The timer calls – either
	/// [`wm_timer`](crate::gui::events::WindowEvents::wm_timer) message or
	/// callback function – will continuously be executed until you call
	/// `KillTimer`. If you don't call `KillTimer`, the timer calls will
	/// continue until the window is destroyed – at this point, any remaining
	/// timers will be automatically cleared.
	///
	/// # Why not closures?
	///
	/// A common C++ technique to use closures with `SetTimer` is allocating a
	/// closure on the heap and use its pointer as the timer ID. When the
	/// callback function is called, the pointer is dereferenced and the closure
	/// is then executed.
	///
	/// The problem with this approach is that the closure must be freed after
	/// `KillTimer`, which can be called from anywhere, including from the
	/// closure itself – that means you must keep the pointer outside the
	/// closure and free it somehow after the closure finishes.
	///
	/// Such approach is, obviously, incredibly unsafe, and only possible within
	/// Rust's rigid ownership rules if we use some sort of garbage-collection,
	/// which will free the allocated closure some time after `KillTimer` is
	/// called and the closure itself finishes. Since that would incur in a
	/// performance penalty, the current implementation of `SetTimer` will only
	/// accept ordinary function pointers, not closures.
	///
	/// Handling the `wm_timer` message is simply more practical and efficient,
	/// so the use of a callback is discouraged here.
	pub fn SetTimer(
		&self,
		event_id: usize,
		elapse_ms: u32,
		timer_func: Option<TIMERPROC>,
	) -> SysResult<usize> {
		match unsafe {
			ffi::SetTimer(
				self.ptr(),
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
	/// function.
	pub fn SetWindowDisplayAffinity(&self, affinity: co::WDA) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetWindowDisplayAffinity(self.ptr(), affinity.raw()) }).to_sysresult()
	}

	/// [`SetWindowLongPtr`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
	/// function.
	///
	/// If you just want to set the window styles, prefer using
	/// [`HWND::set_style`](crate::HWND::set_style) and
	/// [`HWND::set_style_ex`](crate::HWND::set_style_ex).
	///
	/// # Safety
	///
	/// Changing these values may potentially cause undefined behavior to the
	/// window, and passed pointers must be handled correctly.
	pub unsafe fn SetWindowLongPtr(&self, index: co::GWLP, new_long: isize) -> isize {
		#[cfg(target_pointer_width = "32")]
		unsafe {
			ffi::SetWindowLongW(self.ptr(), index.raw(), new_long)
		}

		#[cfg(target_pointer_width = "64")]
		unsafe {
			ffi::SetWindowLongPtrW(self.ptr(), index.raw(), new_long)
		}
	}

	/// [`SetWindowPlacement`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowplacement)
	/// function.
	pub fn SetWindowPlacement(&self, wp: &WINDOWPLACEMENT) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetWindowPlacement(self.ptr(), pcvoid(wp)) }).to_sysresult()
	}

	/// [`SetWindowPos`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hwnd: w::HWND; // initialized somewhere
	/// # let hwnd = w::HWND::NULL;
	///
	/// hwnd.SetWindowPos(
	///     w::HwndPlace::None,
	///     w::POINT::with(10, 10),
	///     w::SIZE::default(),
	///     co::SWP::NOZORDER | co::SWP::NOSIZE,
	/// )?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn SetWindowPos(
		&self,
		hwnd_insert_after: HwndPlace,
		pos: POINT,
		size: SIZE,
		flags: co::SWP,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::SetWindowPos(
				self.ptr(),
				hwnd_insert_after.as_ptr(),
				pos.x,
				pos.y,
				size.cx,
				size.cy,
				flags.raw(),
			)
		})
		.to_sysresult()
	}

	/// [`SetWindowRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowrgn)
	/// function.
	pub fn SetWindowRgn(&self, hrgn: &HRGN, redraw: bool) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetWindowRgn(self.ptr(), hrgn.ptr(), redraw as _) }).to_sysresult()
	}

	/// [`SetWindowText`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
	/// function.
	pub fn SetWindowText(&self, text: &str) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetWindowTextW(self.ptr(), WString::from_str(text).as_ptr()) })
			.to_sysresult()
	}

	/// [`ShowCaret`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcaret)
	/// function.
	pub fn ShowCaret(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::ShowCaret(self.ptr()) }).to_sysresult()
	}

	/// [`ShowOwnedPopups`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showownedpopups)
	/// function.
	pub fn ShowOwnedPopups(&self, show: bool) -> SysResult<()> {
		BoolRet(unsafe { ffi::ShowOwnedPopups(self.ptr(), show as _) }).to_sysresult()
	}

	/// [`ShowWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	/// function.
	pub fn ShowWindow(&self, show_cmd: co::SW) -> bool {
		unsafe { ffi::ShowWindow(self.ptr(), show_cmd.raw()) != 0 }
	}

	/// [`ShowWindowAsync`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindowasync)
	/// function.
	pub fn ShowWindowAsync(&self, show_cmd: co::SW) -> SysResult<()> {
		BoolRet(unsafe { ffi::ShowWindowAsync(self.ptr(), show_cmd.raw()) }).to_sysresult()
	}

	/// [`ShutdownBlockReasonCreate`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-shutdownblockreasoncreate)
	/// function.
	pub fn ShutdownBlockReasonCreate(&self, reason: &str) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::ShutdownBlockReasonCreate(self.ptr(), WString::from_str(reason).as_ptr())
		})
		.to_sysresult()
	}

	/// [`ShutdownBlockReasonDestroy`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-shutdownblockreasondestroy)
	/// function.
	pub fn ShutdownBlockReasonDestroy(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::ShutdownBlockReasonDestroy(self.ptr()) }).to_sysresult()
	}

	/// [`ShutdownBlockReasonQuery`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-shutdownblockreasonquery)
	/// function.
	#[must_use]
	pub fn ShutdownBlockReasonQuery(&self) -> SysResult<String> {
		let mut sz = 0u32;
		BoolRet(unsafe {
			ffi::ShutdownBlockReasonQuery(self.ptr(), std::ptr::null_mut(), &mut sz)
		})
		.to_sysresult()?;

		let mut buf = WString::new_alloc_buf(sz as _);
		BoolRet(unsafe { ffi::ShutdownBlockReasonQuery(self.ptr(), buf.as_mut_ptr(), &mut sz) })
			.to_sysresult()
			.map(|_| buf.to_string())
	}

	/// [`TileWindows`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-tilewindows)
	/// function.
	pub fn TileWindows(
		&self,
		how: co::MDITILE,
		rect: Option<RECT>,
		kids: &[&HWND],
	) -> SysResult<u16> {
		match unsafe {
			ffi::TileWindows(
				self.ptr(),
				how.raw(),
				pcvoid_or_null(rect.as_ref()),
				kids.len() as _,
				vec_ptr(kids) as _,
			)
		} {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0),
				err => Err(err),
			},
			c => Ok(c),
		}
	}

	/// [`TranslateAccelerator`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translateacceleratorw)
	/// function.
	pub fn TranslateAccelerator(&self, haccel_table: &HACCEL, msg: &mut MSG) -> SysResult<()> {
		BoolRet(unsafe { ffi::TranslateAcceleratorW(self.ptr(), haccel_table.ptr(), pvoid(msg)) })
			.to_sysresult()
	}

	/// [`UnregisterHotKey`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterhotkey)
	/// function.
	pub fn UnregisterHotKey(&self, id: i32) -> SysResult<()> {
		BoolRet(unsafe { ffi::UnregisterHotKey(self.ptr(), id) }).to_sysresult()
	}

	/// [`UpdateLayeredWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatelayeredwindow)
	/// function.
	pub fn UpdateLayeredWindow(
		&self,
		hdc_dest: Option<&HDC>,
		pt_dest: Option<&POINT>,
		size: Option<&SIZE>,
		hdc_src: Option<&HDC>,
		pt_src: Option<&POINT>,
		key: COLORREF,
		blend: &BLENDFUNCTION,
		flags: co::ULW,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::UpdateLayeredWindow(
				self.ptr(),
				hdc_dest.map_or(std::ptr::null_mut(), |hdc| hdc.ptr()),
				pcvoid_or_null(pt_dest),
				pcvoid_or_null(size),
				hdc_src.map_or(std::ptr::null_mut(), |hdc| hdc.ptr()),
				pcvoid_or_null(pt_src),
				key.raw(),
				pcvoid(blend),
				flags.raw(),
			)
		})
		.to_sysresult()
	}

	/// [`UpdateWindow`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatewindow)
	/// function.
	pub fn UpdateWindow(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::UpdateWindow(self.ptr()) }).to_sysresult()
	}

	/// [`ValidateRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validaterect)
	/// function.
	pub fn ValidateRect(&self, rc: Option<RECT>) -> SysResult<()> {
		BoolRet(unsafe { ffi::ValidateRect(self.ptr(), pcvoid_or_null(rc.as_ref())) })
			.to_sysresult()
	}

	/// [`ValidateRgn`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validatergn)
	/// function.
	pub fn ValidateRgn(&self, hrgn: &HRGN) -> SysResult<()> {
		BoolRet(unsafe { ffi::ValidateRgn(self.ptr(), hrgn.ptr()) }).to_sysresult()
	}

	/// [`WindowFromPhysicalPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfromphysicalpoint)
	/// function.
	#[must_use]
	pub fn WindowFromPhysicalPoint(pt: POINT) -> Option<HWND> {
		PtrRet(unsafe { ffi::WindowFromPhysicalPoint(pt.x, pt.y) }).to_opt_handle()
	}

	/// [`WindowFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfrompoint)
	/// function.
	#[must_use]
	pub fn WindowFromPoint(pt: POINT) -> Option<HWND> {
		PtrRet(unsafe { ffi::WindowFromPoint(pt.x, pt.y) }).to_opt_handle()
	}

	/// [`WinHelp`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-winhelpw)
	/// function.
	pub fn WinHelp(&self, help_file: &str, cmd: co::HELPW, data: usize) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::WinHelpW(self.ptr(), WString::from_str(help_file).as_ptr(), cmd.raw(), data)
		})
		.to_sysresult()
	}
}
