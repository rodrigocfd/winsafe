#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};

handle! { HEVENT;
	/// Handle to a named or unnamed
	/// [event](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw)
	/// object. Originally just a `HANDLE`.
}

impl HEVENT {
	/// [`CreateEvent`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventw)
	/// function.
	#[must_use]
	pub fn CreateEvent(
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
		manual_reset: bool,
		initial_state: bool,
		name: Option<&str>,
	) -> SysResult<CloseHandleGuard<HEVENT>> {
		unsafe {
			PtrRet(ffi::CreateEventW(
				pcvoid_or_null(security_attributes),
				manual_reset as _,
				initial_state as _,
				WString::from_opt_str(name).as_ptr(),
			))
			.to_sysresult_handle()
			.map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`CreateEventEx`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createeventexw)
	/// method.
	#[must_use]
	pub fn CreateEventEx(
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
		name: Option<&str>,
		flags: co::CREATE_EVENT,
		desired_access: co::EVENT_RIGHTS,
	) -> SysResult<CloseHandleGuard<HEVENT>> {
		unsafe {
			PtrRet(ffi::CreateEventExW(
				pcvoid_or_null(security_attributes),
				WString::from_opt_str(name).as_ptr(),
				flags.raw(),
				desired_access.raw(),
			))
			.to_sysresult_handle()
			.map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`OpenEvent`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-openeventw)
	/// function.
	#[must_use]
	pub fn OpenEvent(
		&self,
		desired_access: co::EVENT_RIGHTS,
		inherit_handle: bool,
		name: &str,
	) -> SysResult<CloseHandleGuard<HEVENT>> {
		unsafe {
			PtrRet(ffi::OpenEventW(
				desired_access.raw(),
				inherit_handle as _,
				WString::from_str(name).as_ptr(),
			))
			.to_sysresult_handle()
			.map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`PulseEvent`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-pulseevent)
	/// function.
	pub fn PulseEvent(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::PulseEvent(self.ptr()) }).to_sysresult()
	}

	/// [`ResetEvent`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-resetevent)
	/// function.
	pub fn ResetEvent(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::ResetEvent(self.ptr()) }).to_sysresult()
	}

	/// [`SetEvent`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-setevent)
	/// function.
	pub fn SetEvent(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::SetEvent(self.ptr()) }).to_sysresult()
	}

	/// [`WaitForSingleObject`](https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject)
	/// function.
	pub fn WaitForSingleObject(&self, milliseconds: Option<u32>) -> SysResult<co::WAIT> {
		match unsafe {
			co::WAIT::from_raw(ffi::WaitForSingleObject(
				self.ptr(),
				milliseconds.unwrap_or(INFINITE),
			))
		} {
			co::WAIT::FAILED => Err(GetLastError()),
			wait => Ok(wait),
		}
	}
}
