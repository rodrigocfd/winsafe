use crate::kernel::decl::GUID;

/// [`PROPERTYKEY`](https://learn.microsoft.com/en-us/windows/win32/api/wtypes/ns-wtypes-propertykey)
/// struct.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PROPERTYKEY {
	pub fmtid: GUID,
	pub pid: u32,
}

impl_default!(PROPERTYKEY);
