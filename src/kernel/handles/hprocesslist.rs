#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::{co, kernel};
use crate::kernel::decl::{GetLastError, PROCESSENTRY32, SysResult};
use crate::prelude::{Handle, HandleClose};

impl_handle! { HPROCESSLIST: "kernel";
	/// Handle to a process list
	/// [snapshot](https://docs.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes).
	/// Originally just a `HANDLE`.
}

impl HandleClose for HPROCESSLIST {}
impl kernel_Hprocesslist for HPROCESSLIST {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HPROCESSLIST`](crate::HPROCESSLIST).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub trait kernel_Hprocesslist: Handle {
	/// Returns an iterator over the processes list by calling
	/// [`HPROCESSLIST::Process32First`](crate::prelude::kernel_Hprocesslist::Process32First)
	/// and then
	/// [`HPROCESSLIST::Process32Next`](crate::prelude::kernel_Hprocesslist::Process32Next)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HPROCESSLIST, PROCESSENTRY32};
	///
	/// let mut pe = PROCESSENTRY32::default();
	/// let hpl = HPROCESSLIST::
	///     CreateToolhelp32Snapshot(co::TH32CS::SNAPPROCESS, None)?;
	///
	/// for pe in hpl.iter(&mut pe) {
	///     let pe = pe?;
	///     println!("{} {} {}",
	///         pe.szExeFile(), pe.th32ProcessID, pe.cntThreads);
	/// }
	///
	/// hpl.CloseHandle()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter<'a>(&'a self, pe32: &'a mut PROCESSENTRY32)
		-> Box<dyn Iterator<Item = SysResult<&'a PROCESSENTRY32>> + 'a>
	{
		Box::new(ProcessIter::new(HPROCESSLIST(unsafe { self.as_ptr() }), pe32))
	}

	/// [`CreateToolhelp32Snapshot`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HPROCESSLIST::CloseHandle`](crate::prelude::HandleClose::CloseHandle)
	/// call.
	#[must_use]
	fn CreateToolhelp32Snapshot(
		flags: co::TH32CS,
		th32_process_id: Option<u32>) -> SysResult<HPROCESSLIST>
	{
		unsafe {
			kernel::ffi::CreateToolhelp32Snapshot(
				flags.0,
				th32_process_id.unwrap_or_default(),
			).as_mut()
		}.map(|ptr| HPROCESSLIST(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`Process32First`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter`](crate::prelude::kernel_Hprocesslist::iter), which
	/// is simpler.
	fn Process32First(self, pe: &mut PROCESSENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Process32FirstW(self.as_ptr(), pe as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Process32Next`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter`](crate::prelude::kernel_Hprocesslist::iter), which
	/// is simpler.
	fn Process32Next(self, pe: &mut PROCESSENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Process32NextW(self.as_ptr(), pe as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}

//------------------------------------------------------------------------------

struct ProcessIter<'a> {
	hpl: HPROCESSLIST,
	pe32: NonNull<PROCESSENTRY32>,
	first_pass: bool,
	has_more: bool,
	_pe32: PhantomData<&'a mut PROCESSENTRY32>,
}

impl<'a> Iterator for ProcessIter<'a> {
	type Item = SysResult<&'a PROCESSENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		match if self.first_pass {
			self.first_pass = false;
			self.hpl.Process32First(unsafe { self.pe32.as_mut() })
		} else {
			self.hpl.Process32Next(unsafe { self.pe32.as_mut() })
		} {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					Some(Ok(unsafe { self.pe32.as_mut() }))
				} else {
					None // no process found
				}
			},
		}
	}
}

impl<'a> ProcessIter<'a> {
	fn new(hpl: HPROCESSLIST, pe32: &'a mut PROCESSENTRY32) -> Self {
		Self {
			hpl,
			pe32: NonNull::from(pe32),
			first_pass: true,
			has_more: true,
			_pe32: PhantomData,
		}
	}
}
