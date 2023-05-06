#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::{GetLastError, SysResult, WString};
use crate::kernel::ffi_types::BOOL;
use crate::kernel::privs::{bool_to_sysresult, ptr_to_option_handle};
use crate::prelude::Handle;
use crate::user::decl::{HMONITOR, HWND, RECT};

impl_handle! { HDC;
	/// Handle to a
	/// [device context](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc).
}

impl user_Hdc for HDC {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HDC`](crate::HDC).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hdc: Handle {
	/// [`DrawText`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-drawtext)
	/// method.
	fn DrawText(&self,
		text: &str, bounds: &RECT, format: co::DT) -> SysResult<i32>
	{
		let wtext = WString::from_str(text);
		match unsafe {
			user::ffi::DrawText(
				self.as_ptr(),
				wtext.as_ptr(),
				wtext.str_len() as _,
				bounds as *const _ as _,
				format.raw(),
			)
		} {
			0 => Err(GetLastError()),
			i => Ok(i),
		}
	}

	/// [`EnumDisplayMonitors`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaymonitors)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HDC, HMONITOR, RECT};
	///
	/// let hdc: HDC; // initialized somewhere
	/// # let hdc = HDC::NULL;
	///
	/// hdc.EnumDisplayMonitors(
	///     None,
	///     |hmon: HMONITOR, hdc: HDC, rc: &RECT| -> bool {
	///         println!("HMONITOR: {}, ", hmon);
	///         true
	///     },
	/// )?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn EnumDisplayMonitors<F>(&self,
		rc_clip: Option<RECT>,
		func: F,
	) -> SysResult<()>
		where F: Fn(HMONITOR, HDC, &RECT) -> bool,
	{
		bool_to_sysresult(
			unsafe {
				user::ffi::EnumDisplayMonitors(
					self.as_ptr(),
					rc_clip.map_or(std::ptr::null_mut(), |rc| &rc as *const _ as _),
					enum_display_monitors_proc::<F> as _,
					&func as *const _ as _,
				)
			},
		)
	}

	/// [`InvertRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-invertrect)
	/// method.
	fn InvertRect(&self, rc: &RECT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { user::ffi::InvertRect(self.as_ptr(), rc as *const _ as _) },
		)
	}

	/// [`PaintDesktop`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-paintdesktop)
	/// method.
	fn PaintDesktop(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { user::ffi::PaintDesktop(self.as_ptr()) })
	}

	/// [`WindowFromDC`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-windowfromdc)
	/// method.
	#[must_use]
	fn WindowFromDC(&self) -> Option<HWND> {
		ptr_to_option_handle(unsafe { user::ffi::WindowFromDC(self.as_ptr()) })
	}
}

//------------------------------------------------------------------------------

extern "system" fn enum_display_monitors_proc<F>(
	hmon: HMONITOR, hdc: HDC, rc: *const RECT, lparam: isize) -> BOOL
	where F: Fn(HMONITOR, HDC, &RECT) -> bool,
{
	let func = unsafe { &*(lparam as *const F) };
	func(hmon, hdc, unsafe { &*rc }) as _
}
