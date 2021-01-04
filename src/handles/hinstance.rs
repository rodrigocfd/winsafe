#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::co;
use crate::aliases::DLGPROC;
use crate::enums::{IdIdcStr, IdIdiStr, IdStr};
use crate::ffi::{kernel32, user32};
use crate::funcs::GetLastError;
use crate::handles::{HACCEL, HBITMAP, HCURSOR, HICON, HWND};
use crate::priv_funcs::{mut_void, ptr_as_opt};
use crate::structs::{ATOM, WNDCLASSEX};
use crate::WString;

handle_type! {
	/// Handle to an
	/// [instance](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance),
	/// same as `HMODULE`. Exposes methods.
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
		dwInitParam: Option<isize>) -> Result<HWND, co::ERROR>
	{
		match ptr_as_opt(
			unsafe {
				user32::CreateDialogParamW(
					self.0,
					lpTemplateName.as_ptr(),
					match hWndParent {
						Some(hParent) => hParent.as_ptr(),
						None => std::ptr::null_mut(),
					},
					lpDialogFunc as *const c_void,
					dwInitParam.unwrap_or_default(),
				)
			}
		) {
			Some(p) => Ok(unsafe { HWND::from_ptr(p) }),
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
		dwInitParam: Option<isize>) -> Result<isize, co::ERROR>
	{
		match unsafe {
			user32::DialogBoxParamW(
				self.0,
				lpTemplateName.as_ptr(),
				match hWndParent {
					Some(hParent) => hParent.as_ptr(),
					None => std::ptr::null_mut(),
				},
				lpDialogFunc as *const c_void,
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
	/// let mut wcx = WNDCLASSEX::default();
	/// HINSTANCE::GetModuleHandle(None).unwrap()
	///   .GetClassInfoEx("SOME_CLASS_NAME", &mut wcx).unwrap();
	/// ```
	pub fn GetClassInfoEx(self,
		lpszClass: &str, lpwcx: &mut WNDCLASSEX) -> Result<ATOM, co::ERROR>
	{
		match unsafe {
			user32::GetClassInfoExW(
				self.0, WString::from_str(lpszClass).as_ptr(), mut_void(lpwcx))
		} {
			0 => Err(GetLastError()),
			atom => Ok(ATOM::from(atom as u16)),
		}
	}

	/// [`GetModuleHandle`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)
	/// static method.
	///
	/// # Examples
	///
	/// Retrieving current module instance:
	/// ```rust,ignore
	/// let hinstance = HINSTANCE::GetModuleHandle(None).unwrap();
	/// ```
	pub fn GetModuleHandle(
		lpModuleName: Option<&str>) -> Result<HINSTANCE, co::ERROR>
	{
		match ptr_as_opt(
			unsafe {
				kernel32::GetModuleHandleW(
					WString::from_opt_str(lpModuleName).as_ptr()
				)
			}
		) {
			Some(p) => Ok(unsafe { HINSTANCE::from_ptr(p) }),
			None => Err(GetLastError()),
		}
	}

	/// [`LoadAccelerators`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadacceleratorsw)
	/// method.
	pub fn LoadAccelerators(self,
		lpTableName: IdStr) -> Result<HACCEL, co::ERROR>
	{
		match ptr_as_opt(
			unsafe {
				user32::LoadAcceleratorsW(self.0, lpTableName.as_ptr())
			}
		) {
			Some(p) => Ok(unsafe { HACCEL::from_ptr(p) }),
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
	/// let sys_cursor = HINSTANCE::default()
	///   .LoadCursor(IdIdcStr::Idc(co::IDC::ARROW))
	///   .unwrap();
	/// ```
	pub fn LoadCursor(self,
		lpCursorName: IdIdcStr) -> Result<HCURSOR, co::ERROR>
	{
		match ptr_as_opt(
			unsafe { user32::LoadCursorW(self.0, lpCursorName.as_ptr()) }
		) {
			Some(p) => Ok(unsafe { HCURSOR::from_ptr(p) }),
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
	/// let sys_icon = HINSTANCE::default()
	///   .LoadIcon(IdIdiStr::Idi(co::IDI::INFORMATION))
	///   .unwrap();
	/// ```
	pub fn LoadIcon(self,
		lpIconName: IdIdiStr) -> Result<HICON, co::ERROR>
	{
		match ptr_as_opt(
			unsafe { user32::LoadIconW(self.0, lpIconName.as_ptr()) }
		) {
			Some(p) => Ok(unsafe { HICON::from_ptr(p) }),
			None => Err(GetLastError()),
		}
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HBITMAP`](crate::HBITMAP).
	pub fn LoadImageBitmap(self,
		name: IdStr, cx: i32, cy: i32, fuLoad: co::LR) -> Result<HBITMAP, co::ERROR>
	{
		match ptr_as_opt(
			unsafe {
				user32::LoadImageW(self.0, name.as_ptr(), 0, cx, cy, fuLoad.into())
			}
		) {
			Some(p) => Ok(unsafe { HBITMAP::from_ptr(p) }),
			None => Err(GetLastError()),
		}
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HCURSOR`](crate::HCURSOR).
	pub fn LoadImageCursor(self,
		name: IdStr, cx: i32, cy: i32, fuLoad: co::LR) -> Result<HCURSOR, co::ERROR>
	{
		match ptr_as_opt(
			unsafe {
				user32::LoadImageW(self.0, name.as_ptr(), 2, cx, cy, fuLoad.into())
			}
		) {
			Some(p) => Ok(unsafe { HCURSOR::from_ptr(p) }),
			None => Err(GetLastError()),
		}
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HICON`](crate::HICON).
	pub fn LoadImageIcon(self,
		name: IdStr, cx: i32, cy: i32, fuLoad: co::LR) -> Result<HICON, co::ERROR>
	{
		match ptr_as_opt(
			unsafe {
				user32::LoadImageW(self.0, name.as_ptr(), 1, cx, cy, fuLoad.into())
			}
		) {
			Some(p) => Ok(unsafe { HICON::from_ptr(p) }),
			None => Err(GetLastError()),
		}
	}
}
