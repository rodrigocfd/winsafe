#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, gdi};
use crate::gdi::decl::LOGFONT;
use crate::gdi::guard::DeleteObjectGuard;
use crate::kernel::decl::{GetLastError, SysResult, WString};
use crate::kernel::privs::ptr_to_sysresult_handle;
use crate::prelude::{GdiObject, GdiObjectSelect, Handle};
use crate::user::decl::SIZE;

impl_handle! { HFONT;
	/// Handle to a
	/// [font](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hfont).
}

impl GdiObject for HFONT {}
impl GdiObjectSelect for HFONT {}
impl gdi_Hfont for HFONT {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HFONT`](crate::HFONT).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hfont: Handle {
	/// [`CreateFont`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontw)
	/// static method.
	#[must_use]
	fn CreateFont(
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
	) -> SysResult<DeleteObjectGuard<HFONT>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				gdi::ffi::CreateFontW(
					sz.cy, sz.cx, escapement, orientation,
					weight.0 as _,
					italic as _, underline as _, strike_out as _,
					char_set.0 as _,
					out_precision.0 as _, clip_precision.0 as _,
					quality.0 as _, pitch_and_family.0 as _,
					WString::from_str(face_name).as_ptr(),
				),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateFontIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontindirectw)
	/// static method.
	#[must_use]
	fn CreateFontIndirect(lf: &LOGFONT) -> SysResult<DeleteObjectGuard<HFONT>> {
		unsafe {
			ptr_to_sysresult_handle(
				gdi::ffi::CreateFontIndirectW(lf as *const _ as _),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// method.
	fn GetObject(&self, lf: &mut LOGFONT) -> SysResult<()> {
		match unsafe {
			gdi::ffi::GetObjectW(
				self.as_ptr(),
				std::mem::size_of::<LOGFONT>() as _,
				lf as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`GetStockObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// static method.
	#[must_use]
	fn GetStockObject(sf: co::STOCK_FONT) -> SysResult<HFONT> {
		ptr_to_sysresult_handle(unsafe { gdi::ffi::GetStockObject(sf.0) })
	}
}
