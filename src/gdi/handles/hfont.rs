#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, gdi};
use crate::gdi::decl::LOGFONT;
use crate::gdi::guard::DeleteObjectGuard;
use crate::kernel::decl::{SysResult, WString};
use crate::kernel::privs::{bool_to_sysresult, ptr_to_sysresult_handle};
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
	/// function.
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
				),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateFontIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createfontindirectw)
	/// function.
	#[must_use]
	fn CreateFontIndirect(lf: &LOGFONT) -> SysResult<DeleteObjectGuard<HFONT>> {
		unsafe {
			ptr_to_sysresult_handle(
				gdi::ffi::CreateFontIndirectW(lf as *const _ as _),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// function.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HFONT, LOGFONT};
	///
	/// let hfont: HFONT; // initialized somewhere
	/// # let hfont = HFONT::NULL;
	///
	/// let mut log_font = LOGFONT::default();
	/// hfont.GetObject(&mut log_font)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	fn GetObject(&self, lf: &mut LOGFONT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				gdi::ffi::GetObjectW(
					self.ptr(),
					std::mem::size_of::<LOGFONT>() as _,
					lf as *mut _ as _,
				)
			},
		)
	}

	/// [`GetStockObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// function.
	#[must_use]
	fn GetStockObject(sf: co::STOCK_FONT) -> SysResult<HFONT> {
		ptr_to_sysresult_handle(unsafe { gdi::ffi::GetStockObject(sf.raw()) })
	}
}
