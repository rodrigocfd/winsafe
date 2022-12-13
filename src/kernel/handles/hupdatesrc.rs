#![allow(non_camel_case_types, non_snake_case)]

use std::ops::Deref;

use crate::kernel;
use crate::kernel::decl::{
	GetLastError, IdStr, LANGID, RtStr, SysResult, WString,
};
use crate::kernel::privs::bool_to_sysresult;
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
		delete_existing_resources: bool) -> SysResult<HupdatersrcGuard>
	{
		unsafe {
			kernel::ffi::BeginUpdateResourceW(
				WString::from_str(file_name).as_ptr(),
				delete_existing_resources as _,
			).as_mut()
		}.map(|ptr| HupdatersrcGuard { hupdatersrc: HUPDATERSRC(ptr) })
			.ok_or_else(|| GetLastError())
	}

	/// [`UpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-updateresourcew)
	/// method.
	fn UpdateResource(&self,
		resource_type: RtStr,
		resource_id: IdStr,
		language: LANGID,
		data: &[u8]) -> SysResult<()>
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

//------------------------------------------------------------------------------

/// RAII implementation [`HUPDATERSRC`](crate::HUPDATERSRC) which automatically
/// calls
/// [`EndUpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-endupdateresourcew)
/// when the object goes out of scope.
pub struct HupdatersrcGuard {
	pub(crate) hupdatersrc: HUPDATERSRC,
}

impl Drop for HupdatersrcGuard {
	fn drop(&mut self) {
		if let Some(h) = self.hupdatersrc.as_opt() {
			unsafe { kernel::ffi::EndUpdateResourceW(h.as_ptr(), false as _); } // ignore errors
		}
	}
}

impl Deref for HupdatersrcGuard {
	type Target = HUPDATERSRC;

	fn deref(&self) -> &Self::Target {
		&self.hupdatersrc
	}
}
