#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::winspool::ffi;

impl_handle! { HPRINTER;
	/// Handle to a
	/// [printer](https://learn.microsoft.com/en-us/windows/win32/printdocs/openprinter).
	/// Originally just a `HANDLE`.
}

impl winspool_Hprinter for HPRINTER {}

/// This trait is enabled with the `winspool` feature, and provides methods for
/// [`HPRINTER`](crate::HPRINTER).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait winspool_Hprinter: Handle {
	/// [`OpenPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/openprinter)
	/// function.
	#[must_use]
	fn OpenPrinter(
		printer_name: Option<&str>,
		default: Option<&PRINTER_DEFAULTS>,
	) -> SysResult<ClosePrinterGuard>
	{
		let mut hprinter = HPRINTER::NULL;
		unsafe {
			bool_to_sysresult(
				ffi::OpenPrinterW(
					WString::from_opt_str(printer_name).as_mut_ptr(),
					hprinter.as_mut(),
					default.map_or(std::ptr::null_mut(), |d| d as *const _ as _),
				),
			).map(|_| ClosePrinterGuard::new(hprinter))
		}
	}
}
