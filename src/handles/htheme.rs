#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::uxtheme;
use crate::handles::{HDC, HRGN};
use crate::structs::RECT;

handle_type! {
	/// Handle to a
	/// [theme](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/).
	/// Exposes methods.
	HTHEME
}

impl HTHEME {
	/// [`CloseThemeData`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-closethemedata)
	/// method.
	pub fn CloseThemeData(self) -> WinResult<()> {
		match unsafe { uxtheme::CloseThemeData(self.ptr) } {
			0 => Ok(()),
			err => Err(co::ERROR(err)),
		}
	}

	/// [`DrawThemeBackground`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-drawthemebackground)
	/// method.
	pub fn DrawThemeBackground(self,
		hdc: HDC, iPartId: co::VS_PART, iStateId: co::VS_STATE,
		pRect: RECT, pClipRect: RECT) -> WinResult<()>
	{
		match unsafe {
			uxtheme::DrawThemeBackground(
				self.ptr,
				hdc.ptr,
				iPartId.into(),
				iStateId.into(),
				&pRect as *const _ as *const _,
				&pClipRect as *const _ as *const _,
			)
		} {
			0 => Ok(()),
			err => Err(co::ERROR(err)),
		}
	}

	/// [`GetThemeAppProperties`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemeappproperties)
	/// static method.
	pub fn GetThemeAppProperties() -> co::STAP {
		co::STAP(unsafe { uxtheme::GetThemeAppProperties() })
	}

	/// [`GetThemeBackgroundContentRect`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundcontentrect)
	/// method.
	pub fn GetThemeBackgroundContentRect(self,
		hdc: HDC, iPartId: co::VS_PART, iStateId: co::VS_STATE,
		pBoundingRect: RECT) -> WinResult<RECT>
	{
		let mut pContentRect = RECT::default();

		match unsafe {
			uxtheme::GetThemeBackgroundContentRect(
				self.ptr,
				hdc.ptr,
				iPartId.into(),
				iStateId.into(),
				&pBoundingRect as *const _ as *const _,
				&mut pContentRect as *mut _ as *mut _,
			)
		} {
			0 => Ok(pContentRect),
			err => Err(co::ERROR(err)),
		}
	}

	/// [`GetThemeBackgroundExtent`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundextent)
	/// method.
	pub fn GetThemeBackgroundExtent(self,
		hdc: HDC, iPartId: co::VS_PART, iStateId: co::VS_STATE,
		pContentRect: RECT) -> WinResult<RECT>
	{
		let mut pExtentRect = RECT::default();

		match unsafe {
			uxtheme::GetThemeBackgroundExtent(
				self.ptr,
				hdc.ptr,
				iPartId.into(),
				iStateId.into(),
				&pContentRect as *const _ as *const _,
				&mut pExtentRect as *mut _ as *mut _,
			)
		} {
			0 => Ok(pExtentRect),
			err => Err(co::ERROR(err)),
		}
	}

	/// [`GetThemeBackgroundRegion`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundregion)
	/// method.
	pub fn GetThemeBackgroundRegion(self,
		hdc: HDC, iPartId: co::VS_PART, iStateId: co::VS_STATE,
		pRect: RECT) -> WinResult<HRGN>
	{
		let mut pRegion = unsafe { HRGN::null_handle() };

		match unsafe {
			uxtheme::GetThemeBackgroundRegion(
				self.ptr,
				hdc.ptr,
				iPartId.into(),
				iStateId.into(),
				&pRect as *const _ as *const _,
				&mut pRegion as *mut _ as *mut _,
			)
		} {
			0 => Ok(pRegion),
			err => Err(co::ERROR(err)),
		}
	}

	/// [`IsAppThemed`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isappthemed)
	/// static method.
	pub fn IsAppThemed() -> bool {
		unsafe { uxtheme::IsAppThemed() != 0 }
	}

	/// [`IsCompositionActive`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-iscompositionactive)
	/// static method.
	pub fn IsCompositionActive() -> bool {
		unsafe { uxtheme::IsCompositionActive() != 0 }
	}

	/// [`IsThemeActive`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemeactive)
	/// static method.
	pub fn IsThemeActive() -> bool {
		unsafe { uxtheme::IsThemeActive() != 0 }
	}

	/// [`IsThemeDialogTextureEnabled`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemedialogtextureenabled)
	/// static method.
	pub fn IsThemeDialogTextureEnabled() -> bool {
		unsafe { uxtheme::IsThemeDialogTextureEnabled() != 0 }
	}
}
