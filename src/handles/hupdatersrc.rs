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
		pFileName: &str, bDeleteExistingResources: bool) -> WinResult<HUPDATERSRC>
	{
		unsafe {
			kernel32::BeginUpdateResourceW(
				WString::from_str(pFileName).as_ptr(),
				bDeleteExistingResources as _,
			).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`EndUpdateResource`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-endupdateresourcew)
	/// method.
	pub fn EndUpdateResource(self, fDiscard: bool) -> WinResult<()> {
		bool_to_winresult(
			unsafe { kernel32::EndUpdateResourceW(self.ptr, fDiscard as _) },
		)
	}

	/// [`UpdateResource`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-updateresourcew)
	/// method.
	pub fn UpdateResource(self,
		lpType: RtStr, lpName: IdStr,
		wLanguage: LANGID, lpData: &[u8]) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				kernel32::UpdateResourceW(
					self.ptr,
					lpType.as_ptr(),
					lpName.as_ptr(),
					wLanguage.0,
					lpData.as_ptr() as _,
					lpData.len() as _,
				)
			},
		)
	}
}
