#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel;
use crate::kernel::decl::{IdStr, LANGID, RtStr, SysResult, WString};
use crate::kernel::guard::EndUpdateResourceGuard;
use crate::kernel::privs::{bool_to_sysresult, ptr_to_sysresult_handle};
use crate::prelude::Handle;

impl_handle! { HUPDATERSRC;
	/// Handle to an
	/// [updateable resource](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-beginupdateresourcew).
	/// Originally just a `HANDLE`.
}

impl kernel_Hupdatersrc for HUPDATERSRC {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HUPDATERSRC`](crate::HUPDATERSRC).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hupdatersrc: Handle {
	/// [`BeginUpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-beginupdateresourcew)
	/// static method.
	#[must_use]
	fn BeginUpdateResource(
		file_name: &str,
		delete_existing_resources: bool,
	) -> SysResult<EndUpdateResourceGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				kernel::ffi::BeginUpdateResourceW(
					WString::from_str(file_name).as_ptr(),
					delete_existing_resources as _,
				),
			).map(|h| EndUpdateResourceGuard::new(h))
		}
	}

	/// [`UpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-updateresourcew)
	/// method.
	fn UpdateResource(&self,
		resource_type: RtStr,
		resource_id: IdStr,
		language: LANGID,
		data: &[u8],
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::UpdateResourceW(
					self.as_ptr(),
					resource_type.as_ptr(),
					resource_id.as_ptr(),
					language.into(),
					data.as_ptr() as _,
					data.len() as _,
				)
			},
		)
	}
}
