#![allow(non_snake_case)]

use crate::co;
use crate::ffi::gdi32;
use crate::funcs_priv::{const_void, mut_void, ptr_as_opt};
use crate::funcs::GetLastError;
use crate::handles::{HBITMAP, HBRUSH, HFONT, HPEN, HRGN};
use crate::structs::{POINT, SIZE};
use crate::WString;

handle_type! {
	/// Handle to a
	/// [device context](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc).
	/// Exposes methods.
	HDC
}

/// Converts expression to `Result<(), co::ERROR>`, zero being an error.
macro_rules! zero_err {
	($what:expr) => {
		match unsafe { $what } {
			0 => Err(GetLastError()),
			_ => Ok(()),
		}
	};
}

impl HDC {
	/// [`CreateCompatibleDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createcompatibledc)
	/// method.
	///
	/// Must be paired with a [`DeleteDC`](crate::HDC::DeleteDC) call.
	pub fn CreateCompatibleDC(self) -> Result<HDC, co::ERROR> {
		match ptr_as_opt(unsafe { gdi32::CreateCompatibleDC(self.ptr) }) {
			Some(ptr) => Ok(Self { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`DeleteDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-deletedc)
	/// method.
	pub fn DeleteDC(self) -> Result<(), co::ERROR> {
		zero_err!(gdi32::DeleteDC(self.ptr))
	}

	/// [`GetDeviceCaps`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-getdevicecaps)
	/// method.
	pub fn GetDeviceCaps(self, index: co::GDC) -> i32 {
		unsafe { gdi32::GetDeviceCaps(self.ptr, index.into()) }
	}

	/// [`GetTextExtentPoint32`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextextentpoint32w)
	/// method.
	pub fn GetTextExtentPoint32(self, lpString: &str) -> Result<SIZE, co::ERROR> {
		let mut sz = SIZE::default();

		match unsafe {
			gdi32::GetTextExtentPoint32W(
				self.ptr,
				WString::from_str(lpString).as_ptr(),
				lpString.chars().count() as i32,
				mut_void(&mut sz),
			)
		} {
			0 => Err(GetLastError()),
			_ => Ok(sz),
		}
	}

	/// [`LineTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-lineto)
	/// method.
	pub fn LineTo(self, x: i32, y: i32) -> Result<(), co::ERROR> {
		zero_err!(gdi32::LineTo(self.ptr, x, y))
	}

	/// [`MoveToEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-movetoex)
	/// method.
	pub fn MoveToEx(self,
		x: i32, y: i32, lppt: Option<&mut POINT>) -> Result<(), co::ERROR>
	{
		let pt = match lppt {
			None => std::ptr::null_mut(),
			Some(ptRef) => mut_void(ptRef),
		};

		zero_err!(gdi32::MoveToEx(self.ptr, x, y, pt))
	}

	/// [`Pie`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-pie)
	/// method.
	pub fn Pie(self,
		left: i32, top: i32, right: i32, bottom: i32,
		xr1: i32, yr1: i32, xr2: i32, yr2: i32) -> Result<(), co::ERROR>
	{
		zero_err!(
			gdi32::Pie(self.ptr, left, top, right, bottom, xr1, yr1, xr2, yr2)
		)
	}

	/// [`PolyBezier`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezier)
	/// method.
	pub fn PolyBezier(self, apt: &[POINT]) -> Result<(), co::ERROR> {
		zero_err!(
			gdi32::PolyBezier(self.ptr, const_void(&apt[0]), apt.len() as u32)
		)
	}

	/// [`PolyBezierTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezierto)
	/// method.
	pub fn PolyBezierTo(self, apt: &[POINT]) -> Result<(), co::ERROR> {
		zero_err!(
			gdi32::PolyBezierTo(self.ptr, const_void(&apt[0]), apt.len() as u32)
		)
	}

	/// [`Polyline`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polyline)
	/// method.
	pub fn Polyline(self, apt: &[POINT]) -> Result<(), co::ERROR> {
		zero_err!(
			gdi32::Polyline(self.ptr, const_void(&apt[0]), apt.len() as u32)
		)
	}

	/// [`PolylineTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polylineto)
	/// method.
	pub fn PolylineTo(self, apt: &[POINT]) -> Result<(), co::ERROR> {
		zero_err!(
			gdi32::PolylineTo(self.ptr, const_void(&apt[0]), apt.len() as u32)
		)
	}

	/// [`PtVisible`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-ptvisible)
	/// method.
	pub fn PtVisible(self, x: i32, y: i32) -> Result<bool, co::ERROR> {
		match unsafe { gdi32::PtVisible(self.ptr, x, y) } {
			-1 => Err(GetLastError()),
			0 => Ok(false),
			_ => Ok(true),
		}
	}

	/// [`Rectangle`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-rectangle)
	/// method.
	pub fn Rectangle(self,
		left: i32, top: i32, right: i32, bottom: i32) -> Result<(), co::ERROR>
	{
		zero_err!(gdi32::Rectangle(self.ptr, left, top, right, bottom))
	}

	/// [`RestoreDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-restoredc)
	/// method.
	pub fn RestoreDC(self, nSavedDC: i32) -> Result<(), co::ERROR> {
		zero_err!(gdi32::RestoreDC(self.ptr, nSavedDC))
	}

	/// [`RoundRect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-roundrect)
	/// method.
	pub fn RoundRect(self,
		left: i32, top: i32, right: i32, bottom: i32,
		width: i32, height: i32) -> Result<(), co::ERROR>
	{
		zero_err!(
			gdi32::RoundRect(self.ptr, left, top, right, bottom, width, height)
		)
	}

	/// [`SaveDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-savedc)
	/// method.
	pub fn SaveDC(self) -> Result<i32, co::ERROR> {
		match unsafe { gdi32::SaveDC(self.ptr) } {
			0 => Err(GetLastError()),
			id => Ok(id),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBITMAP`](crate::HBITMAP).
	pub fn SelectObjectBitmap(self, h: HBITMAP) -> Result<HBITMAP, co::ERROR> {
		match ptr_as_opt(unsafe { gdi32::SelectObject(self.ptr, h.ptr) }) {
			Some(ptr) => Ok(HBITMAP { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HBRUSH`](crate::HBRUSH).
	pub fn SelectObjectBrush(self, h: HBRUSH) -> Result<HBRUSH, co::ERROR> {
		match ptr_as_opt(unsafe { gdi32::SelectObject(self.ptr, h.ptr) }) {
			Some(ptr) => Ok(HBRUSH { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HFONT`](crate::HFONT).
	pub fn SelectObjectFont(self, h: HFONT) -> Result<HFONT, co::ERROR> {
		match ptr_as_opt(unsafe { gdi32::SelectObject(self.ptr, h.ptr) }) {
			Some(ptr) => Ok(HFONT { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HPEN`](crate::HPEN).
	pub fn SelectObjectPen(self, h: HPEN) -> Result<HPEN, co::ERROR> {
		match ptr_as_opt(unsafe { gdi32::SelectObject(self.ptr, h.ptr) }) {
			Some(ptr) => Ok(HPEN { ptr }),
			None => Err(GetLastError()),
		}
	}

	/// [`SelectObject`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-selectobject)
	/// method for [`HRGN`](crate::HRGN).
	pub fn SelectObjectRgn(self, h: HRGN) -> Result<co::REGION, co::ERROR> {
		match ptr_as_opt(unsafe { gdi32::SelectObject(self.ptr, h.ptr) }) {
			Some(p) => Ok(co::REGION::from(p as i32)),
			None => Err(GetLastError()),
		}
	}

	/// [`SetBkMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	/// method
	pub fn SetBkMode(self, mode: co::BKMODE) -> Result<co::BKMODE, co::ERROR> {
		match unsafe { gdi32::SetBkMode(self.ptr, mode.into()) } {
			0 => Err(GetLastError()),
			bk => Ok(co::BKMODE::from(bk)),
		}
	}
}
