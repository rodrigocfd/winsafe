use crate::{co, ole};
use crate::prelude::ole_IUnknown;

/// RAII implementation which automatically calls
/// [`CoLockObjectExternal`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-colockobjectexternal)
/// to unlock the COM pointer.
pub struct CoLockObjectExternalGuard<'a, T>
	where T: ole_IUnknown,
{
	com_obj: &'a T,
}

impl<'a, T> Drop for CoLockObjectExternalGuard<'a, T>
	where T: ole_IUnknown,
{
	fn drop(&mut self) {
		unsafe {
			ole::ffi::CoLockObjectExternal(self.com_obj.ptr(), 0, 1); // ignore errors
		}
	}
}

impl<'a, T> CoLockObjectExternalGuard<'a, T>
	where T: ole_IUnknown,
{
	/// Constructs the guard by keeping the reference to the COM pointer.
	///
	/// # Safety
	///
	/// Be sure the COM pointer has been locked with a previous call to
	/// [`CoLockObjectExternal`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-colockobjectexternal).
	#[must_use]
	pub const unsafe fn new(com_obj: &'a T) -> Self {
		Self { com_obj }
	}
}

//------------------------------------------------------------------------------

/// RAII implementation which automatically calls
/// [`CoUninitialize`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
/// when the object goes out of scope.
pub struct CoUninitializeGuard {
	hr: co::HRESULT,
}

impl Drop for CoUninitializeGuard {
	fn drop(&mut self) {
		unsafe { ole::ffi::CoUninitialize() }
	}
}

impl CoUninitializeGuard {
	/// Constructs the guard by taking ownership of the code.
	///
	/// # Safety
	///
	/// Be sure you need to call
	/// [`CoUninitialize`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
	/// at the end of scope.
	///
	/// This method is used internally by the library, and not intended to be
	/// used externally.
	#[must_use]
	pub const unsafe fn new(hr: co::HRESULT) -> Self {
		Self { hr }
	}

	/// Returns the informational success code returned by
	/// [`CoInitializeEx`](crate::CoInitializeEx).
	#[must_use]
	pub const fn hr(&self) -> co::HRESULT {
		self.hr
	}
}
