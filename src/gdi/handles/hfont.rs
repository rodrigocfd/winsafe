#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

handle! { HFONT;
	/// Handle to a
	/// [font](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfont).
}

impl GdiObject for HFONT {}

impl HFONT {
	/// [`CreateFont`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontw)
	/// function.
	#[must_use]
	pub fn CreateFont(
		sz: SIZE,
		escapement: i32,
		orientation: i32,
		weight: co::FW,
		italic: bool,
		underline: bool,
		strike_out: bool,
		char_set: co::CHARSET,
		out_precision: co::OUT_PRECIS,
		clip_precision: co::CLIP,
		quality: co::QUALITY,
		pitch_and_family: co::PITCH,
		face_name: &str,
	) -> SysResult<DeleteObjectGuard<HFONT>> {
		unsafe {
			PtrRet(ffi::CreateFontW(
				sz.cy,
				sz.cx,
				escapement,
				orientation,
				weight.raw() as _,
				italic as _,
				underline as _,
				strike_out as _,
				char_set.raw() as _,
				out_precision.raw() as _,
				clip_precision.raw() as _,
				quality.raw() as _,
				pitch_and_family.raw() as _,
				WString::from_str(face_name).as_ptr(),
			))
			.to_invalidparm_handle()
			.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateFontIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontindirectw)
	/// function.
	#[must_use]
	pub fn CreateFontIndirect(lf: &LOGFONT) -> SysResult<DeleteObjectGuard<HFONT>> {
		unsafe {
			PtrRet(ffi::CreateFontIndirectW(pcvoid(lf)))
				.to_invalidparm_handle()
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// function.
	pub fn GetObject(&self) -> SysResult<LOGFONT> {
		let mut lf = LOGFONT::default();
		BoolRet(unsafe {
			ffi::GetObjectW(self.ptr(), std::mem::size_of::<LOGFONT>() as _, pvoid(&mut lf))
		})
		.to_invalidparm()
		.map(|_| lf)
	}

	/// [`GetStockObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// function.
	#[must_use]
	pub fn GetStockObject(sf: co::STOCK_FONT) -> SysResult<HFONT> {
		PtrRet(unsafe { ffi::GetStockObject(sf.raw()) }).to_invalidparm_handle()
	}
}
