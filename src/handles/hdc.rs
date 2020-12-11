#![allow(non_snake_case)]

use crate::ffi::{gdi32, HANDLE};
use crate::internal_defs::{const_void, mut_void};
use crate::structs::{POINT, SIZE};
use crate::Utf16;

handle_type! {
	/// Handle to a
	/// [device context](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hdc).
	/// Exposes methods.
	HDC
}

macro_rules! zero_res {
	($what:expr) => {
		match unsafe { $what } {
			0 => Err(()),
			_ => Ok(()),
		}
	};
}

impl HDC {
	/// [`GetTextExtentPoint32`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-gettextextentpoint32w)
	/// method.
	pub fn GetTextExtentPoint32(self, lpString: &str) -> Result<SIZE, ()> {
		let mut sz = SIZE::default();

		match unsafe {
			gdi32::GetTextExtentPoint32W(
				self.0,
				Utf16::from_str(lpString).as_ptr(),
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
		zero_res!(gdi32::LineTo(self.0, x, y))
	}

	/// [`MoveToEx`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-movetoex)
	/// method.
	pub fn MoveToEx(
		self, x: i32, y: i32, lppt: Option<&mut POINT>) -> Result<(), ()>
	{
		let pt = match lppt {
			None => std::ptr::null_mut(),
			Some(ptRef) => mut_void(ptRef),
		};

		zero_res!(gdi32::MoveToEx(self.0, x, y, pt))
	}

	/// [`PolyBezier`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezier)
	/// method.
	pub fn PolyBezier(self, apt: &[POINT]) -> Result<(), ()> {
		zero_res!(
			gdi32::PolyBezier(self.0, const_void(&apt[0]), apt.len() as u32)
		)
	}

	/// [`PolyBezierTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polybezierto)
	/// method.
	pub fn PolyBezierTo(self, apt: &[POINT]) -> Result<(), ()> {
		zero_res!(
			gdi32::PolyBezierTo(self.0, const_void(&apt[0]), apt.len() as u32)
		)
	}

	/// [`Polyline`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polyline)
	/// method.
	pub fn Polyline(self, apt: &[POINT]) -> Result<(), ()> {
		zero_res!(
			gdi32::Polyline(self.0, const_void(&apt[0]), apt.len() as u32)
		)
	}

	/// [`PolylineTo`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-polylineto)
	/// method.
	pub fn PolylineTo(self, apt: &[POINT]) -> Result<(), ()> {
		zero_res!(
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
	pub fn Rectangle(
		self, left: i32, top: i32, right: i32, bottom: i32) -> Result<(), ()>
	{
		zero_res!(gdi32::Rectangle(self.0, left, top, right, bottom))
	}

	/// [`RestoreDC`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-restoredc)
	/// method.
	pub fn RestoreDC(self, nSavedDC: i32) -> Result<(), ()> {
		zero_res!(gdi32::RestoreDC(self.0, nSavedDC))
	}

	/// [`RoundRect`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-roundrect)
	/// method.
	pub fn RoundRect(self, left: i32, top: i32, right: i32, bottom: i32,
		width: i32, height: i32) -> Result<(), ()>
	{
		zero_res!(
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
}