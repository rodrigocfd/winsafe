#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;

use crate::winspool::ffi;

impl HWND {
	/// [`AddPrinterConnection2`](https://learn.microsoft.com/en-us/windows/win32/printdocs/addprinterconnection2)
	/// function.
	pub fn AddPrinterConnection2(
		&self,
		name: &str,
		connection_info: &PRINTER_CONNECTION_INFO_1,
	) -> SysResult<()> {
		BoolRet(unsafe {
			ffi::AddPrinterConnection2W(
				self.ptr(),
				WString::from_str(name).as_ptr(),
				1,
				pcvoid(connection_info),
			)
		})
		.to_sysresult()
	}

	/// [`AdvancedDocumentProperties`](https://learn.microsoft.com/en-us/windows/win32/printdocs/advanceddocumentproperties)
	/// function.
	pub fn AdvancedDocumentProperties(
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
				pvoid(&mut mode_output),
				pcvoid(mode_input),
			)
		} {
			1 => Ok(mode_output),
			_ => Err(co::ERROR::INVALID_PARAMETER),
		}
	}

	/// [`PrinterProperties`](https://learn.microsoft.com/en-us/windows/win32/printdocs/printerproperties)
	/// function.
	pub fn PrinterProperties(&self, hprinter: &HPRINTER) -> SysResult<()> {
		BoolRet(unsafe { ffi::PrinterProperties(self.ptr(), hprinter.ptr()) }).to_invalidparm()
	}
}
