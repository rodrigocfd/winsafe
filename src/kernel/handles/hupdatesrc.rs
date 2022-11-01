#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel;
use crate::kernel::decl::{
	GetLastError, IdStr, LANGID, RtStr, SysResult, WString,
};
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::Handle;

impl_handle! { HUPDATERSRC: "kernel";
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
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait kernel_Hupdatersrc: Handle {
	/// [`BeginUpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-beginupdateresourcew)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HUPDATERSRC::EndUpdateResource`](crate::prelude::kernel_Hupdatersrc::EndUpdateResource)
	/// call.
	#[must_use]
	fn BeginUpdateResource(
		file_name: &str, delete_existing_resources: bool) -> SysResult<HUPDATERSRC>
	{
		unsafe {
			kernel::ffi::BeginUpdateResourceW(
				WString::from_str(file_name).as_ptr(),
				delete_existing_resources as _,
			).as_mut()
		}.map(|ptr| HUPDATERSRC(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`EndUpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-endupdateresourcew)
	/// method.
	fn EndUpdateResource(self, discard: bool) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				kernel::ffi::EndUpdateResourceW(self.as_ptr(), discard as _)
			},
		)
	}

	/// [`UpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-updateresourcew)
	/// method.
	fn UpdateResource(self,
		resource_type: RtStr, resource_id: IdStr,
		language: LANGID, data: &[u8]) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				kernel::ffi::UpdateResourceW(
					self.as_ptr(),
					resource_type.as_ptr(),
					resource_id.as_ptr(),
					language.0,
					data.as_ptr() as _,
					data.len() as _,
				)
			},
		)
	}
}
