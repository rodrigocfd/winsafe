#![allow(non_snake_case)]

use crate::{co, user};
use crate::kernel::decl::WinResult;
use crate::kernel::privs::bool_to_winresult;
use crate::prelude::Handle;
use crate::user::decl::{MONITORINFO, POINT, RECT};

impl_handle! { HMONITOR: "user";
	/// Handle to a
	/// [display monitor](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmonitor).
}

impl UserHmonitor for HMONITOR {}

/// [`HMONITOR`](crate::HMONITOR) methods from `user` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "user")))]
pub trait UserHmonitor: Handle {
	/// [`GetMonitorInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmonitorinfow)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HMONITOR, MONITORINFO};
	///
	/// let hmon: HMONITOR; // initialized somewhere
	/// # let hmon = HMONITOR::NULL;
	///
	/// let mut mi = MONITORINFO::default();
	/// hmon.GetMonitorInfo(&mut mi)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn GetMonitorInfo(self, mi: &mut MONITORINFO) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				user::ffi::GetMonitorInfoW(self.as_ptr(), mi as *mut _ as _)
			},
		)
	}

	/// [`MonitorFromPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfrompoint)
	/// static method.
	#[must_use]
	fn MonitorFromPoint(pt: POINT, flags: co::MONITOR) -> HMONITOR {
		HMONITOR(unsafe { user::ffi::MonitorFromPoint(pt.x, pt.y, flags.0) })
	}

	/// [`MonitorFromRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromrect)
	/// static method.
	#[must_use]
	fn MonitorFromRect(rc: RECT, flags: co::MONITOR) -> HMONITOR {
		HMONITOR(
			unsafe { user::ffi::MonitorFromRect(&rc as *const _ as _, flags.0) },
		)
	}
}
