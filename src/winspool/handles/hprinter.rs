#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::winspool::ffi;

handle! { HPRINTER;
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
	/// [`AbortPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/abortprinter)
	/// function.
	fn AbortPrinter(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::AbortPrinter(self.ptr()) })
	}

	/// [`DeleteForm`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteform)
	/// function.
	fn DeleteForm(&self, form_name: &str) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::DeleteFormW(
					self.ptr(),
					WString::from_str(form_name).as_mut_ptr(),
				)
			},
		)
	}

	/// [`DeletePrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteprinter)
	/// function.
	fn DeletePrinter(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::DeletePrinter(self.ptr()) })
	}

	/// [`DeletePrinterData`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteprinterdata)
	/// function.
	fn DeletePrinterData(&self, value_name: &str) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::DeletePrinterDataW(
					self.ptr(),
					WString::from_str(value_name).as_mut_ptr(),
				)
			},
		)
	}

	/// [`DeletePrinterDataEx`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteprinterdataex)
	/// function.
	fn DeletePrinterDataEx(&self,
		key_name: &str,
		value_name: &str,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::DeletePrinterDataExW(
					self.ptr(),
					WString::from_str(key_name).as_ptr(),
					WString::from_str(value_name).as_ptr(),
				)
			},
		)
	}

	/// [`DeletePrinterKey`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteprinterkey)
	/// function.
	fn DeletePrinterKey(&self, key_name: &str) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::DeletePrinterKeyW(
					self.ptr(),
					WString::from_str(key_name).as_ptr(),
				)
			},
		)
	}

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

	/// [`ResetPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/resetprinter)
	/// function.
	fn ResetPrinter(&self, default: &PRINTER_DEFAULTS) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::ResetPrinterW(self.ptr(), default as *const _ as _) },
		)
	}
}
