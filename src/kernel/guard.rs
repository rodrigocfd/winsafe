use std::ops::Deref;

use crate::kernel;
use crate::kernel::decl::{
	HFILEMAPVIEW, HFINDFILE, HGLOBAL, HIDWORD, HINSTANCE, HUPDATERSRC, LODWORD,
	PROCESS_INFORMATION,
};
use crate::prelude::{Handle, kernel_Hfile};

/// RAII implementation for a [`Handle`](crate::prelude::Handle) which
/// automatically calls
/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
/// when the object goes out of scope.
pub struct HandleGuard<T>
	where T: Handle,
{
	pub(crate) handle: T,
}

impl<T> Drop for HandleGuard<T>
	where T: Handle,
{
	fn drop(&mut self) {
		if let Some(h) = self.handle.as_opt() {
			unsafe { kernel::ffi::CloseHandle(h.as_ptr()); } // ignore errors
		}
	}
}

impl<T> Deref for HandleGuard<T>
	where T: Handle,
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.handle
	}
}

impl<T> HandleGuard<T>
	where T: Handle,
{
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

/// RAII implementation for the [`HFILE`](crate::HFILE) lock which automatically
/// calls
/// [`UnlockFile`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-lockfile)
/// when the object goes out of scope.
pub struct HfileLockGuard<'a, H>
	where H: kernel_Hfile,
{
	pub(crate) hfile: &'a H,
	pub(crate) offset: u64,
	pub(crate) num_bytes_to_lock: u64,
}

impl<'a, H> Drop for HfileLockGuard<'a, H>
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

handle_guard! { HfilemapviewGuard: HFILEMAPVIEW;
	kernel::ffi::UnmapViewOfFile;
	/// RAII implementation for [`HFILEMAPVIEW`](crate::HFILEMAPVIEW) which
	/// automatically calls
	/// [`UnmapViewOfFile`](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-unmapviewoffile)
	/// when the object goes out of scope.
}

handle_guard! { HfindfileGuard: HFINDFILE;
	kernel::ffi::FindClose;
	/// RAII implementation for [`HFINDFILE`](crate::HFINDFILE) which
	/// automatically calls
	/// [`FindClose`](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-findclose)
	/// when the object goes out of scope.
}

handle_guard! { HglobalGuard: HGLOBAL;
	kernel::ffi::GlobalFree;
	/// RAII implementation for [`HGLOBAL`](crate::HGLOBAL) which automatically
	/// calls
	/// [`GlobalFree`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-globalfree)
	/// when the object goes out of scope.
}

handle_guard! { HinstanceGuard: HINSTANCE;
	kernel::ffi::FreeLibrary;
	/// RAII implementation for [`HINSTANCE`](crate::HINSTANCE) which
	/// automatically calls
	/// [`FreeLibrary`](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-freelibrary)
	/// when the object goes out of scope.
}

/// RAII implementation [`HUPDATERSRC`](crate::HUPDATERSRC) which automatically
/// calls
/// [`EndUpdateResource`](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-endupdateresourcew)
/// when the object goes out of scope.
pub struct HupdatersrcGuard {
	pub(crate) hupsrc: HUPDATERSRC,
}

impl Drop for HupdatersrcGuard {
	fn drop(&mut self) {
		if let Some(h) = self.hupsrc.as_opt() {
			unsafe { kernel::ffi::EndUpdateResourceW(h.as_ptr(), false as _); } // ignore errors
		}
	}
}

impl Deref for HupdatersrcGuard {
	type Target = HUPDATERSRC;

	fn deref(&self) -> &Self::Target {
		&self.hupsrc
	}
}

impl HupdatersrcGuard {
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

/// RAII implementation for [`PROCESS_INFORMATION`](crate::PROCESS_INFORMATION)
/// which automatically calls
/// [`CloseHandle`](https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle)
/// on `hProcess` and `hThread` fields when the object goes out of scope.
pub struct ProcessInformationGuard {
	pub(crate) pi: PROCESS_INFORMATION,
}

impl Drop for ProcessInformationGuard {
	fn drop(&mut self) {
		if let Some(h) = self.pi.hProcess.as_opt() {
			unsafe { kernel::ffi::CloseHandle(h.as_ptr()); } // ignore errors
		}
		if let Some(h) = self.pi.hThread.as_opt() {
			unsafe { kernel::ffi::CloseHandle(h.as_ptr()); }
		}
	}
}

impl Deref for ProcessInformationGuard {
	type Target = PROCESS_INFORMATION;

	fn deref(&self) -> &Self::Target {
		&self.pi
	}
}

impl ProcessInformationGuard {
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
