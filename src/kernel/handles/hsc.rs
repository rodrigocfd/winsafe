#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::{ffi, guard::*, privs::*};
use crate::prelude::*;

impl_handle! { HSC;
	/// Handle to a
	/// [Service Control Manager](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openscmanagerw).
	/// originally `SC_HANDLE`.
}

impl kernel_Hsc for HSC {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HSC`](crate::HSC).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hsc: Handle {
	/// [`OpenSCManager`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-openscmanagerw)
	/// function.
	fn OpenSCManager(
		machine_name: Option<&str>,
		desired_access: co::SC_MANAGER,
	) -> SysResult<CloseServiceHandleGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::OpenSCManagerW(
					WString::from_opt_str(machine_name).as_ptr(),
					std::ptr::null(),
					desired_access.raw(),
				),
			).map(|h| CloseServiceHandleGuard::new(h))
		}
	}
}
