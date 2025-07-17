#![allow(non_camel_case_types, non_snake_case)]

use std::mem::ManuallyDrop;

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::user::ffi;

handle! { HDESK;
	/// Handle to a
	/// [desktop](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdesk).
}

impl HDESK {
	/// [`CreateDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopw)
	/// function.
	#[must_use]
	pub fn CreateDesktop(
		name: &str,
		flags: Option<co::DF>,
		desired_access: co::DESKTOP_RIGHTS,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
	) -> SysResult<CloseDesktopGuard> {
		unsafe {
			PtrRet(ffi::CreateDesktopW(
				WString::from_str(name).as_ptr(),
				std::ptr::null(),
				std::ptr::null(),
				flags.unwrap_or_default().raw(),
				desired_access.raw(),
				pcvoid_or_null(security_attributes),
			))
			.to_sysresult_handle()
			.map(|h| CloseDesktopGuard::new(h))
		}
	}

	/// [`CreateDesktopEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopexw)
	/// function.
	#[must_use]
	pub fn CreateDesktopEx(
		name: &str,
		flags: Option<co::DF>,
		desired_access: co::DESKTOP_RIGHTS,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
		heap_size_kb: u32,
	) -> SysResult<CloseDesktopGuard> {
		unsafe {
			PtrRet(ffi::CreateDesktopExW(
				WString::from_str(name).as_ptr(),
				std::ptr::null(),
				std::ptr::null(),
				flags.unwrap_or_default().raw(),
				desired_access.raw(),
				pcvoid_or_null(security_attributes),
				heap_size_kb,
				std::ptr::null_mut(),
			))
			.to_sysresult_handle()
			.map(|h| CloseDesktopGuard::new(h))
		}
	}

	/// [`GetThreadDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getthreaddesktop)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hdesk = w::HDESK::GetThreadDesktop(w::GetCurrentThreadId())?;
	/// # w::SysResult::Ok(())
	#[must_use]
	pub fn GetThreadDesktop(thread_id: u32) -> SysResult<ManuallyDrop<CloseDesktopGuard>> {
		unsafe {
			PtrRet(ffi::GetThreadDesktop(thread_id))
				.to_sysresult_handle()
				.map(|h| ManuallyDrop::new(CloseDesktopGuard::new(h)))
		}
	}

	/// [`OpenDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-opendesktopw)
	/// function.
	#[must_use]
	pub fn OpenDesktop(
		name: &str,
		flags: Option<co::DF>,
		inherit: bool,
		desired_access: co::DESKTOP_RIGHTS,
	) -> SysResult<CloseDesktopGuard> {
		unsafe {
			PtrRet(ffi::OpenDesktopW(
				WString::from_str(name).as_ptr(),
				flags.unwrap_or_default().raw(),
				inherit as _,
				desired_access.raw(),
			))
			.to_sysresult_handle()
			.map(|h| CloseDesktopGuard::new(h))
		}
	}

	/// [`OpenInputDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openinputdesktop)
	/// function.
	#[must_use]
	pub fn OpenInputDesktop(
		flags: Option<co::DF>,
		inherit: bool,
		desired_access: co::DESKTOP_RIGHTS,
	) -> SysResult<CloseDesktopGuard> {
		unsafe {
			PtrRet(ffi::OpenInputDesktop(
				flags.unwrap_or_default().raw(),
				inherit as _,
				desired_access.raw(),
			))
			.to_sysresult_handle()
			.map(|h| CloseDesktopGuard::new(h))
		}
	}

	/// [`SetThreadDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setthreaddesktop)
	/// function.
	pub fn SetThreadDesktop(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetThreadDesktop(self.ptr()) }).to_sysresult()
	}

	/// [`SwitchDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-switchdesktop)
	/// function.
	pub fn SwitchDesktop(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::SwitchDesktop(self.ptr()) }).to_sysresult()
	}
}
