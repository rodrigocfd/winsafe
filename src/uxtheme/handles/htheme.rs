#![allow(non_snake_case)]

use crate::co;
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::Handle;
use crate::user::decl::{COLORREF, HDC, HRGN, RECT};
use crate::uxtheme;

impl_handle! { HTHEME: "uxtheme";
	/// Handle to a
	/// [theme](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/).
}

impl UxthemeHtheme for HTHEME {}

/// [`HTHEME`](crate::HTHEME) methods from `uxtheme` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "uxtheme")))]
pub trait UxthemeHtheme: Handle {
	/// [`CloseThemeData`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-closethemedata)
	/// method.
	fn CloseThemeData(self) -> HrResult<()> {
		ok_to_hrresult(unsafe { uxtheme::ffi::CloseThemeData(self.as_ptr()) })
	}

	/// [`DrawThemeBackground`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-drawthemebackground)
	/// method.
	fn DrawThemeBackground(self,
		hdc: HDC, part_state: co::VS,
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

	/// [`GetThemeAppProperties`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemeappproperties)
	/// static method.
	fn GetThemeAppProperties() -> co::STAP {
		co::STAP(unsafe { uxtheme::ffi::GetThemeAppProperties() })
	}

	/// [`GetThemeBackgroundContentRect`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundcontentrect)
	/// method.
	fn GetThemeBackgroundContentRect(self,
		hdc: HDC, part_state: co::VS, bounds: RECT) -> HrResult<RECT>
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

	/// [`GetThemeBackgroundExtent`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundextent)
	/// method.
	fn GetThemeBackgroundExtent(self,
		hdc: HDC, part_state: co::VS, rc_content: RECT) -> HrResult<RECT>
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

	/// [`GetThemeBackgroundRegion`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundregion)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HRGN::DeleteObject`](crate::prelude::HandleGdi::DeleteObject) call.
	fn GetThemeBackgroundRegion(self,
		hdc: HDC, part_state: co::VS, rc: RECT) -> HrResult<HRGN>
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

	/// [`GetThemeColor`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemecolor)
	/// method.
	fn GetThemeColor(self,
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

	/// [`IsThemeBackgroundPartiallyTransparent`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemebackgroundpartiallytransparent)
	/// method.
	fn IsThemeBackgroundPartiallyTransparent(self,
		part_state: co::VS) -> bool
	{
		unsafe {
			uxtheme::ffi::IsThemeBackgroundPartiallyTransparent(
				self.as_ptr(), part_state.part, part_state.state) != 0
		}
	}

	/// [`IsThemePartDefined`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemepartdefined)
	/// method.
	fn IsThemePartDefined(self, part_state: co::VS) -> bool {
		unsafe {
			uxtheme::ffi::IsThemePartDefined(
				self.as_ptr(), part_state.part, part_state.state) != 0
		}
	}
}
