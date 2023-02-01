#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, gdi};
use crate::gdi::decl::LOGBRUSH;
use crate::gdi::guard::DeleteObjectGuard;
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::{bool_to_sysresult, ptr_to_sysresult};
use crate::prelude::GdiObject;
use crate::user::decl::{COLORREF, HBITMAP, HBRUSH};

impl GdiObject for HBRUSH {}
impl gdi_Hbrush for HBRUSH {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HBRUSH`](crate::HBRUSH).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hbrush: GdiObject {
	/// Creates a brush with the given system color.
	///
	/// **Note:** This should be used only to initialize the
	/// [`WNDCLASSEX`](crate::WNDCLASSEX)'s `hbrBackground` field. Any other use
	/// will yield an invalid handle.
	#[must_use]
	fn from_sys_color(color: co::COLOR) -> HBRUSH {
		HBRUSH((color.0 + 1) as _ )
	}

	/// [`CreateBrushIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbrushindirect)
	/// static method.
	#[must_use]
	fn CreateBrushIndirect(lb: &LOGBRUSH) -> SysResult<DeleteObjectGuard<HBRUSH>> {
		ptr_to_sysresult(
			unsafe { gdi::ffi::CreateBrushIndirect(lb as *const _ as _) },
			|ptr| DeleteObjectGuard::new(HBRUSH(ptr)),
		)
	}

	/// [`CreateHatchBrush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createhatchbrush)
	/// static method.
	#[must_use]
	fn CreateHatchBrush(
		hatch: co::HS, color: COLORREF) -> SysResult<DeleteObjectGuard<HBRUSH>>
	{
		ptr_to_sysresult(
			unsafe { gdi::ffi::CreateHatchBrush(hatch.0, color.0) },
			|ptr| DeleteObjectGuard::new(HBRUSH(ptr)),
		)
	}

	/// [`CreatePatternBrush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpatternbrush)
	/// static method.
	#[must_use]
	fn CreatePatternBrush(hbmp: &HBITMAP) -> SysResult<DeleteObjectGuard<HBRUSH>> {
		ptr_to_sysresult(
			unsafe { gdi::ffi::CreatePatternBrush(hbmp.0) },
			|ptr| DeleteObjectGuard::new(HBRUSH(ptr)),
		)
	}

	/// [`CreateSolidBrush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createsolidbrush)
	/// static method.
	#[must_use]
	fn CreateSolidBrush(color: COLORREF) -> SysResult<DeleteObjectGuard<HBRUSH>> {
		ptr_to_sysresult(
			unsafe { gdi::ffi::CreateSolidBrush(color.0) },
			|ptr| DeleteObjectGuard::new(HBRUSH(ptr)),
		)
	}

	/// [`GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// method.
	fn GetObject(&self, pv: &mut LOGBRUSH) -> SysResult<()> {
		match unsafe {
			gdi::ffi::GetObjectW(
				self.as_ptr(),
				std::mem::size_of::<LOGBRUSH>() as _,
				pv as *mut _ as _,
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	}

	/// [`GetStockObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// static method.
	#[must_use]
	fn GetStockObject(sb: co::STOCK_BRUSH) -> SysResult<HBRUSH> {
		ptr_to_sysresult(
			unsafe { gdi::ffi::GetStockObject(sb.0) },
			|ptr| HBRUSH(ptr),
		)
	}

	/// [`GetSysColorBrush`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolorbrush)
	/// static method.
	#[must_use]
	fn GetSysColorBrush(index: co::COLOR) -> SysResult<HBRUSH> {
		ptr_to_sysresult(
			unsafe { gdi::ffi::GetSysColorBrush(index.0) },
			|ptr| HBRUSH(ptr),
		)
	}

	/// [`UnrealizeObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-unrealizeobject)
	/// method.
	fn UnrealizeObject(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::UnrealizeObject(self.as_ptr()) })
	}
}
