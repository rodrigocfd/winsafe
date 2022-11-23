#![allow(non_snake_case)]

use std::{fmt, hash};

use crate::kernel;
use crate::kernel::decl::SysResult;
use crate::kernel::privs::{bool_to_sysresult, invalidate_handle};

/// A native
/// [handle](https://learn.microsoft.com/en-us/windows/win32/sysinfo/handles-and-objects),
/// implemented by all handle types.
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait Handle: Sized
	+ PartialEq + Eq + Send + hash::Hash
	+ fmt::Debug + fmt::Display
	+ fmt::LowerHex + fmt::UpperHex
{
	/// The null, uninitialized handle; equals to `0`.
	const NULL: Self;

	/// The invalid handle; equals to `-1`.
	///
	/// Operations upon this handle will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	const INVALID: Self;

	/// Creates a new handle instance by wrapping a pointer.
	///
	/// # Safety
	///
	/// Be sure the pointer has the correct type and isn't owned by anyone else,
	/// otherwise you may cause memory access violations.
	///
	/// This method is used internally by the library.
	#[must_use]
	unsafe fn from_ptr(p: *mut std::ffi::c_void) -> Self;

	/// Returns the underlying raw pointer.
	///
	/// # Safety
	///
	/// This method exposes the raw pointer used by raw Windows calls. It's an
	/// opaque pointer to an internal Windows structure, and no dereferencings
	/// should be attempted.
	///
	/// This method is used internally by the library.
	#[must_use]
	unsafe fn as_ptr(&self) -> *mut std::ffi::c_void;

	/// Returns a raw copy of the underlying handle pointer.
	///
	/// # Safety
	///
	/// When a handle is closed (with methods like
	/// [`CloseHandle`](crate::prelude::HandleClose::CloseHandle)), its internal
	/// pointer is invalidated, so no further operations can be done. This
	/// safety measure is necessary because Windows can reuse handle values, so
	/// invalidating the pointer prevents this.
	///
	/// As the name implies, `raw_copy` returns a raw copy of the pointer value,
	/// so closing one handle will **not** invalidate the other copies – if the
	/// underlying handle value is reused by Windows, you can execute an
	/// operation on a completely different handle, what can be catastrophic.
	/// So after closing a handle, be sure to not use its copies anymore.
	///
	/// This method is necessary because in some cases the Windows API *demands*
	/// a copy of a handle – `raw_copy` is an escape hatch to fill this gap.
	#[must_use]
	unsafe fn raw_copy(&self) -> Self {
		Self::from_ptr(self.as_ptr())
	}

	/// Returns `None` if the handle is null or invalid, otherwise returns
	/// `Some(&self)`.
	#[must_use]
	fn as_opt(&self) -> Option<&Self> {
		if *self == Self::NULL || *self == Self::INVALID {
			None
		} else {
			Some(self)
		}
	}
}

/// A [`Handle`](crate::prelude::Handle) which can be closed. Note that only
/// some handles can be directly closed, so not all handles implement this
/// trait.
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait HandleClose: Handle {
	/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
	/// method.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	fn CloseHandle(&self) -> SysResult<()> {
		let ret = bool_to_sysresult(
			unsafe { kernel::ffi::CloseHandle(self.as_ptr()) },
		);
		invalidate_handle(self);
		ret
	}
}
