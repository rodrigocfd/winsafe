#![allow(non_camel_case_types, non_snake_case)]

use std::ops::Deref;

use crate::co;
use crate::advapi::decl::ConvertSidToStringSid;

/// [`SID_IDENTIFIER_AUTHORITY`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid_identifier_authority)
/// struct.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SID_IDENTIFIER_AUTHORITY {
	pub Value: [u8; 6],
}

impl std::fmt::Display for SID_IDENTIFIER_AUTHORITY {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{:?}", self.Value) // delegate to array Debug
	}
}

macro_rules! predef_sid_ident_au {
	($name:ident, $val:expr) => {
		/// Predefined `SID_IDENTIFIER_AUTHORITY`. Originally has `SECURITY`
		/// prefix and `AUTHORITY` suffix.
		pub const $name: Self = Self { Value: $val };
	};
}

impl SID_IDENTIFIER_AUTHORITY {
	predef_sid_ident_au!(NULL, [0, 0, 0, 0, 0, 1]);
	predef_sid_ident_au!(LOCAL, [0, 0, 0, 0, 0, 2]);
	predef_sid_ident_au!(CREATOR, [0, 0, 0, 0, 0, 3]);
	predef_sid_ident_au!(NON_UNIQUE, [0, 0, 0, 0, 0, 4]);
	predef_sid_ident_au!(RESOURCE_MANAGER, [0, 0, 0, 0, 0, 9]);
	predef_sid_ident_au!(NT, [0, 0, 0, 0, 0, 5]);
	predef_sid_ident_au!(APP_PACKAGE, [0, 0, 0, 0, 0, 15]);
	predef_sid_ident_au!(MANDATORY_LABEL, [0, 0, 0, 0, 0, 16]);
	predef_sid_ident_au!(SCOPED_POLICY_ID, [0, 0, 0, 0, 0, 17]);
	predef_sid_ident_au!(AUTHENTICATION, [0, 0, 0, 0, 0, 18]);
	predef_sid_ident_au!(PROCESS_TRUST, [0, 0, 0, 0, 0, 19]);
}

/// [`SID`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-sid)
/// struct.
/// 
/// Note that you cannot directly instantiate this struct, because the
/// `SubAuthority` field is dynamically allocated. That's why the
/// [`new`](crate::SID::new) static method returns a
/// [`SID_wrap`](crate::SID_wrap) object.
#[repr(C)]
pub struct SID {
	pub Revision: u8,
	SubAuthorityCount: u8,
	pub IdentifierAuthority: SID_IDENTIFIER_AUTHORITY,
	SubAuthority: [u32; 1],
}

impl std::fmt::Display for SID {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match ConvertSidToStringSid(self) {
			Ok(name) => write!(f, "{}", name),
			Err(err) => write!(f, "{}", err),
		}
	}
}

impl SID {
	/// Returns a [`SID_wrap`](crate::SID_wrap) with an underlying `SID` struct,
	/// which will parse the raw bytes.
	#[must_use]
	pub fn new(raw: Vec<u8>) -> SID_wrap {
		SID_wrap { raw }
	}

	/// Returns the `SubAuthorityCount` field.
	#[must_use]
	pub fn SubAuthorityCount(&self) -> u8 {
		self.SubAuthority().len() as _
	}

	/// Returns the `SubAuthority` field.
	#[must_use]
	pub fn SubAuthority(&self) -> &[u32] {
		unsafe {
			std::slice::from_raw_parts(
				self.SubAuthority.as_ptr(), self.SubAuthorityCount as _)
		}
	}
}

/// Safe wrapper over [`SID`](crate::SID), which automatically manages the
/// dynamic allocation.
pub struct SID_wrap {
	raw: Vec<u8>,
}

impl Deref for SID_wrap {
	type Target = SID;

	fn deref(&self) -> &Self::Target {
		unsafe { std::mem::transmute::<_, _>(self.raw.as_ptr()) }
	}
}

impl std::fmt::Display for SID_wrap {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.deref().fmt(f) // delegate the underlying SID
	}
}

impl SID_wrap {
	pub(crate) fn new(raw: Vec<u8>) -> Self {
		Self { raw }
	}
}

/// [`VALENT`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/ns-winreg-valentw)
/// struct.
#[repr(C)]
#[derive(Clone)]
pub struct VALENT {
	pub ve_valuename: *mut u16,
	pub ve_valuelen: u32,
	pub ve_valueptr: usize,
	pub ve_type: co::REG,
}

impl_default!(VALENT);

impl VALENT {
	/// Returns a projection over `src`, delimited by `ve_valueptr` and
	/// `ve_valuelen` fields.
	pub unsafe fn buf_projection<'a>(&'a self, src: &'a [u8]) -> &'a [u8] {
		let proj_idx = self.ve_valueptr - src.as_ptr() as usize;
		let proj_past_idx = proj_idx + self.ve_valuelen as usize;
		&src[proj_idx..proj_past_idx]
	}
}
