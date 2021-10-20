#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::user32;
use crate::privs::bool_to_winresult;
use crate::structs::{MONITORINFO, POINT, RECT};

pub_struct_handle! {
	/// Handle to a
	/// [display monitor](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmonitor).
	HMONITOR
}

impl HMONITOR {
	/// [`GetMonitorInfo`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmonitorinfow)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{HMONITOR, MONITORINFO};
	///
	/// let hmon: HMONITOR; // initialized somewhere
	///
	/// let mut mi = MONITORINFO::default();
	/// hmon.GetMonitorInfo(&mut mi)?;
	/// ```
	pub fn GetMonitorInfo(self, mi: &mut MONITORINFO) -> WinResult<()> {
		bool_to_winresult(
			unsafe { user32::GetMonitorInfoW(self.ptr, mi as *mut _ as _) },
		)
	}

	/// [`MonitorFromPoint`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfrompoint)
	/// static method.
	pub fn MonitorFromPoint(pt: POINT, flags: co::MONITOR) -> HMONITOR {
		Self {
			ptr: unsafe { user32::MonitorFromPoint(pt.x, pt.y, flags.0) },
		}
	}

	/// [`MonitorFromRect`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromrect)
	/// static method.
	pub fn MonitorFromRect(rc: RECT, flags: co::MONITOR) -> HMONITOR {
		Self {
			ptr: unsafe { user32::MonitorFromRect(&rc as *const _ as _, flags.0) },
		}
	}
}
