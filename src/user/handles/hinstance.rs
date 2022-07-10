#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::{GetLastError, HINSTANCE, IdStr, WinResult, WString};
use crate::prelude::Handle;
use crate::user::decl::{
	ATOM, DLGPROC, HACCEL, HBITMAP, HCURSOR, HICON, HMENU, HWND, IdIdcStr,
	IdIdiStr, IdObmStr, IdOcrStr, IdOicStr, SIZE, WNDCLASSEX,
};

impl user_Hinstance for HINSTANCE {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HINSTANCE`](crate::HINSTANCE).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub trait user_Hinstance: Handle {
	/// [`CreateDialogParam`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdialogparamw)
	/// method.
	fn CreateDialogParam(
		self,
		resource_id: IdStr,
		hwnd_parent: Option<HWND>,
		dialog_proc: DLGPROC,
		init_param: Option<isize>) -> WinResult<HWND>
	{
		unsafe {
			user::ffi::CreateDialogParamW(
				self.as_ptr(),
				resource_id.as_ptr(),
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.0),
				dialog_proc as _,
				init_param.unwrap_or_default(),
			).as_mut()
		}.map(|ptr| HWND(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`DialogBoxParam`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dialogboxparamw)
	/// method.
	fn DialogBoxParam(
		self,
		resource_id: IdStr,
		hwnd_parent: Option<HWND>,
		dialog_proc: DLGPROC,
		init_param: Option<isize>) -> WinResult<isize>
	{
		match unsafe {
			user::ffi::DialogBoxParamW(
				self.as_ptr(),
				resource_id.as_ptr(),
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.0),
				dialog_proc as _,
				init_param.unwrap_or_default(),
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
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HINSTANCE, WNDCLASSEX};
	///
	/// let mut wcx = WNDCLASSEX::default();
	/// HINSTANCE::GetModuleHandle(None)?
	///     .GetClassInfoEx("SOME_CLASS_NAME", &mut wcx)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn GetClassInfoEx(self,
		class_name: &str, wcx: &mut WNDCLASSEX) -> WinResult<ATOM>
	{
		match unsafe {
			user::ffi::GetClassInfoExW(
				self.as_ptr(),
				WString::from_str(class_name).as_ptr(),
				wcx as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			atom => Ok(ATOM(atom as _)),
		}
	}

	/// [`LoadAccelerators`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadacceleratorsw)
	/// method.
	#[must_use]
	fn LoadAccelerators(self, table_name: IdStr) -> WinResult<HACCEL> {
		unsafe {
			user::ffi::LoadAcceleratorsW(self.as_ptr(), table_name.as_ptr())
				.as_mut()
		}.map(|ptr| HACCEL(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
	/// method.
	///
	/// # Examples
	///
	/// Loading a system cursor:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HINSTANCE, IdIdcStr};
	///
	/// let sys_cursor = HINSTANCE::NULL
	///     .LoadCursor(IdIdcStr::Idc(co::IDC::ARROW))?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn LoadCursor(self, resource_id: IdIdcStr) -> WinResult<HCURSOR> {
		unsafe {
			user::ffi::LoadCursorW(self.as_ptr(), resource_id.as_ptr()).as_mut()
		}.map(|ptr| HCURSOR(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadIcon`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadiconw)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HICON::DestroyIcon`](crate::prelude::user_Hicon::DestroyIcon) call.
	///
	/// # Examples
	///
	/// Loading a system icon:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, IdIdiStr, HINSTANCE};
	///
	/// let sys_icon = HINSTANCE::NULL
	///     .LoadIcon(IdIdiStr::Idi(co::IDI::INFORMATION))?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn LoadIcon(self, icon_id: IdIdiStr) -> WinResult<HICON> {
		unsafe {
			user::ffi::LoadIconW(self.as_ptr(), icon_id.as_ptr()).as_mut()
		}.map(|ptr| HICON(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HBITMAP`](crate::HBITMAP).
	///
	/// **Note:** Must be paired with an
	/// [`HBITMAP::DeleteObject`](crate::prelude::gdi_Hgdiobj::DeleteObject)
	/// call.
	#[must_use]
	fn LoadImageBitmap(self,
		name: IdObmStr, sz: SIZE, load: co::LR) -> WinResult<HBITMAP>
	{
		unsafe {
			user::ffi::LoadImageW(
				self.as_ptr(), name.as_ptr(), 0, sz.cx, sz.cy, load.0,
			).as_mut()
		}.map(|ptr| HBITMAP(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HCURSOR`](crate::HCURSOR).
	///
	/// **Note:** Must be paired with an
	/// [`HCURSOR::DestroyCursor`](crate::prelude::user_Hcursor::DestroyCursor)
	/// call.
	#[must_use]
	fn LoadImageCursor(self,
		name: IdOcrStr, sz: SIZE, load: co::LR) -> WinResult<HCURSOR>
	{
		unsafe {
			user::ffi::LoadImageW(
				self.as_ptr(), name.as_ptr(), 2, sz.cx, sz.cy, load.0,
			).as_mut()
		}.map(|ptr| HCURSOR(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HICON`](crate::HICON).
	///
	/// **Note:** Must be paired with an
	/// [`HICON::DestroyIcon`](crate::prelude::user_Hicon::DestroyIcon) call.
	#[must_use]
	fn LoadImageIcon(self,
		name: IdOicStr, sz: SIZE, load: co::LR) -> WinResult<HICON>
	{
		unsafe {
			user::ffi::LoadImageW(
				self.as_ptr(), name.as_ptr(), 1, sz.cx, sz.cy, load.0,
			).as_mut()
		}.map(|ptr| HICON(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadmenuw)
	/// method.
	#[must_use]
	fn LoadMenu(self, resource_id: IdStr) -> WinResult<HMENU> {
		unsafe {
			user::ffi::LoadMenuW(self.as_ptr(), resource_id.as_ptr()).as_mut()
		}.map(|ptr| HMENU(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadString`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadstringw)
	/// method.
	#[must_use]
	fn LoadString(self, id: u16) -> WinResult<String> {
		let mut pdata: *const u16 = std::ptr::null_mut();
		match unsafe {
			user::ffi::LoadStringW(
				self.as_ptr(),
				id as _,
				&mut pdata as *mut _ as  _, 0,
			)
		} {
			0 => Err(GetLastError()),
			len => Ok(WString::from_wchars_count(pdata, len as _).to_string())
		}
	}
}
