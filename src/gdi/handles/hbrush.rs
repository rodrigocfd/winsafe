#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl GdiObject for HBRUSH {}

impl HBRUSH {
	/// Creates a brush with the given system color.
	///
	/// **Note:** This should be used only to initialize the
	/// [`WNDCLASSEX`](crate::WNDCLASSEX)'s `hbrBackground` field. Any other use
	/// will yield an invalid handle.
	#[must_use]
	pub fn from_sys_color(color: co::COLOR) -> HBRUSH {
		unsafe { HBRUSH::from_ptr((color.raw() + 1) as _) }
	}

	/// [`CreateBrushIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createbrushindirect)
	/// function.
	#[must_use]
	pub fn CreateBrushIndirect(lb: &LOGBRUSH) -> SysResult<DeleteObjectGuard<HBRUSH>> {
		unsafe {
			ptr_to_invalidparm_handle(ffi::CreateBrushIndirect(pcvoid(lb)))
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateHatchBrush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createhatchbrush)
	/// function.
	#[must_use]
	pub fn CreateHatchBrush(
		hatch: co::HS,
		color: COLORREF,
	) -> SysResult<DeleteObjectGuard<HBRUSH>> {
		unsafe {
			ptr_to_invalidparm_handle(ffi::CreateHatchBrush(hatch.raw(), color.into()))
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreatePatternBrush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpatternbrush)
	/// function.
	#[must_use]
	pub fn CreatePatternBrush(hbmp: &HBITMAP) -> SysResult<DeleteObjectGuard<HBRUSH>> {
		unsafe {
			ptr_to_invalidparm_handle(ffi::CreatePatternBrush(hbmp.ptr()))
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateSolidBrush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createsolidbrush)
	/// function.
	#[must_use]
	pub fn CreateSolidBrush(color: COLORREF) -> SysResult<DeleteObjectGuard<HBRUSH>> {
		unsafe {
			ptr_to_invalidparm_handle(ffi::CreateSolidBrush(color.into()))
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let hbr: w::HBRUSH; // initialized somewhere
	/// # let hbr = w::HBRUSH::NULL;
	///
	/// let mut brush = w::LOGBRUSH::default();
	/// hbr.GetObject(&mut brush)?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn GetObject(&self, pv: &mut LOGBRUSH) -> SysResult<()> {
		match unsafe {
			ffi::GetObjectW(self.ptr(), std::mem::size_of::<LOGBRUSH>() as _, pvoid(pv))
		} {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			_ => Ok(()),
		}
	}

	/// [`GetStockObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// function.
	#[must_use]
	pub fn GetStockObject(sb: co::STOCK_BRUSH) -> SysResult<HBRUSH> {
		ptr_to_invalidparm_handle(unsafe { ffi::GetStockObject(sb.raw()) })
	}

	/// [`GetSysColorBrush`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolorbrush)
	/// function.
	#[must_use]
	pub fn GetSysColorBrush(index: co::COLOR) -> SysResult<HBRUSH> {
		ptr_to_invalidparm_handle(unsafe { ffi::GetSysColorBrush(index.raw()) })
	}

	/// [`UnrealizeObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-unrealizeobject)
	/// function.
	pub fn UnrealizeObject(&self) -> SysResult<()> {
		bool_to_invalidparm(unsafe { ffi::UnrealizeObject(self.ptr()) })
	}
}
