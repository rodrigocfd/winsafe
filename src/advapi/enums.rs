use crate::co;
use crate::kernel::decl::WString;
use crate::kernel::privs::parse_multi_z_str;

/// Registry value types.
#[derive(Clone)]
pub enum RegistryValue {
	/// Binary value, defined as [`REG::BINARY`](crate::co::REG::BINARY).
	Binary(Vec<u8>),
	/// An `u32` integer value, defined as [`REG::DWORD`](crate::co::REG::DWORD).
	Dword(u32),
	/// An `u64` integer value, defined as [`REG::QWORD`](crate::co::REG::QWORD).
	Qword(u64),
	/// String value, defined as [`REG::SZ`](crate::co::REG::SZ).
	Sz(String),
	/// Multiple strings, defined as [`REG::MULTI_SZ`](crate::co::REG::MULTI_SZ).
	MultiSz(Vec<String>),
	/// No value, defined as [`REG::NONE`](crate::co::REG::NONE). Also used for
	/// non-implemented value types.
	None,
}

impl RegistryValue {
	/// Parses a binary data block as a `RegistryValue`.
	///
	/// # Safety
	///
	/// Assumes the binary data block has the correct content, according to the
	/// informed [`co::REG`](crate::co::REG).
	///
	/// This method is used internally by the library, and not intended to be
	/// used externally.
	#[must_use]
	pub unsafe fn from_raw(buf: Vec<u8>, reg_type: co::REG) -> RegistryValue {
		match reg_type {
			co::REG::NONE => RegistryValue::None,
			co::REG::DWORD => RegistryValue::Dword(
				u32::from_ne_bytes(unsafe {
					*std::mem::transmute::<_, *const [u8; 4]>(buf.as_ptr())
				})
			),
			co::REG::QWORD => RegistryValue::Qword(
				u64::from_ne_bytes(unsafe {
					*std::mem::transmute::<_, *const [u8; 8]>(buf.as_ptr())
				})
			),
			co::REG::SZ => {
				let (_, vec16, _) = unsafe { buf.align_to::<u16>() };
				RegistryValue::Sz(WString::from_wchars_slice(&vec16).to_string())
			},
			co::REG::MULTI_SZ => {
				let (_, vec16, _) = unsafe { buf.align_to::<u16>() };
				RegistryValue::MultiSz(parse_multi_z_str(vec16.as_ptr()))
			},
			co::REG::BINARY => RegistryValue::Binary(buf),
			_ => RegistryValue::None, // other types not implemented yet
		}
	}

	/// Returns a pointer to the raw data, along with the raw data length.
	#[must_use]
	pub fn as_ptr_with_len(&self,
		str_buf: &mut WString) -> (*const std::ffi::c_void, u32)
	{
		match self {
			Self::Binary(b) => (b.as_ptr() as _, b.len() as _),
			Self::Dword(n) => (n as *const _ as _, std::mem::size_of::<u32>() as _),
			Self::Qword(n) => (n as *const _ as _, std::mem::size_of::<u64>() as _),
			Self::Sz(s) => {
				*str_buf = WString::from_str(s);
				Self::as_ptr_with_len_str(&str_buf)
			},
			Self::MultiSz(v) => {
				*str_buf = WString::from_str_vec(v);
				Self::as_ptr_with_len_str(&str_buf)
			},
			Self::None => (std::ptr::null(), 0),
		}
	}

	fn as_ptr_with_len_str(str_buf: &WString) -> (*const std::ffi::c_void, u32) {
		(
			unsafe { str_buf.as_ptr() as *const std::ffi::c_void },
			(str_buf.buf_len() * std::mem::size_of::<u16>()) as _, // will include terminating null
		)
	}

	/// Returns the correspondent [`co::REG`](crate::co::REG) constant.
	#[must_use]
	pub const fn reg_type(&self) -> co::REG {
		match self {
			Self::Binary(_) => co::REG::BINARY,
			Self::Dword(_) => co::REG::DWORD,
			Self::Qword(_) => co::REG::QWORD,
			Self::Sz(_) => co::REG::SZ,
			Self::MultiSz(_) => co::REG::MULTI_SZ,
			Self::None => co::REG::NONE,
		}
	}
}
