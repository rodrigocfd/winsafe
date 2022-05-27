#![allow(non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, HFILEMAPVIEW, WinResult};
use crate::prelude::{Handle, HandleClose};

impl_handle! { HFILEMAP: "kernel";
	/// Handle to a
	/// [file mapping](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HFILEMAP {}
impl KernelHfilemap for HFILEMAP {}

/// [`HFILEMAP`](crate::HFILEMAP) methods from `kernel` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait KernelHfilemap: Handle {
	/// [`MapViewOfFile`](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HFILEMAPVIEW::UnmapViewOfFile`](crate::prelude::KernelHfilemapview::UnmapViewOfFile)
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
