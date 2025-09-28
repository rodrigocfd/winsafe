#![allow(dead_code, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::prelude::*;

const_values_num_privs! {
	GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS u32 = 0x0000_0004
	GMEM_INVALID_HANDLE u32 = 0x8000
	INFINITE u32 = 0xffff_ffff
	INVALID_FILE_ATTRIBUTES i32 = -1
	LMEM_INVALID_HANDLE u32 = 0x8000
	MAX_COMPUTERNAME_LENGTH usize = 15
	MAX_MODULE_NAME32 usize = 255
	MAX_PATH usize = 260
	SECURITY_SQOS_PRESENT u32 = 0x0010_0000
}

/// [`IS_INTRESOURCE`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-is_intresource)
/// macro.
#[must_use]
pub(crate) fn IS_INTRESOURCE(val: *const u16) -> bool {
	(unsafe { std::mem::transmute::<_, usize>(val) } >> 16) == 0
}

/// [`MAKEINTRESOURCE`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-makeintresourcew)
/// macro.
#[must_use]
pub(crate) const fn MAKEINTRESOURCE(val: isize) -> *const u16 {
	val as u16 as _
}

/// Wraps a `BOOL` value returned by a FFI call, providing treatment options.
pub(crate) struct BoolRet(pub(crate) BOOL);

impl BoolRet {
	/// If value is `FALSE`, yields `Err(GetLastError())`, otherwise `Ok()`.
	#[must_use]
	pub(crate) fn to_sysresult(self) -> SysResult<()> {
		match self.0 {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// If value is `FALSE`, yields `Err(co::ERROR::INVALID_PARAMETER)`,
	/// otherwise `Ok()`.
	#[must_use]
	pub(crate) const fn to_invalidparm(self) -> SysResult<()> {
		match self.0 {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			_ => Ok(()),
		}
	}
}

/// Wraps a pointer value returned by a FFI call, providing treatment options.
pub(crate) struct PtrRet(pub(crate) *mut std::ffi::c_void);

impl PtrRet {
	/// If pointer is null, yields `Err(GetLastError())`, otherwise `Ok(ptr)`.
	#[must_use]
	pub(crate) fn to_sysresult(self) -> SysResult<*mut std::ffi::c_void> {
		if self.0.is_null() { Err(GetLastError()) } else { Ok(self.0) }
	}

	/// If pointer is null, yields `Err(co::ERROR::INVALID_PARAMETER)`, otherwise
	/// `Ok(ptr)`.
	#[must_use]
	pub(crate) const fn to_invalidparm(self) -> SysResult<HANDLE> {
		if self.0.is_null() { Err(co::ERROR::INVALID_PARAMETER) } else { Ok(self.0) }
	}

	/// If pointer is null or invalid, yields `Err(GetLastError())`, otherwise
	/// `Ok(Handle)`.
	#[must_use]
	pub(crate) fn to_sysresult_handle<H: Handle>(self) -> SysResult<H> {
		self.to_invalidparm_handle().map_err(|_| GetLastError())
	}

	/// If pointer is null or invalid, yields
	/// `Err(co::ERROR::INVALID_PARAMETER)`, otherwise `Ok(Handle)`.
	#[must_use]
	pub(crate) fn to_invalidparm_handle<H: Handle>(self) -> SysResult<H> {
		// Using match{} yields E0158.
		let ptr = unsafe { H::from_ptr(self.0) };
		if ptr == H::NULL || ptr == H::INVALID {
			Err(co::ERROR::INVALID_PARAMETER)
		} else {
			Ok(ptr)
		}
	}

	/// If the pointer is null or invalid, yields `None`, otherwise
	/// `Some(Handle)`.
	#[must_use]
	pub(crate) fn to_opt_handle<H: Handle>(self) -> Option<H> {
		self.to_invalidparm_handle().ok()
	}
}

/// Wraps a returned `co::ERROR` value, providing treatment options.
pub(crate) struct ErrorRet(pub(crate) i32);

impl ErrorRet {
	/// If value is `co::ERROR::SUCCESS`, yields `Ok(())`, otherwise `Err(err)`.
	#[must_use]
	pub(crate) const fn to_sysresult(self) -> SysResult<()> {
		match unsafe { co::ERROR::from_raw(self.0 as _) } {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
	}
}

/// If value is -1, yields `Err(GetLastError())`, otherwise `Ok(dword)`.
#[must_use]
pub(crate) fn minus1_as_error(dword: u32) -> SysResult<u32> {
	const MINUS_ONE: u32 = -1i32 as u32;
	match dword {
		MINUS_ONE => Err(GetLastError()),
		dword => Ok(dword),
	}
}

/// Converts a constant reference to FFI's `PCVOID`.
#[must_use]
pub(crate) const fn pcvoid<T>(reference: &T) -> PCVOID {
	reference as *const _ as _
}

/// Converts an optional constant reference to FFI's `PCVOID`.
#[must_use]
pub(crate) const fn pcvoid_or_null<T>(reference: Option<&T>) -> PCVOID {
	match reference {
		Some(p) => pcvoid(p),
		None => std::ptr::null(),
	}
}

/// Converts a mutable reference to FFI's `PVOID`.
#[must_use]
pub(crate) const fn pvoid<T>(reference: &mut T) -> PVOID {
	reference as *mut _ as _
}

/// Converts an optional mutable reference to FFI's `PCVOID`.
#[must_use]
pub(crate) const fn pvoid_or_null<T>(reference: Option<&mut T>) -> PVOID {
	match reference {
		Some(p) => pvoid(p),
		None => std::ptr::null_mut(),
	}
}

/// If the vector is empty, returns null, otherwise calls `as_ptr`.
///
/// This is necessary because an empty vector returns garbage as its underlying
/// pointer, see:
/// * https://github.com/rust-lang/rust/issues/39625
#[must_use]
pub(crate) const fn vec_ptr<T>(v: &[T]) -> *const T {
	if v.is_empty() { std::ptr::null() } else { v.as_ptr() }
}

/// Value returned to `DoubleIterIndex`'s callback.
pub(crate) enum DoubleIter<T> {
	/// Yield this value and keep going.
	Yield(T),
	/// Yield this value and halt immediately.
	YieldLast(T),
	/// Halt immediately.
	Halt,
}

/// Controls the indexes of a [`DoubleEndedIterator`].
pub(crate) struct DoubleIterIndex {
	front_idx: u32,
	past_back_idx: u32,
}

impl DoubleIterIndex {
	#[must_use]
	pub(crate) const fn new(item_count: u32) -> Self {
		Self { front_idx: 0, past_back_idx: item_count }
	}

	#[must_use]
	pub(crate) fn grab<F, T>(&mut self, is_front: bool, f: F) -> Option<T>
	where
		F: FnOnce(u32) -> DoubleIter<T>,
	{
		if self.front_idx == self.past_back_idx {
			return None; // iterator already exhausted
		}

		let cur_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };

		match f(cur_idx) {
			DoubleIter::Yield(val) => {
				if is_front {
					self.front_idx += 1;
				} else {
					self.past_back_idx -= 1;
				}
				Some(val)
			},
			DoubleIter::YieldLast(val) => {
				(self.front_idx, self.past_back_idx) = (0, 0); // halt
				Some(val)
			},
			DoubleIter::Halt => {
				(self.front_idx, self.past_back_idx) = (0, 0); // halt
				None
			},
		}
	}
}

/// Parses a null-delimited multi-string, ending with two terminating nulls.
///
/// # Safety
///
/// If `len` is not informed, make sure the string has two terminating nulls.
#[must_use]
pub(crate) unsafe fn parse_multi_z_str(src: *const u16, len: Option<usize>) -> Vec<String> {
	let given_len = len.unwrap_or(usize::MAX);
	let mut src = src;
	let mut strings = Vec::<String>::new();
	let mut ch = 0; // relative index of char in current string
	let mut tot_ch = 0; // absolute index of char in original src

	loop {
		if unsafe { *src.add(ch) } == 0 || tot_ch == given_len {
			let slice = unsafe { std::slice::from_raw_parts(src, ch) };
			if slice.is_empty() {
				break; // empty string means two consecutive nulls
			}
			strings.push(WString::from_wchars_slice(slice).to_string());
			src = unsafe { src.add(ch + 1) };
			ch = 0;
		} else {
			ch += 1;
		}

		if len.is_some() && tot_ch == given_len {
			break;
		}
		tot_ch += 1;
	}
	strings
}

/// Creates two vectors:
/// * the first with each string converted to `WString`;
/// * the second with the pointers to each `WString` in the first vector.
#[must_use]
pub(crate) fn create_wstr_ptr_vecs(strings: &[impl AsRef<str>]) -> (Vec<WString>, Vec<*const u16>) {
	if strings.is_empty() {
		(Vec::new(), Vec::new())
	} else {
		let wstrs = strings
			.iter()
			.map(|s| WString::from_str(s))
			.collect::<Vec<_>>();
		let pwstrs = wstrs.iter().map(|w| w.as_ptr()).collect::<Vec<_>>();
		(wstrs, pwstrs)
	}
}
