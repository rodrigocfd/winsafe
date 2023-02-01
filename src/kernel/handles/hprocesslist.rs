#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	GetLastError, HEAPLIST32, MODULEENTRY32, PROCESSENTRY32, SysResult,
	THREADENTRY32,
};
use crate::kernel::guard::CloseHandleGuard;
use crate::kernel::privs::{as_mut, ptr_to_sysresult};
use crate::prelude::Handle;

impl_handle! { HPROCESSLIST;
	/// Handle to a process list
	/// [snapshot](https://learn.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes).
	/// Originally just a `HANDLE`.
}

impl kernel_Hprocesslist for HPROCESSLIST {}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HPROCESSLIST`](crate::HPROCESSLIST).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hprocesslist: Handle {
	/// Returns an iterator over the heaps of a process, with
	/// [`HEAPLIST32`](crate::HEAPLIST32) structs. Calls
	/// [`HPROCESSLIST::Heap32ListFirst`](crate::prelude::kernel_Hprocesslist::Heap32ListFirst)
	/// and then
	/// [`HPROCESSLIST::Heap32ListNext`](crate::prelude::kernel_Hprocesslist::Heap32ListNext)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HPROCESSLIST};
	///
	/// let hpl = HPROCESSLIST::
	///     CreateToolhelp32Snapshot(co::TH32CS::SNAPHEAPLIST, None)?;
	///
	/// for heap_entry in hpl.iter_heaps() {
	///     let heap_entry = heap_entry?;
	///     let is_default_heap = heap_entry.dwFlags == co::HF32::DEFAULT;
	///     println!("{} {}",
	///         heap_entry.th32HeapID, heap_entry.th32ProcessID);
	/// }
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_heaps(&self)
		-> Box<dyn Iterator<Item = SysResult<&HEAPLIST32>> + '_>
	{
		Box::new(HeapIter::new(self))
	}

	/// Returns an iterator over the modules of a process, with
	/// [`MODULEENTRY32`](crate::MODULEENTRY32) structs. Calls
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
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_modules(&self)
		-> Box<dyn Iterator<Item = SysResult<&MODULEENTRY32>> + '_>
	{
		Box::new(ModuleIter::new(self))
	}

	/// Returns an iterator over the processes of a process, with
	/// [`PROCESSENTRY32`](crate::PROCESSENTRY32) structs. Calls
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
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_processes(&self)
		-> Box<dyn Iterator<Item = SysResult<&PROCESSENTRY32>> + '_>
	{
		Box::new(ProcessIter::new(self))
	}

	/// Returns an iterator over the threads of a process, with
	/// [`THREADENTRY32`](crate::THREADENTRY32) structs. Calls
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
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn iter_threads(&self)
		-> Box<dyn Iterator<Item = SysResult<&THREADENTRY32>> + '_>
	{
		Box::new(ThreadIter::new(self))
	}

	/// [`CreateToolhelp32Snapshot`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)
	/// static method.
	#[must_use]
	fn CreateToolhelp32Snapshot(
		flags: co::TH32CS,
		th32_process_id: Option<u32>) -> SysResult<CloseHandleGuard<HPROCESSLIST>>
	{
		ptr_to_sysresult(
			unsafe {
				kernel::ffi::CreateToolhelp32Snapshot(
					flags.0,
					th32_process_id.unwrap_or_default(),
				)
			},
			|ptr| CloseHandleGuard::new(HPROCESSLIST(ptr)),
		)
	}

	/// [`HeapList32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-heap32listfirst)
	/// method.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_heaps`](crate::prelude::kernel_Hprocesslist::iter_heaps),
	/// which is simpler.
	#[must_use]
	fn Heap32ListFirst(&self, hl: &mut HEAPLIST32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Heap32ListFirst(self.as_ptr(), hl as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*unsafe { as_mut(self) } = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`HeapList32Next`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-heap32listnext)
	/// method.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_heaps`](crate::prelude::kernel_Hprocesslist::iter_heaps),
	/// which is simpler.
	#[must_use]
	fn Heap32ListNext(&self, hl: &mut HEAPLIST32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Heap32ListNext(self.as_ptr(), hl as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*unsafe { as_mut(self) } = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Module32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-module32firstw)
	/// method.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_modules`](crate::prelude::kernel_Hprocesslist::iter_modules),
	/// which is simpler.
	#[must_use]
	fn Module32First(&self, me: &mut MODULEENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Module32FirstW(self.as_ptr(), me as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*unsafe { as_mut(self) } = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Module32Next`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-module32nextw)
	/// method.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_modules`](crate::prelude::kernel_Hprocesslist::iter_modules),
	/// which is simpler.
	#[must_use]
	fn Module32Next(&self, me: &mut MODULEENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Module32NextW(self.as_ptr(), me as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*unsafe { as_mut(self) } = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Process32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw)
	/// method.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_processes`](crate::prelude::kernel_Hprocesslist::iter_processes),
	/// which is simpler.
	#[must_use]
	fn Process32First(&self, pe: &mut PROCESSENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Process32FirstW(self.as_ptr(), pe as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*unsafe { as_mut(self) } = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Process32Next`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw)
	/// method.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_processes`](crate::prelude::kernel_Hprocesslist::iter_processes),
	/// which is simpler.
	#[must_use]
	fn Process32Next(&self, pe: &mut PROCESSENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Process32NextW(self.as_ptr(), pe as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*unsafe { as_mut(self) } = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Thread32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-thread32first)
	/// method.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_threads`](crate::prelude::kernel_Hprocesslist::iter_threads),
	/// which is simpler.
	#[must_use]
	fn Thread32First(&self, te: &mut THREADENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Thread32First(self.as_ptr(), te as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*unsafe { as_mut(self) } = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Thread32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-thread32next)
	/// method.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_threads`](crate::prelude::kernel_Hprocesslist::iter_threads),
	/// which is simpler.
	#[must_use]
	fn Thread32Next(&self, te: &mut THREADENTRY32) -> SysResult<bool> {
		match unsafe {
			kernel::ffi::Thread32Next(self.as_ptr(), te as *mut _ as _)
		} {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*unsafe { as_mut(self) } = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}

//------------------------------------------------------------------------------

struct HeapIter<'a, H>
	where H: kernel_Hprocesslist,
{
	hpl: &'a H,
	hl32: HEAPLIST32,
	first_pass: bool,
	has_more: bool,
}

impl<'a, H> Iterator for HeapIter<'a, H>
	where H: kernel_Hprocesslist,
{
	type Item = SysResult<&'a HEAPLIST32>;

	fn next(&mut self) -> Option<Self::Item> {
		if !self.has_more {
			return None;
		}

		let has_more_res = if self.first_pass {
			self.first_pass = false;
			self.hpl.Heap32ListFirst(&mut self.hl32)
		} else {
			self.hpl.Heap32ListNext(&mut self.hl32)
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
					let ptr = &self.hl32 as *const HEAPLIST32;
					Some(Ok(unsafe { &*ptr }))
				} else {
					None // no heap found
				}
			},
		}
	}
}

impl<'a, H> HeapIter<'a, H>
	where H: kernel_Hprocesslist,
{
	fn new(hpl: &'a H) -> Self {
		Self {
			hpl,
			hl32: HEAPLIST32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}

//------------------------------------------------------------------------------

struct ModuleIter<'a, H>
	where H: kernel_Hprocesslist,
{
	hpl: &'a H,
	me32: MODULEENTRY32,
	first_pass: bool,
	has_more: bool,
}

impl<'a, H> Iterator for ModuleIter<'a, H>
	where H: kernel_Hprocesslist,
{
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

impl<'a, H> ModuleIter<'a, H>
	where H: kernel_Hprocesslist,
{
	fn new(hpl: &'a H) -> Self {
		Self {
			hpl,
			me32: MODULEENTRY32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}

//------------------------------------------------------------------------------

struct ProcessIter<'a, H>
	where H: kernel_Hprocesslist,
{
	hpl: &'a H,
	pe32: PROCESSENTRY32,
	first_pass: bool,
	has_more: bool,
}

impl<'a, H> Iterator for ProcessIter<'a, H>
	where H: kernel_Hprocesslist,
{
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

impl<'a, H> ProcessIter<'a, H>
	where H: kernel_Hprocesslist,
{
	fn new(hpl: &'a H) -> Self {
		Self {
			hpl,
			pe32: PROCESSENTRY32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}

//------------------------------------------------------------------------------

struct ThreadIter<'a, H>
	where H: kernel_Hprocesslist,
{
	hpl: &'a H,
	te32: THREADENTRY32,
	first_pass: bool,
	has_more: bool,
}

impl<'a, H> Iterator for ThreadIter<'a, H>
	where H: kernel_Hprocesslist,
{
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

impl<'a, H> ThreadIter<'a, H>
	where H: kernel_Hprocesslist,
{
	fn new(hpl: &'a H) -> Self {
		Self {
			hpl,
			te32: THREADENTRY32::default(),
			first_pass: true,
			has_more: true,
		}
	}
}
