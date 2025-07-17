#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;

impl HINSTANCE {
	/// [`LoadImage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HBITMAP`](crate::HBITMAP).
	#[must_use]
	pub fn LoadImageBitmap(
		&self,
		name: IdObmStr,
		sz: SIZE,
		load: co::LR,
	) -> SysResult<DeleteObjectGuard<HBITMAP>> {
		unsafe {
			PtrRet(ffi::LoadImageW(self.ptr(), name.as_ptr(), 0, sz.cx, sz.cy, load.raw()))
				.to_sysresult_handle()
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`LoadImage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HCURSOR`](crate::HCURSOR).
	#[must_use]
	pub fn LoadImageCursor(
		&self,
		name: IdOcrStr,
		sz: SIZE,
		load: co::LR,
	) -> SysResult<DestroyCursorGuard> {
		unsafe {
			PtrRet(ffi::LoadImageW(self.ptr(), name.as_ptr(), 2, sz.cx, sz.cy, load.raw()))
				.to_sysresult_handle()
				.map(|h| DestroyCursorGuard::new(h))
		}
	}

	/// [`LoadImage`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadimagew)
	/// method for [`HICON`](crate::HICON).
	#[must_use]
	pub fn LoadImageIcon(
		&self,
		name: IdOicStr,
		sz: SIZE,
		load: co::LR,
	) -> SysResult<DestroyIconGuard> {
		unsafe {
			PtrRet(ffi::LoadImageW(self.ptr(), name.as_ptr(), 1, sz.cx, sz.cy, load.raw()))
				.to_sysresult_handle()
				.map(|h| DestroyIconGuard::new(h))
		}
	}
}
