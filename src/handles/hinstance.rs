#![allow(non_snake_case)]

use crate::aliases::{DLGPROC, WinResult};
use crate::co;
use crate::enums::{IdIdcStr, IdIdiStr, IdStr, RtStr};
use crate::ffi::{BOOL, kernel32, user32};
use crate::funcs::GetLastError;
use crate::handles::{HACCEL, HBITMAP, HCURSOR, HICON, HMENU, HWND};
use crate::privs::{bool_to_winresult, IS_INTRESOURCE, MAX_PATH, str_to_iso88591};
use crate::structs::{ATOM, WNDCLASSEX};
use crate::various::WString;

pub_struct_handle! {
	/// Handle to an
	/// [instance](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hinstance),
	/// same as `HMODULE`.
	HINSTANCE
}

impl HINSTANCE {
	/// [`CreateDialogParam`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdialogparamw)
	/// method.
	pub fn CreateDialogParam(
		self,
		lpTemplateName: u16,
		hWndParent: Option<HWND>,
		lpDialogFunc: DLGPROC,
		dwInitParam: Option<isize>) -> WinResult<HWND>
	{
		unsafe {
			user32::CreateDialogParamW(
				self.ptr,
				lpTemplateName as _,
				hWndParent.map_or(std::ptr::null_mut(), |h| h.ptr),
				lpDialogFunc as _,
				dwInitParam.unwrap_or_default(),
			).as_mut()
		}.map(|ptr| HWND { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`DialogBoxParam`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dialogboxparamw)
	/// method.
	pub fn DialogBoxParam(
		self,
		lpTemplateName: u16,
		hWndParent: Option<HWND>,
		lpDialogFunc: DLGPROC,
		dwInitParam: Option<isize>) -> WinResult<isize>
	{
		match unsafe {
			user32::DialogBoxParamW(
				self.ptr,
				lpTemplateName as _,
				hWndParent.map_or(std::ptr::null_mut(), |h| h.ptr),
				lpDialogFunc as _,
				dwInitParam.unwrap_or_default(),
			)
		} {
			-1 => Err(GetLastError()),
			res => Ok(res), // assumes hWndParent as valid, so no check for zero
		}
	}

	/// [`EnumResourceNames`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-enumresourcenamesw)
	/// method.
	pub fn EnumResourceNames<F>(self, lpType: RtStr, func: F) -> WinResult<()>
		where F: Fn(IdStr) -> bool
	{
		bool_to_winresult(
			unsafe {
				kernel32::EnumResourceNamesW(
					self.ptr,
					lpType.as_ptr(),
					Self::EnumResNameProc::<F> as _,
					&func as *const _ as _,
				)
			},
		)
	}
	extern "system" fn EnumResNameProc<F>(
		_: HINSTANCE, _: *const u16, lpName: *mut u16, lParam: isize) -> BOOL
		where F: Fn(IdStr) -> bool
	{
		let func = unsafe { &*(lParam as *const F) };
		func(if IS_INTRESOURCE(lpName) {
			IdStr::Id(lpName as _)
		} else {
			IdStr::Str(WString::from_wchars_nullt(lpName).to_string())
		}) as _
	}

	/// [`EnumResourceTypes`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-enumresourcetypesw)
	/// method.
	pub fn EnumResourceTypes<F>(self, func: F) -> WinResult<()>
		where F: Fn(RtStr) -> bool
	{
		bool_to_winresult(
			unsafe {
				kernel32::EnumResourceTypesW(
					self.ptr,
					Self::EnumResTypeProc::<F> as _,
					&func as *const _ as _,
				)
			},
		)
	}
	extern "system" fn EnumResTypeProc<F>(
		_: HINSTANCE, lpszType: *const u16, lParam: isize) -> BOOL
		where F: Fn(RtStr) -> bool
	{
		let func = unsafe { &*(lParam as *const F) };
		func(if IS_INTRESOURCE(lpszType) {
			RtStr::Rt(co::RT(lpszType as _))
		} else {
			RtStr::Str(WString::from_wchars_nullt(lpszType).to_string())
		}) as _
	}

	/// [`FreeLibrary`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary)
	/// method.
	pub fn FreeLibrary(self) -> WinResult<()> {
		bool_to_winresult(unsafe { kernel32::FreeLibrary(self.ptr) })
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
				lpwcx as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			atom => Ok(ATOM(atom as _)),
		}
	}

	/// [`GetModuleFileName`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulefilenamew)
	/// method.
	///
	/// # Examples
	///
	/// Retrieving the full path of currently running .exe file:
	///
	/// ```rust,ignore
	/// use winsafe::HINSTANCE;
	///
	/// println!("EXE: {}", HINSTANCE::NULL.GetModuleFileName().unwrap());
	/// ```
	pub fn GetModuleFileName(self) -> WinResult<String> {
		let mut buf = [0; MAX_PATH];
		match unsafe {
			kernel32::GetModuleFileNameW(
				self.ptr,
				buf.as_mut_ptr(),
				buf.len() as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(WString::from_wchars_slice(&buf).to_string()),
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
		unsafe {
			kernel32::GetModuleHandleW(
				WString::from_opt_str(lpModuleName).as_ptr()
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`GetProcAddress`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress)
	/// method.
	pub fn GetProcAddress(self,
		lpProcName: &str) -> WinResult<*const std::ffi::c_void>
	{
		unsafe {
			kernel32::GetProcAddress(
				self.ptr,
				str_to_iso88591(lpProcName).as_ptr(),
			).as_ref()
		}.map(|ptr| ptr as _)
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadAccelerators`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadacceleratorsw)
	/// method.
	pub fn LoadAccelerators(self, lpTableName: IdStr) -> WinResult<HACCEL> {
		unsafe {
			user32::LoadAcceleratorsW(self.ptr, lpTableName.as_ptr()).as_mut()
		}.map(|ptr| HACCEL { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadCursor`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
	/// method.
	///
	/// # Examples
	///
	/// Loading a system cursor:
	/// ```rust,ignore
	/// use winsafe::{co, HINSTANCE, IdIdc};
	///
	/// let sys_cursor = HINSTANCE::default()
	///     .LoadCursor(IdIdc::Idc(co::IDC::ARROW))
	///     .unwrap();
	/// ```
	pub fn LoadCursor(self, lpCursorName: IdIdcStr) -> WinResult<HCURSOR> {
		unsafe { user32::LoadCursorW(self.ptr, lpCursorName.as_ptr()).as_mut() }
			.map(|ptr| HCURSOR { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadIcon`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadiconw)
	/// method.
	///
	/// # Examples
	///
	/// Loading a system icon:
	/// ```rust,ignore
	/// use winsafe::{co, IdIdi, HINSTANCE};
	///
	/// let sys_icon = HINSTANCE::default()
	///     .LoadIcon(IdIdi::Idi(co::IDI::INFORMATION))
	///     .unwrap();
	/// ```
	pub fn LoadIcon(self, lpIconName: IdIdiStr) -> WinResult<HICON> {
		unsafe { user32::LoadIconW(self.ptr, lpIconName.as_ptr()).as_mut() }
			.map(|ptr| HICON { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HBITMAP`](crate::HBITMAP).
	pub fn LoadImageBitmap(self,
		name: u16, cx: i32, cy: i32, fuLoad: co::LR) -> WinResult<HBITMAP>
	{
		unsafe {
			user32::LoadImageW(self.ptr, name as _, 0, cx, cy, fuLoad.0)
				.as_mut()
		}.map(|ptr| HBITMAP { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HCURSOR`](crate::HCURSOR).
	pub fn LoadImageCursor(self,
		name: u16, cx: i32, cy: i32, fuLoad: co::LR) -> WinResult<HCURSOR>
	{
		unsafe {
			user32::LoadImageW(self.ptr, name as _, 2, cx, cy, fuLoad.0)
				.as_mut()
		}.map(|ptr| HCURSOR { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadImage`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HICON`](crate::HICON).
	pub fn LoadImageIcon(self,
		name: u16, cx: i32, cy: i32, fuLoad: co::LR) -> WinResult<HICON>
	{
		unsafe {
			user32::LoadImageW(self.ptr, name as _, 1, cx, cy, fuLoad.0)
				.as_mut()
		}.map(|ptr| HICON { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadLibrary`](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryw)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HINSTANCE::FreeLibrary`](crate::HINSTANCE::FreeLibrary) call.
	pub fn LoadLibrary(lpLibFileName: &str) -> WinResult<HINSTANCE> {
		unsafe {
			kernel32::LoadLibraryW(WString::from_str(lpLibFileName).as_ptr())
				.as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadMenu`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadmenuw)
	/// method.
	pub fn LoadMenu(self, lpMenuName: u16) -> WinResult<HMENU> {
		unsafe {
			user32::LoadMenuW(self.ptr, lpMenuName as _).as_mut()
		}.map(|ptr| HMENU { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`LoadString`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadstringw)
	/// method.
	pub fn LoadString(self, uID: u16) -> WinResult<String> {
		let mut pData: *const u16 = std::ptr::null_mut();
		match unsafe {
			user32::LoadStringW(
				self.ptr,
				uID as _,
				&mut pData as *mut _ as  _, 0,
			)
		} {
			0 => Err(GetLastError()),
			len => Ok(WString::from_wchars_count(pData, len as _).to_string())
		}
	}
}
