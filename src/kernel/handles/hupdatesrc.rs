#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};

handle! { HUPDATERSRC;
	/// Handle to an
	/// [updateable resource](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-beginupdateresourcew).
	/// Originally just a `HANDLE`.
}

impl HUPDATERSRC {
	/// [`BeginUpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-beginupdateresourcew)
	/// function.
	#[must_use]
	pub fn BeginUpdateResource(
		file_name: &str,
		delete_existing_resources: bool,
	) -> SysResult<EndUpdateResourceGuard> {
		unsafe {
			PtrRet(ffi::BeginUpdateResourceW(
				WString::from_str(file_name).as_ptr(),
				delete_existing_resources as _,
			))
			.to_sysresult_handle()
			.map(|h| EndUpdateResourceGuard::new(h))
		}
	}

	/// [`UpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-updateresourcew)
	/// function.
	pub fn UpdateResource(
		&self,
		resource_type: RtStr,
		resource_id: IdStr,
		language: LANGID,
		data: &[u8],
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::UpdateResourceW(
				self.ptr(),
				resource_type.as_ptr(),
				resource_id.as_ptr(),
				language.into(),
				vec_ptr(data) as _,
				data.len() as _,
			)
		})
		.to_sysresult()
	}
}
