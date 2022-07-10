#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, HFILEMAPVIEW, WinResult};
use crate::prelude::{Handle, HandleClose};

impl_handle! { HFILEMAP: "kernel";
	/// Handle to a
	/// [file mapping](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HFILEMAP {}
impl kernel_Hfilemap for HFILEMAP {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HFILEMAP`](crate::HFILEMAP).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait kernel_Hfilemap: Handle {
	/// [`MapViewOfFile`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HFILEMAPVIEW::UnmapViewOfFile`](crate::prelude::kernel_Hfilemapview::UnmapViewOfFile)
	/// call.
	#[must_use]
	fn MapViewOfFile(self,
		desired_access: co::FILE_MAP,
		offset: u64,
		number_of_bytes_to_map: Option<usize>) -> WinResult<HFILEMAPVIEW>
	{
		unsafe {
			kernel::ffi::MapViewOfFileFromApp(
				self.as_ptr(),
				desired_access.0,
				offset,
				number_of_bytes_to_map.unwrap_or_default(),
			).as_mut()
		}.map(|ptr| HFILEMAPVIEW(ptr))
			.ok_or_else(|| GetLastError())
	}
}
