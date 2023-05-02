#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, gdi};
use crate::gdi::decl::LOGBRUSH;
use crate::gdi::guard::DeleteObjectGuard;
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::{bool_to_sysresult, ptr_to_sysresult_handle};
use crate::prelude::{GdiObject, GdiObjectSelect, Handle};
use crate::user::decl::{COLORREF, HBITMAP, HBRUSH};

impl GdiObject for HBRUSH {}
impl GdiObjectSelect for HBRUSH {}
impl gdi_Hbrush for HBRUSH {}

/// This trait is enabled with the `gdi` feature, and provides methods for
/// [`HBRUSH`](crate::HBRUSH).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait gdi_Hbrush: Handle {
	/// Creates a brush with the given system color.
	///
	/// **Note:** This should be used only to initialize the
	/// [`WNDCLASSEX`](crate::WNDCLASSEX)'s `hbrBackground` field. Any other use
	/// will yield an invalid handle.
	#[must_use]
	fn from_sys_color(color: co::COLOR) -> HBRUSH {
		unsafe { HBRUSH::from_ptr((color.0 + 1) as _ ) }
	}

	/// [`CreateBrushIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbrushindirect)
	/// static method.
	#[must_use]
	fn CreateBrushIndirect(
		lb: &LOGBRUSH) -> SysResult<DeleteObjectGuard<HBRUSH>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				gdi::ffi::CreateBrushIndirect(lb as *const _ as _),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateHatchBrush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createhatchbrush)
	/// static method.
	#[must_use]
	fn CreateHatchBrush(
		hatch: co::HS, color: COLORREF) -> SysResult<DeleteObjectGuard<HBRUSH>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				gdi::ffi::CreateHatchBrush(hatch.0, color.0),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreatePatternBrush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpatternbrush)
	/// static method.
	#[must_use]
	fn CreatePatternBrush(
		hbmp: &HBITMAP) -> SysResult<DeleteObjectGuard<HBRUSH>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				gdi::ffi::CreatePatternBrush(hbmp.as_ptr()),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateSolidBrush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createsolidbrush)
	/// static method.
	#[must_use]
	fn CreateSolidBrush(
		color: COLORREF) -> SysResult<DeleteObjectGuard<HBRUSH>>
	{
		unsafe {
			ptr_to_sysresult_handle(
				gdi::ffi::CreateSolidBrush(color.0),
			).map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HBRUSH, LOGBRUSH};
	///
	/// let hbr: HBRUSH; // initialized somewhere
	/// # let hbr = HBRUSH::NULL;
	///
	/// let mut brush = LOGBRUSH::default();
	/// hbr.GetObject(&mut brush)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
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
		ptr_to_sysresult_handle(unsafe { gdi::ffi::GetStockObject(sb.0) })
	}

	/// [`GetSysColorBrush`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolorbrush)
	/// static method.
	#[must_use]
	fn GetSysColorBrush(index: co::COLOR) -> SysResult<HBRUSH> {
		ptr_to_sysresult_handle(unsafe { gdi::ffi::GetSysColorBrush(index.0) })
	}

	/// [`UnrealizeObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-unrealizeobject)
	/// method.
	fn UnrealizeObject(&self) -> SysResult<()> {
		bool_to_sysresult(unsafe { gdi::ffi::UnrealizeObject(self.as_ptr()) })
	}
}
