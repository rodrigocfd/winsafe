#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::user::ffi;

handle! { HMONITOR;
	/// Handle to a
	/// [display monitor](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmonitor).
}

impl user_Hmonitor for HMONITOR {}

/// This trait is enabled with the `user` feature, and provides methods for
/// [`HMONITOR`](crate::HMONITOR).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait user_Hmonitor: Handle {
	/// [`GetMonitorInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmonitorinfow)
	/// function.
	fn GetMonitorInfo(&self) -> SysResult<MONITORINFOEX> {
		let mut mi = MONITORINFOEX::default();
		bool_to_sysresult(unsafe { ffi::GetMonitorInfoW(self.ptr(), pvoid(&mut mi)) }).map(|_| mi)
	}

	/// [`MonitorFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfrompoint)
	/// function.
	#[must_use]
	fn MonitorFromPoint(pt: POINT, flags: co::MONITOR) -> HMONITOR {
		unsafe { HMONITOR::from_ptr(ffi::MonitorFromPoint(pt.x, pt.y, flags.raw())) }
	}

	/// [`MonitorFromRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromrect)
	/// function.
	#[must_use]
	fn MonitorFromRect(rc: RECT, flags: co::MONITOR) -> HMONITOR {
		unsafe { HMONITOR::from_ptr(ffi::MonitorFromRect(pcvoid(&rc), flags.raw())) }
	}
}
