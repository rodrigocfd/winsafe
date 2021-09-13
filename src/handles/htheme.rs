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
		hdc: HDC, part_state: co::VS,
		rc: RECT, rc_clip: RECT) -> WinResult<()>
	{
		hr_to_winresult(
			unsafe {
				uxtheme::DrawThemeBackground(
					self.ptr,
					hdc.ptr,
					part_state.part,
					part_state.state,
					&rc as *const _ as _,
					&rc_clip as *const _ as _,
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
		hdc: HDC, part_state: co::VS, bounds: RECT) -> WinResult<RECT>
	{
		let mut rc_content = RECT::default();

		hr_to_winresult(
			unsafe {
				uxtheme::GetThemeBackgroundContentRect(
					self.ptr,
					hdc.ptr,
					part_state.part,
					part_state.state,
					&bounds as *const _ as _,
					&mut rc_content as *mut _ as _,
				)
			},
		).map(|_| rc_content)
	}

	/// [`GetThemeBackgroundExtent`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundextent)
	/// method.
	pub fn GetThemeBackgroundExtent(self,
		hdc: HDC, part_state: co::VS, rc_content: RECT) -> WinResult<RECT>
	{
		let mut rc_extent = RECT::default();

		hr_to_winresult(
			unsafe {
				uxtheme::GetThemeBackgroundExtent(
					self.ptr,
					hdc.ptr,
					part_state.part,
					part_state.state,
					&rc_content as *const _ as _,
					&mut rc_extent as *mut _ as _,
				)
			},
		 ).map(|_| rc_extent)
	}

	/// [`GetThemeBackgroundRegion`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundregion)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::HRGN::DeleteObject) call.
	pub fn GetThemeBackgroundRegion(self,
		hdc: HDC, part_state: co::VS, rc: RECT) -> WinResult<HRGN>
	{
		let mut hrgn = HRGN::NULL;

		hr_to_winresult(
			unsafe {
				uxtheme::GetThemeBackgroundRegion(
					self.ptr,
					hdc.ptr,
					part_state.part,
					part_state.state,
					&rc as *const _ as _,
					&mut hrgn as *mut _ as _,
				)
			},
		).map(|_| hrgn)
	}

	/// [`GetThemeColor`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemecolor)
	/// method.
	pub fn GetThemeColor(self,
		part_state: co::VS, prop: co::TMT) -> WinResult<COLORREF>
	{
		let mut color = COLORREF(0);

		hr_to_winresult(
			unsafe {
				uxtheme::GetThemeColor(
					self.ptr,
					part_state.part,
					part_state.state,
					prop.0,
					&mut color as *mut _ as _,
				)
			},
		).map(|_| color)
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
		part_state: co::VS) -> bool
	{
		unsafe {
			uxtheme::IsThemeBackgroundPartiallyTransparent(
				self.ptr, part_state.part, part_state.state) != 0
		}
	}

	/// [`IsThemeDialogTextureEnabled`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemedialogtextureenabled)
	/// static method.
	pub fn IsThemeDialogTextureEnabled() -> bool {
		unsafe { uxtheme::IsThemeDialogTextureEnabled() != 0 }
	}

	/// [`IsThemePartDefined`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemepartdefined)
	/// method.
	pub fn IsThemePartDefined(self, part_state: co::VS) -> bool {
		unsafe {
			uxtheme::IsThemePartDefined(
				self.ptr, part_state.part, part_state.state) != 0
		}
	}
}
