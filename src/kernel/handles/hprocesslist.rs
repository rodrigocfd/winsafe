#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::{ffi, iterators::*, privs::*};
use crate::prelude::*;

handle! { HPROCESSLIST;
	/// Handle to a process list
	/// [snapshot](https://learn.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes).
	/// Originally just a `HANDLE`.
}

impl HPROCESSLIST {
	/// Returns an iterator over the heaps of a process, with
	/// [`HEAPLIST32`](crate::HEAPLIST32) structs. Calls
	/// [`HPROCESSLIST::Heap32ListFirst`](crate::HPROCESSLIST::Heap32ListFirst)
	/// and then
	/// [`HPROCESSLIST::Heap32ListNext`](crate::HPROCESSLIST::Heap32ListNext)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let mut hpl = w::HPROCESSLIST::
	///     CreateToolhelp32Snapshot(co::TH32CS::SNAPHEAPLIST, None)?;
	///
	/// for heap_entry in hpl.iter_heaps() {
	///     let heap_entry = heap_entry?;
	///     let is_default_heap = heap_entry.dwFlags == co::HF32::DEFAULT;
	///     println!("{} {}",
	///         heap_entry.th32HeapID, heap_entry.th32ProcessID);
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn iter_heaps(&mut self) -> impl Iterator<Item = SysResult<&HEAPLIST32>> + '_ {
		HprocesslistHeapIter::new(self)
	}

	/// Returns an iterator over the modules of a process, with
	/// [`MODULEENTRY32`](crate::MODULEENTRY32) structs. Calls
	/// [`HPROCESSLIST::Module32First`](crate::HPROCESSLIST::Module32First) and
	/// then [`HPROCESSLIST::Module32Next`](crate::HPROCESSLIST::Module32Next)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let mut hpl = w::HPROCESSLIST::
	///     CreateToolhelp32Snapshot(co::TH32CS::SNAPMODULE, None)?;
	///
	/// for mod_entry in hpl.iter_modules() {
	///     let mod_entry = mod_entry?;
	///     println!("{} {}",
	///         mod_entry.szModule(), mod_entry.th32ProcessID);
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn iter_modules(&mut self) -> impl Iterator<Item = SysResult<&MODULEENTRY32>> + '_ {
		HprocesslistModuleIter::new(self)
	}

	/// Returns an iterator over the processes of a process, with
	/// [`PROCESSENTRY32`](crate::PROCESSENTRY32) structs. Calls
	/// [`HPROCESSLIST::Process32First`](crate::HPROCESSLIST::Process32First)
	/// and then
	/// [`HPROCESSLIST::Process32Next`](crate::HPROCESSLIST::Process32Next)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let mut hpl = w::HPROCESSLIST::
	///     CreateToolhelp32Snapshot(co::TH32CS::SNAPPROCESS, None)?;
	///
	/// for proc_entry in hpl.iter_processes() {
	///     let proc_entry = proc_entry?;
	///     println!("{} {} {}",
	///         proc_entry.szExeFile(), proc_entry.th32ProcessID, proc_entry.cntThreads);
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn iter_processes(&mut self) -> impl Iterator<Item = SysResult<&PROCESSENTRY32>> + '_ {
		HprocesslistProcessIter::new(self)
	}

	/// Returns an iterator over the threads of a process, with
	/// [`THREADENTRY32`](crate::THREADENTRY32) structs. Calls
	/// [`HPROCESSLIST::Thread32First`](crate::HPROCESSLIST::Thread32First) and
	/// then [`HPROCESSLIST::Thread32Next`](crate::HPROCESSLIST::Thread32Next)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let mut hpl = w::HPROCESSLIST::CreateToolhelp32Snapshot(
	///     co::TH32CS::SNAPTHREAD,
	///     Some(w::GetCurrentProcessId()),
	/// )?;
	///
	/// for thread_entry in hpl.iter_threads() {
	///     let thread_entry = thread_entry?;
	///     println!("{} {}",
	///         thread_entry.th32ThreadID, thread_entry.th32OwnerProcessID);
	/// }
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn iter_threads(&mut self) -> impl Iterator<Item = SysResult<&THREADENTRY32>> + '_ {
		HprocesslistThreadIter::new(self)
	}

	/// [`CreateToolhelp32Snapshot`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)
	/// function.
	#[must_use]
	pub fn CreateToolhelp32Snapshot(
		flags: co::TH32CS,
		th32_process_id: Option<u32>,
	) -> SysResult<CloseHandleGuard<HPROCESSLIST>> {
		unsafe {
			PtrRet(ffi::CreateToolhelp32Snapshot(flags.raw(), th32_process_id.unwrap_or_default()))
				.to_sysresult_handle()
				.map(|h| CloseHandleGuard::new(h))
		}
	}

	/// [`HeapList32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-heap32listfirst)
	/// function.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_heaps`](crate::HPROCESSLIST::iter_heaps), which is
	/// simpler.
	#[must_use]
	pub fn Heap32ListFirst(&mut self, hl: &mut HEAPLIST32) -> SysResult<bool> {
		match unsafe { ffi::Heap32ListFirst(self.ptr(), pvoid(hl)) } {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*self = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`HeapList32Next`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-heap32listnext)
	/// function.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_heaps`](crate::HPROCESSLIST::iter_heaps), which is
	/// simpler.
	#[must_use]
	pub fn Heap32ListNext(&mut self, hl: &mut HEAPLIST32) -> SysResult<bool> {
		match unsafe { ffi::Heap32ListNext(self.ptr(), pvoid(hl)) } {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*self = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Module32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-module32firstw)
	/// function.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_modules`](crate::HPROCESSLIST::iter_modules), which
	/// is simpler.
	#[must_use]
	pub fn Module32First(&mut self, me: &mut MODULEENTRY32) -> SysResult<bool> {
		match unsafe { ffi::Module32FirstW(self.ptr(), pvoid(me)) } {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*self = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Module32Next`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-module32nextw)
	/// function.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_modules`](crate::HPROCESSLIST::iter_modules), which
	/// is simpler.
	#[must_use]
	pub fn Module32Next(&mut self, me: &mut MODULEENTRY32) -> SysResult<bool> {
		match unsafe { ffi::Module32NextW(self.ptr(), pvoid(me)) } {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*self = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Process32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32firstw)
	/// function.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_processes`](crate::HPROCESSLIST::iter_processes),
	/// which is simpler.
	#[must_use]
	pub fn Process32First(&mut self, pe: &mut PROCESSENTRY32) -> SysResult<bool> {
		match unsafe { ffi::Process32FirstW(self.ptr(), pvoid(pe)) } {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*self = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Process32Next`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32nextw)
	/// function.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_processes`](crate::HPROCESSLIST::iter_processes),
	/// which is simpler.
	#[must_use]
	pub fn Process32Next(&mut self, pe: &mut PROCESSENTRY32) -> SysResult<bool> {
		match unsafe { ffi::Process32NextW(self.ptr(), pvoid(pe)) } {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*self = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Thread32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-thread32first)
	/// function.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_threads`](crate::HPROCESSLIST::iter_threads), which
	/// is simpler.
	#[must_use]
	pub fn Thread32First(&mut self, te: &mut THREADENTRY32) -> SysResult<bool> {
		match unsafe { ffi::Thread32First(self.ptr(), pvoid(te)) } {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*self = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}

	/// [`Thread32First`](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-thread32next)
	/// function.
	///
	/// After the listing ends, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	///
	/// Prefer using
	/// [`HPROCESSLIST::iter_threads`](crate::HPROCESSLIST::iter_threads), which
	/// is simpler.
	#[must_use]
	pub fn Thread32Next(&mut self, te: &mut THREADENTRY32) -> SysResult<bool> {
		match unsafe { ffi::Thread32Next(self.ptr(), pvoid(te)) } {
			0 => match GetLastError() {
				co::ERROR::NO_MORE_FILES => {
					*self = Self::INVALID;
					Ok(false)
				},
				err => Err(err),
			},
			_ => Ok(true),
		}
	}
}
