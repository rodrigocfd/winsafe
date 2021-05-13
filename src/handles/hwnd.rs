#![allow(non_snake_case)]

use crate::aliases::{SUBCLASSPROC, TIMERPROC, WinResult, WNDENUMPROC};
use crate::co;
use crate::enums::{AtomStr, HwndPlace, IdMenu, IdPos};
use crate::ffi::{BOOL, comctl32, user32, uxtheme};
use crate::funcs::{GetLastError, SetLastError};
use crate::handles::{HACCEL, HDC, HINSTANCE, HMENU, HRGN, HTHEME};
use crate::msg::MsgSend;
use crate::privs::{bool_to_winresult, ref_as_pcvoid, ref_as_pvoid};
use crate::structs::{
	ALTTABINFO,
	MSG,
	PAINTSTRUCT,
	POINT,
	RECT,
	SCROLLINFO,
	WINDOWINFO,
	WINDOWPLACEMENT,
};
use crate::WString;

pub_struct_handle! {
	/// Handle to a
	/// [window](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
	HWND
}

impl HWND {
	/// For some specific functions, represents all top-level windows.
	pub const BROADCAST: Self = Self { ptr: std::ptr::null_mut() };

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
	/// **Note:** Must be paired with an [`EndPaint`](crate::HWND::EndPaint)
	/// call.
	pub fn BeginPaint(self, lpPaint: &mut PAINTSTRUCT) -> WinResult<HDC> {
		unsafe { user32::BeginPaint(self.ptr, ref_as_pvoid(lpPaint)).as_mut() }
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
	pub fn ChildWindowFromPoint(self, point: POINT) -> Option<HWND> {
		unsafe {
			user32::ChildWindowFromPoint(self.ptr, point.x, point.y).as_mut()
		}.map(|ptr| Self { ptr })
	}

	/// [`ClientToScreen`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clienttoscreen)
	/// method.
	pub fn ClientToScreen(self, lpPoint: &mut POINT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::ClientToScreen(self.ptr, ref_as_pvoid(lpPoint))
			},
		)
	}

	/// [`ClientToScreen`](crate::HWND::ClientToScreen) method for a
	/// [`RECT`](crate::RECT).
	pub fn ClientToScreenRc(self, lpRect: &mut RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::ClientToScreen(
					self.ptr,
					ref_as_pvoid(&mut lpRect.left),
				)
			},
		).and_then(|_| bool_to_winresult(
			unsafe {
				user32::ClientToScreen(
					self.ptr,
					ref_as_pvoid(&mut lpRect.right),
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
		dwExStyle: co::WS_EX,
		lpClassName: AtomStr,
		lpWindowName: Option<&str>,
		dwStyle: co::WS,
		X: i32, Y: i32,
		nWidth: i32, nHeight: i32,
		hWndParent: Option<HWND>,
		hMenu: IdMenu,
		hInstance: HINSTANCE,
		lpParam: Option<isize>) -> WinResult<HWND>
	{
		unsafe {
			user32::CreateWindowExW(
				dwExStyle.0,
				lpClassName.as_ptr(),
				WString::from_opt_str(lpWindowName).as_ptr(),
				dwStyle.0,
				X, Y, nWidth, nHeight,
				hWndParent.map_or(std::ptr::null_mut(), |h| h.ptr),
				hMenu.as_ptr(),
				hInstance.ptr,
				lpParam.unwrap_or_default() as _,
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
	pub fn DefSubclassProc<M: MsgSend>(self, uMsg: M) -> M::RetType {
		let wmAny = uMsg.as_generic_wm();
		uMsg.convert_ret(
			unsafe {
				comctl32::DefSubclassProc(
					self.ptr, wmAny.msg_id.0, wmAny.wparam, wmAny.lparam,
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
	pub fn DefWindowProc<M: MsgSend>(self, uMsg: M) -> M::RetType {
		let wmAny = uMsg.as_generic_wm();
		uMsg.convert_ret(
			unsafe {
				user32::DefWindowProcW(
					self.ptr, wmAny.msg_id.0, wmAny.wparam, wmAny.lparam,
				)
			},
		)
	}

	/// [`DestroyWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)
	/// method.
	pub fn DestroyWindow(self) {
		unsafe { user32::DestroyWindow(self.ptr); }
	}

	/// [`EnableWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enablewindow)
	/// method.
	pub fn EnableWindow(self, bEnable: bool) -> bool {
		unsafe { user32::EnableWindow(self.ptr, bEnable as _) != 0 }
	}

	/// [`EndDialog`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddialog)
	/// method.
	pub fn EndDialog(self, nResult: isize) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::EndDialog(self.ptr, nResult) })
	}

	/// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
	/// method.
	pub fn EndPaint(self, lpPaint: &PAINTSTRUCT) {
		unsafe { user32::EndPaint(self.ptr, ref_as_pcvoid(lpPaint)); }
	}

	/// [`EnumChildWindows`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumchildwindows)
	/// method.
	///
	/// This method can be more performant than
	/// [`EnumChildWindowsVec`](crate::HWND::EnumChildWindowsVec), which passes
	/// through all children and allocates a `Vec`. However, it has the
	/// inconvenient of the manual function pointer.
	pub fn EnumChildWindows(self, lpEnumFunc: WNDENUMPROC, lParam: isize) {
		unsafe {
			user32::EnumChildWindows(self.ptr, lpEnumFunc as _, lParam);
		}
	}

	/// A more convenient [`EnumChildWindows`](crate::HWND::EnumChildWindows),
	/// which returns a `Vec` with the handles of all child windows, instead of
	/// taking a function pointer.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// for hchild in my_hwnd.EnumChildWindowsVec() {
	///     println!("HWND: {}", hchild);
	/// }
	/// ```
	pub fn EnumChildWindowsVec(self) -> Vec<HWND> {
		let mut hchildren = Vec::new();
		self.EnumChildWindows(
			Self::EnumChildWindowsVecProc,
			&mut hchildren as *mut Vec<_> as _, // pass pointer to Vec
		);
		hchildren
	}
	extern "system" fn EnumChildWindowsVecProc(
		hchild: HWND, lparam: isize) -> BOOL
	{
		let hchildren = unsafe { &mut *(lparam as *mut Vec<HWND>) }; // retrieve pointer to Vec
		hchildren.push(hchild);
		true as _
	}

	/// [`FindWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindoww)
	/// static method.
	pub fn FindWindow(
		lpClassName: &str, lpWindowName: &str) -> WinResult<HWND>
	{
		unsafe {
			user32::FindWindowW(
				WString::from_str(lpClassName).as_ptr(),
				WString::from_str(lpWindowName).as_ptr(),
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`FindWindowEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindowexw)
	/// method.
	pub fn FindWindowEx(self,
		hWndChildAfter: Option<HWND>,
		lpszClass: AtomStr, lpszWindow: Option<&str>) -> WinResult<HWND>
	{
		unsafe {
			user32::FindWindowExW(
				self.ptr,
				hWndChildAfter.map_or(std::ptr::null_mut(), |h| h.ptr),
				lpszClass.as_ptr(),
				lpszWindow.map_or(std::ptr::null(), |s| WString::from_str(s).as_ptr()),
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
	/// If `iItem` is `None`, the item text is not retrieved.
	///
	/// The `cchItemText` is the maximum number of expected chars for the item
	/// text. If `None`, defaults to 100.
	pub fn GetAltTabInfo(self,
		iItem: Option<u32>, pati: &mut ALTTABINFO,
		cchItemText: Option<u32>) -> WinResult<String>
	{
		let bufSz = cchItemText.unwrap_or(100) + 1;
		let mut buf = iItem.map_or(
			WString::default(),
			|_| WString::new_alloc_buffer(bufSz as _), // room for terminating null
		);

		bool_to_winresult(
			unsafe {
				user32::GetAltTabInfoW(
					self.ptr,
					iItem.map(|iItem| iItem as _).unwrap_or(-1),
					ref_as_pvoid(pati),
					iItem.map_or(std::ptr::null_mut(), |_| buf.as_mut_ptr()),
					bufSz,
				)
			},
		).map(|_| buf.to_string())
	}

	/// [`GetAncestor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getancestor)
	/// method.
	pub fn GetAncestor(self, gaFlags: co::GA) -> Option<HWND> {
		unsafe { user32::GetAncestor(self.ptr, gaFlags.0).as_mut() }
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
	pub fn GetClassLongPtr(self, nIndex: co::GCLP) -> usize {
		unsafe { user32::GetClassLongPtrW(self.ptr, nIndex.0) }
	}

	/// [`GetClientRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)
	/// method.
	pub fn GetClientRect(self) -> WinResult<RECT> {
		let mut rc = RECT::default();
		match unsafe {
			user32::GetClientRect(self.ptr, ref_as_pvoid(&mut rc))
		} {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
	/// method.
	///
	/// **Note:** Must be paired with a [`ReleaseDC`](crate::HWND::ReleaseDC)
	/// call.
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
	pub fn GetDlgCtrlID(self) -> WinResult<i32> {
		match unsafe { user32::GetDlgCtrlID(self.ptr) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual ID is zero
				err => Err(err),
			},
			id => Ok(id),
		}
	}

	/// [`GetDlgItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdlgitem)
	/// method.
	pub fn GetDlgItem(self, nIDDlgItem: i32) -> WinResult<HWND> {
		unsafe { user32::GetDlgItem(self.ptr, nIDDlgItem).as_mut() }
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

	/// [`GetNextDlgGroupItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlggroupitem)
	/// method.
	pub fn GetNextDlgGroupItem(self,
		hCtl: HWND, bPrevious: bool) -> WinResult<HWND>
	{
		unsafe {
			user32::GetNextDlgGroupItem(self.ptr, hCtl.ptr, bPrevious as _)
				.as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetNextDlgTabItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlgtabitem)
	/// method.
	pub fn GetNextDlgTabItem(self,
		hCtl: HWND, bPrevious: bool) -> WinResult<HWND>
	{
		unsafe {
			user32::GetNextDlgTabItem(self.ptr, hCtl.ptr, bPrevious as _).as_mut()
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
		nBar: co::SBB, lpsi: &mut SCROLLINFO) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::GetScrollInfo(self.ptr, nBar.0, ref_as_pvoid(lpsi))
			},
		)
	}

	/// [`GetScrollPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getscrollpos)
	/// method.
	pub fn GetScrollPos(self, nBar: co::SBB) -> WinResult<i32> {
		match unsafe { user32::GetScrollPos(self.ptr, nBar.0) } {
			0 => match GetLastError() {
				co::ERROR::SUCCESS => Ok(0), // actual zero position
				err => Err(err),
			},
			pos => Ok(pos),
		}
	}

	/// [`GetUpdateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getupdatergn)
	/// method.
	pub fn GetUpdateRgn(self, hRgn: HRGN, bErase: bool) -> WinResult<co::REGION> {
		match unsafe { user32::GetUpdateRgn(self.ptr, hRgn.ptr, bErase as _) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	/// method.
	pub fn GetWindow(self, uCmd: co::GW) -> WinResult<HWND> {
		unsafe { user32::GetWindow(self.ptr, uCmd.0).as_mut() }
			.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetWindowDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdc)
	/// method.
	///
	/// **Note:** Must be paired with a [`ReleaseDC`](crate::HWND::ReleaseDC)
	/// call.
	pub fn GetWindowDC(self) -> WinResult<HDC> {
		unsafe { user32::GetWindowDC(self.ptr).as_mut() }
			.map(|ptr| HDC { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetWindowDisplayAffinity`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdisplayaffinity)
	/// method.
	pub fn GetWindowDisplayAffinity(self) -> WinResult<co::WDA> {
		let mut pdwAffinity = co::WDA::default();
		match unsafe {
			user32::GetWindowDisplayAffinity(
				self.ptr,
				ref_as_pvoid(&mut pdwAffinity),
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(pdwAffinity),
		}
	}

	/// [`GetWindowInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowinfo)
	/// method.
	pub fn GetWindowInfo(self, pwi: &mut WINDOWINFO) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::GetWindowInfo(self.ptr, ref_as_pvoid(pwi)) },
		)
	}

	/// [`GetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
	/// method.
	pub fn GetWindowLongPtr(self, nIndex: co::GWLP) -> isize {
		unsafe { user32::GetWindowLongPtrW(self.ptr, nIndex.0) }
	}

	/// [`GetWindowPlacement`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowplacement)
	/// method.
	pub fn GetWindowPlacement(self,
		lpwndpl: &mut WINDOWPLACEMENT) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::GetWindowPlacement(self.ptr, ref_as_pvoid(lpwndpl))
			},
		)
	}

	/// [`GetWindowRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)
	/// method.
	pub fn GetWindowRect(self) -> WinResult<RECT> {
		let mut rc = RECT::default();
		match unsafe {
			user32::GetWindowRect(self.ptr, ref_as_pvoid(&mut rc))
		} {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetWindowRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgn)
	/// method.
	pub fn GetWindowRgn(self, hRgn: HRGN) -> WinResult<co::REGION> {
		match unsafe { user32::GetWindowRgn(self.ptr, hRgn.ptr) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindowRgnBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgnbox)
	/// method.
	pub fn GetWindowRgnBox(self, lprc: &mut RECT) -> WinResult<co::REGION> {
		match unsafe {
			user32::GetWindowRgnBox(self.ptr, ref_as_pvoid(lprc))
		} {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION(ret)),
		}
	}

	/// [`GetWindowText`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowtextw)
	/// method.
	///
	/// The passed buffer will be automatically allocated with
	/// [`GetWindowTextLength`](crate::HWND::GetWindowTextLength).
	///
	/// This method can be more performant than
	/// [`GetWindowTextStr`](crate::HWND::GetWindowTextStr) because the buffer
	/// can be reused, avoiding multiple allocations. However, it has the
	/// inconvenient of the manual conversion from [`WString`](crate::WString)
	/// to `String`.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{HWND, WString};
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// let mut buf = WString::new();
	/// my_hwnd.GetWindowText(&mut buf).unwrap();
	/// println!("Text: {}", buf.to_string());
	/// ```
	pub fn GetWindowText(self, buf: &mut WString) -> WinResult<i32> {
		match self.GetWindowTextLength()? {
			0 => { // window has no text, simply clear buffer
				buf.realloc_buffer(0);
				Ok(0)
			},
			len => {
				buf.realloc_buffer(len as usize + 1); // plus terminating null

				match unsafe {
					user32::GetWindowTextW(self.ptr, buf.as_mut_ptr(), len + 1)
				} {
					0 => match GetLastError() {
						co::ERROR::SUCCESS => {
							buf.realloc_buffer(0); // no chars copied for some reason
							Ok(0)
						},
						err => Err(err),
					},
					nCopied => Ok(nCopied), // return number of copied chars without terminating null
				}
			},
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

	/// A more convenient [`GetWindowText`](crate::HWND::GetWindowText), which
	/// directly returns a `String` instead of requiring an external buffer.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// let text = my_hwnd.GetWindowTextStr().unwrap();
	/// println!("Text: {}", text);
	/// ```
	pub fn GetWindowTextStr(self) -> WinResult<String> {
		let mut buf = WString::default();
		self.GetWindowText(&mut buf)?;
		Ok(buf.to_string())
	}

	/// [`HiliteMenuItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-hilitemenuitem)
	/// method.
	pub fn HiliteMenuItem(self,
		hMenu: HMENU, uIDHiliteItem: IdPos, uHilite: co::MF) -> bool
	{
		unsafe {
			user32::HiliteMenuItem(
				self.ptr, hMenu.ptr, uIDHiliteItem.id_or_pos_u32(), uHilite.0,
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
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// my_hwnd.InvalidateRect(None, true)
	///     .unwrap();
	/// ```
	pub fn InvalidateRect(self,
		lpRect: Option<&RECT>, bErase: bool) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::InvalidateRect(
					self.ptr,
					lpRect.map_or(std::ptr::null(), |lp| ref_as_pcvoid(lp)),
					bErase as _,
				)
			},
		)
	}

	/// [`InvalidateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidatergn)
	/// method.
	pub fn InvalidateRgn(self, hRgn: HRGN, bErase: bool) {
		unsafe { user32::InvalidateRgn(self.ptr, hRgn.ptr, bErase as _); }
	}

	/// [`IsChild`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ischild)
	/// method.
	pub fn IsChild(self, hWndPossibleChild: HWND) -> bool {
		unsafe { user32::IsChild(self.ptr, hWndPossibleChild.ptr) != 0 }
	}

	/// [`IsDialogMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isdialogmessagew)
	/// method.
	pub fn IsDialogMessage(self, lpMsg: &mut MSG) -> bool {
		unsafe {
			user32::IsDialogMessageW(self.ptr, ref_as_pvoid(lpMsg)) != 0
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
	pub fn KillTimer(self, uIDEvent: usize) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::KillTimer(self.ptr, uIDEvent) })
	}

	/// [`MapDialogRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapdialogrect)
	/// method.
	pub fn MapDialogRect(self, lpRect: &mut RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::MapDialogRect(self.ptr, ref_as_pvoid(lpRect)) },
		)
	}

	/// [`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// method.
	///
	/// # Examples
	///
	/// A modal message box, which blocks its parent:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HWND};
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// my_hwnd.MessageBox("Hello, world", "title", co::MB::OKCANCEL | co::MB::ICONINFORMATION)
	///     .unwrap();
	/// ```
	///
	/// Usually the message box has a valid parent window, however, if for some
	/// reason you don't have a window to serve as parent, you still can show a
	/// non-modal, parent-less message box by retrieving the desktop handle:
	///
	/// ```rust,ignore
	/// HWND::GetDesktopWindow()
	///     .MessageBox("Hello, world", "Title", co::MB::ICONEXCLAMATION)
	///     .unwrap();
	/// ```
	pub fn MessageBox(self, lpText: &str,
		lpCaption: &str, uType: co::MB) -> WinResult<co::DLGID>
	{
		match unsafe {
			user32::MessageBoxW(
				self.ptr,
				WString::from_str(lpText).as_ptr(),
				WString::from_str(lpCaption).as_ptr(),
				uType.0,
			)
		} {
			0 => Err(GetLastError()),
			ret => Ok(co::DLGID(ret as _)),
		}
	}

	/// [`MoveWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-movewindow)
	/// method.
	pub fn MoveWindow(self,
		x: i32, y: i32, nWidth: i32, nHeight: i32, bRepaint: bool) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::MoveWindow(
					self.ptr,
					x, y,
					nWidth, nHeight,
					bRepaint as _,
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
	/// **Note:** Must be paired with a
	/// [`CloseThemeData`](crate::HTHEME::CloseThemeData) call.
	pub fn OpenThemeData(self, pszClassList: &str) -> Option<HTHEME> {
		unsafe {
			uxtheme::OpenThemeData(
				self.ptr,
				WString::from_str(pszClassList).as_ptr(),
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
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// my_hwnd.PostMessage(wm::Close {}).unwrap();
	/// ```
	///
	/// Sending a message to all top-level windows:
	///
	/// ```rust,ignore
	/// use winsafe::{HWND, msg::wm};
	///
	/// HWND::BROADCAST.PostMessage(
	///     wm::ExitMenuLoop { is_shortcut: false },
	/// ).unwrap();
	/// ```
	pub fn PostMessage<M: MsgSend>(self, uMsg: M) -> WinResult<()> {
		let wmAny = uMsg.as_generic_wm();
		bool_to_winresult(
			unsafe {
				user32::PostMessageW(
					self.ptr, wmAny.msg_id.0, wmAny.wparam, wmAny.lparam,
				)
			},
		)
	}

	/// [`RealChildWindowFromPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-realchildwindowfrompoint)
	/// method.
	pub fn RealChildWindowFromPoint(self,
		ptParentClientCoords: POINT) -> Option<HWND>
	{
		unsafe {
			user32::RealChildWindowFromPoint(
				self.ptr,
				ptParentClientCoords.x,
				ptParentClientCoords.y,
			).as_mut()
		}.map(|ptr| Self { ptr })
	}

	/// [`RedrawWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-redrawwindow)
	/// method.
	pub fn RedrawWindow(self,
		lprcUpdate: &RECT, hrgnUpdate: HRGN, flags: co::RDW) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::RedrawWindow(
					self.ptr,
					ref_as_pcvoid(lprcUpdate),
					hrgnUpdate.ptr,
					flags.0,
				)
			},
		)
	}

	/// [`ReleaseDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
	/// method.
	pub fn ReleaseDC(self, hDC: HDC) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::ReleaseDC(self.ptr, hDC.ptr) },
		)
	}

	/// [`RemoveWindowSubclass`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-removewindowsubclass)
	/// method.
	pub fn RemoveWindowSubclass(self,
		pfnSubclass: SUBCLASSPROC, uIdSubclass: usize) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				comctl32::RemoveWindowSubclass(
					self.ptr,
					pfnSubclass as _,
					uIdSubclass,
				)
			},
		)
	}

	/// [`ScreenToClient`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-screentoclient)
	/// method.
	pub fn ScreenToClient(self, lpPoint: &mut POINT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::ScreenToClient(self.ptr, ref_as_pvoid(lpPoint))
			},
		)
	}

	/// [`ScreenToClient`](crate::HWND::ScreenToClient) method for a
	/// [`RECT`](crate::RECT).
	pub fn ScreenToClientRc(self, lpRect: &mut RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::ScreenToClient(
					self.ptr,
					ref_as_pvoid(&mut lpRect.left),
				)
			},
		).and_then(|_| bool_to_winresult(
			unsafe {
				user32::ScreenToClient(
					self.ptr,
					ref_as_pvoid(&mut lpRect.right),
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
	/// Sending a [`LVM_SETITEM`](crate::msg::lvm::SetItem) list view message,
	/// which demands a reference to an [`LVITEM`](crate::LVITEM) object:
	///
	/// ```rust,ignore
	/// use winsafe::{co, HWND, LVITEM, msg::lvm};
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// let mut lvi = LVITEM::default(); // object to be sent
	/// lvi.mask = co::LVIF::IMAGE;
	/// lvi.iImage = 3;
	///
	/// my_hwnd.SendMessage(lvm::SetItem {
	///     lvitem: &lvi,
	/// });
	/// ```
	pub fn SendMessage<M: MsgSend>(self, uMsg: M) -> M::RetType {
		let wmAny = uMsg.as_generic_wm();
		uMsg.convert_ret(
			unsafe {
				user32::SendMessageW(
					self.ptr, wmAny.msg_id.0, wmAny.wparam, wmAny.lparam,
				)
			},
		)
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

	/// [`SetParent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setparent)
	/// method.
	pub fn SetParent(self, hWndNewParent: HWND) -> WinResult<Option<HWND>> {
		match unsafe {
			user32::SetParent(self.ptr, hWndNewParent.ptr).as_mut()
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
		nBar: co::SBB, lpsi: &SCROLLINFO, redraw: bool) -> i32
	{
		unsafe {
			user32::SetScrollInfo(
				self.ptr,
				nBar.0,
				ref_as_pcvoid(lpsi),
				redraw as _,
			)
		}
	}

	/// [`SetScrollPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setscrollpos)
	/// method.
	pub fn SetScrollPos(self,
		nBar: co::SBB, nPos: i32, bRedraw: bool) -> WinResult<i32>
	{
		match unsafe {
			user32::SetScrollPos(self.ptr, nBar.0, nPos, bRedraw as BOOL)
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
		nBar: co::SBB, nMinPos: i32, nMaxPos: i32, bRedraw: bool) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::SetScrollRange(
					self.ptr,
					nBar.0,
					nMinPos,
					nMaxPos,
					bRedraw as _,
				)
			},
		)
	}

	/// [`SetTimer`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-settimer)
	/// method.
	pub fn SetTimer(self,
		nIDEvent: usize, uElapse: u32,
		lpTimerFunc: Option<TIMERPROC>) -> WinResult<usize>
	{
		match unsafe {
			user32::SetTimer(
				self.ptr,
				nIDEvent,
				uElapse,
				lpTimerFunc.map_or(std::ptr::null(), |lp| lp as _),
			)
		} {
			0 => Err(GetLastError()),
			tid => Ok(tid),
		}
	}

	/// [`SetWindowDisplayAffinity`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity)
	/// method.
	pub fn SetWindowDisplayAffinity(self, dwAffinity: co::WDA) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::SetWindowDisplayAffinity(self.ptr, dwAffinity.0) },
		)
	}

	/// [`SetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
	/// method.
	pub fn SetWindowLongPtr(self, nIndex: co::GWLP, dwNewLong: isize) -> isize {
		unsafe { user32::SetWindowLongPtrW(self.ptr, nIndex.0, dwNewLong) }
	}

	/// [`SetWindowPlacement`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowplacement)
	/// method.
	pub fn SetWindowPlacement(self, lpwndpl: &WINDOWPLACEMENT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::SetWindowPlacement(self.ptr, ref_as_pcvoid(lpwndpl))
			},
		)
	}

	/// [`SetWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)
	/// method.
	pub fn SetWindowPos(self,
		hWndInsertAfter: HwndPlace,
		x: i32, y: i32, cx: i32, cy: i32, uFlags: co::SWP) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::SetWindowPos(
					self.ptr,
					hWndInsertAfter.as_ptr(),
					x, y, cx, cy,
					uFlags.0,
				)
			},
		)
	}

	/// [`SetWindowRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowrgn)
	/// method.
	pub fn SetWindowRgn(self, hRgn: HRGN, bRedraw: bool) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::SetWindowRgn(self.ptr, hRgn.ptr, bRedraw as _) },
		)
	}

	/// [`SetWindowSubclass`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass)
	/// method.
	pub fn SetWindowSubclass(self,
		pfnSubclass: SUBCLASSPROC,
		uIdSubclass: usize, dwRefData: usize) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				comctl32::SetWindowSubclass(
					self.ptr,
					pfnSubclass as _,
					uIdSubclass,
					dwRefData,
				)
			},
		)
	}

	/// [`SetWindowText`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
	/// method.
	pub fn SetWindowText(self, lpString: &str) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::SetWindowTextW(
					self.ptr,
					WString::from_str(lpString).as_ptr(),
				)
			},
		)
	}

	/// [`ShowCaret`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showcaret)
	/// method.
	pub fn ShowCaret(self) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::ShowCaret(self.ptr) })
	}

	/// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	/// method.
	pub fn ShowWindow(self, nCmdShow: co::SW) -> bool {
		unsafe { user32::ShowWindow(self.ptr, nCmdShow.0) != 0 }
	}

	/// [`TranslateAccelerator`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translateacceleratorw)
	/// method.
	pub fn TranslateAccelerator(self,
		hAccTable: HACCEL, lpMsg: &mut MSG) -> WinResult<()>
	{
		match unsafe {
			user32::TranslateAcceleratorW(
				self.ptr,
				hAccTable.ptr,
				ref_as_pvoid(lpMsg),
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
	pub fn ValidateRect(self, lpRect: &RECT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user32::ValidateRect(self.ptr, ref_as_pcvoid(lpRect))
			},
		)
	}

	/// [`ValidateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validatergn)
	/// method.
	pub fn ValidateRgn(self, hRgn: HRGN) -> WinResult<()> {
		bool_to_winresult(unsafe { user32::ValidateRgn(self.ptr, hRgn.ptr) })
	}

	/// [`WindowFromPhysicalPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfromphysicalpoint)
	/// static method.
	pub fn WindowFromPhysicalPoint(point: POINT) -> Option<HWND> {
		unsafe { user32::WindowFromPhysicalPoint(point.x, point.y).as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`WindowFromPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfrompoint)
	/// static method.
	pub fn WindowFromPoint(point: POINT) -> Option<HWND> {
		unsafe { user32::WindowFromPoint(point.x, point.y).as_mut() }
			.map(|ptr| Self { ptr })
	}

	/// [`WinHelp`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-winhelpw)
	/// method.
	pub fn WinHelp(self,
		lpszHelp: &str, uCommand: co::HELPW, dwData: usize) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				user32::WinHelpW(
					self.ptr,
					WString::from_str(lpszHelp).as_ptr(),
					uCommand.0,
					dwData,
				)
			},
		)
	}
}
