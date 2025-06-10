use std::ops::{Deref, DerefMut};

use crate::decl::*;
use crate::kernel::ffi;
use crate::prelude::*;

/// RAII implementation for a [`Handle`](crate::prelude::Handle) which
/// automatically calls
/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
/// when the object goes out of scope.
pub struct CloseHandleGuard<T>
where
	T: Handle,
{
	handle: T,
}

impl<T> Drop for CloseHandleGuard<T>
where
	T: Handle,
{
	fn drop(&mut self) {
		if let Some(h) = self.handle.as_opt() {
			unsafe {
				ffi::CloseHandle(h.ptr()); // ignore errors
			}
		}
	}
}

impl<T> Deref for CloseHandleGuard<T>
where
	T: Handle,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.handle
	}
}

impl<T> DerefMut for CloseHandleGuard<T>
where
	T: Handle,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.handle
	}
}

impl<T> CloseHandleGuard<T>
where
	T: Handle,
{
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(handle: T) -> Self {
		Self { handle }
	}

	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> T {
		std::mem::replace(&mut self.handle, T::INVALID)
	}
}

/// RAII implementation for [`PROCESS_INFORMATION`](crate::PROCESS_INFORMATION)
/// which automatically calls
/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
/// on `hProcess` and `hThread` fields when the object goes out of scope.
pub struct CloseHandlePiGuard {
	pi: PROCESS_INFORMATION,
}

impl Drop for CloseHandlePiGuard {
	fn drop(&mut self) {
		if let Some(h) = self.pi.hProcess.as_opt() {
			let _ = unsafe { CloseHandleGuard::new(h.raw_copy()) };
		}
		if let Some(h) = self.pi.hThread.as_opt() {
			let _ = unsafe { CloseHandleGuard::new(h.raw_copy()) };
		}
	}
}

impl Deref for CloseHandlePiGuard {
	type Target = PROCESS_INFORMATION;

	fn deref(&self) -> &Self::Target {
		&self.pi
	}
}

impl DerefMut for CloseHandlePiGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.pi
	}
}

impl CloseHandlePiGuard {
	/// Constructs the guard by taking ownership of the struct.
	///
	/// # Safety
	///
	/// Be sure the handles must be freed with
	/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
	/// at the end of the scope.
	#[must_use]
	pub const unsafe fn new(pi: PROCESS_INFORMATION) -> Self {
		Self { pi }
	}

	/// Ejects the underlying struct, leaving
	/// [`PROCESS_INFORMATION::default`](crate::PROCESS_INFORMATION::default) in
	/// its place.
	///
	/// Since the internal handles will be invalidated, the destructor will not
	/// run. It's your responsibility to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> PROCESS_INFORMATION {
		std::mem::take(&mut self.pi)
	}
}

/// RAII implementation [`HUPDATERSRC`](crate::HUPDATERSRC) which automatically
/// calls
/// [`EndUpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-endupdateresourcew)
/// when the object goes out of scope.
pub struct EndUpdateResourceGuard {
	hupsrc: HUPDATERSRC,
}

impl Drop for EndUpdateResourceGuard {
	fn drop(&mut self) {
		if let Some(h) = self.hupsrc.as_opt() {
			unsafe {
				ffi::EndUpdateResourceW(h.ptr(), false as _);
			} // ignore errors
		}
	}
}

impl Deref for EndUpdateResourceGuard {
	type Target = HUPDATERSRC;

	fn deref(&self) -> &Self::Target {
		&self.hupsrc
	}
}

impl DerefMut for EndUpdateResourceGuard {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.hupsrc
	}
}

impl EndUpdateResourceGuard {
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`EndUpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-endupdateresourcew)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hupsrc: HUPDATERSRC) -> Self {
		Self { hupsrc }
	}

	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub fn leak(&mut self) -> HUPDATERSRC {
		std::mem::replace(&mut self.hupsrc, HUPDATERSRC::INVALID)
	}
}

handle_guard! { FindCloseGuard: HFINDFILE;
	ffi::FindClose;
	/// RAII implementation for [`HFINDFILE`](crate::HFINDFILE) which
	/// automatically calls
	/// [`FindClose`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findclose)
	/// when the object goes out of scope.
}

handle_guard! { FreeLibraryGuard: HINSTANCE;
	ffi::FreeLibrary;
	/// RAII implementation for [`HINSTANCE`](crate::HINSTANCE) which
	/// automatically calls
	/// [`FreeLibrary`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary)
	/// when the object goes out of scope.
}

handle_guard! { GlobalFreeGuard: HGLOBAL;
	ffi::GlobalFree;
	/// RAII implementation for [`HGLOBAL`](crate::HGLOBAL) which automatically
	/// calls
	/// [`GlobalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalfree)
	/// when the object goes out of scope.
}

/// RAII implementation for [`HGLOBAL`](crate::HGLOBAL) lock which automatically
/// calls
/// [`GlobalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalunlock)
/// when the object goes out of scope.
pub struct GlobalUnlockGuard<'a> {
	hglobal: &'a HGLOBAL,
	pmem: *mut std::ffi::c_void,
	sz: usize,
}

impl<'a> Drop for GlobalUnlockGuard<'a> {
	fn drop(&mut self) {
		if let Some(h) = self.hglobal.as_opt() {
			unsafe {
				ffi::GlobalUnlock(h.ptr()); // ignore errors
			}
		}
	}
}

impl<'a> GlobalUnlockGuard<'a> {
	/// Constructs the guard.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`GlobalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalunlock)
	/// at the end of scope, the pointer is valid, and the size is correct.
	#[must_use]
	pub const unsafe fn new(hglobal: &'a HGLOBAL, pmem: *mut std::ffi::c_void, sz: usize) -> Self {
		Self { hglobal, pmem, sz }
	}

	pub_fn_mem_block!();
}

handle_guard! { HeapDestroyGuard: HHEAP;
	ffi::HeapDestroy;
	/// RAII implementation for [`HHEAP`](crate::HHEAP) which automatically
	/// calls
	/// [`HeapDestroy`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapdestroy)
	/// when the object goes out of scope.
}

/// RAII implementation for the memory allocated by
/// [`HHEAP::HeapAlloc`](crate::HHEAP::HeapAlloc) which automatically calls
/// [`HeapFree`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree)
/// when the object goes out of scope.
pub struct HeapFreeGuard<'a> {
	hheap: &'a HHEAP,
	pmem: *mut std::ffi::c_void,
	sz: usize,
}

impl<'a> Drop for HeapFreeGuard<'a> {
	fn drop(&mut self) {
		if let Some(h) = self.hheap.as_opt() {
			if !self.pmem.is_null() {
				unsafe {
					ffi::HeapFree(h.ptr(), 0, self.pmem); // ignore errors
				}
			}
		}
	}
}

impl<'a> HeapFreeGuard<'a> {
	/// Constructs the guard by taking ownership of the handle.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`HeapFree`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree)
	/// at the end of scope, the pointer is valid, and the size is correct.
	#[must_use]
	pub const unsafe fn new(hheap: &'a HHEAP, pmem: *mut std::ffi::c_void, sz: usize) -> Self {
		Self { hheap, pmem, sz }
	}

	/// Ejects the underlying memory pointer and size, leaving null and zero in
	/// their places.
	///
	/// Since the internal memory pointer will be invalidated, the destructor
	/// will not run. It's your responsibility to run it, otherwise you'll cause
	/// a memory leak.
	#[must_use]
	pub fn leak(&mut self) -> (*mut std::ffi::c_void, usize) {
		(
			std::mem::replace(&mut self.pmem, std::ptr::null_mut()),
			std::mem::replace(&mut self.sz, 0),
		)
	}

	pub_fn_mem_block!();
}

/// RAII implementation for [`HHEAP`](crate::HHEAP) which automatically calls
/// [`HeapUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapunlock)
/// when the object goes out of scope.
pub struct HeapUnlockGuard<'a> {
	hheap: &'a HHEAP,
}

impl<'a> Drop for HeapUnlockGuard<'a> {
	fn drop(&mut self) {
		if let Some(h) = self.hheap.as_opt() {
			unsafe {
				ffi::HeapUnlock(h.ptr()); // ignore errors
			}
		}
	}
}

impl<'a> HeapUnlockGuard<'a> {
	/// Constructs the guard.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`HeapUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapunlock)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hheap: &'a HHEAP) -> Self {
		Self { hheap }
	}
}

handle_guard! { LocalFreeGuard: HLOCAL;
	ffi::LocalFree;
	/// RAII implementation for [`HLOCAL`](crate::HLOCAL) which automatically
	/// calls
	/// [`LocalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localfree)
	/// when the object goes out of scope.
}

/// RAII implementation for [`HLOCAL`](crate::HLOCAL) lock which automatically
/// calls
/// [`LocalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localunlock)
/// when the object goes out of scope.
pub struct LocalUnlockGuard<'a> {
	hlocal: &'a HLOCAL,
	pmem: *mut std::ffi::c_void,
	sz: usize,
}

impl<'a> Drop for LocalUnlockGuard<'a> {
	fn drop(&mut self) {
		if let Some(h) = self.hlocal.as_opt() {
			unsafe {
				ffi::LocalUnlock(h.ptr()); // ignore errors
			}
		}
	}
}

impl<'a> LocalUnlockGuard<'a> {
	/// Constructs the guard.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`LocalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-localunlock)
	/// at the end of scope, the pointer is valid, and the size is correct.
	#[must_use]
	pub const unsafe fn new(hlocal: &'a HLOCAL, pmem: *mut std::ffi::c_void, sz: usize) -> Self {
		Self { hlocal, pmem, sz }
	}

	pub_fn_mem_block!();
}

/// RAII implementation for the [`HFILE`](crate::HFILE) lock which automatically
/// calls
/// [`UnlockFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
/// when the object goes out of scope.
pub struct UnlockFileGuard<'a> {
	hfile: &'a HFILE,
	offset: u64,
	num_bytes_to_lock: u64,
}

impl<'a> Drop for UnlockFileGuard<'a> {
	fn drop(&mut self) {
		if let Some(h) = self.hfile.as_opt() {
			unsafe {
				ffi::UnlockFile(
					h.ptr(),
					LODWORD(self.offset),
					HIDWORD(self.offset),
					LODWORD(self.num_bytes_to_lock),
					HIDWORD(self.num_bytes_to_lock),
				); // ignore errors
			}
		}
	}
}

impl<'a> UnlockFileGuard<'a> {
	/// Constructs the guard by taking ownership of the objects.
	///
	/// # Safety
	///
	/// Be sure the handle must be freed with
	/// [`UnlockFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
	/// at the end of scope.
	#[must_use]
	pub const unsafe fn new(hfile: &'a HFILE, offset: u64, num_bytes_to_lock: u64) -> Self {
		Self { hfile, offset, num_bytes_to_lock }
	}

	/// Returns the memory offset of the lock.
	#[must_use]
	pub const fn offset(&self) -> u64 {
		self.offset
	}

	/// Returns the number of locked bytes.
	#[must_use]
	pub const fn num_bytes_to_lock(&self) -> u64 {
		self.num_bytes_to_lock
	}
}

handle_guard! { UnmapViewOfFileGuard: HFILEMAPVIEW;
	ffi::UnmapViewOfFile;
	/// RAII implementation for [`HFILEMAPVIEW`](crate::HFILEMAPVIEW) which
	/// automatically calls
	/// [`UnmapViewOfFile`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-unmapviewoffile)
	/// when the object goes out of scope.
}
