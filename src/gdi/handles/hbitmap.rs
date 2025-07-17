#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl GdiObject for HBITMAP {}

impl HBITMAP {
	/// [`CreateBitmap`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbitmap)
	/// function.
	#[must_use]
	pub fn CreateBitmap(
		sz: SIZE,
		num_planes: u32,
		bit_count: u32,
		bits: *mut u8,
	) -> SysResult<DeleteObjectGuard<HBITMAP>> {
		unsafe {
			PtrRet(ffi::CreateBitmap(sz.cx, sz.cy, num_planes, bit_count, bits as _))
				.to_invalidparm_handle()
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// function.
	pub fn GetObject(&self) -> SysResult<BITMAP> {
		let mut bm = BITMAP::default();
		BoolRet(unsafe {
			ffi::GetObjectW(self.ptr(), std::mem::size_of::<BITMAP>() as _, pvoid(&mut bm))
		})
		.to_invalidparm()
		.map(|_| bm)
	}
}
