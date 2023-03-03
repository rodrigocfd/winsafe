#![allow(non_camel_case_types, non_snake_case)]

use crate::{advapi_ktm, co};
use crate::advapi::decl::HKEY;
use crate::advapi::guard::RegCloseKeyGuard;
use crate::kernel::decl::{SECURITY_ATTRIBUTES, SysResult, WString};
use crate::kernel::privs::error_to_sysresult;
use crate::ktm::decl::HTRANSACTION;
use crate::prelude::Handle;

impl advapi_ktm_Hkey for HKEY {}

/// This trait is enabled with `advapi` and `ktm` features, and provides methods
/// for [`HKEY`](crate::HKEY).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait advapi_ktm_Hkey: Handle {
	/// [`RegCreateKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeytransactedw)
	/// method.
	#[must_use]
	fn RegCreateKeyTransacted(&self,
		sub_key: &str,
		class: Option<&str>,
		options: co::REG_OPTION,
		access_rights: co::KEY,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
		htransaction: &HTRANSACTION) -> SysResult<(RegCloseKeyGuard, co::REG_DISPOSITION)>
	{
		let mut hkey = HKEY::NULL;
		let mut disposition = co::REG_DISPOSITION::NoValue;

		error_to_sysresult(
			unsafe {
				advapi_ktm::ffi::RegCreateKeyTransactedW(
					self.as_ptr(),
					WString::from_str(sub_key).as_ptr(),
					0,
					WString::from_opt_str(class).as_ptr(),
					options.0,
					access_rights.0,
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
					&mut hkey.0,
					&mut disposition.0,
					htransaction.as_ptr(),
					std::ptr::null_mut(),
				)
			},
		).map(|_| (RegCloseKeyGuard::new(hkey), disposition))
	}

	/// [`RegDeleteKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeytransactedw)
	/// method.
	fn RegDeleteKeyTransacted(&self,
		sub_key: &str,
		access_rights: co::KEY,
		htransaction: &HTRANSACTION) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				advapi_ktm::ffi::RegDeleteKeyTransactedW(
					self.as_ptr(),
					WString::from_str(sub_key).as_ptr(),
					access_rights.0,
					0,
					htransaction.as_ptr(),
					std::ptr::null_mut(),
				)
			},
		)
	}

	/// [`RegOpenKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeytransactedw)
	/// method.
	#[must_use]
	fn RegOpenKeyTransacted(&self,
		sub_key: &str,
		options: co::REG_OPTION,
		access_rights: co::KEY,
		htransaction: &HTRANSACTION) -> SysResult<RegCloseKeyGuard>
	{
		let mut hkey = HKEY::NULL;
		error_to_sysresult(
			unsafe {
				advapi_ktm::ffi::RegOpenKeyTransactedW(
					self.as_ptr(),
					WString::from_str(sub_key).as_ptr(),
					options.0,
					access_rights.0,
					&mut hkey.0,
					htransaction.as_ptr(),
					std::ptr::null_mut(),
				)
			},
		).map(|_| RegCloseKeyGuard::new(hkey))
	}
}
