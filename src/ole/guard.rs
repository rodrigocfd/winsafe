use crate::co;
use crate::ole;

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
	/// Constructs the guard by taking ownership of the handle.
	#[must_use]
	pub const fn new(hr: co::HRESULT) -> CoUninitializeGuard {
		Self { hr }
	}

	/// Returns the informational success code returned by
	/// [`CoInitializeEx`](crate::CoInitializeEx).
	#[must_use]
	pub const fn hr(&self) -> co::HRESULT {
		self.hr
	}
}
