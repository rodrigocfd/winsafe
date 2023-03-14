#![allow(non_camel_case_types, non_snake_case)]

use std::mem::ManuallyDrop;

use crate::{co, user};
use crate::kernel::decl::{SECURITY_ATTRIBUTES, SysResult, WString};
use crate::kernel::privs::{bool_to_sysresult, ptr_to_sysresult};
use crate::prelude::Handle;
use crate::user::guard::CloseDesktopGuard;

impl_handle! { HDESK;
	/// Handle to a
	/// [desktop](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdesk).
}

impl user_Hdesk for HDESK {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HDESK`](crate::HDESK).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hdesk: Handle {
	/// [`CreateDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopw)
	/// static method.
	#[must_use]
	fn CreateDesktop(
		name: &str,
		flags: Option<co::DF>,
		desired_access: co::DESKTOP_RIGHTS,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
	) -> SysResult<CloseDesktopGuard>
	{
		unsafe {
			ptr_to_sysresult(
				user::ffi::CreateDesktopW(
					WString::from_str(name).as_ptr(),
					std::ptr::null(),
					std::ptr::null(),
					flags.unwrap_or(co::DF::NoValue).0,
					desired_access.0,
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
				),
				|ptr| CloseDesktopGuard::new(HDESK::from_ptr(ptr)),
			)
		}
	}

	/// [`CreateDesktopEx`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createdesktopexw)
	/// static method.
	#[must_use]
	fn CreateDesktopEx(
		name: &str,
		flags: Option<co::DF>,
		desired_access: co::DESKTOP_RIGHTS,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
		heap_size_kb: u32,
	) -> SysResult<CloseDesktopGuard>
	{
		unsafe {
			ptr_to_sysresult(
				user::ffi::CreateDesktopExW(
					WString::from_str(name).as_ptr(),
					std::ptr::null(),
					std::ptr::null(),
					flags.unwrap_or(co::DF::NoValue).0,
					desired_access.0,
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
					heap_size_kb,
					std::ptr::null_mut(),
				),
				|ptr| CloseDesktopGuard::new(HDESK::from_ptr(ptr)),
			)
		}
	}

	/// [`GetThreadDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getthreaddesktop)
	/// static method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{GetCurrentThreadId, HDESK, HTHREAD};
	///
	/// let hdesk = HDESK::GetThreadDesktop(GetCurrentThreadId())?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	#[must_use]
	fn GetThreadDesktop(
		thread_id: u32) -> SysResult<ManuallyDrop<CloseDesktopGuard>>
	{
		unsafe {
			ptr_to_sysresult(
				user::ffi::GetThreadDesktop(thread_id),
				|ptr| ManuallyDrop::new(CloseDesktopGuard::new(HDESK::from_ptr(ptr))),
			)
		}
	}

	/// [`OpenDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-opendesktopw)
	/// static method.
	#[must_use]
	fn OpenDesktop(
		name: &str,
		flags: Option<co::DF>,
		inherit: bool,
		desired_access: co::DESKTOP_RIGHTS,
	) -> SysResult<CloseDesktopGuard>
	{
		unsafe {
			ptr_to_sysresult(
				user::ffi::OpenDesktopW(
					WString::from_str(name).as_ptr(),
					flags.unwrap_or(co::DF::NoValue).0,
					inherit as _,
					desired_access.0,
				),
				|ptr| CloseDesktopGuard::new(HDESK::from_ptr(ptr)),
			)
		}
	}

	/// [`OpenInputDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-openinputdesktop)
	/// static method.
	#[must_use]
	fn OpenInputDesktop(
		flags: Option<co::DF>,
		inherit: bool,
		desired_access: co::DESKTOP_RIGHTS,
	) -> SysResult<CloseDesktopGuard>
	{
		unsafe {
			ptr_to_sysresult(
				user::ffi::OpenInputDesktop(
					flags.unwrap_or(co::DF::NoValue).0,
					inherit as _,
					desired_access.0,
				),
				|ptr| CloseDesktopGuard::new(HDESK::from_ptr(ptr)),
			)
		}
	}

	/// [`SetThreadDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setthreaddesktop)
	/// method.
	fn SetThreadDesktop(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { user::ffi::SetThreadDesktop(self.as_ptr()) })
	}

	/// [`SwitchDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-switchdesktop)
	/// method.
	fn SwitchDesktop(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { user::ffi::SwitchDesktop(self.as_ptr()) })
	}
}
