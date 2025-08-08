#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::user::ffi;

impl HINSTANCE {
	/// [`CreateDialogParam`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdialogparamw)
	/// function.
	///
	/// # Safety
	///
	/// To create a dialog, you must provide a dialog procedure.
	pub unsafe fn CreateDialogParam(
		&self,
		resource_id: IdStr,
		hwnd_parent: Option<&HWND>,
		dialog_proc: DLGPROC,
		init_param: Option<isize>,
	) -> SysResult<HWND> {
		PtrRet(unsafe {
			ffi::CreateDialogParamW(
				self.ptr(),
				resource_id.as_ptr(),
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.ptr()),
				dialog_proc as _,
				init_param.unwrap_or_default(),
			)
		})
		.to_sysresult_handle()
	}

	/// [`DialogBoxIndirectParam`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dialogboxindirectparamw)
	/// function.
	///
	/// # Safety
	///
	/// To create a dialog, you must provide a dialog procedure.
	pub unsafe fn DialogBoxIndirectParam(
		&self,
		dialog_template: &DLGTEMPLATE,
		hwnd_parent: Option<&HWND>,
		dialog_proc: DLGPROC,
		init_param: Option<isize>,
	) -> SysResult<isize> {
		match unsafe {
			ffi::DialogBoxIndirectParamW(
				self.ptr(),
				pcvoid(dialog_template),
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.ptr()),
				dialog_proc as _,
				init_param.unwrap_or_default(),
			)
		} {
			-1 => Err(GetLastError()),
			res => Ok(res), // assumes hWndParent as valid, so no check for zero
		}
	}

	/// [`DialogBoxParam`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dialogboxparamw)
	/// function.
	///
	/// # Safety
	///
	/// To create a dialog, you must provide a dialog procedure.
	pub unsafe fn DialogBoxParam(
		&self,
		resource_id: IdStr,
		hwnd_parent: Option<&HWND>,
		dialog_proc: DLGPROC,
		init_param: Option<isize>,
	) -> SysResult<isize> {
		match unsafe {
			ffi::DialogBoxParamW(
				self.ptr(),
				resource_id.as_ptr(),
				hwnd_parent.map_or(std::ptr::null_mut(), |h| h.ptr()),
				dialog_proc as _,
				init_param.unwrap_or_default(),
			)
		} {
			-1 => Err(GetLastError()),
			res => Ok(res), // assumes hwnd_parent as valid, so no check for zero
		}
	}

	/// [`GetClassInfoEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclassinfoexw)
	/// function.
	///
	/// # Examples
	///
	/// Retrieving information of a window class created in our application:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let (atom, wcx) = w::HINSTANCE::GetModuleHandle(None)?
	///     .GetClassInfoEx("SOME_CLASS_NAME")?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn GetClassInfoEx(&self, class_name: &str) -> SysResult<(ATOM, WNDCLASSEX<'_>)> {
		let mut wcx = WNDCLASSEX::default();
		match unsafe {
			ffi::GetClassInfoExW(
				self.ptr(),
				WString::from_str(class_name).as_ptr(),
				pvoid(&mut wcx),
			)
		} {
			0 => Err(GetLastError()),
			atom => Ok((unsafe { ATOM::from_raw(atom as _) }, wcx)),
		}
	}

	/// [`LoadAccelerators`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadacceleratorsw)
	/// function.
	#[must_use]
	pub fn LoadAccelerators(&self, table_name: IdStr) -> SysResult<DestroyAcceleratorTableGuard> {
		unsafe {
			PtrRet(ffi::LoadAcceleratorsW(self.ptr(), table_name.as_ptr()))
				.to_sysresult_handle()
				.map(|h| DestroyAcceleratorTableGuard::new(h))
		}
	}

	/// [`LoadCursor`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)
	/// function.
	///
	/// # Examples
	///
	/// Loading a system cursor:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let sys_cursor = w::HINSTANCE::NULL
	///     .LoadCursor(w::IdIdcStr::Idc(co::IDC::ARROW))?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn LoadCursor(&self, resource_id: IdIdcStr) -> SysResult<DestroyCursorGuard> {
		unsafe {
			PtrRet(ffi::LoadCursorW(self.ptr(), resource_id.as_ptr()))
				.to_sysresult_handle()
				.map(|h| DestroyCursorGuard::new(h))
		}
	}

	/// [`LoadIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadiconw)
	/// function.
	///
	/// # Examples
	///
	/// Loading a system icon:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let sys_icon = w::HINSTANCE::NULL
	///     .LoadIcon(w::IdIdiStr::Idi(co::IDI::INFORMATION))?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn LoadIcon(&self, icon_id: IdIdiStr) -> SysResult<DestroyIconGuard> {
		unsafe {
			PtrRet(ffi::LoadIconW(self.ptr(), icon_id.as_ptr()))
				.to_sysresult_handle()
				.map(|h| DestroyIconGuard::new(h))
		}
	}

	/// [`LoadMenu`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadmenuw)
	/// function.
	#[must_use]
	pub fn LoadMenu(&self, resource_id: IdStr) -> SysResult<DestroyMenuGuard> {
		unsafe {
			PtrRet(ffi::LoadMenuW(self.ptr(), resource_id.as_ptr()))
				.to_sysresult_handle()
				.map(|h| DestroyMenuGuard::new(h))
		}
	}

	/// [`LoadString`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadstringw)
	/// function.
	#[must_use]
	pub fn LoadString(&self, id: u16) -> SysResult<String> {
		let mut pdata: *const u16 = std::ptr::null_mut();
		match unsafe { ffi::LoadStringW(self.ptr(), id as _, &mut pdata as *mut _ as _, 0) } {
			0 => Err(GetLastError()),
			len => Ok(WString::from_wchars_count(pdata, len as _).to_string()),
		}
	}
}
