#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::{ATOM, HCURSOR, HICON, WNDCLASSEX};
use crate::{IdIdcStr, IdIdiStr};
use crate::co;
use crate::ffi::{user32, kernel32};
use crate::Utf16;

handle_type! {
	/// Handle to an
	/// [instance](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance).
	/// Same as `HMODULE`.
	HINSTANCE
}

impl HINSTANCE {
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
	pub fn GetClassInfoEx(&self,
		lpszClass: &str, lpwcx: &mut WNDCLASSEX) -> Result<ATOM, co::ERROR>
	{
		match unsafe {
			user32::GetClassInfoExW(
				self.0,
				Utf16::from_str(lpszClass).as_ptr(),
				lpwcx as *mut WNDCLASSEX as *mut c_void,
			)
		} {
			0 => Err(co::ERROR::GetLastError()),
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
		match ptr_to_opt!(
			kernel32::GetModuleHandleW(
				Utf16::from_opt_str(lpModuleName).as_ptr(),
			)
		) {
			Some(p) => Ok(HINSTANCE(p)),
			None => Err(co::ERROR::GetLastError()),
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
	pub fn LoadCursor(
		&self, lpCursorName: IdIdcStr) -> Result<HCURSOR, co::ERROR>
	{
		let mut buf16 = Utf16::default();

		match ptr_to_opt!(
			user32::LoadCursorW(self.0, lpCursorName.as_ptr(&mut buf16))
		) {
			Some(p) => Ok(unsafe { HCURSOR::from_ptr(p) }),
			None => Err(co::ERROR::GetLastError()),
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
	pub fn LoadIcon(
		&self, lpIconName: IdIdiStr) -> Result<HICON, co::ERROR>
	{
		let mut buf16 = Utf16::default();

		match ptr_to_opt!(
			user32::LoadIconW(self.0, lpIconName.as_ptr(&mut buf16))
		) {
			Some(p) => Ok(unsafe { HICON::from_ptr(p) }),
			None => Err(co::ERROR::GetLastError()),
		}
	}
}