use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;

/// Variable parameter for:
///
/// * [`HACCESSTOKEN::AdjustTokenPrivileges`](crate::prelude::kernel_Haccesstoken::AdjustTokenPrivileges).
pub enum DisabPriv<'a> {
	/// Disables all privileges.
	Disab,
	/// An array of privileges and its attributes.
	Privs(&'a TOKEN_PRIVILEGES)
}

/// A resource identifier.
///
/// Variable parameter for:
///
/// * [`HINSTANCE::CreateDialogParam`](crate::prelude::user_Hinstance::CreateDialogParam);
/// * [`HINSTANCE::EnumResourceLanguages`](crate::prelude::kernel_Hinstance::EnumResourceLanguages);
/// * [`HINSTANCE::EnumResourceNames`](crate::prelude::kernel_Hinstance::EnumResourceNames);
/// * [`HINSTANCE::FindResource`](crate::prelude::kernel_Hinstance::FindResource);
/// * [`HINSTANCE::FindResourceEx`](crate::prelude::kernel_Hinstance::FindResourceEx);
/// * [`HINSTANCE::LoadAccelerators`](crate::prelude::user_Hinstance::LoadAccelerators);
/// * [`HINSTANCE::LoadMenu`](crate::prelude::user_Hinstance::LoadMenu);
/// * [`HUPDATERSRC::UpdateResource`](crate::prelude::kernel_Hupdatersrc::UpdateResource);
/// * [`BmpIdbRes`](crate::BmpIdbRes);
/// * [`ResStrs`](crate::ResStrs).
#[derive(Clone)]
pub enum IdStr {
	/// A resource ID.
	Id(u16),
	/// A resource string identifier.
	Str(WString),
}

impl std::fmt::Display for IdStr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Id(rt) => write!(f, "ID: {}", rt),
			Self::Str(str) => write!(f, "Str: {}", str),
		}
	}
}

impl IdStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Constructs the enum from a raw pointer.
	#[must_use]
	pub fn from_ptr(ptr: *const u16) -> IdStr {
		if IS_INTRESOURCE(ptr) {
			Self::Id(ptr as _)
		} else {
			Self::Str(WString::from_wchars_nullt(ptr))
		}
	}

	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Str(ws) => ws.as_ptr(),
		}
	}
}

/// Registry value types.
///
/// This is a high-level abstraction over the [`co::REG`](crate::co::REG)
/// constants, plus the value they carry.
#[derive(Clone, Debug)]
pub enum RegistryValue {
	/// Binary value, defined as [`REG::BINARY`](crate::co::REG::BINARY).
	Binary(Vec<u8>),
	/// An `u32` integer value, defined as [`REG::DWORD`](crate::co::REG::DWORD).
	Dword(u32),
	/// An `u64` integer value, defined as [`REG::QWORD`](crate::co::REG::QWORD).
	Qword(u64),
	/// String value, defined as [`REG::SZ`](crate::co::REG::SZ).
	Sz(String),
	/// String value that contains unexpanded references to environment
	/// variables, for example, `%PATH%`. To expand the environment variable
	/// references, use
	/// [`ExpandEnvironmentStrings`](crate::ExpandEnvironmentStrings).
	ExpandSz(String),
	/// Multiple strings, defined as [`REG::MULTI_SZ`](crate::co::REG::MULTI_SZ).
	MultiSz(Vec<String>),
	/// No value, defined as [`REG::NONE`](crate::co::REG::NONE). Also used for
	/// non-implemented value types.
	None,
}

impl std::fmt::Display for RegistryValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Binary(b) => write!(
				f,
				"[REG_BINARY] {}",
				b.iter()
					.map(|n| format!("{:02}", *n))
					.collect::<Vec<_>>()
					.join(" "),
			),
			Self::Dword(n) => write!(f, "[REG_DWORD] {}", *n),
			Self::Qword(n) => write!(f, "[REG_QWORD] {}", *n),
			Self::Sz(s) => write!(f, "[REG_SZ] \"{}\"", s),
			Self::ExpandSz(s) => write!(f, "[REG_EXPAND_SZ] \"{}\"", s),
			Self::MultiSz(v) => write!(
				f,
				"[REG_MULTI_SZ] {}",
				v.iter()
					.map(|s| format!("\"{}\"", s))
					.collect::<Vec<_>>()
					.join(", "),
			),
			Self::None => write!(f, "[REG_NONE]"),
		}
	}
}

impl RegistryValue {
	/// Parses a binary data block as a `RegistryValue`.
	///
	/// This method can be used as an escape hatch to interoperate with other
	/// libraries.
	///
	/// # Safety
	///
	/// Assumes the binary data block has the correct content, according to the
	/// informed [`co::REG`](crate::co::REG).
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
			co::REG::EXPAND_SZ => {
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
		str_buf: &mut WString,
	) -> (*const std::ffi::c_void, u32)
	{
		match self {
			Self::Binary(b) => (b.as_ptr() as _, b.len() as _),
			Self::Dword(n) => (n as *const _ as _, std::mem::size_of::<u32>() as _),
			Self::Qword(n) => (n as *const _ as _, std::mem::size_of::<u64>() as _),
			Self::Sz(s) => {
				*str_buf = WString::from_str(s);
				Self::as_ptr_with_len_str(&str_buf)
			},
			Self::ExpandSz(s) => {
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
			str_buf.as_ptr() as _,
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
			Self::ExpandSz(_) => co::REG::EXPAND_SZ,
			Self::MultiSz(_) => co::REG::MULTI_SZ,
			Self::None => co::REG::NONE,
		}
	}
}

/// A predefined resource identifier.
///
/// Variant parameter for:
///
/// * [`HINSTANCE::EnumResourceLanguages`](crate::prelude::kernel_Hinstance::EnumResourceLanguages);
/// * [`HINSTANCE::EnumResourceNames`](crate::prelude::kernel_Hinstance::EnumResourceNames);
/// * [`HINSTANCE::EnumResourceTypes`](crate::prelude::kernel_Hinstance::EnumResourceTypes);
/// * [`HINSTANCE::FindResource`](crate::prelude::kernel_Hinstance::FindResource);
/// * [`HINSTANCE::FindResourceEx`](crate::prelude::kernel_Hinstance::FindResourceEx);
/// * [`HUPDATERSRC`](crate::prelude::kernel_Hupdatersrc::UpdateResource).
#[derive(Clone)]
pub enum RtStr {
	/// A predefined resource ID.
	Rt(co::RT),
	/// A resource string identifier.
	Str(WString),
}

impl std::fmt::Display for RtStr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Rt(rt) => write!(f, "RT: {}", rt),
			Self::Str(str) => write!(f, "Str: {}", str),
		}
	}
}

impl RtStr {
	/// Constructs the enum directly from a string.
	#[must_use]
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	/// Constructs the enum from a pointer to raw data.
	#[must_use]
	pub fn from_ptr(ptr: *const u16) -> RtStr {
		if IS_INTRESOURCE(ptr) {
			Self::Rt(unsafe { co::RT::from_raw(ptr as _) })
		} else {
			Self::Str(WString::from_wchars_nullt(ptr))
		}
	}

	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Rt(id) => MAKEINTRESOURCE(id.raw() as _),
			Self::Str(ws) => ws.as_ptr(),
		}
	}
}
