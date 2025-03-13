use crate::mf::ffi;

/// RAII implementation which automatically calls
/// [`MFShutdown`](https://learn.microsoft.com/en-us/windows/win32/api/mfapi/nf-mfapi-mfshutdown)
/// when the object goes out of scope.
///
/// Returned by [`MFStartup`](crate::MFStartup).
pub struct MFShutdownGuard {}

impl Drop for MFShutdownGuard {
	fn drop(&mut self) {
		unsafe {
			ffi::MFShutdown(); // ignore errors
		}
	}
}

impl MFShutdownGuard {
	/// Constructs the guard.
	///
	/// # Safety
	///
	/// Be sure you need to call
	/// [`MFShutdown`](https://learn.microsoft.com/en-us/windows/win32/api/mfapi/nf-mfapi-mfshutdown)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new() -> Self {
		Self {}
	}
}
