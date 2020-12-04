/// [`ATOM`](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#atom)
/// returned by
/// [`RegisterClassEx`](crate::RegisterClassEx).
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ATOM(u16);

impl From<u16> for ATOM {
	fn from(n: u16) -> ATOM {
		ATOM(n)
	}
}

impl ATOM {
	/// Useful to pass the atom as class name.
	pub fn as_ptr(&self) -> *const u16 {
		self.0 as *const u16
	}
}