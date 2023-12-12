#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::kernel::{ffi, ffi_types::*, privs::*};
use crate::prelude::*;

impl_handle! { HSERVICESTATUS;
	/// Handle to a
	/// [service status](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-registerservicectrlhandlerexw).
	/// Originally `SERVICE_STATUS_HANDLE`.
}

impl kernel_Hservicestatus for HSERVICESTATUS {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HSERVICESTATUS`](crate::HSERVICESTATUS).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hservicestatus: Handle {
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
					register_service_ctrl_handler_ex_proc::<F> as _,
					&handler_proc as *const _ as _,
				)
			},
		)
	}
}

//------------------------------------------------------------------------------

extern "system" fn register_service_ctrl_handler_ex_proc<F>(
	control: u32, event_type: u32, event_data: PVOID, context: PVOID) -> u32
	where F: FnMut(SvcCtl) -> u32,
{
	let func = unsafe { &mut *(context as *mut F) };
	func(unsafe { SvcCtl::from_raw(control, event_type, event_data) })
}
