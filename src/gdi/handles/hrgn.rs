#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::gdi::ffi;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

impl GdiObject for HRGN {}

impl HRGN {
	/// [`CreateRectRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createrectrgn)
	/// function.
	#[must_use]
	pub fn CreateRectRgn(bounds: RECT) -> SysResult<DeleteObjectGuard<HRGN>> {
		unsafe {
			ptr_to_invalidparm_handle(ffi::CreateRectRgn(
				bounds.left,
				bounds.top,
				bounds.right,
				bounds.bottom,
			))
			.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateRectRgnIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createrectrgnindirect)
	/// function.
	#[must_use]
	pub fn CreateRectRgnIndirect(rc: RECT) -> SysResult<DeleteObjectGuard<HRGN>> {
		unsafe {
			ptr_to_invalidparm_handle(ffi::CreateRectRgnIndirect(pcvoid(&rc)))
				.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CreateRoundRectRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createroundrectrgn)
	/// function.
	#[must_use]
	pub fn CreateRoundRectRgn(bounds: RECT, size: SIZE) -> SysResult<DeleteObjectGuard<HRGN>> {
		unsafe {
			ptr_to_invalidparm_handle(ffi::CreateRoundRectRgn(
				bounds.left,
				bounds.top,
				bounds.right,
				bounds.top,
				size.cx,
				size.cy,
			))
			.map(|h| DeleteObjectGuard::new(h))
		}
	}

	/// [`CombineRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-combinergn)
	/// function.
	///
	/// # Examples
	///
	/// Creating a clipping region with a square hole in it:
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hdc: w::HDC; // initialized somewhere
	/// # let hdc = w::HDC::NULL;
	///
	/// let rc_hole = w::RECT { left: 0, top: 0, right: 100, bottom: 100 };
	/// let hrgn_hole = w::HRGN::CreateRectRgnIndirect(rc_hole)?;
	///
	/// let hrgn_clip = w::HRGN::CreateRectRgnIndirect(
	///     w::InflateRect(rc_hole, 10, 10)?,
	/// )?;
	/// hrgn_clip.CombineRgn(&hrgn_clip, &hrgn_hole, co::RGN::DIFF)?;
	///
	/// hdc.SelectClipRgn(&hrgn_clip)?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn CombineRgn(&self, src1: &HRGN, src2: &HRGN, mode: co::RGN) -> SysResult<co::REGION> {
		match unsafe { ffi::CombineRgn(self.ptr(), src1.ptr(), src2.ptr(), mode.raw()) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			ret => Ok(unsafe { co::REGION::from_raw(ret) }),
		}
	}

	/// [`EqualRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-equalrgn)
	/// function.
	#[must_use]
	pub fn EqualRgn(&self, other: &HRGN) -> bool {
		unsafe { ffi::EqualRgn(self.ptr(), other.ptr()) != 0 }
	}

	/// [`OffsetClipRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-offsetcliprgn)
	/// function.
	pub fn OffsetClipRgn(&self, x: i32, y: i32) -> SysResult<co::REGION> {
		match unsafe { ffi::OffsetClipRgn(self.ptr(), x, y) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			ret => Ok(unsafe { co::REGION::from_raw(ret) }),
		}
	}

	/// [`OffsetRgn`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-offsetrgn)
	/// function.
	pub fn OffsetRgn(&self, x: i32, y: i32) -> SysResult<co::REGION> {
		match unsafe { ffi::OffsetRgn(self.ptr(), x, y) } {
			0 => Err(co::ERROR::INVALID_PARAMETER),
			ret => Ok(unsafe { co::REGION::from_raw(ret) }),
		}
	}

	/// [`PtInRegion`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptinregion)
	/// function.
	#[must_use]
	pub fn PtInRegion(&self, x: i32, y: i32) -> bool {
		unsafe { ffi::PtInRegion(self.ptr(), x, y) != 0 }
	}

	/// [`RectInRegion`](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectinregion)
	/// function.
	#[must_use]
	pub fn RectInRegion(&self, rc: RECT) -> bool {
		unsafe { ffi::RectInRegion(self.ptr(), pcvoid(&rc)) != 0 }
	}
}
