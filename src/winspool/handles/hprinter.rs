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

impl HPRINTER {
	/// [`AbortPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/abortprinter)
	/// function.
	pub fn AbortPrinter(&self) -> SysResult<()> {
		bool_to_invalidparm(unsafe { ffi::AbortPrinter(self.ptr()) })
	}

	/// [`AddForm`](https://learn.microsoft.com/en-us/windows/win32/printdocs/addform)
	/// function for [`FORM_INFO_1`](crate::FORM_INFO_1).
	pub fn AddForm1(&self, form: &FORM_INFO_1) -> SysResult<()> {
		bool_to_invalidparm(unsafe { ffi::AddFormW(self.ptr(), 1, pcvoid(form)) })
	}

	/// [`AddForm`](https://learn.microsoft.com/en-us/windows/win32/printdocs/addform)
	/// function for [`FORM_INFO_2`](crate::FORM_INFO_2).
	pub fn AddForm2(&self, form: &FORM_INFO_2) -> SysResult<()> {
		bool_to_invalidparm(unsafe { ffi::AddFormW(self.ptr(), 2, pcvoid(form)) })
	}

	/// [`AddJob`](https://learn.microsoft.com/en-us/windows/win32/printdocs/addjob)
	/// function.
	///
	/// Returns the path and the job ID.
	pub fn AddJob(&self) -> SysResult<(String, u32)> {
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
	pub fn AddPrinter(
		name: Option<&str>,
		printer: &PRINTER_INFO_2,
	) -> SysResult<ClosePrinterGuard> {
		unsafe {
			ptr_to_sysresult_handle(ffi::AddPrinterW(
				WString::from_opt_str(name).as_ptr(),
				2,
				pcvoid(printer),
			))
			.map(|h| ClosePrinterGuard::new(h))
		}
	}

	/// [`DeleteForm`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteform)
	/// function.
	pub fn DeleteForm(&self, form_name: &str) -> SysResult<()> {
		bool_to_invalidparm(unsafe {
			ffi::DeleteFormW(self.ptr(), WString::from_str(form_name).as_mut_ptr())
		})
	}

	/// [`DeletePrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteprinter)
	/// function.
	pub fn DeletePrinter(&self) -> SysResult<()> {
		bool_to_invalidparm(unsafe { ffi::DeletePrinter(self.ptr()) })
	}

	/// [`DeletePrinterData`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteprinterdata)
	/// function.
	pub fn DeletePrinterData(&self, value_name: &str) -> SysResult<()> {
		error_to_sysresult(unsafe {
			ffi::DeletePrinterDataW(self.ptr(), WString::from_str(value_name).as_mut_ptr())
		})
	}

	/// [`DeletePrinterDataEx`](https://learn.microsoft.com/en-us/windows/win32/printdocs/deleteprinterdataex)
	/// function.
	pub fn DeletePrinterDataEx(&self, key_name: &str, value_name: &str) -> SysResult<()> {
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
	pub fn DeletePrinterKey(&self, key_name: &str) -> SysResult<()> {
		error_to_sysresult(unsafe {
			ffi::DeletePrinterKeyW(self.ptr(), WString::from_str(key_name).as_ptr())
		})
	}

	/// [`GetPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/getprinter)
	/// function for [`PRINTER_INFO_2`](crate::PRINTER_INFO_2).
	#[must_use]
	pub fn GetPrinter2(&self) -> SysResult<PRINTER_INFO_2> {
		let mut nfo = PRINTER_INFO_2::default();
		let mut needed = u32::default();

		bool_to_invalidparm(unsafe {
			ffi::GetPrinterW(
				self.ptr(),
				2,
				pvoid(&mut nfo),
				std::mem::size_of::<PRINTER_INFO_2>() as _,
				&mut needed,
			)
		})
		.map(|_| nfo)
	}

	/// [`GetPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/getprinter)
	/// function for [`PRINTER_INFO_3`](crate::PRINTER_INFO_3).
	#[must_use]
	pub fn GetPrinter3(&self) -> SysResult<PRINTER_INFO_3> {
		let mut nfo = PRINTER_INFO_3::default();
		let mut needed = u32::default();

		bool_to_invalidparm(unsafe {
			ffi::GetPrinterW(
				self.ptr(),
				3,
				pvoid(&mut nfo),
				std::mem::size_of::<PRINTER_INFO_3>() as _,
				&mut needed,
			)
		})
		.map(|_| nfo)
	}

	/// [`GetPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/getprinter)
	/// function for [`PRINTER_INFO_4`](crate::PRINTER_INFO_4).
	#[must_use]
	pub fn GetPrinter4(&self) -> SysResult<PRINTER_INFO_4> {
		let mut nfo = PRINTER_INFO_4::default();
		let mut needed = u32::default();

		bool_to_invalidparm(unsafe {
			ffi::GetPrinterW(
				self.ptr(),
				4,
				pvoid(&mut nfo),
				std::mem::size_of::<PRINTER_INFO_4>() as _,
				&mut needed,
			)
		})
		.map(|_| nfo)
	}

	/// [`OpenPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/openprinter)
	/// function.
	#[must_use]
	pub fn OpenPrinter(
		printer_name: Option<&str>,
		default: Option<&PRINTER_DEFAULTS>,
	) -> SysResult<ClosePrinterGuard> {
		let mut hprinter = HPRINTER::NULL;
		unsafe {
			bool_to_invalidparm(ffi::OpenPrinterW(
				WString::from_opt_str(printer_name).as_mut_ptr(),
				hprinter.as_mut(),
				pcvoid_or_null(default),
			))
			.map(|_| ClosePrinterGuard::new(hprinter))
		}
	}

	/// [`OpenPrinter2`](https://learn.microsoft.com/en-us/windows/win32/printdocs/openprinter2)
	/// function.
	#[must_use]
	pub fn OpenPrinter2(
		printer_name: Option<&str>,
		default: Option<&PRINTER_DEFAULTS>,
		options: Option<&PRINTER_OPTIONS>,
	) -> SysResult<ClosePrinterGuard> {
		let mut hprinter = HPRINTER::NULL;
		unsafe {
			bool_to_sysresult(ffi::OpenPrinter2W(
				WString::from_opt_str(printer_name).as_mut_ptr(),
				hprinter.as_mut(),
				pcvoid_or_null(default),
				pcvoid_or_null(options),
			))
			.map(|_| ClosePrinterGuard::new(hprinter))
		}
	}

	/// [`ResetPrinter`](https://learn.microsoft.com/en-us/windows/win32/printdocs/resetprinter)
	/// function.
	pub fn ResetPrinter(&self, default: &PRINTER_DEFAULTS) -> SysResult<()> {
		bool_to_invalidparm(unsafe { ffi::ResetPrinterW(self.ptr(), pcvoid(default)) })
	}

	/// [`ScheduleJob`](https://learn.microsoft.com/en-us/windows/win32/printdocs/schedulejob)
	/// function.
	pub fn ScheduleJob(&self, job_id: u32) -> SysResult<()> {
		bool_to_invalidparm(unsafe { ffi::ScheduleJob(self.ptr(), job_id) })
	}
}
