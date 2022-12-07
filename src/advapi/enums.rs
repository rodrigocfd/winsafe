use crate::co;
use crate::kernel::decl::WString;

/// Registry value types.
#[cfg_attr(docsrs, doc(cfg(feature = "advapi")))]
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
	/// No value, defined as [`REG::NONE`](crate::co::REG::NONE). Also used for
	/// non-implemented value types.
	None,
}

impl RegistryValue {
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
				(
					unsafe { str_buf.as_ptr() as *const std::ffi::c_void },
					(str_buf.buf_len() * std::mem::size_of::<u16>()) as _, // will include terminating null
				)
			},
			Self::None => (std::ptr::null(), 0),
		}
	}

	/// Returns the correspondent [`co::REG`](crate::co::REG) constant.
	#[must_use]
	pub fn reg_type(&self) -> co::REG {
		match self {
			Self::Binary(_) => co::REG::BINARY,
			Self::Dword(_) => co::REG::DWORD,
			Self::Qword(_) => co::REG::QWORD,
			Self::Sz(_) => co::REG::SZ,
			Self::None => co::REG::NONE,
		}
	}
}
