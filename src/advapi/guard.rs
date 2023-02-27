use std::ops::{Deref, DerefMut};

use crate::advapi;
use crate::advapi::decl::HKEY;
use crate::prelude::{advapi_Hkey, Handle};

/// RAII implementation for [`HKEY`](crate::HKEY) which automatically calls
/// [`RegCloseKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey)
/// when the object goes out of scope.
pub struct RegCloseKeyGuard {
	hkey: HKEY,
}

impl Drop for RegCloseKeyGuard {
	fn drop(&mut self) {
		if let Some(h) = self.hkey.as_opt() {
			if h.0 < HKEY::CLASSES_ROOT.0 || h.0 > HKEY::PERFORMANCE_NLSTEXT.0 { // guard predefined keys
				unsafe { advapi::ffi::RegCloseKey(h.as_ptr()); } // ignore errors
			}
		}
	}
}

impl Deref for RegCloseKeyGuard {
	type Target = HKEY;

	fn deref(&self) -> &Self::Target {
		&self.hkey
	}
}

impl DerefMut for RegCloseKeyGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.hkey
	}
}

impl RegCloseKeyGuard {
	/// Constructs the guard by taking ownership of the handle.
	#[must_use]
	pub const fn new(hkey: HKEY) -> Self {
		Self { hkey }
	}

	/// Ejects the underlying handle, leaving
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsibility to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> HKEY {
		std::mem::replace(&mut self.hkey, HKEY::INVALID)
	}
}
