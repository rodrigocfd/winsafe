use crate::co;
use crate::ole;

/// RAII implementation which automatically calls
/// [`CoUninitialize`](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize)
/// when the object goes out of scope.
pub struct ComLibraryGuard {
	pub(crate) hr: co::HRESULT,
}

impl Drop for ComLibraryGuard {
	fn drop(&mut self) {
		unsafe { ole::ffi::CoUninitialize() }
	}
}

impl ComLibraryGuard {
	/// Returns the informational success code returned by
	/// [`CoInitializeEx`](crate::CoInitializeEx).
	pub const fn hr(&self) -> co::HRESULT {
		self.hr
	}
}
