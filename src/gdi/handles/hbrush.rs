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
			PtrRet(ffi::CreateBrushIndirect(pcvoid(lb)))
				.to_invalidparm_handle()
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
			PtrRet(ffi::CreateHatchBrush(hatch.raw(), color.into()))
				.to_invalidparm_handle()
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreatePatternBrush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createpatternbrush)
	/// function.
	#[must_use]
	pub fn CreatePatternBrush(hbmp: &HBITMAP) -> SysResult<DeleteObjectGuard<HBRUSH>> {
		unsafe {
			PtrRet(ffi::CreatePatternBrush(hbmp.ptr()))
				.to_invalidparm_handle()
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateSolidBrush`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createsolidbrush)
	/// function.
	#[must_use]
	pub fn CreateSolidBrush(color: COLORREF) -> SysResult<DeleteObjectGuard<HBRUSH>> {
		unsafe {
			PtrRet(ffi::CreateSolidBrush(color.into()))
				.to_invalidparm_handle()
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`GetObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getobjectw)
	/// function.
	pub fn GetObject(&self) -> SysResult<LOGBRUSH> {
		let mut lb = LOGBRUSH::default();
		BoolRet(unsafe {
			ffi::GetObjectW(self.ptr(), std::mem::size_of::<BITMAP>() as _, pvoid(&mut lb))
		})
		.to_invalidparm()
		.map(|_| lb)
	}

	/// [`GetStockObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getstockobject)
	/// function.
	#[must_use]
	pub fn GetStockObject(sb: co::STOCK_BRUSH) -> SysResult<HBRUSH> {
		PtrRet(unsafe { ffi::GetStockObject(sb.raw()) }).to_invalidparm_handle()
	}

	/// [`GetSysColorBrush`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolorbrush)
	/// function.
	#[must_use]
	pub fn GetSysColorBrush(index: co::COLOR) -> SysResult<HBRUSH> {
		PtrRet(unsafe { ffi::GetSysColorBrush(index.raw()) }).to_invalidparm_handle()
	}

	/// [`UnrealizeObject`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-unrealizeobject)
	/// function.
	pub fn UnrealizeObject(&self) -> SysResult<()> {
		BoolRet(unsafe { ffi::UnrealizeObject(self.ptr()) }).to_invalidparm()
	}
}
