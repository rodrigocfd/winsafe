#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, uxtheme};
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::Handle;
use crate::user::decl::{COLORREF, HDC, HRGN, RECT};

impl_handle! { HTHEME;
	/// Handle to a
	/// [theme](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/).
}

impl uxtheme_Htheme for HTHEME {}

/// This trait is enabled with the `uxtheme` feature, and provides methods for
/// [`HTHEME`](crate::HTHEME).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait uxtheme_Htheme: Handle {
	/// [`DrawThemeBackground`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-drawthemebackground)
	/// method.
	fn DrawThemeBackground(&self,
		hdc: &HDC, part_state: co::VS,
		rc: RECT, rc_clip: RECT) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				uxtheme::ffi::DrawThemeBackground(
					self.as_ptr(),
					hdc.0,
					part_state.part,
					part_state.state,
					&rc as *const _ as _,
					&rc_clip as *const _ as _,
				)
			},
		)
	}

	/// [`GetThemeAppProperties`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemeappproperties)
	/// static method.
	#[must_use]
	fn GetThemeAppProperties() -> co::STAP {
		co::STAP(unsafe { uxtheme::ffi::GetThemeAppProperties() })
	}

	/// [`GetThemeBackgroundContentRect`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundcontentrect)
	/// method.
	#[must_use]
	fn GetThemeBackgroundContentRect(&self,
		hdc: &HDC, part_state: co::VS, bounds: RECT) -> HrResult<RECT>
	{
		let mut rc_content = RECT::default();

		ok_to_hrresult(
			unsafe {
				uxtheme::ffi::GetThemeBackgroundContentRect(
					self.as_ptr(),
					hdc.0,
					part_state.part,
					part_state.state,
					&bounds as *const _ as _,
					&mut rc_content as *mut _ as _,
				)
			},
		).map(|_| rc_content)
	}

	/// [`GetThemeBackgroundExtent`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundextent)
	/// method.
	#[must_use]
	fn GetThemeBackgroundExtent(&self,
		hdc: &HDC, part_state: co::VS, rc_content: RECT) -> HrResult<RECT>
	{
		let mut rc_extent = RECT::default();

		ok_to_hrresult(
			unsafe {
				uxtheme::ffi::GetThemeBackgroundExtent(
					self.as_ptr(),
					hdc.0,
					part_state.part,
					part_state.state,
					&rc_content as *const _ as _,
					&mut rc_extent as *mut _ as _,
				)
			},
		 ).map(|_| rc_extent)
	}

	/// [`GetThemeBackgroundRegion`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundregion)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::prelude::gdi_Hgdiobj::DeleteObject) call.
	#[must_use]
	fn GetThemeBackgroundRegion(&self,
		hdc: &HDC, part_state: co::VS, rc: RECT) -> HrResult<HRGN>
	{
		let mut hrgn = HRGN::NULL;

		ok_to_hrresult(
			unsafe {
				uxtheme::ffi::GetThemeBackgroundRegion(
					self.as_ptr(),
					hdc.0,
					part_state.part,
					part_state.state,
					&rc as *const _ as _,
					&mut hrgn as *mut _ as _,
				)
			},
		).map(|_| hrgn)
	}

	/// [`GetThemeColor`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemecolor)
	/// method.
	#[must_use]
	fn GetThemeColor(&self,
		part_state: co::VS, prop: co::TMT) -> HrResult<COLORREF>
	{
		let mut color = COLORREF(0);

		ok_to_hrresult(
			unsafe {
				uxtheme::ffi::GetThemeColor(
					self.as_ptr(),
					part_state.part,
					part_state.state,
					prop.0,
					&mut color as *mut _ as _,
				)
			},
		).map(|_| color)
	}

	/// [`IsThemeBackgroundPartiallyTransparent`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemebackgroundpartiallytransparent)
	/// method.
	#[must_use]
	fn IsThemeBackgroundPartiallyTransparent(&self,
		part_state: co::VS) -> bool
	{
		unsafe {
			uxtheme::ffi::IsThemeBackgroundPartiallyTransparent(
				self.as_ptr(), part_state.part, part_state.state) != 0
		}
	}

	/// [`IsThemePartDefined`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemepartdefined)
	/// method.
	#[must_use]
	fn IsThemePartDefined(&self, part_state: co::VS) -> bool {
		unsafe {
			uxtheme::ffi::IsThemePartDefined(
				self.as_ptr(), part_state.part, part_state.state) != 0
		}
	}
}

//------------------------------------------------------------------------------

handle_guard! { HthemeGuard: HTHEME;
	uxtheme::ffi::CloseThemeData;
	/// RAII implementation for [`HTHEME`](crate::HTHEME) which automatically calls
	/// [`CloseThemeData`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-closethemedata)
	/// when the object goes out of scope.
}
