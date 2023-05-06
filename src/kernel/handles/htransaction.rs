#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	GetLastError, GUID, SECURITY_ATTRIBUTES, SysResult, WString,
};
use crate::kernel::guard::CloseHandleGuard;
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::Handle;

impl_handle! { HTRANSACTION;
	/// Handle to a
	/// [transaction](https://learn.microsoft.com/en-us/windows/win32/ktm/ktm-security-and-access-rights).
	/// Originally just a `HANDLE`.
}

impl kernel_Htransaction for HTRANSACTION {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HTRANSACTION`](crate::HTRANSACTION).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Htransaction: Handle {
	/// [`CommitTransaction`](https://learn.microsoft.com/en-us/windows/win32/api/ktmw32/nf-ktmw32-committransaction)
	/// method.
	fn CommitTransaction(&self) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { kernel::ffi::CommitTransaction(self.as_ptr()) },
		)
	}

	/// [`CreateTransaction`](https://learn.microsoft.com/en-us/windows/win32/api/ktmw32/nf-ktmw32-createtransaction)
	/// static method.
	#[must_use]
	fn CreateTransaction(
		transaction_attributes: Option<&SECURITY_ATTRIBUTES>,
		options: Option<co::TRANSACTION_OPT>,
		timeout: Option<u32>,
		description: &str,
	) -> SysResult<CloseHandleGuard<HTRANSACTION>>
	{
		unsafe {
			match HTRANSACTION(
				kernel::ffi::CreateTransaction(
					transaction_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
					std::ptr::null_mut(),
					options.map_or(0, |opt| opt.raw()),
					0,
					0,
					timeout.map_or(0, |t| t),
					WString::from_str(description).as_ptr() as _,
				),
			) {
				HTRANSACTION::INVALID => Err(GetLastError()),
				handle => Ok(CloseHandleGuard::new(handle)),
			}
		}
	}

	/// [`GetTransactionId`](https://learn.microsoft.com/en-us/windows/win32/api/ktmw32/nf-ktmw32-gettransactionid)
	/// method.
	#[must_use]
	fn GetTransactionId(&self) -> SysResult<GUID> {
		let mut guid = GUID::default();
		bool_to_sysresult(
			unsafe {
				kernel::ffi::GetTransactionId(
					self.as_ptr(),
					&mut guid as *mut _ as _,
				)
			},
		).map(|_| guid)
	}

	/// [`OpenTransaction`](https://learn.microsoft.com/en-us/windows/win32/api/ktmw32/nf-ktmw32-opentransaction)
	/// static method.
	#[must_use]
	fn OpenTransaction(
		desired_access: co::TRANSACTION,
		transaction_id: &GUID,
	) -> SysResult<CloseHandleGuard<HTRANSACTION>>
	{
		unsafe {
			match HTRANSACTION(
				kernel::ffi::OpenTransaction(
					desired_access.raw(),
					transaction_id as *const _ as _,
				),
			) {
				HTRANSACTION::INVALID => Err(GetLastError()),
				handle => Ok(CloseHandleGuard::new(handle)),
			}
		}
	}

	/// [`RollbackTransaction`](https://learn.microsoft.com/en-us/windows/win32/api/ktmw32/nf-ktmw32-rollbacktransaction)
	/// method.
	fn RollbackTransaction(&self) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { kernel::ffi::RollbackTransaction(self.as_ptr()) },
		)
	}
}
