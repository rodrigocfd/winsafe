#![allow(non_camel_case_types, non_snake_case)]

use crate::advapi::ffi;
use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl advapi_Hthread for HTHREAD {}

/// This trait is enabled with the `advapi` feature, and provides methods for
/// [`HTHREAD`](crate::HTHREAD).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait advapi_Hthread: kernel_Hthread {
	/// [`OpenThreadToken`](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openthreadtoken)
	/// function.
	#[must_use]
	fn OpenThreadToken(&self,
		desired_access: co::TOKEN,
		open_as_self: bool,
	) -> SysResult<CloseHandleGuard<HACCESSTOKEN>>
	{
		let mut handle = HACCESSTOKEN::NULL;
		unsafe {
			bool_to_sysresult(
				ffi::OpenThreadToken(
					self.ptr(),
					desired_access.raw(),
					open_as_self as _,
					handle.as_mut(),
				),
			).map(|_| CloseHandleGuard::new(handle))
		}
	}
}
