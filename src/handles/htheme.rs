#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::uxtheme;
use crate::handles::{HDC, HRGN};
use crate::structs::RECT;
use crate::privs::{hr_to_winresult, ref_as_pcvoid, ref_as_pvoid};

handle_type! {
	/// Handle to a
	/// [theme](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/).
	HTHEME
}

impl HTHEME {
	/// [`CloseThemeData`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-closethemedata)
	/// method.
	pub fn CloseThemeData(self) -> WinResult<()> {
		hr_to_winresult(unsafe { uxtheme::CloseThemeData(self.ptr) })
	}

	/// [`DrawThemeBackground`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-drawthemebackground)
	/// method.
	pub fn DrawThemeBackground(self,
		hdc: HDC, iPartId: co::VS_PART, iStateId: co::VS_STATE,
		pRect: RECT, pClipRect: RECT) -> WinResult<()>
	{
		hr_to_winresult(
			unsafe {
				uxtheme::DrawThemeBackground(
					self.ptr,
					hdc.ptr,
					iPartId.0,
					iStateId.0,
					ref_as_pcvoid(&pRect),
					ref_as_pcvoid(&pClipRect),
				)
			},
		)
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

		hr_to_winresult(
			unsafe {
				uxtheme::GetThemeBackgroundContentRect(
					self.ptr,
					hdc.ptr,
					iPartId.0,
					iStateId.0,
					ref_as_pcvoid(&pBoundingRect),
					ref_as_pvoid(&mut pContentRect),
				)
			},
		).map(|_| pContentRect)
	}

	/// [`GetThemeBackgroundExtent`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundextent)
	/// method.
	pub fn GetThemeBackgroundExtent(self,
		hdc: HDC, iPartId: co::VS_PART, iStateId: co::VS_STATE,
		pContentRect: RECT) -> WinResult<RECT>
	{
		let mut pExtentRect = RECT::default();

		hr_to_winresult(
			unsafe {
				uxtheme::GetThemeBackgroundExtent(
					self.ptr,
					hdc.ptr,
					iPartId.0,
					iStateId.0,
					ref_as_pcvoid(&pContentRect),
					ref_as_pvoid(&mut pExtentRect),
				)
			},
		 ).map(|_| pExtentRect)
	}

	/// [`GetThemeBackgroundRegion`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundregion)
	/// method.
	///
	/// **Note:** Must be paired with a
	/// [`DeleteObject`](crate::HRGN::DeleteObject) call.
	pub fn GetThemeBackgroundRegion(self,
		hdc: HDC, iPartId: co::VS_PART, iStateId: co::VS_STATE,
		pRect: RECT) -> WinResult<HRGN>
	{
		let mut pRegion = HRGN::NULL;

		hr_to_winresult(
			unsafe {
				uxtheme::GetThemeBackgroundRegion(
					self.ptr,
					hdc.ptr,
					iPartId.0,
					iStateId.0,
					ref_as_pcvoid(&pRect),
					ref_as_pvoid(&mut pRegion),
				)
			},
		).map(|_| pRegion)
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
