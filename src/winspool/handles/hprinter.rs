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
		bool_to_invalidparm(unsafe { ffi::AbortPrinter(self.ptr()) })
	}

	/// [`AddForm`](https://learn.microsoft.com/en-us/windows/win32/printdocs/addform)
	/// function for [`FORM_INFO_1`](crate::FORM_INFO_1).
	fn AddForm1(&self, form: &FORM_INFO_1) -> SysResult<()> {
		bool_to_invalidparm(unsafe { ffi::AddFormW(self.ptr(), 1, form as *const _ as _) })
	}

	/// [`AddForm`](https://learn.microsoft.com/en-us/windows/win32/printdocs/addform)
	/// function for [`FORM_INFO_2`](crate::FORM_INFO_2).
	fn AddForm2(&self, form: &FORM_INFO_2) -> SysResult<()> {
		bool_to_invalidparm(unsafe { ffi::AddFormW(self.ptr(), 2, form as *const _ as _) })
	}

	/// [`AddJob`](https://learn.microsoft.com/en-us/windows/win32/printdocs/addjob)
	/// function.
	///
	/// Returns the path and the job ID.
	fn AddJob(&self) -> SysResult<(String, u32)> {
		let mut sz = u32::default();
		if let Err(e) = bool_to_invalidparm(unsafe {
			ffi::AddJobW(self.ptr(), 1, std::ptr::null_mut(), 0, &mut sz)
		}) {
			return Err(e);
		}

		let mut buf = vec![0u8; sz as _];
		bool_to_invalidparm(unsafe {
			ffi::AddJobW(self.ptr(), 1, buf.as_mut_ptr() as _, buf.len() as _, &mut sz)
		})
		.map(|_| {
			let refjob_info = unsafe {
				let pjob_info = std::mem::transmute::<_, *const ADDJOB_INFO_1>(buf.as_ptr());
				&*pjob_info
			};
			(WString::from_opt_str(refjob_info.pPath()).to_string(), refjob_info.JobId)
		})
	}

	/// [`AddPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/addprinter)
	/// function.
	#[must_use]
	fn AddPrinter(name: Option<&str>, printer: &PRINTER_INFO_2) -> SysResult<ClosePrinterGuard> {
		unsafe {
			ptr_to_sysresult_handle(ffi::AddPrinterW(
				WString::from_opt_str(name).as_ptr(),
				2,
				printer as *const _ as _,
			))
			.map(|h| ClosePrinterGuard::new(h))
		}
	}

	/// [`DeleteForm`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteform)
	/// function.
	fn DeleteForm(&self, form_name: &str) -> SysResult<()> {
		bool_to_invalidparm(unsafe {
			ffi::DeleteFormW(self.ptr(), WString::from_str(form_name).as_mut_ptr())
		})
	}

	/// [`DeletePrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteprinter)
	/// function.
	fn DeletePrinter(&self) -> SysResult<()> {
		bool_to_invalidparm(unsafe { ffi::DeletePrinter(self.ptr()) })
	}

	/// [`DeletePrinterData`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteprinterdata)
	/// function.
	fn DeletePrinterData(&self, value_name: &str) -> SysResult<()> {
		error_to_sysresult(unsafe {
			ffi::DeletePrinterDataW(self.ptr(), WString::from_str(value_name).as_mut_ptr())
		})
	}

	/// [`DeletePrinterDataEx`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteprinterdataex)
	/// function.
	fn DeletePrinterDataEx(&self, key_name: &str, value_name: &str) -> SysResult<()> {
		error_to_sysresult(unsafe {
			ffi::DeletePrinterDataExW(
				self.ptr(),
				WString::from_str(key_name).as_ptr(),
				WString::from_str(value_name).as_ptr(),
			)
		})
	}

	/// [`DeletePrinterKey`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteprinterkey)
	/// function.
	fn DeletePrinterKey(&self, key_name: &str) -> SysResult<()> {
		error_to_sysresult(unsafe {
			ffi::DeletePrinterKeyW(self.ptr(), WString::from_str(key_name).as_ptr())
		})
	}

	/// [`GetPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/getprinter)
	/// function for [`PRINTER_INFO_2`](crate::PRINTER_INFO_2).
	#[must_use]
	fn GetPrinter2(&self) -> SysResult<PRINTER_INFO_2> {
		let mut nfo = PRINTER_INFO_2::default();
		let mut needed = u32::default();

		bool_to_invalidparm(unsafe {
			ffi::GetPrinterW(
				self.ptr(),
				2,
				&mut nfo as *mut _ as _,
				std::mem::size_of::<PRINTER_INFO_2>() as _,
				&mut needed,
			)
		})
		.map(|_| nfo)
	}

	/// [`GetPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/getprinter)
	/// function for [`PRINTER_INFO_3`](crate::PRINTER_INFO_3).
	#[must_use]
	fn GetPrinter3(&self) -> SysResult<PRINTER_INFO_3> {
		let mut nfo = PRINTER_INFO_3::default();
		let mut needed = u32::default();

		bool_to_invalidparm(unsafe {
			ffi::GetPrinterW(
				self.ptr(),
				3,
				&mut nfo as *mut _ as _,
				std::mem::size_of::<PRINTER_INFO_3>() as _,
				&mut needed,
			)
		})
		.map(|_| nfo)
	}

	/// [`GetPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/getprinter)
	/// function for [`PRINTER_INFO_4`](crate::PRINTER_INFO_4).
	#[must_use]
	fn GetPrinter4(&self) -> SysResult<PRINTER_INFO_4> {
		let mut nfo = PRINTER_INFO_4::default();
		let mut needed = u32::default();

		bool_to_invalidparm(unsafe {
			ffi::GetPrinterW(
				self.ptr(),
				4,
				&mut nfo as *mut _ as _,
				std::mem::size_of::<PRINTER_INFO_4>() as _,
				&mut needed,
			)
		})
		.map(|_| nfo)
	}

	/// [`OpenPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/openprinter)
	/// function.
	#[must_use]
	fn OpenPrinter(
		printer_name: Option<&str>,
		default: Option<&PRINTER_DEFAULTS>,
	) -> SysResult<ClosePrinterGuard> {
		let mut hprinter = HPRINTER::NULL;
		unsafe {
			bool_to_invalidparm(ffi::OpenPrinterW(
				WString::from_opt_str(printer_name).as_mut_ptr(),
				hprinter.as_mut(),
				default.map_or(std::ptr::null_mut(), |d| d as *const _ as _),
			))
			.map(|_| ClosePrinterGuard::new(hprinter))
		}
	}

	/// [`ResetPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/resetprinter)
	/// function.
	fn ResetPrinter(&self, default: &PRINTER_DEFAULTS) -> SysResult<()> {
		bool_to_invalidparm(unsafe { ffi::ResetPrinterW(self.ptr(), default as *const _ as _) })
	}
}
