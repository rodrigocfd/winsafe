#![allow(non_snake_case)]

use crate::co;
use crate::ffi::gdi32;
use crate::handles::{HBITMAP, HBRUSH, HFONT, HPEN, HRGN};
use crate::priv_funcs::{const_void, mut_void, ptr_as_opt};
use crate::structs::{POINT, SIZE};
use crate::WString;

handle_type! {
	/// Handle to a
	/// [device context](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc).
	/// Exposes methods.
	HDC
}

/// Converts expression to `Result<(), ()>`.
macro_rules! empty_res {
	($what:expr) => {
		match unsafe { $what } {
			0 => Err(()),
			_ => Ok(()),
		}
	};
}

impl HDC {
	/// [`CreateCompatibleDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatibledc)
	/// method.
	///
	/// Must be paired with a [`DeleteDC`](crate::HDC::DeleteDC) call.
	pub fn CreateCompatibleDC(self) -> Result<HDC, ()> {
		match ptr_as_opt(unsafe { gdi32::CreateCompatibleDC(self.0) }) {
			Some(p) => Ok(Self(p)),
			None => Err(()),
		}
	}

	/// [`DeleteDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deletedc)
	/// method.
	pub fn DeleteDC(self) -> Result<(), ()> {
		empty_res!(gdi32::DeleteDC(self.0))
	}

	/// [`GetDeviceCaps`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdevicecaps)
	/// method.
	pub fn GetDeviceCaps(self, index: co::GDC) -> i32 {
		unsafe { gdi32::GetDeviceCaps(self.0, index.into()) }
	}

	/// [`GetTextExtentPoint32`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextextentpoint32w)
	/// method.
	pub fn GetTextExtentPoint32(self, lpString: &str) -> Result<SIZE, ()> {
		let mut sz = SIZE::default();

		match unsafe {
			gdi32::GetTextExtentPoint32W(
				self.0,
				WString::from_str(lpString).as_ptr(),
				lpString.chars().count() as i32,
				mut_void(&mut sz),
			)
		} {
			0 => Err(()),
			_ => Ok(sz),
		}
	}

	/// [`LineTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-lineto)
	/// method.
	pub fn LineTo(self, x: i32, y: i32) -> Result<(), ()> {
		empty_res!(gdi32::LineTo(self.0, x, y))
	}

	/// [`MoveToEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-movetoex)
	/// method.
	pub fn MoveToEx(self,
		x: i32, y: i32, lppt: Option<&mut POINT>) -> Result<(), ()>
	{
		let pt = match lppt {
			None => std::ptr::null_mut(),
			Some(ptRef) => mut_void(ptRef),
		};

		empty_res!(gdi32::MoveToEx(self.0, x, y, pt))
	}

	/// [`Pie`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pie)
	/// method.
	pub fn Pie(self,
		left: i32, top: i32, right: i32, bottom: i32,
		xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> Result<(), ()>
	{
		empty_res!(
			gdi32::Pie(self.0, left, top, right, bottom, xr1, yr1, xr2, yr2)
		)
	}

	/// [`PolyBezier`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezier)
	/// method.
	pub fn PolyBezier(self, apt: &[POINT]) -> Result<(), ()> {
		empty_res!(
			gdi32::PolyBezier(self.0, const_void(&apt[0]), apt.len() as u32)
		)
	}

	/// [`PolyBezierTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezierto)
	/// method.
	pub fn PolyBezierTo(self, apt: &[POINT]) -> Result<(), ()> {
		empty_res!(
			gdi32::PolyBezierTo(self.0, const_void(&apt[0]), apt.len() as u32)
		)
	}

	/// [`Polyline`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polyline)
	/// method.
	pub fn Polyline(self, apt: &[POINT]) -> Result<(), ()> {
		empty_res!(
			gdi32::Polyline(self.0, const_void(&apt[0]), apt.len() as u32)
		)
	}

	/// [`PolylineTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polylineto)
	/// method.
	pub fn PolylineTo(self, apt: &[POINT]) -> Result<(), ()> {
		empty_res!(
			gdi32::PolylineTo(self.0, const_void(&apt[0]), apt.len() as u32)
		)
	}

	/// [`PtVisible`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptvisible)
	/// method.
	pub fn PtVisible(self, x: i32, y: i32) -> Result<bool, ()> {
		match unsafe { gdi32::PtVisible(self.0, x, y) } {
			-1 => Err(()),
			0 => Ok(false),
			_ => Ok(true),
		}
	}

	/// [`Rectangle`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectangle)
	/// method.
	pub fn Rectangle(self,
		left: i32, top: i32, right: i32, bottom: i32) -> Result<(), ()>
	{
		empty_res!(gdi32::Rectangle(self.0, left, top, right, bottom))
	}

	/// [`RestoreDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-restoredc)
	/// method.
	pub fn RestoreDC(self, nSavedDC: i32) -> Result<(), ()> {
		empty_res!(gdi32::RestoreDC(self.0, nSavedDC))
	}

	/// [`RoundRect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-roundrect)
	/// method.
	pub fn RoundRect(self,
		left: i32, top: i32, right: i32, bottom: i32,
		width: i32, height: i32) -> Result<(), ()>
	{
		empty_res!(
			gdi32::RoundRect(self.0, left, top, right, bottom, width, height)
		)
	}

	/// [`SaveDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-savedc)
	/// method.
	pub fn SaveDC(self) -> Result<i32, ()> {
		match unsafe { gdi32::SaveDC(self.0) } {
			0 => Err(()),
			id => Ok(id),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBITMAP`](crate::HBITMAP).
	pub fn SelectObjectBitmap(self, h: HBITMAP) -> Result<HBITMAP, ()> {
		match ptr_as_opt(unsafe { gdi32::SelectObject(self.0, h.as_ptr()) }) {
			Some(p) => Ok(unsafe { HBITMAP::from_ptr(p) }),
			None => Err(()),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBRUSH`](crate::HBRUSH).
	pub fn SelectObjectBrush(self, h: HBRUSH) -> Result<HBRUSH, ()> {
		match ptr_as_opt(unsafe { gdi32::SelectObject(self.0, h.as_ptr()) }) {
			Some(p) => Ok(unsafe { HBRUSH::from_ptr(p) }),
			None => Err(()),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HFONT`](crate::HFONT).
	pub fn SelectObjectFont(self, h: HFONT) -> Result<HFONT, ()> {
		match ptr_as_opt(unsafe { gdi32::SelectObject(self.0, h.as_ptr()) }) {
			Some(p) => Ok(unsafe { HFONT::from_ptr(p) }),
			None => Err(()),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HPEN`](crate::HPEN).
	pub fn SelectObjectPen(self, h: HPEN) -> Result<HPEN, ()> {
		match ptr_as_opt(unsafe { gdi32::SelectObject(self.0, h.as_ptr()) }) {
			Some(p) => Ok(unsafe { HPEN::from_ptr(p) }),
			None => Err(()),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HRGN`](crate::HRGN).
	pub fn SelectObjectRgn(self, h: HRGN) -> Result<co::REGION, ()> {
		match ptr_as_opt(unsafe { gdi32::SelectObject(self.0, h.as_ptr()) }) {
			Some(p) => Ok(co::REGION::from(p as i32)),
			None => Err(()),
		}
	}

	/// [`SetBkMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	/// method
	pub fn SetBkMode(self, mode: co::BKMODE) -> Result<co::BKMODE, ()> {
		match unsafe { gdi32::SetBkMode(self.0, mode.into()) } {
			0 => Err(()),
			bk => Ok(co::BKMODE::from(bk)),
		}
	}
}
