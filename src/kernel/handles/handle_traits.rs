#![allow(non_snake_case)]

use std::{fmt, hash};

use crate::kernel;
use crate::kernel::decl::SysResult;
use crate::kernel::privs::bool_to_sysresult;

/// A native
/// [handle](https://docs.microsoft.com/en-us/windows/win32/sysinfo/handles-and-objects),
/// base to all other handles.
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait Handle: Sized
	+ Clone + Copy + PartialEq + Eq + Send + hash::Hash
	+ fmt::Debug + fmt::Display
	+ fmt::LowerHex + fmt::UpperHex
{
	/// The null, invalid handle.
	const NULL: Self;

	/// Creates a new handle instance by wrapping a pointer.
	///
	/// **Note:** Be sure the pointer is of the correct type and not owned by
	/// anyone else, otherwise you may cause memory access violations.
	#[must_use]
	unsafe fn from_ptr<T>(p: *mut T) -> Self;

	/// Returns the underlying raw pointer.
	#[must_use]
	unsafe fn as_ptr(self) -> *mut std::ffi::c_void;

	/// Tells if the handle is invalid (null).
	#[must_use]
	fn is_null(self) -> bool {
		unsafe { self.as_ptr().is_null() }
	}

	/// Returns `None` if the handle is null, otherwise returns `Some(self)`.
	#[must_use]
	fn as_opt(self) -> Option<Self> {
		if self.is_null() {
			None
		} else {
			Some(self)
		}
	}
}

/// A [`Handle`](crate::prelude::Handle) which can be closed. Note that only
/// some handles can be closed, so not all handles implement this trait.
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait HandleClose: Handle {
	/// [`CloseHandle`](https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
	/// method.
	fn CloseHandle(self) -> SysResult<()> {
		bool_to_sysresult(unsafe { kernel::ffi::CloseHandle(self.as_ptr()) })
	}
}
