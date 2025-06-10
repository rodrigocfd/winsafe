#![allow(non_camel_case_types, non_snake_case)]

use crate::advapi::{ffi, proc};
use crate::decl::*;
use crate::kernel::privs::*;

handle! { HSERVICESTATUS;
	/// Handle to a
	/// [service status](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-registerservicectrlhandlerexw).
	/// Originally `SERVICE_STATUS_HANDLE`.
}

impl HSERVICESTATUS {
	/// [`RegisterServiceCtrlHandlerEx`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-registerservicectrlhandlerexw)
	/// function.
	pub fn RegisterServiceCtrlHandlerEx<F>(
		service_name: &str,
		handler_proc: F,
	) -> SysResult<HSERVICESTATUS>
	where
		F: FnMut(SvcCtl) -> u32,
	{
		ptr_to_sysresult_handle(unsafe {
			ffi::RegisterServiceCtrlHandlerExW(
				WString::from_str(service_name).as_ptr(),
				proc::hservicestatus_register_service_ctrl_handler_ex::<F> as _,
				pcvoid(&handler_proc),
			)
		})
	}

	/// [`SetServiceStatus`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-setservicestatus)
	/// function.
	pub fn SetServiceStatus(&self, status: &mut SERVICE_STATUS) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::SetServiceStatus(self.ptr(), pvoid(status)) })
	}
}
