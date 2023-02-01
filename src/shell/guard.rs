use std::ops::Deref;

use crate::prelude::Handle;
use crate::shell::decl::{SHFILEINFO, SHSTOCKICONINFO};
use crate::user::guard::DestroyIconGuard;

/// RAII implementation for [`SHFILEINFO`](crate::SHFILEINFO) which
/// automatically calls
/// [`DestroyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
/// on `hIcon` field when the object goes out of scope.
pub struct DestroyIconShfiGuard {
	shfi: SHFILEINFO,
}

impl Drop for DestroyIconShfiGuard {
	fn drop(&mut self) {
		if let Some(h) = self.shfi.hIcon.as_opt() {
			let _ = DestroyIconGuard::new(unsafe { h.raw_copy() });
		}
	}
}

impl Deref for DestroyIconShfiGuard {
	type Target = SHFILEINFO;

	fn deref(&self) -> &Self::Target {
		&self.shfi
	}
}

impl DestroyIconShfiGuard {
	/// Constructs the guard by taking ownership of the struct.
	#[must_use]
	pub const fn new(shfi: SHFILEINFO) -> Self {
		Self { shfi }
	}

	/// Ejects the underlying struct, leaving
	/// [`SHFILEINFO::default`](crate::SHFILEINFO::default) in its place.
	///
	/// # Safety
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsibility to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub unsafe fn leak(&mut self) -> SHFILEINFO {
		std::mem::take(&mut self.shfi)
	}
}

/// RAII implementation for [`SHSTOCKICONINFO`](crate::SHSTOCKICONINFO) which
/// automatically calls
/// [`DestroyIcon`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroyicon)
/// on `hIcon` field when the object goes out of scope.
pub struct DestroyIconSiiGuard {
	sii: SHSTOCKICONINFO,
}

impl Drop for DestroyIconSiiGuard {
	fn drop(&mut self) {
		if let Some(h) = self.sii.hIcon.as_opt() {
			let _ = DestroyIconGuard::new(unsafe { h.raw_copy() });
		}
	}
}

impl Deref for DestroyIconSiiGuard {
	type Target = SHSTOCKICONINFO;

	fn deref(&self) -> &Self::Target {
		&self.sii
	}
}

impl DestroyIconSiiGuard {
	/// Constructs the guard by taking ownership of the struct.
	#[must_use]
	pub const fn new(sii: SHSTOCKICONINFO) -> Self {
		Self { sii }
	}

	/// Ejects the underlying struct, leaving
	/// [`SHSTOCKICONINFO::default`](crate::SHSTOCKICONINFO::default) in its
	/// place.
	///
	/// # Safety
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsibility to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub unsafe fn leak(&mut self) -> SHSTOCKICONINFO {
		std::mem::take(&mut self.sii)
	}
}
