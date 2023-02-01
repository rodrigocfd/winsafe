#![allow(non_snake_case)]

use crate::co;
use crate::kernel::decl::WString;
use crate::oleaut;

/// A
/// [string data type](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/automat/bstr)
/// used with COM automation.
///
/// Automatically calls
/// [`SysFreeString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysfreestring)
/// when the object goes out of scope.
#[repr(transparent)]
pub struct BSTR(pub(crate) *mut u16);

impl Default for BSTR {
	fn default() -> Self {
		Self(std::ptr::null_mut())
	}
}

impl Drop for BSTR {
	fn drop(&mut self) {
		if !self.0.is_null() {
			unsafe { oleaut::ffi::SysFreeString(self.0) }
		}
	}
}

impl std::fmt::Display for BSTR {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl BSTR {
	/// [`SysAllocString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysallocstring)
	/// function.
	///
	/// # Panics
	///
	/// Panics if there is not enough memory.
	#[must_use]
	pub fn SysAllocString(s: &str) -> BSTR {
		let str_obj = WString::from_str(s);
		let ptr = unsafe { oleaut::ffi::SysAllocString(str_obj.as_ptr()) };
		if ptr.is_null() {
			panic!("{}", co::HRESULT::E_OUTOFMEMORY)
		} else {
			Self(ptr)
		}
	}

	/// [`SysReAllocString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysreallocstring)
	/// function.
	///
	/// # Panics
	///
	/// Panics if there is not enough memory.
	pub fn SysReAllocString(&mut self, s: &str) {
		let str_obj = WString::from_str(s);
		let ptr = unsafe {
			oleaut::ffi::SysReAllocString(self.0, str_obj.as_ptr())
		};
		if ptr.is_null() {
			panic!("{}", co::HRESULT::E_OUTOFMEMORY);
		} else {
			self.0 = ptr;
		}
	}

	/// [`SysStringLen`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysstringlen)
	/// function.
	#[must_use]
	pub fn SysStringLen(&self) -> u32 {
		unsafe { oleaut::ffi::SysStringLen(self.0) }
	}

	/// Returns the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// pointer.
	#[must_use]
	pub const fn as_ptr(&self) -> *mut u16 {
		self.0
	}

	/// Returns the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// memory block as a null-terminated `u16` slice.
	#[must_use]
	pub fn as_slice(&self) -> &[u16] {
		unsafe {
			std::slice::from_raw_parts(self.0, self.SysStringLen() as usize + 1)
		}
	}

	/// Ejects the underlying
	/// [`LPWSTR`](https://learn.microsoft.com/en-us/windows/win32/learnwin32/working-with-strings)
	/// pointer leaving a null pointer in its place, so that
	/// [`SysFreeString`](https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysfreestring)
	/// won't be called.
	///
	/// # Safety
	///
	/// Be sure to free the pointer, otherwise, as the name of this method
	/// implies, you will cause a memory leak.
	#[must_use]
	pub unsafe fn leak(&mut self) -> *mut u16 {
		std::mem::replace(&mut self.0, std::ptr::null_mut())
	}

	/// Converts into
	/// [`String`](https://doc.rust-lang.org/std/string/struct.String.html).
	#[must_use]
	pub fn to_string(&self) -> String {
		WString::from_wchars_nullt(self.0).to_string()
	}
}
