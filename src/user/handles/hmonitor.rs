#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::user::ffi;

handle! { HMONITOR;
	/// Handle to a
	/// [display monitor](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmonitor).
}

impl HMONITOR {
	/// [`GetMonitorInfo`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmonitorinfow)
	/// function.
	pub fn GetMonitorInfo(&self) -> SysResult<MONITORINFOEX> {
		let mut mi = MONITORINFOEX::default();
		BoolRet(unsafe { ffi::GetMonitorInfoW(self.ptr(), pvoid(&mut mi)) })
			.to_sysresult()
			.map(|_| mi)
	}

	/// [`MonitorFromPoint`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfrompoint)
	/// function.
	#[must_use]
	pub fn MonitorFromPoint(pt: POINT, flags: co::MONITOR) -> HMONITOR {
		unsafe { HMONITOR::from_ptr(ffi::MonitorFromPoint(pt.into(), flags.raw())) }
	}

	/// [`MonitorFromRect`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-monitorfromrect)
	/// function.
	#[must_use]
	pub fn MonitorFromRect(rc: RECT, flags: co::MONITOR) -> HMONITOR {
		unsafe { HMONITOR::from_ptr(ffi::MonitorFromRect(pcvoid(&rc), flags.raw())) }
	}
}
