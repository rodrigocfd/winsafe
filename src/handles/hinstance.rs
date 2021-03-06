#![allow(non_snake_case)]

use crate::aliases::{DLGPROC, WinResult};
use crate::co;
use crate::enums::{IdIdcStr, IdIdiStr, IdStr};
use crate::ffi::{kernel32, user32};
use crate::funcs::GetLastError;
use crate::handles::{HACCEL, HBITMAP, HCURSOR, HICON, HWND};
use crate::privs::ptr_as_opt;
use crate::structs::{ATOM, WNDCLASSEX};
use crate::WString;

handle_type! {
	/// Handle to an
	/// [instance](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance),
	/// same as `HMODULE`.
	HINSTANCE
}

impl HINSTANCE {
	/// Returns a handle to the system OEM instance. This is used to load
	/// built-in system resources.
	pub fn oem() -> HINSTANCE {
		unsafe { Self::null_handle() }
	}

	/// [`CreateDialogParam`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdialogparamw)
	/// method.
	pub fn CreateDialogParam(
		self,
		lpTemplateName: IdStr,
		hWndParent: Option<HWND>,
		lpDialogFunc: DLGPROC,
		dwInitParam: Option<isize>) -> WinResult<HWND>
	{
		match ptr_as_opt(
			unsafe {
				user32::CreateDialogParamW(
					self.ptr,
					lpTemplateName.as_ptr(),
					match hWndParent {
						Some(hParent) => hParent.ptr,
						None => std::ptr::null_mut(),
					},
					lpDialogFunc as *const _,
					dwInitParam.unwrap_or_default(),
				)
			},
		) {
			Some(ptr) => Ok(HWND { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`DialogBoxParam`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dialogboxparamw)
	/// method.
	pub fn DialogBoxParam(
		self,
		lpTemplateName: IdStr,
		hWndParent: Option<HWND>,
		lpDialogFunc: DLGPROC,
		dwInitParam: Option<isize>) -> WinResult<isize>
	{
		match unsafe {
			user32::DialogBoxParamW(
				self.ptr,
				lpTemplateName.as_ptr(),
				match hWndParent {
					Some(hParent) => hParent.ptr,
					None => std::ptr::null_mut(),
				},
				lpDialogFunc as *const _,
				dwInitParam.unwrap_or_default(),
			)
		} {
			-1 => Err(GetLastError()),
			res => Ok(res), // assumes hWndParent as valid, so no check for zero
		}
	}

	/// [`GetClassInfoEx`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclassinfoexw)
	/// method.
	///
	/// # Examples
	///
	/// Retrieving information of a window class created in our application:
	/// ```rust,ignore
	/// use winsafe::{HINSTANCE, WNDCLASSEX};
	///
	/// let mut wcx = WNDCLASSEX::default();
	/// HINSTANCE::GetModuleHandle(None).unwrap()
	///     .GetClassInfoEx("SOME_CLASS_NAME", &mut wcx).unwrap();
	/// ```
	pub fn GetClassInfoEx(self,
		lpszClass: &str, lpwcx: &mut WNDCLASSEX) -> WinResult<ATOM>
	{
		match unsafe {
			user32::GetClassInfoExW(
				self.ptr,
				WString::from_str(lpszClass).as_ptr(),
				lpwcx as *mut _ as *mut _,
			)
		} {
			0 => Err(GetLastError()),
			atom => Ok(ATOM(atom as u16)),
		}
	}

	/// [`GetModuleHandle`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
	/// static method.
	///
	/// # Examples
	///
	/// Retrieving current module instance:
	/// ```rust,ignore
	/// use winsafe::HINSTANCE;
	///
	/// let hinstance = HINSTANCE::GetModuleHandle(None).unwrap();
	/// ```
	pub fn GetModuleHandle(
		lpModuleName: Option<&str>) -> WinResult<HINSTANCE>
	{
		match ptr_as_opt(
			unsafe {
				kernel32::GetModuleHandleW(
					WString::from_opt_str(lpModuleName).as_ptr()
				)
			},
		) {
			Some(ptr) => Ok(HINSTANCE { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`LoadAccelerators`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadacceleratorsw)
	/// method.
	pub fn LoadAccelerators(self, lpTableName: IdStr) -> WinResult<HACCEL> {
		match ptr_as_opt(
			unsafe {
				user32::LoadAcceleratorsW(self.ptr, lpTableName.as_ptr())
			},
		) {
			Some(ptr) => Ok(HACCEL { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`LoadCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
	/// method.
	///
	/// # Examples
	///
	/// Loading a system cursor:
	/// ```rust,ignore
	/// use winsafe::{co, HINSTANCE, IdIdcStr};
	///
	/// let sys_cursor = HINSTANCE::default()
	///     .LoadCursor(IdIdcStr::Idc(co::IDC::ARROW))
	///     .unwrap();
	/// ```
	pub fn LoadCursor(self, lpCursorName: IdIdcStr) -> WinResult<HCURSOR> {
		match ptr_as_opt(
			unsafe { user32::LoadCursorW(self.ptr, lpCursorName.as_ptr()) },
		) {
			Some(ptr) => Ok(HCURSOR { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`LoadIcon`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadiconw)
	/// method.
	///
	/// # Examples
	///
	/// Loading a system icon:
	/// ```rust,ignore
	/// use winsafe::{co, IdIdiStr, HINSTANCE};
	///
	/// let sys_icon = HINSTANCE::default()
	///     .LoadIcon(IdIdiStr::Idi(co::IDI::INFORMATION))
	///     .unwrap();
	/// ```
	pub fn LoadIcon(self, lpIconName: IdIdiStr) -> WinResult<HICON> {
		match ptr_as_opt(
			unsafe { user32::LoadIconW(self.ptr, lpIconName.as_ptr()) },
		) {
			Some(ptr) => Ok(HICON { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HBITMAP`](crate::HBITMAP).
	pub fn LoadImageBitmap(self,
		name: IdStr, cx: i32, cy: i32, fuLoad: co::LR) -> WinResult<HBITMAP>
	{
		match ptr_as_opt(
			unsafe {
				user32::LoadImageW(self.ptr, name.as_ptr(), 0, cx, cy, fuLoad.0)
			},
		) {
			Some(ptr) => Ok(HBITMAP { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HCURSOR`](crate::HCURSOR).
	pub fn LoadImageCursor(self,
		name: IdStr, cx: i32, cy: i32, fuLoad: co::LR) -> WinResult<HCURSOR>
	{
		match ptr_as_opt(
			unsafe {
				user32::LoadImageW(self.ptr, name.as_ptr(), 2, cx, cy, fuLoad.0)
			},
		) {
			Some(ptr) => Ok(HCURSOR { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HICON`](crate::HICON).
	pub fn LoadImageIcon(self,
		name: IdStr, cx: i32, cy: i32, fuLoad: co::LR) -> WinResult<HICON>
	{
		match ptr_as_opt(
			unsafe {
				user32::LoadImageW(self.ptr, name.as_ptr(), 1, cx, cy, fuLoad.0)
			},
		) {
			Some(ptr) => Ok(HICON { ptr }),
			None => Err(GetLastError()),
		}
	}
}
