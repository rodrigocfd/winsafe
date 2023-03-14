#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::SysResult;
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::Handle;
use crate::user::decl::{MONITORINFOEX, POINT, RECT};

impl_handle! { HMONITOR;
	/// Handle to a
	/// [display monitor](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmonitor).
}

impl user_Hmonitor for HMONITOR {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HMONITOR`](crate::HMONITOR).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hmonitor: Handle {
	/// [`GetMonitorInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmonitorinfow)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HMONITOR, MONITORINFOEX};
	///
	/// let hmon: HMONITOR; // initialized somewhere
	/// # let hmon = HMONITOR::NULL;
	///
	/// let mut mi = MONITORINFOEX::default();
	/// hmon.GetMonitorInfo(&mut mi)?;
	///
	/// println!("{}", mi.szDevice());
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn GetMonitorInfo(&self, mi: &mut MONITORINFOEX) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				user::ffi::GetMonitorInfoW(self.as_ptr(), mi as *mut _ as _)
			},
		)
	}

	/// [`MonitorFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfrompoint)
	/// static method.
	#[must_use]
	fn MonitorFromPoint(pt: POINT, flags: co::MONITOR) -> HMONITOR {
		unsafe {
			HMONITOR::from_ptr(user::ffi::MonitorFromPoint(pt.x, pt.y, flags.0))
		}
	}

	/// [`MonitorFromRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromrect)
	/// static method.
	#[must_use]
	fn MonitorFromRect(rc: RECT, flags: co::MONITOR) -> HMONITOR {
		unsafe {
			HMONITOR::from_ptr(
				user::ffi::MonitorFromRect(&rc as *const _ as _, flags.0))
		}
	}
}
