#![allow(non_camel_case_types, non_snake_case)]

use crate::ffi_types::BOOL;
use crate::kernel::decl::WinResult;
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::Handle;
use crate::user;
use crate::user::decl::{HMONITOR, RECT};

impl_handle! { HDC: "user";
	/// Handle to a
	/// [device context](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc).
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
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub trait user_Hdc: Handle {
	/// [`EnumDisplayMonitors`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-enumdisplaymonitors)
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
	fn EnumDisplayMonitors<F>(self,
		rc_clip: Option<RECT>, func: F) -> WinResult<()>
		where F: Fn(HMONITOR, HDC, &RECT) -> bool,
	{
		bool_to_winresult(
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
}

extern "system" fn enum_display_monitors_proc<F>(
	hmon: HMONITOR, hdc: HDC, rc: *const RECT, lparam: isize) -> BOOL
	where F: Fn(HMONITOR, HDC, &RECT) -> bool,
{
	let func = unsafe { &*(lparam as *const F) };
	func(hmon, hdc, unsafe { &*rc }) as _
}
