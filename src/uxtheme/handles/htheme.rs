#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::uxtheme::ffi;

handle! { HTHEME;
	/// Handle to a
	/// [theme](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/).
}

impl HTHEME {
	/// [`DrawThemeBackground`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-drawthemebackground)
	/// function.
	pub fn DrawThemeBackground(
		&self,
		hdc: &HDC,
		part_state: co::VS,
		rc: RECT,
		rc_clip: Option<RECT>,
	) -> HrResult<()> {
		HrRet(unsafe {
			ffi::DrawThemeBackground(
				self.ptr(),
				hdc.ptr(),
				part_state.part,
				part_state.state,
				pcvoid(&rc),
				pcvoid_or_null(rc_clip.as_ref()),
			)
		})
		.to_hrresult()
	}

	/// [`GetThemeAppProperties`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemeappproperties)
	/// function.
	#[must_use]
	pub fn GetThemeAppProperties() -> co::STAP {
		unsafe { co::STAP::from_raw(ffi::GetThemeAppProperties()) }
	}

	/// [`GetThemeBackgroundContentRect`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundcontentrect)
	/// function.
	#[must_use]
	pub fn GetThemeBackgroundContentRect(
		&self,
		hdc: &HDC,
		part_state: co::VS,
		bounds: RECT,
	) -> HrResult<RECT> {
		let mut rc_content = RECT::default();
		HrRet(unsafe {
			ffi::GetThemeBackgroundContentRect(
				self.ptr(),
				hdc.ptr(),
				part_state.part,
				part_state.state,
				pcvoid(&bounds),
				pvoid(&mut rc_content),
			)
		})
		.to_hrresult()
		.map(|_| rc_content)
	}

	/// [`GetThemeBackgroundExtent`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundextent)
	/// function.
	#[must_use]
	pub fn GetThemeBackgroundExtent(
		&self,
		hdc: &HDC,
		part_state: co::VS,
		rc_content: RECT,
	) -> HrResult<RECT> {
		let mut rc_extent = RECT::default();

		HrRet(unsafe {
			ffi::GetThemeBackgroundExtent(
				self.ptr(),
				hdc.ptr(),
				part_state.part,
				part_state.state,
				pcvoid(&rc_content),
				pvoid(&mut rc_extent),
			)
		})
		.to_hrresult()
		.map(|_| rc_extent)
	}

	/// [`GetThemeBackgroundRegion`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundregion)
	/// function.
	#[must_use]
	pub fn GetThemeBackgroundRegion(
		&self,
		hdc: &HDC,
		part_state: co::VS,
		rc: RECT,
	) -> HrResult<DeleteObjectGuard<HRGN>> {
		let mut hrgn = HRGN::NULL;
		unsafe {
			HrRet(ffi::GetThemeBackgroundRegion(
				self.ptr(),
				hdc.ptr(),
				part_state.part,
				part_state.state,
				pcvoid(&rc),
				hrgn.as_mut(),
			))
			.to_hrresult()
			.map(|_| DeleteObjectGuard::new(hrgn))
		}
	}

	/// [`GetThemeColor`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemecolor)
	/// function.
	#[must_use]
	pub fn GetThemeColor(&self, part_state: co::VS, prop: co::TMT) -> HrResult<COLORREF> {
		let mut color = COLORREF::default();
		HrRet(unsafe {
			ffi::GetThemeColor(
				self.ptr(),
				part_state.part,
				part_state.state,
				prop.raw(),
				color.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| color)
	}

	/// [`GetThemeMargins`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthememargins)
	/// function.
	#[must_use]
	pub fn GetThemeMargins(
		&self,
		hdc_fonts: Option<&HDC>,
		part_state: co::VS,
		prop: co::TMT,
		draw_dest: Option<&RECT>,
	) -> HrResult<MARGINS> {
		let mut margins = MARGINS::default();
		HrRet(unsafe {
			ffi::GetThemeMargins(
				self.ptr(),
				hdc_fonts.map_or(std::ptr::null_mut(), |h| h.ptr()),
				part_state.part(),
				part_state.state(),
				prop.raw(),
				pcvoid_or_null(draw_dest),
				pvoid(&mut margins),
			)
		})
		.to_hrresult()
		.map(|_| margins)
	}

	/// [`GetThemeMetric`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthememetric)
	/// function.
	#[must_use]
	pub fn GetThemeMetric(
		&self,
		hdc_fonts: Option<&HDC>,
		part_state: co::VS,
		prop: co::TMT,
	) -> HrResult<i32> {
		let mut val = 0i32;
		HrRet(unsafe {
			ffi::GetThemeMetric(
				self.ptr(),
				hdc_fonts.map_or(std::ptr::null_mut(), |h| h.ptr()),
				part_state.part(),
				part_state.state(),
				prop.raw(),
				&mut val,
			)
		})
		.to_hrresult()
		.map(|_| val)
	}

	/// [`GetThemePartSize`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemepartsize)
	/// function.
	#[must_use]
	pub fn GetThemePartSize(
		&self,
		hdc_fonts: Option<&HDC>,
		part_state: co::VS,
		draw_dest: Option<&RECT>,
		esize: co::THEMESIZE,
	) -> HrResult<SIZE> {
		let mut sz = SIZE::default();
		HrRet(unsafe {
			ffi::GetThemePartSize(
				self.ptr(),
				hdc_fonts.map_or(std::ptr::null_mut(), |h| h.ptr()),
				part_state.part(),
				part_state.state(),
				pcvoid_or_null(draw_dest),
				esize.raw(),
				pvoid(&mut sz),
			)
		})
		.to_hrresult()
		.map(|_| sz)
	}

	/// [`GetThemePosition`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemeposition)
	/// function.
	#[must_use]
	pub fn GetThemePosition(&self, part_state: co::VS, prop: co::TMT) -> HrResult<POINT> {
		let mut pt = POINT::default();
		HrRet(unsafe {
			ffi::GetThemePosition(
				self.ptr(),
				part_state.part(),
				part_state.state(),
				prop.raw(),
				pvoid(&mut pt),
			)
		})
		.to_hrresult()
		.map(|_| pt)
	}

	/// [`GetThemePropertyOrigin`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemepropertyorigin)
	/// function.
	#[must_use]
	pub fn GetThemePropertyOrigin(
		&self,
		part_state: co::VS,
		prop: co::TMT,
	) -> HrResult<co::PROPERTYORIGIN> {
		let mut origin = co::PROPERTYORIGIN::default();
		HrRet(unsafe {
			ffi::GetThemePropertyOrigin(
				self.ptr(),
				part_state.part(),
				part_state.state(),
				prop.raw(),
				origin.as_mut(),
			)
		})
		.to_hrresult()
		.map(|_| origin)
	}

	/// [`GetThemeRect`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemerect)
	/// function.
	#[must_use]
	pub fn GetThemeRect(&self, part_state: co::VS, prop: co::TMT) -> HrResult<RECT> {
		let mut rc = RECT::default();
		HrRet(unsafe {
			ffi::GetThemeRect(
				self.ptr(),
				part_state.part(),
				part_state.state(),
				prop.raw(),
				pvoid(&mut rc),
			)
		})
		.to_hrresult()
		.map(|_| rc)
	}

	/// [`IsThemeBackgroundPartiallyTransparent`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemebackgroundpartiallytransparent)
	/// function.
	#[must_use]
	pub fn IsThemeBackgroundPartiallyTransparent(&self, part_state: co::VS) -> bool {
		unsafe {
			ffi::IsThemeBackgroundPartiallyTransparent(
				self.ptr(),
				part_state.part,
				part_state.state,
			) != 0
		}
	}

	/// [`IsThemePartDefined`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemepartdefined)
	/// function.
	#[must_use]
	pub fn IsThemePartDefined(&self, part_state: co::VS) -> bool {
		unsafe { ffi::IsThemePartDefined(self.ptr(), part_state.part, part_state.state) != 0 }
	}
}
