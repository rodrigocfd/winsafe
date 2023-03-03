use crate::co;

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
