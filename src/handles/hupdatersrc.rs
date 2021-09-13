#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::enums::{IdStr, RtStr};
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::privs::bool_to_winresult;
use crate::structs::LANGID;
use crate::various::WString;

pub_struct_handle! {
	/// Handle to an
	/// [updateable resource](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-beginupdateresourcew).
	/// Originally just a `HANDLE`.
	HUPDATERSRC
}

impl HUPDATERSRC {
	/// [`BeginUpdateResource`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-beginupdateresourcew)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HUPDATERSRC::EndUpdateResource`](crate::HUPDATERSRC::EndUpdateResource)
	/// call.
	pub fn BeginUpdateResource(
		file_name: &str, delete_existing_resources: bool) -> WinResult<HUPDATERSRC>
	{
		unsafe {
			kernel32::BeginUpdateResourceW(
				WString::from_str(file_name).as_ptr(),
				delete_existing_resources as _,
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`EndUpdateResource`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-endupdateresourcew)
	/// method.
	pub fn EndUpdateResource(self, discard: bool) -> WinResult<()> {
		bool_to_winresult(
			unsafe { kernel32::EndUpdateResourceW(self.ptr, discard as _) },
		)
	}

	/// [`UpdateResource`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-updateresourcew)
	/// method.
	pub fn UpdateResource(self,
		resource_type: RtStr, resource_id: IdStr,
		language: LANGID, data: &[u8]) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel32::UpdateResourceW(
					self.ptr,
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
