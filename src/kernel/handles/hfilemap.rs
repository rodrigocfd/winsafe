#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::SysResult;
use crate::kernel::guard::UnmapViewOfFileGuard;
use crate::kernel::privs::ptr_to_sysresult_handle;
use crate::prelude::Handle;

impl_handle! { HFILEMAP;
	/// Handle to a
	/// [file mapping](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw).
	/// Originally just a `HANDLE`.
	///
	/// Unless you need something specific, consider using the
	/// [`FileMapped`](crate::FileMapped) high-level abstraction.
}

impl kernel_Hfilemap for HFILEMAP {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HFILEMAP`](crate::HFILEMAP).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hfilemap: Handle {
	/// [`MapViewOfFile`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile)
	/// method.
	#[must_use]
	fn MapViewOfFile(&self,
		desired_access: co::FILE_MAP,
		offset: u64,
		number_of_bytes_to_map: Option<usize>,
	) -> SysResult<UnmapViewOfFileGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				kernel::ffi::MapViewOfFileFromApp(
					self.as_ptr(),
					desired_access.0,
					offset,
					number_of_bytes_to_map.unwrap_or_default(),
				),
			).map(|h| UnmapViewOfFileGuard::new(h))
		}
	}
}
