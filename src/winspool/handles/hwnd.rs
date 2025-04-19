#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::winspool::ffi;

impl winspool_Hwnd for HWND {}

/// This trait is enabled with the `winspool` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait winspool_Hwnd: user_Hwnd {
	/// [`AddPrinterConnection2`]()
	/// function.
	fn AddPrinterConnection2(
		&self,
		name: &str,
		connection_info: &PRINTER_CONNECTION_INFO_1,
	) -> SysResult<()> {
		bool_to_sysresult(unsafe {
			ffi::AddPrinterConnection2W(
				self.ptr(),
				WString::from_str(name).as_ptr(),
				1,
				connection_info as *const _ as _,
			)
		})
	}

	/// [`AdvancedDocumentProperties`](https://learn.microsoft.com/en-us/windows/win32/printdocs/advanceddocumentproperties)
	/// function.
	fn AdvancedDocumentProperties(
		&self,
		hprinter: &HPRINTER,
		device_name: &str,
		mode_input: &DEVMODE,
	) -> SysResult<DEVMODE> {
		let mut mode_output = DEVMODE::default();
		match unsafe {
			ffi::AdvancedDocumentPropertiesW(
				self.ptr(),
				hprinter.ptr(),
				WString::from_str(device_name).as_ptr(),
				&mut mode_output as *mut _ as _,
				mode_input as *const _ as _,
			)
		} {
			1 => Ok(mode_output),
			_ => Err(co::ERROR::INVALID_PARAMETER),
		}
	}
}
