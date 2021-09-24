#![allow(non_snake_case)]

use crate::co;
use crate::ffi::user32;
use crate::structs::{POINT, RECT};

pub_struct_handle! {
	/// Handle to a
	/// [display monitor](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hmonitor).
	HMONITOR
}

impl HMONITOR {
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
