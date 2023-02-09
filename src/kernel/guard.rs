use std::ops::{Deref, DerefMut};

use crate::kernel;
use crate::kernel::decl::{
	HFILEMAPVIEW, HFINDFILE, HGLOBAL, HIDWORD, HINSTANCE, HUPDATERSRC, LODWORD,
	PROCESS_INFORMATION,
};
use crate::prelude::{Handle, kernel_Hfile, kernel_Hglobal};

/// RAII implementation for a [`Handle`](crate::prelude::Handle) which
/// automatically calls
/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
/// when the object goes out of scope.
pub struct CloseHandleGuard<T>
	where T: Handle,
{
	handle: T,
}

impl<T> Drop for CloseHandleGuard<T>
	where T: Handle,
{
	fn drop(&mut self) {
		if let Some(h) = self.handle.as_opt() {
			unsafe { kernel::ffi::CloseHandle(h.as_ptr()); } // ignore errors
		}
	}
}

impl<T> Deref for CloseHandleGuard<T>
	where T: Handle,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.handle
	}
}

impl<T> DerefMut for CloseHandleGuard<T>
	where T: Handle,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.handle
	}
}

impl<T> CloseHandleGuard<T>
	where T: Handle,
{
	/// Constructs the guard by taking ownership of the handle.
	#[must_use]
	pub const fn new(handle: T) -> Self {
		Self { handle }
	}

	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// # Safety
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub unsafe fn leak(&mut self) -> T {
		std::mem::replace(&mut self.handle, T::INVALID)
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
			unsafe { kernel::ffi::EndUpdateResourceW(h.as_ptr(), false as _); } // ignore errors
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
	#[must_use]
	pub const fn new(hupsrc: HUPDATERSRC) -> Self {
		Self { hupsrc }
	}

	/// Ejects the underlying handle, leaving a
	/// [`Handle::INVALID`](crate::prelude::Handle::INVALID) in its place.
	///
	/// # Safety
	///
	/// Since the internal handle will be invalidated, the destructor will not
	/// run. It's your responsability to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub unsafe fn leak(&mut self) -> HUPDATERSRC {
		std::mem::replace(&mut self.hupsrc, HUPDATERSRC::INVALID)
	}
}

handle_guard! { FindCloseGuard: HFINDFILE;
	kernel::ffi::FindClose;
	/// RAII implementation for [`HFINDFILE`](crate::HFINDFILE) which
	/// automatically calls
	/// [`FindClose`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findclose)
	/// when the object goes out of scope.
}

handle_guard! { FreeLibraryGuard: HINSTANCE;
	kernel::ffi::FreeLibrary;
	/// RAII implementation for [`HINSTANCE`](crate::HINSTANCE) which
	/// automatically calls
	/// [`FreeLibrary`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary)
	/// when the object goes out of scope.
}

handle_guard! { GlobalFreeGuard: HGLOBAL;
	kernel::ffi::GlobalFree;
	/// RAII implementation for [`HGLOBAL`](crate::HGLOBAL) which automatically
	/// calls
	/// [`GlobalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalfree)
	/// when the object goes out of scope.
}

/// RAII implementation for [`HGLOBAL`](crate::HGLOBAL) lock which automatically
/// calls
/// [`GlobalUnlock`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalunlock)
/// when the object goes out of scope.
pub struct GlobalUnlockGuard<'a, H>
	where H: kernel_Hglobal,
{
	hglobal: &'a H,
}

impl<'a, H> Drop for GlobalUnlockGuard<'a, H>
	where H: kernel_Hglobal,
{
	fn drop(&mut self) {
		if let Some(h) = self.hglobal.as_opt() {
			unsafe { kernel::ffi::GlobalUnlock(h.as_ptr()); } // ignore errors
		}
	}
}

impl<'a, H> GlobalUnlockGuard<'a, H>
	where H: kernel_Hglobal,
{
	/// Constructs the guard by taking ownership of the objects.
	#[must_use]
	pub const fn new(hglobal: &'a H) -> Self {
		Self { hglobal }
	}
}

/// RAII implementation for the [`HFILE`](crate::HFILE) lock which automatically
/// calls
/// [`UnlockFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
/// when the object goes out of scope.
pub struct UnlockFileGuard<'a, H>
	where H: kernel_Hfile,
{
	hfile: &'a H,
	offset: u64,
	num_bytes_to_lock: u64,
}

impl<'a, H> Drop for UnlockFileGuard<'a, H>
	where H: kernel_Hfile,
{
	fn drop(&mut self) {
		unsafe {
			kernel::ffi::UnlockFile( // ignore errors
				self.hfile.as_ptr(),
				LODWORD(self.offset),
				HIDWORD(self.offset),
				LODWORD(self.num_bytes_to_lock),
				HIDWORD(self.num_bytes_to_lock),
			);
		}
	}
}

impl<'a, H> UnlockFileGuard<'a, H>
	where H: kernel_Hfile,
{
	/// Constructs the guard by taking ownership of the objects.
	#[must_use]
	pub const fn new(
		hfile: &'a H,
		offset: u64,
		num_bytes_to_lock: u64) -> Self
	{
		Self { hfile, offset, num_bytes_to_lock }
	}
}

handle_guard! { UnmapViewOfFileGuard: HFILEMAPVIEW;
	kernel::ffi::UnmapViewOfFile;
	/// RAII implementation for [`HFILEMAPVIEW`](crate::HFILEMAPVIEW) which
	/// automatically calls
	/// [`UnmapViewOfFile`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-unmapviewoffile)
	/// when the object goes out of scope.
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
			let _ = CloseHandleGuard::new(unsafe { h.raw_copy() });
		}
		if let Some(h) = self.pi.hThread.as_opt() {
			let _ = CloseHandleGuard::new(unsafe { h.raw_copy() });
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
	#[must_use]
	pub const fn new(pi: PROCESS_INFORMATION) -> Self {
		Self { pi }
	}

	/// Ejects the underlying struct, leaving
	/// [`PROCESS_INFORMATION::default`](crate::PROCESS_INFORMATION::default) in
	/// its place.
	///
	/// # Safety
	///
	/// Since the internal handles will be invalidated, the destructor will not
	/// run. It's your responsibility to run it, otherwise you'll cause a
	/// resource leak.
	#[must_use]
	pub unsafe fn leak(&mut self) -> PROCESS_INFORMATION {
		std::mem::take(&mut self.pi)
	}
}
