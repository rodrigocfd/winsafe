#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, privs::*};

handle! { HFILEMAP;
	/// Handle to a
	/// [file mapping](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-createfilemappingw).
	/// Originally just a `HANDLE`.
	///
	/// Unless you need something specific, consider using the
	/// [`FileMapped`](crate::FileMapped) high-level abstraction.
}

impl HFILEMAP {
	/// [`MapViewOfFile`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-mapviewoffile)
	/// function.
	#[must_use]
	pub fn MapViewOfFile(
		&self,
		desired_access: co::FILE_MAP,
		offset: u64,
		number_of_bytes_to_map: Option<usize>,
	) -> SysResult<UnmapViewOfFileGuard> {
		unsafe {
			ptr_to_sysresult_handle(ffi::MapViewOfFileFromApp(
				self.ptr(),
				desired_access.raw(),
				offset,
				number_of_bytes_to_map.unwrap_or_default(),
			))
			.map(|h| UnmapViewOfFileGuard::new(h))
		}
	}
}
