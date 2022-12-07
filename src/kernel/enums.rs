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
			Self::Str(ws) => unsafe { ws.as_ptr() },
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
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
#[derive(Clone)]
pub enum RtStr {
	/// A predefined resource ID.
	Rt(co::RT),
	/// A resource string identifier.
	Str(WString),
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
			Self::Rt(co::RT(ptr as _))
		} else {
			Self::Str(WString::from_wchars_nullt(ptr))
		}
	}

	/// Returns a pointer to the raw data content.
	#[must_use]
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			Self::Rt(id) => MAKEINTRESOURCE(id.0 as _),
			Self::Str(ws) => unsafe { ws.as_ptr() },
		}
	}
}
