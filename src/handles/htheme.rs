#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::uxtheme;
use crate::handles::{HDC, HRGN};
use crate::structs::{COLORREF, RECT};
use crate::privs::hr_to_winresult;

pub_struct_handle! {
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
					&pRect as *const _ as _,
					&pClipRect as *const _ as _,
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
					&pBoundingRect as *const _ as _,
					&mut pContentRect as *mut _ as _,
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
					&pContentRect as *const _ as _,
					&mut pExtentRect as *mut _ as _,
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
					&pRect as *const _ as _,
					&mut pRegion as *mut _ as _,
				)
			},
		).map(|_| pRegion)
	}

	/// [`GetThemeColor`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemecolor)
	/// method.
	pub fn GetThemeColor(self,
		iPartId: co::VS_PART, iStateId: co::VS_STATE,
		iPropId: co::TMT) -> WinResult<COLORREF>
	{
		let mut pColor = COLORREF(0);

		hr_to_winresult(
			unsafe {
				uxtheme::GetThemeColor(
					self.ptr,
					iPartId.0,
					iStateId.0,
					iPropId.0,
					&mut pColor as *mut _ as _,
				)
			},
		).map(|_| pColor)
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

	/// [`IsThemeBackgroundPartiallyTransparent`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemebackgroundpartiallytransparent)
	/// method.
	pub fn IsThemeBackgroundPartiallyTransparent(self,
		iPartId: co::VS_PART, iStateId: co::VS_STATE) -> bool
	{
		unsafe {
			uxtheme::IsThemeBackgroundPartiallyTransparent(
				self.ptr, iPartId.0, iStateId.0) != 0
		}
	}

	/// [`IsThemeDialogTextureEnabled`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemedialogtextureenabled)
	/// static method.
	pub fn IsThemeDialogTextureEnabled() -> bool {
		unsafe { uxtheme::IsThemeDialogTextureEnabled() != 0 }
	}

	/// [`IsThemePartDefined`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemepartdefined)
	/// method.
	pub fn IsThemePartDefined(self,
		iPartId: co::VS_PART, iStateId: co::VS_STATE) -> bool
	{
		unsafe {
			uxtheme::IsThemePartDefined(self.ptr, iPartId.0, iStateId.0) != 0
		}
	}
}
