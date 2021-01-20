#![allow(non_snake_case)]

use crate::aliases::{SUBCLASSPROC, WinResult, WNDENUMPROC};
use crate::co;
use crate::enums::{AtomStr, HwndPlace, IdMenu, IdPos};
use crate::ffi::{comctl32, user32, uxtheme};
use crate::funcs::{GetLastError, SetLastError};
use crate::handles::{HACCEL, HDC, HINSTANCE, HMENU, HRGN, HTHEME};
use crate::msg::Message;
use crate::privs::{const_void, mut_void, ptr_as_opt};
use crate::structs::{MSG, PAINTSTRUCT, POINT, RECT, WINDOWINFO, WINDOWPLACEMENT};
use crate::WString;

handle_type! {
	/// Handle to a
	/// [window](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hwnd).
	/// Exposes methods.
	HWND
}

impl HWND {
	/// [`GetWindowLongPtr`](crate::HWND::GetWindowLongPtr) wrapper to retrieve
	/// the window [`HINSTANCE`](crate::HINSTANCE).
	pub fn hinstance(self) -> HINSTANCE {
		HINSTANCE { ptr: self.GetWindowLongPtr(co::GWLP::HINSTANCE) as *mut _ }
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
	/// **Note:** Must be paired with an [`EndPaint`](crate::HWND::EndPaint) call.
	pub fn BeginPaint(self, lpPaint: &mut PAINTSTRUCT) -> WinResult<HDC> {
		match ptr_as_opt(
			unsafe { user32::BeginPaint(self.ptr, mut_void(lpPaint)) },
		 ) {
			Some(ptr) => Ok(HDC { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`BringWindowToTop`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-bringwindowtotop)
	/// method.
	pub fn BringWindowToTop(self) -> WinResult<()> {
		match unsafe { user32::BringWindowToTop(self.ptr) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`ClientToScreen`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-clienttoscreen)
	/// method.
	pub fn ClientToScreen(self, lpPoint: &mut POINT) -> WinResult<()> {
		match unsafe { user32::ClientToScreen(self.ptr, mut_void(lpPoint)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`ClientToScreen`](crate::HWND::ClientToScreen) method for a
	/// [`RECT`](crate::RECT).
	pub fn ClientToScreenRc(self, lpRect: &mut RECT) -> WinResult<()> {
		match unsafe {
			user32::ClientToScreen(self.ptr, mut_void(&mut lpRect.left))
		} {
			0 => Err(GetLastError()),
			_ => match unsafe {
				user32::ClientToScreen(self.ptr, mut_void(&mut lpRect.right))
			} {
				0 => Err(GetLastError()),
				_ => Ok(()),
			},
		}
	}

	/// [`CloseWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindow)
	/// method.
	///
	/// Note that this method will actually minimize the window, not destroy it.
	pub fn CloseWindow(self) -> WinResult<()> {
		match unsafe { user32::CloseWindow(self.ptr) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
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
		match ptr_as_opt(
			unsafe {
				user32::CreateWindowExW(
					dwExStyle.into(),
					lpClassName.as_ptr(),
					WString::from_opt_str(lpWindowName).as_ptr(),
					dwStyle.into(),
					X, Y, nWidth, nHeight,
					match hWndParent {
						Some(hParent) => hParent.ptr,
						None => std::ptr::null_mut(),
					},
					hMenu.as_ptr(),
					hInstance.ptr,
					lpParam.unwrap_or_default() as *mut _,
				)
			},
		) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`DefSubclassProc`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-defsubclassproc)
	/// method.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`Message`](crate::msg::Message) trait. That means each
	/// message can define its own return type.
	pub fn DefSubclassProc<M: Message>(self, uMsg: M) -> M::RetType {
		let wmAny = uMsg.as_generic_wm();
		uMsg.convert_ret(
			unsafe {
				comctl32::DefSubclassProc(
					self.ptr, wmAny.msg_id.into(), wmAny.wparam, wmAny.lparam,
				)
			},
		)
	}

	/// [`DefWindowProc`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)
	/// method.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`Message`](crate::msg::Message) trait. That means each
	/// message can define its own return type.
	pub fn DefWindowProc<M: Message>(self, uMsg: M) -> M::RetType {
		let wmAny = uMsg.as_generic_wm();
		uMsg.convert_ret(
			unsafe {
				user32::DefWindowProcW(
					self.ptr, wmAny.msg_id.into(), wmAny.wparam, wmAny.lparam,
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
		unsafe { user32::EnableWindow(self.ptr, bEnable as i32) != 0 }
	}

	/// [`EndDialog`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enddialog)
	/// method.
	pub fn EndDialog(self, nResult: isize) -> WinResult<()> {
		match unsafe { user32::EndDialog(self.ptr, nResult) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`EndPaint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-endpaint)
	/// method.
	pub fn EndPaint(self, lpPaint: &PAINTSTRUCT) {
		unsafe { user32::EndPaint(self.ptr, const_void(lpPaint)); }
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
			user32::EnumChildWindows(self.ptr, lpEnumFunc as *const _, lParam);
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
	///   println!("HWND: {}", hchild);
	/// }
	/// ```
	pub fn EnumChildWindowsVec(self) -> Vec<HWND> {
		let mut hchildren = Vec::new();
		self.EnumChildWindows(
			Self::EnumChildWindowsVecProc,
			&mut hchildren as *mut Vec<_> as isize, // pass pointer to Vec
		);
		hchildren
	}
	extern "system" fn EnumChildWindowsVecProc(
		hchild: HWND, lparam: isize) -> i32
	{
		let hchildren = unsafe { &mut *(lparam as *mut Vec<HWND>) }; // retrieve pointer to Vec
		hchildren.push(hchild);
		true as i32
	}

	/// [`FindWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-findwindoww)
	/// static method.
	pub fn FindWindow(
		lpClassName: &str, lpWindowName: &str) -> WinResult<HWND>
	{
		match ptr_as_opt(
			unsafe {
				user32::FindWindowW(
					WString::from_str(lpClassName).as_ptr(),
					WString::from_str(lpWindowName).as_ptr(),
				)
			},
		) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`GetAncestor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getancestor)
	/// method.
	pub fn GetAncestor(self, gaFlags: co::GA) -> Option<HWND> {
		ptr_as_opt(unsafe { user32::GetAncestor(self.ptr, gaFlags.into()) })
			.map(|ptr| Self { ptr })
	}

	/// [`GetClassLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclasslongptrw)
	/// method.
	pub fn GetClassLongPtr(self, nIndex: co::GCLP) -> usize {
		unsafe { user32::GetClassLongPtrW(self.ptr, nIndex.into()) }
	}

	/// [`GetClientRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)
	/// method.
	pub fn GetClientRect(self) -> WinResult<RECT> {
		let mut rc = RECT::default();
		match unsafe { user32::GetClientRect(self.ptr, mut_void(&mut rc)) } {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdc)
	/// method.
	///
	/// **Note:** Must be paired with a [`ReleaseDC`](crate::HWND::ReleaseDC) call.
	pub fn GetDC(self) -> WinResult<HDC> {
		match ptr_as_opt(unsafe { user32::GetDC(self.ptr) }) {
			Some(ptr) => Ok(HDC { ptr }),
			None => Err(GetLastError()),
		}
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
		match ptr_as_opt(unsafe { user32::GetDlgItem(self.ptr, nIDDlgItem) }) {
			None => Err(GetLastError()),
			Some(ptr) => Ok(Self { ptr }),
		}
	}

	/// [`GetFocus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getfocus)
	/// static method.
	pub fn GetFocus() -> Option<HWND> {
		ptr_as_opt(unsafe { user32::GetFocus() })
			.map(|ptr| Self { ptr })
	}

	/// [`GetForegroundWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)
	/// static method.
	pub fn GetForegroundWindow() -> Option<HWND> {
		ptr_as_opt(unsafe { user32::GetForegroundWindow() })
			.map(|ptr| Self { ptr })
	}

	/// [`GetNextDlgGroupItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlggroupitem)
	/// method.
	pub fn GetNextDlgGroupItem(self,
		hCtl: HWND, bPrevious: bool) -> WinResult<HWND>
	{
		match ptr_as_opt(
			unsafe {
				user32::GetNextDlgGroupItem(self.ptr, hCtl.ptr, bPrevious as i32)
			},
		) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`GetNextDlgTabItem`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getnextdlgtabitem)
	/// method.
	pub fn GetNextDlgTabItem(self,
		hCtl: HWND, bPrevious: bool) -> WinResult<HWND>
	{
		match ptr_as_opt(
			unsafe {
				user32::GetNextDlgTabItem(self.ptr, hCtl.ptr, bPrevious as i32)
			},
		) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`GetParent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getparent)
	/// method.
	pub fn GetParent(self) -> WinResult<HWND> {
		match ptr_as_opt(unsafe { user32::GetParent(self.ptr) }) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`GetUpdateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getupdatergn)
	/// method.
	pub fn GetUpdateRgn(self, hRgn: HRGN, bErase: bool) -> WinResult<co::REGION> {
		match unsafe {
			user32::GetUpdateRgn(self.ptr, hRgn.ptr, bErase as i32)
		} {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION::from(ret)),
		}
	}

	/// [`GetWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	/// method.
	pub fn GetWindow(self, uCmd: co::GW) -> WinResult<HWND> {
		match ptr_as_opt(unsafe { user32::GetWindow(self.ptr, uCmd.into()) }) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`GetWindowDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdc)
	/// method.
	///
	/// **Note:** Must be paired with a [`ReleaseDC`](crate::HWND::ReleaseDC)
	/// call.
	pub fn GetWindowDC(self) -> WinResult<HDC> {
		match ptr_as_opt(unsafe { user32::GetWindowDC(self.ptr) }) {
			Some(ptr) => Ok(HDC { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`GetWindowDisplayAffinity`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowdisplayaffinity)
	/// method.
	pub fn GetWindowDisplayAffinity(self) -> WinResult<co::WDA> {
		let mut pdwAffinity = co::WDA::default();
		match unsafe {
			user32::GetWindowDisplayAffinity(self.ptr, mut_void(&mut pdwAffinity))
		} {
			0 => Err(GetLastError()),
			_ => Ok(pdwAffinity),
		}
	}

	/// [`GetWindowInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowinfo)
	/// method.
	pub fn GetWindowInfo(self, pwi: &mut WINDOWINFO) -> WinResult<()> {
		match unsafe { user32::GetWindowInfo(self.ptr, mut_void(pwi)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`GetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)
	/// method.
	pub fn GetWindowLongPtr(self, nIndex: co::GWLP) -> isize {
		unsafe { user32::GetWindowLongPtrW(self.ptr, nIndex.into()) }
	}

	/// [`GetWindowPlacement`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowplacement)
	/// method.
	pub fn GetWindowPlacement(self,
		lpwndpl: &mut WINDOWPLACEMENT) -> WinResult<()>
	{
		match unsafe { user32::GetWindowPlacement(self.ptr, mut_void(lpwndpl)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`GetWindowRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)
	/// method.
	pub fn GetWindowRect(self) -> WinResult<RECT> {
		let mut rc = RECT::default();
		match unsafe { user32::GetWindowRect(self.ptr, mut_void(&mut rc)) } {
			0 => Err(GetLastError()),
			_ => Ok(rc),
		}
	}

	/// [`GetWindowRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgn)
	/// method.
	pub fn GetWindowRgn(self, hRgn: HRGN) -> WinResult<co::REGION> {
		match unsafe { user32::GetWindowRgn(self.ptr, hRgn.ptr) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION::from(ret)),
		}
	}

	/// [`GetWindowRgnBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrgnbox)
	/// method.
	pub fn GetWindowRgnBox(self, lprc: &mut RECT) -> WinResult<co::REGION> {
		match unsafe { user32::GetWindowRgnBox(self.ptr, mut_void(lprc)) } {
			0 => Err(GetLastError()),
			ret => Ok(co::REGION::from(ret)),
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
	/// inconvenient of the manual conversion from `WString` to `String`.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::HWND;
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
	/// returns a `String` instead of requiring an external buffer.
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
		let mut buf = WString::new();
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
				self.ptr, hMenu.ptr, uIDHiliteItem.into(), uHilite.into(),
			) != 0
		}
	}

	/// [`InvalidateRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidaterect)
	/// method.
	///
	/// # Examples
	///
	/// Most of the time you'll just want update the entire client area:
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// my_hwnd.InvalidateRect(None, true)
	///   .unwrap();
	/// ```
	pub fn InvalidateRect(self,
		lpRect: Option<&RECT>, bErase: bool) -> WinResult<()>
	{
		match unsafe {
			user32::InvalidateRect(
				self.ptr,
				lpRect.map_or(
					std::ptr::null(),
					|lpRect| const_void(lpRect),
				),
				bErase as i32,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`InvalidateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invalidatergn)
	/// method.
	pub fn InvalidateRgn(self, hRgn: HRGN, bErase: bool) {
		unsafe { user32::InvalidateRgn(self.ptr, hRgn.ptr, bErase as i32); }
	}

	/// [`IsChild`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ischild)
	/// method.
	pub fn IsChild(self, hWndPossibleChild: HWND) -> bool {
		unsafe { user32::IsChild(self.ptr, hWndPossibleChild.ptr) != 0 }
	}

	/// [`IsDialogMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isdialogmessagew)
	/// method.
	pub fn IsDialogMessage(self, lpMsg: &mut MSG) -> bool {
		unsafe { user32::IsDialogMessageW(self.ptr, mut_void(lpMsg)) != 0 }
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

	/// [`MapDialogRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mapdialogrect)
	/// method.
	pub fn MapDialogRect(self, lpRect: &mut RECT) -> WinResult<()> {
		match unsafe { user32::MapDialogRect(self.ptr, mut_void(lpRect)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	/// method.
	///
	/// # Examples
	///
	/// A modal message box, which blocks its parent:
	///
	/// ```rust,ignore
	/// use winsafe::HWND;
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// my_hwnd.MessageBox("Hello, world", "title", co::MB::OKCANCEL | co::MB::ICONINFORMATION)
	///   .unwrap();
	/// ```
	///
	/// Usually the message box has a valid parent window, however, if for some
	/// reason you don't have a window to serve as parent, you still can show a
	/// non-modal, parent-less message box by retrieving the desktop handle:
	///
	/// ```rust,ignore
	/// HWND::GetDesktopWindow()
	///   .MessageBox("Hello, world", "Title", co::MB::ICONEXCLAMATION)
	///   .unwrap();
	/// ```
	pub fn MessageBox(self, lpText: &str,
		lpCaption: &str, uType: co::MB) -> WinResult<co::DLGID>
	{
		match unsafe {
			user32::MessageBoxW(
				self.ptr,
				WString::from_str(lpText).as_ptr(),
				WString::from_str(lpCaption).as_ptr(),
				uType.into(),
			)
		} {
			0 => Err(GetLastError()),
			ret => Ok(co::DLGID::from(ret as u16)),
		}
	}

	/// [`OpenThemeData`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-openthemedata)
	/// method.
	///
	/// **Note:** Must be paired with a
	/// [`CloseThemeData`](crate::HTHEME::CloseThemeData) call.
	pub fn OpenThemeData(self, pszClassList: &str) -> Option<HTHEME> {
		ptr_as_opt(
			unsafe {
				uxtheme::OpenThemeData(
					self.ptr,
					WString::from_str(pszClassList).as_ptr(),
				)
			},
		).map(|ptr| HTHEME { ptr })
	}

	/// [`PostMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postmessagew)
	/// method. Note that this method is asychronous.
	///
	/// **Note:** To use `HWND_BROADCAST` or `NULL` as the first argument, see
	/// the [`PostMessage`](crate::PostMessage) free function.
	///
	/// # Examples
	///
	/// Programatically closing a window:
	///
	/// ```rust,ignore
	/// use winsafe::{HWND, msg::WmClose};
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// my_hwnd.PostMessage(WmClose {}).unwrap();
	/// ```
	pub fn PostMessage<M: Message>(self, uMsg: M) -> WinResult<()> {
		let wmAny = uMsg.as_generic_wm();
		match unsafe {
			user32::PostMessageW(
				self.ptr, wmAny.msg_id.into(), wmAny.wparam, wmAny.lparam,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`ReleaseDC`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-releasedc)
	/// method.
	pub fn ReleaseDC(self, hDC: HDC) -> WinResult<()> {
		match unsafe { user32::ReleaseDC(self.ptr, hDC.ptr) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`RemoveWindowSubclass`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-removewindowsubclass)
	/// method.
	pub fn RemoveWindowSubclass(self,
		pfnSubclass: SUBCLASSPROC, uIdSubclass: usize) -> WinResult<()>
	{
		match unsafe {
			comctl32::RemoveWindowSubclass(
				self.ptr, pfnSubclass as *const _, uIdSubclass,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`ScreenToClient`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-screentoclient)
	/// method;
	pub fn ScreenToClient(self, lpPoint: &mut POINT) -> WinResult<()> {
		match unsafe { user32::ScreenToClient(self.ptr, mut_void(lpPoint)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`ScreenToClient`](crate::HWND::ScreenToClient) method for a
	/// [`RECT`](crate::RECT).
	pub fn ScreenToClientRc(self, lpRect: &mut RECT) -> WinResult<()> {
		match unsafe {
			user32::ScreenToClient(self.ptr, mut_void(&mut lpRect.left))
		} {
			0 => Err(GetLastError()),
			_ => match unsafe {
				user32::ScreenToClient(self.ptr, mut_void(&mut lpRect.right))
			} {
				0 => Err(GetLastError()),
				_ => Ok(()),
			},
		}
	}

	/// [`SendMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)
	/// method.
	///
	/// The return type is variable, being defined by the `RetType` associated
	/// type of the [`Message`](crate::msg::Message) trait. That means each
	/// message can define its own return type.
	///
	/// # Examples
	///
	/// Sending a [`LVM_SETITEM`](crate::msg::LvmSetItem) list view message,
	/// which demands a reference to an [`LVITEM`](crate::LVITEM) object:
	///
	/// ```rust,ignore
	/// use winsafe::co;
	/// use winsafe::msg::LvmSetItem;
	/// use winsafe::{LVITEM, HWND};
	///
	/// let my_hwnd: HWND; // initialize it somewhere...
	///
	/// let mut lvi = LVITEM::default(); // object to be sent
	/// lvi.mask = co::LVIF::IMAGE;
	/// lvi.iImage = 3;
	///
	/// my_hwnd.SendMessage(LvmSetItem {
	///   lvitem: &lvi,
	/// });
	/// ```
	pub fn SendMessage<M: Message>(self, uMsg: M) -> M::RetType {
		let wmAny = uMsg.as_generic_wm();
		uMsg.convert_ret(
			unsafe {
				user32::SendMessageW(
					self.ptr, wmAny.msg_id.into(), wmAny.wparam, wmAny.lparam,
				)
			},
		)
	}

	/// [`SetFocus`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setfocus)
	/// method.
	pub fn SetFocus(self) -> Option<HWND> {
		ptr_as_opt(unsafe { user32::SetFocus(self.ptr) })
			.map(|ptr| Self { ptr })
	}

	/// [`SetParent`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setparent)
	/// method.
	pub fn SetParent(self, hWndNewParent: HWND) -> WinResult<Option<HWND>> {
		match ptr_as_opt(
			unsafe { user32::SetParent(self.ptr, hWndNewParent.ptr) }
		) {
			Some(ptr) => Ok(Some(Self { ptr })),
			None => match GetLastError() {
				co::ERROR::SUCCESS => Ok(None), // no previous parent
				err => Err(err),
			},
		}
	}

	/// [`SetWindowDisplayAffinity`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity)
	/// method.
	pub fn SetWindowDisplayAffinity(self, dwAffinity: co::WDA) -> WinResult<()> {
		match unsafe {
			user32::SetWindowDisplayAffinity(self.ptr, dwAffinity.into())
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`SetWindowLongPtr`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowlongptrw)
	/// method.
	pub fn SetWindowLongPtr(self, nIndex: co::GWLP, dwNewLong: isize) -> isize {
		unsafe { user32::SetWindowLongPtrW(self.ptr, nIndex.into(), dwNewLong) }
	}

	/// [`SetWindowPlacement`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowplacement)
	/// method.
	pub fn SetWindowPlacement(self, lpwndpl: &WINDOWPLACEMENT) -> WinResult<()> {
		match unsafe {
			user32::SetWindowPlacement(self.ptr, const_void(lpwndpl))
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`SetWindowPos`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)
	/// method.
	pub fn SetWindowPos(self,
		hWndInsertAfter: HwndPlace,
		X: i32, Y: i32, cx: u32, cy: u32, uFlags: co::SWP) -> WinResult<()>
	{
		match unsafe {
			user32::SetWindowPos(
				self.ptr, hWndInsertAfter.as_ptr(),
				X, Y, cx as i32, cy as i32, uFlags.into(),
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`SetWindowRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowrgn)
	/// method.
	pub fn SetWindowRgn(self, hRgn: HRGN, bRedraw: bool) -> WinResult<()> {
		match unsafe {
			user32::SetWindowRgn(self.ptr, hRgn.ptr, bRedraw as i32)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`SetWindowSubclass`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass)
	/// method.
	pub fn SetWindowSubclass(self,
		pfnSubclass: SUBCLASSPROC,
		uIdSubclass: usize, dwRefData: usize) -> WinResult<()>
	{
		match unsafe {
			comctl32::SetWindowSubclass(
				self.ptr, pfnSubclass as *const _, uIdSubclass, dwRefData,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`SetWindowText`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)
	/// method.
	pub fn SetWindowText(self, lpString: &str) -> WinResult<()> {
		match unsafe {
			user32::SetWindowTextW(self.ptr, WString::from_str(lpString).as_ptr())
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`ShowWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)
	/// method.
	pub fn ShowWindow(self, nCmdShow: co::SW) -> bool {
		unsafe { user32::ShowWindow(self.ptr, nCmdShow.into()) != 0 }
	}

	/// [`TranslateAccelerator`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translateacceleratorw)
	/// method.
	pub fn TranslateAccelerator(self,
		hAccTable: HACCEL, lpMsg: &mut MSG) -> WinResult<()>
	{
		match unsafe {
			user32::TranslateAcceleratorW(self.ptr, hAccTable.ptr, mut_void(lpMsg))
		} {
			0 => Err(GetLastError()),
			_ => Ok(())
		}
	}

	/// [`UpdateWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-updatewindow)
	/// method.
	pub fn UpdateWindow(self) -> WinResult<()> {
		match unsafe { user32::UpdateWindow(self.ptr) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`ValidateRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validaterect)
	/// method.
	pub fn ValidateRect(self, lpRect: &RECT) -> WinResult<()> {
		match unsafe { user32::ValidateRect(self.ptr, const_void(lpRect)) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`ValidateRgn`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-validatergn)
	/// method.
	pub fn ValidateRgn(self, hRgn: HRGN) -> WinResult<()> {
		match unsafe { user32::ValidateRgn(self.ptr, hRgn.ptr) } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}
}
