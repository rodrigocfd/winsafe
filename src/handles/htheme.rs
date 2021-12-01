#![allow(non_snake_case)]

use crate::aliases::HrResult;
use crate::co;
use crate::ffi::uxtheme;
use crate::handles::{prelude::Handle, HDC, HRGN};
use crate::privs::ok_to_hrresult;
use crate::structs::{COLORREF, RECT};

/// Handle to a
/// [theme](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HTHEME(pub(crate) *mut std::ffi::c_void);

impl_handle!(HTHEME);

impl HTHEME {
	/// [`CloseThemeData`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-closethemedata)
	/// method.
	pub fn CloseThemeData(self) -> HrResult<()> {
		ok_to_hrresult(unsafe { uxtheme::CloseThemeData(self.0) })
	}

	/// [`DrawThemeBackground`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-drawthemebackground)
	/// method.
	pub fn DrawThemeBackground(self,
		hdc: HDC, part_state: co::VS,
		rc: RECT, rc_clip: RECT) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				uxtheme::DrawThemeBackground(
					self.0,
					hdc.0,
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
		hdc: HDC, part_state: co::VS, bounds: RECT) -> HrResult<RECT>
	{
		let mut rc_content = RECT::default();

		ok_to_hrresult(
			unsafe {
				uxtheme::GetThemeBackgroundContentRect(
					self.0,
					hdc.0,
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
		hdc: HDC, part_state: co::VS, rc_content: RECT) -> HrResult<RECT>
	{
		let mut rc_extent = RECT::default();

		ok_to_hrresult(
			unsafe {
				uxtheme::GetThemeBackgroundExtent(
					self.0,
					hdc.0,
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
	/// [`HRGN::DeleteObject`](crate::prelude::HandleGdi::DeleteObject) call.
	pub fn GetThemeBackgroundRegion(self,
		hdc: HDC, part_state: co::VS, rc: RECT) -> HrResult<HRGN>
	{
		let mut hrgn = HRGN::NULL;

		ok_to_hrresult(
			unsafe {
				uxtheme::GetThemeBackgroundRegion(
					self.0,
					hdc.0,
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
		part_state: co::VS, prop: co::TMT) -> HrResult<COLORREF>
	{
		let mut color = COLORREF(0);

		ok_to_hrresult(
			unsafe {
				uxtheme::GetThemeColor(
					self.0,
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
				self.0, part_state.part, part_state.state) != 0
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
				self.0, part_state.part, part_state.state) != 0
		}
	}
}
