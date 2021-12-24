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
	Sz(WString),
	/// No value, defined as [`REG::NONE`](crate::co::REG::NONE). Also used for
	/// non-implemented value types.
	None,
}

impl RegistryValue {
	/// Creates a new `RegistryValue::Sz` value from a `&str`.
	pub fn new_sz(s: &str) -> RegistryValue {
		Self::Sz(WString::from_str(s))
	}

	pub fn as_ptr(&self) -> *const std::ffi::c_void {
		match self {
			Self::Binary(b) => b.as_ptr() as _,
			Self::Dword(n) => *n as _,
			Self::Qword(n) => *n as _,
			Self::Sz(ws) => unsafe { ws.as_ptr() as _ },
			Self::None => std::ptr::null(),
		}
	}

	/// Returns the correspondent [`co::REG`](crate::co::REG) constant.
	pub fn reg_type(&self) -> co::REG {
		match self {
			Self::Binary(_) => co::REG::BINARY,
			Self::Dword(_) => co::REG::DWORD,
			Self::Qword(_) => co::REG::QWORD,
			Self::Sz(_) => co::REG::SZ,
			Self::None => co::REG::NONE,
		}
	}

	/// Returns the length of the stored data.
	pub fn len(&self) -> usize {
		match self {
			Self::Binary(b) => b.len(),
			Self::Dword(_) => std::mem::size_of::<u32>(),
			Self::Qword(_) => std::mem::size_of::<u64>(),
			Self::Sz(ws) => (ws.len() + 1) * std::mem::size_of::<u16>(), // including terminating null
			Self::None => 0,
		}
	}
}
