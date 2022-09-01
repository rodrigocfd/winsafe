#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::{co, kernel};
use crate::kernel::decl::{
	GetLastError, MODULEENTRY32, PROCESSENTRY32, SysResult, THREADENTRY32,
};
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
	/// Returns an iterator over the [`MODULEENTRY32`](crate::MODULEENTRY32)
	/// structs of the processes list by calling
	/// [`HPROCESSLIST::Module32First`](crate::prelude::kernel_Hprocesslist::Module32First)
	/// and then
	/// [`HPROCESSLIST::Module32Next`](crate::prelude::kernel_Hprocesslist::Module32Next)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HPROCESSLIST};
	///
	/// let hpl = HPROCESSLIST::
	///     CreateToolhelp32Snapshot(co::TH32CS::SNAPMODULE, None)?;
	///
	/// for mod_entry in hpl.iter_modules() {
	///     let mod_entry = mod_entry?;
	///     println!("{} {}",
	///         mod_entry.szModule(), mod_entry.th32ProcessID);
	/// }
	///
	/// hpl.CloseHandle()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_modules<'a>(&'a self)
		-> Box<dyn Iterator<Item = SysResult<&'a MODULEENTRY32>> + 'a>
	{
		Box::new(ModuleIter::new(HPROCESSLIST(unsafe { self.as_ptr() })))
	}

	/// Returns an iterator over the [`PROCESSENTRY32`](crate::PROCESSENTRY32)
	/// structs of the processes list by calling
	/// [`HPROCESSLIST::Process32First`](crate::prelude::kernel_Hprocesslist::Process32First)
	/// and then
	/// [`HPROCESSLIST::Process32Next`](crate::prelude::kernel_Hprocesslist::Process32Next)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HPROCESSLIST};
	///
	/// let hpl = HPROCESSLIST::
	///     CreateToolhelp32Snapshot(co::TH32CS::SNAPPROCESS, None)?;
	///
	/// for proc_entry in hpl.iter_processes() {
	///     let proc_entry = proc_entry?;
	///     println!("{} {} {}",
	///         proc_entry.szExeFile(), proc_entry.th32ProcessID, proc_entry.cntThreads);
	/// }
	///
	/// hpl.CloseHandle()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_processes<'a>(&'a self)
		-> Box<dyn Iterator<Item = SysResult<&'a PROCESSENTRY32>> + 'a>
	{
		Box::new(ProcessIter::new(HPROCESSLIST(unsafe { self.as_ptr() })))
	}

	/// Returns an iterator over the [`THREADENTRY32`](crate::THREADENTRY32)
	/// structs of the threads list by calling
	/// [`HPROCESSLIST::Thread32First`](crate::prelude::kernel_Hprocesslist::Thread32First)
	/// and then
	/// [`HPROCESSLIST::Thread32Next`](crate::prelude::kernel_Hprocesslist::Thread32Next)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HPROCESS, HPROCESSLIST};
	///
	/// let hpl = HPROCESSLIST::CreateToolhelp32Snapshot(
	///     co::TH32CS::SNAPTHREAD,
	///     Some(HPROCESS::GetCurrentProcessId()),
	/// )?;
	///
	/// for thread_entry in hpl.iter_threads() {
	///     let thread_entry = thread_entry?;
	///     println!("{} {}",
	///         thread_entry.th32ThreadID, thread_entry.th32OwnerProcessID);
	/// }
	///
	/// hpl.CloseHandle()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_threads<'a>(&'a self)
		-> Box<dyn Iterator<Item = SysResult<&'a THREADENTRY32>> + 'a>
	{
		Box::new(ThreadIter::new(HPROCESSLIST(unsafe { self.as_ptr() })))
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

	/// [`Module32First`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-module32firstw)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_modules`](crate::prelude::kernel_Hprocesslist::iter_modules),
	/// which is simpler.
	#[must_use]
	fn Module32First(self, me: &mut MODULEENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Module32FirstW(self.as_ptr(), me as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Module32Next`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-module32nextw)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_modules`](crate::prelude::kernel_Hprocesslist::iter_modules),
	/// which is simpler.
	#[must_use]
	fn Module32Next(self, me: &mut MODULEENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Module32NextW(self.as_ptr(), me as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Process32First`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_processes`](crate::prelude::kernel_Hprocesslist::iter_processes),
	/// which is simpler.
	#[must_use]
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
	/// [`HPROCESSLIST::iter_processes`](crate::prelude::kernel_Hprocesslist::iter_processes),
	/// which is simpler.
	#[must_use]
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

	/// [`Thread32First`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-thread32first)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_threads`](crate::prelude::kernel_Hprocesslist::iter_threads),
	/// which is simpler.
	#[must_use]
	fn Thread32First(self, te: &mut THREADENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Thread32First(self.as_ptr(), te as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => Ok(false),
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Thread32First`](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-thread32next)
	/// method.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_threads`](crate::prelude::kernel_Hprocesslist::iter_threads),
	/// which is simpler.
	#[must_use]
	fn Thread32Next(self, te: &mut THREADENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Thread32Next(self.as_ptr(), te as *mut _ as _)
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

struct ModuleIter<'a> {
	hpl: HPROCESSLIST,
	me32: MODULEENTRY32,
	first_pass: bool,
	has_more: bool,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for ModuleIter<'a> {
	type Item = SysResult<&'a MODULEENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Module32First(&mut self.me32)
		} else {
			self.hpl.Module32Next(&mut self.me32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.me32 as *const MODULEENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no module found
				}
			},
		}
	}
}

impl<'a> ModuleIter<'a> {
	fn new(hpl: HPROCESSLIST) -> Self {
		Self {
			hpl,
			me32: MODULEENTRY32::default(),
			first_pass: true,
			has_more: true,
			_owner: PhantomData,
		}
	}
}

//------------------------------------------------------------------------------

struct ProcessIter<'a> {
	hpl: HPROCESSLIST,
	pe32: PROCESSENTRY32,
	first_pass: bool,
	has_more: bool,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for ProcessIter<'a> {
	type Item = SysResult<&'a PROCESSENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Process32First(&mut self.pe32)
		} else {
			self.hpl.Process32Next(&mut self.pe32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.pe32 as *const PROCESSENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no process found
				}
			},
		}
	}
}

impl<'a> ProcessIter<'a> {
	fn new(hpl: HPROCESSLIST) -> Self {
		Self {
			hpl,
			pe32: PROCESSENTRY32::default(),
			first_pass: true,
			has_more: true,
			_owner: PhantomData,
		}
	}
}

//------------------------------------------------------------------------------

struct ThreadIter<'a> {
	hpl: HPROCESSLIST,
	te32: THREADENTRY32,
	first_pass: bool,
	has_more: bool,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for ThreadIter<'a> {
	type Item = SysResult<&'a THREADENTRY32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Thread32First(&mut self.te32)
		} else {
			self.hpl.Thread32Next(&mut self.te32)
		};

		match has_more_res {
			Err(e) => {
				self.has_more = false; // no further iterations
				Some(Err(e))
			},
			Ok(has_more) => {
				self.has_more = has_more;
				if has_more {
					// Returning a reference cannot be done until GATs
					// stabilization, so we simply cheat the borrow checker.
					let ptr = &self.te32 as *const THREADENTRY32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no thread found
				}
			},
		}
	}
}

impl<'a> ThreadIter<'a> {
	fn new(hpl: HPROCESSLIST) -> Self {
		Self {
			hpl,
			te32: THREADENTRY32::default(),
			first_pass: true,
			has_more: true,
			_owner: PhantomData,
		}
	}
}