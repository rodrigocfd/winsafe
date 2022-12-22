#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, ktm};
use crate::kernel::decl::{
	GetLastError, SECURITY_ATTRIBUTES, SysResult, WString,
};
use crate::kernel::guard::HandleGuard;
use crate::prelude::Handle;

impl_handle! { HTRANSACTION;
	/// Handle to a
	/// [transaction](https://learn.microsoft.com/en-us/windows/win32/ktm/ktm-security-and-access-rights).
	/// Originally just a `HANDLE`.
}

impl ktm_Htransaction for HTRANSACTION {}

/// This trait is enabled with the `ktm` feature, and provides methods for
/// [`HTRANSACTION`](crate::HTRANSACTION).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait ktm_Htransaction: Handle {
	/// [`CreateTransaction`](https://learn.microsoft.com/en-us/windows/win32/api/ktmw32/nf-ktmw32-createtransaction)
	/// method.
	#[must_use]
	fn CreateTransaction(
		transaction_attributes: Option<&SECURITY_ATTRIBUTES>,
		options: Option<co::TRANSACTION>,
		timeout: Option<u32>,
		description: &str) -> SysResult<HandleGuard<HTRANSACTION>>
	{
		match HTRANSACTION(unsafe {
			ktm::ffi::CreateTransaction(
				transaction_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
				std::ptr::null_mut(),
				options.map_or(0, |opt| opt.0),
				0,
				0,
				timeout.map_or(0, |t| t),
				WString::from_str(description).as_ptr() as _,
			)
		}) {
			HTRANSACTION::INVALID => Err(GetLastError()),
			handle => Ok(HandleGuard { handle }),
		}
	}
}
