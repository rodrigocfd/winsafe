#![allow(non_camel_case_types, non_snake_case)]

use crate::advapi::ffi;
use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

handle! { HTRANSACTION;
	/// Handle to a
	/// [transaction](https://learn.microsoft.com/en-us/windows/win32/ktm/ktm-security-and-access-rights).
	/// Originally just a `HANDLE`.
}

impl HTRANSACTION {
	/// [`CommitTransaction`](https://learn.microsoft.com/en-us/windows/win32/api/ktmw32/nf-ktmw32-committransaction)
	/// function.
	pub fn CommitTransaction(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::CommitTransaction(self.ptr()) }).to_sysresult()
	}

	/// [`CreateTransaction`](https://learn.microsoft.com/en-us/windows/win32/api/ktmw32/nf-ktmw32-createtransaction)
	/// function.
	#[must_use]
	pub fn CreateTransaction(
		transaction_attributes: Option<&SECURITY_ATTRIBUTES>,
		options: Option<co::TRANSACTION_OPT>,
		timeout: Option<u32>,
		description: &str,
	) -> SysResult<CloseHandleGuard<HTRANSACTION>> {
		unsafe {
			match HTRANSACTION(ffi::CreateTransaction(
				pcvoid_or_null(transaction_attributes),
				std::ptr::null_mut(),
				options.unwrap_or_default().raw(),
				0,
				0,
				timeout.unwrap_or_default(),
				WString::from_str(description).as_ptr() as _,
			)) {
				HTRANSACTION::INVALID => Err(GetLastError()),
				handle => Ok(CloseHandleGuard::new(handle)),
			}
		}
	}

	/// [`GetTransactionId`](https://learn.microsoft.com/en-us/windows/win32/api/ktmw32/nf-ktmw32-gettransactionid)
	/// function.
	#[must_use]
	pub fn GetTransactionId(&self) -> SysResult<GUID> {
		let mut guid = GUID::default();
		BoolRet(unsafe { ffi::GetTransactionId(self.ptr(), pvoid(&mut guid)) })
			.to_sysresult()
			.map(|_| guid)
	}

	/// [`OpenTransaction`](https://learn.microsoft.com/en-us/windows/win32/api/ktmw32/nf-ktmw32-opentransaction)
	/// function.
	#[must_use]
	pub fn OpenTransaction(
		desired_access: co::TRANSACTION,
		transaction_id: &GUID,
	) -> SysResult<CloseHandleGuard<HTRANSACTION>> {
		unsafe {
			match HTRANSACTION(ffi::OpenTransaction(desired_access.raw(), pcvoid(transaction_id))) {
				HTRANSACTION::INVALID => Err(GetLastError()),
				handle => Ok(CloseHandleGuard::new(handle)),
			}
		}
	}

	/// [`RollbackTransaction`](https://learn.microsoft.com/en-us/windows/win32/api/ktmw32/nf-ktmw32-rollbacktransaction)
	/// function.
	pub fn RollbackTransaction(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::RollbackTransaction(self.ptr()) }).to_sysresult()
	}
}
