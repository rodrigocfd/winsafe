#![allow(non_camel_case_types, non_snake_case)]

use crate::advapi::ffi;
use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;

handle! { HEVENTLOG;
	/// Handle to an
	/// [event log](https://learn.microsoft.com/en-us/windows/win32/eventlog/event-logging).
	/// Originally just a `HANDLE`.
}

impl HEVENTLOG {
	/// [`RegisterEventSource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-registereventsourcew)
	/// function.
	#[must_use]
	pub fn RegisterEventSource(
		unc_server_name: Option<&str>,
		source_name: &str,
	) -> SysResult<DeregisterEventSourceGuard> {
		unsafe {
			ptr_to_sysresult_handle(ffi::RegisterEventSourceW(
				WString::from_opt_str(unc_server_name).as_ptr(),
				WString::from_str(source_name).as_ptr(),
			))
			.map(|h| DeregisterEventSourceGuard::new(h))
		}
	}

	/// [`ReportEvent`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-reporteventw)
	/// function.
	pub fn ReportEvent(
		&self,
		event_type: co::EVENTLOG,
		category: u16,
		event_id: u32,
		user_sid: Option<&SID>,
		strings: &[impl AsRef<str>],
		raw_data: &[u8],
	) -> SysResult<()> {
		let (_wstrs, pwstrs) = create_wstr_ptr_vecs(strings);
		bool_to_sysresult(unsafe {
			ffi::ReportEventW(
				self.ptr(),
				event_type.raw(),
				category,
				event_id,
				pcvoid_or_null(user_sid),
				pwstrs.len() as _,
				raw_data.len() as _,
				vec_ptr(&pwstrs),
				vec_ptr(raw_data) as _,
			)
		})
	}
}
