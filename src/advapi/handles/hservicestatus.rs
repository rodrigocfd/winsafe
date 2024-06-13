#![allow(non_camel_case_types, non_snake_case)]

use crate::advapi::{ffi, proc};
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl_handle! { HSERVICESTATUS;
	/// Handle to a
	/// [service status](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-registerservicectrlhandlerexw).
	/// Originally `SERVICE_STATUS_HANDLE`.
}

impl advapi_Hservicestatus for HSERVICESTATUS {}

/// This trait is enabled with the `advapi` feature, and provides methods for
/// [`HSERVICESTATUS`](crate::HSERVICESTATUS).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait advapi_Hservicestatus: Handle {
	/// [`RegisterServiceCtrlHandlerEx`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-registerservicectrlhandlerexw)
	/// function.
	fn RegisterServiceCtrlHandlerEx<F>(
		service_name: &str,
		handler_proc: F,
	) -> SysResult<HSERVICESTATUS>
		where F: FnMut(SvcCtl) -> u32,
	{
		ptr_to_sysresult_handle(
			unsafe {
				ffi::RegisterServiceCtrlHandlerExW(
					WString::from_str(service_name).as_ptr(),
					proc::hservicestatus_register_service_ctrl_handler_ex::<F> as _,
					&handler_proc as *const _ as _,
				)
			},
		)
	}

  /// [`SetServiceStatus`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-setservicestatus)
  /// function.
  fn SetServiceStatus(&self, status: &mut SERVICE_STATUS) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::SetServiceStatus(self.ptr(), status as *mut _ as _)
			},
		)
	}
}
