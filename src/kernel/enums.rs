use crate::co;
use crate::kernel::decl::WString;
use crate::kernel::privs::{IS_INTRESOURCE, MAKEINTRESOURCE};

/// A resource identifier.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[derive(Clone)]
pub enum IdStr {
	/// A resource ID.
	Id(u16),
	/// A resource string identifier.
	Str(WString),
}

impl IdStr {
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	pub fn from_ptr(ptr: *const u16) -> IdStr {
		if IS_INTRESOURCE(ptr) {
			Self::Id(ptr as _)
		} else {
			Self::Str(WString::from_wchars_nullt(ptr))
		}
	}

	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Id(id) => MAKEINTRESOURCE(*id as _),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}

/// A predefined resource identifier.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[derive(Clone)]
pub enum RtStr {
	/// A predefined resource ID.
	Rt(co::RT),
	/// A resource string identifier.
	Str(WString),
}

impl RtStr {
	pub fn from_str(v: &str) -> Self {
		Self::Str(WString::from_str(v))
	}

	pub fn from_ptr(ptr: *const u16) -> RtStr {
		if IS_INTRESOURCE(ptr) {
			Self::Rt(co::RT(ptr as _))
		} else {
			Self::Str(WString::from_wchars_nullt(ptr))
		}
	}

	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Rt(id) => MAKEINTRESOURCE(id.0 as _),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}
