#![allow(non_snake_case)]

use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::kernel32;
use crate::funcs::GetLastError;
use crate::handles::prelude::HandleClose;
use crate::structs::PROCESSENTRY32;

/// Handle to a process list
/// [snapshot](https://docs.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes).
/// Originally just a `HANDLE`.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HPROCESSLIST(pub(crate) *mut std::ffi::c_void);

impl_handle!(HPROCESSLIST);
impl HandleClose for HPROCESSLIST {}

impl HPROCESSLIST {
	/// [`CreateToolhelp32Snapshot`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HPROCESSLIST::CloseHandle`](crate::prelude::HandleClose::CloseHandle)
	/// call.
	pub fn CreateToolhelp32Snapshot(
		flags: co::TH32CS,
		th32_process_id: Option<u32>) -> WinResult<HPROCESSLIST>
	{
		unsafe {
			kernel32::CreateToolhelp32Snapshot(
				flags.0,
				th32_process_id.unwrap_or_default(),
			).as_mut()
		}.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`Process32First`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw)
	/// method.
	///
	/// Prefer using [`HPROCESSLIST::iter`](crate::HPROCESSLIST::iter), which is
	/// simpler.
	pub fn Process32First(self, pe: &mut PROCESSENTRY32) -> WinResult<bool> {
		match unsafe {
			kernel32::Process32FirstW(self.0, pe as *mut _ as _)
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
	/// Prefer using [`HPROCESSLIST::iter`](crate::HPROCESSLIST::iter), which is
	/// simpler.
	pub fn Process32Next(self, pe: &mut PROCESSENTRY32) -> WinResult<bool> {
		match unsafe {
			kernel32::Process32NextW(self.0, pe as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// Returns an iterator over the processes list by calling
	/// [`HPROCESSLIST::Process32First`](crate::HPROCESSLIST::Process32First)
	/// and then
	/// [`HPROCESSLIST::Process32Next`](crate::HPROCESSLIST::Process32Next)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, HPROCESSLIST, PROCESSENTRY32};
	///
	/// let mut pe = PROCESSENTRY32::default();
	/// let hpl = HPROCESSLIST::
	///     CreateToolhelp32Snapshot(co::TH32CS::SNAPPROCESS, None)?;
	///
	/// for pe in hpl.iter(&pe) {
	///     let pe = pe?;
	///     println!("{} {} {}",
	///         pe.szExeFile(), pe.th32ProcessID, pe.cntThreads);
	/// }
	///
	/// hpl.CloseHandle()?;
	/// ```
	pub fn iter<'a>(&'a self, pe32: &'a mut PROCESSENTRY32)
		-> impl Iterator<Item = WinResult<&'a PROCESSENTRY32>> + 'a
	{
		ProcessIter::new(*self, pe32)
	}
}

//------------------------------------------------------------------------------

struct ProcessIter<'a> {
	hpl: HPROCESSLIST,
	pe32: NonNull<PROCESSENTRY32>,
	first_pass: bool,
	has_more: bool,
	pe32_: PhantomData<&'a mut PROCESSENTRY32>,
}

impl<'a> Iterator for ProcessIter<'a> {
	type Item = WinResult<&'a PROCESSENTRY32>;

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
			pe32_: PhantomData,
		}
	}
}
