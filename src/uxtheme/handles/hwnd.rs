#![allow(non_camel_case_types, non_snake_case)]

use crate::decl::*;
use crate::guard::*;
use crate::ole::privs::*;
use crate::prelude::*;
use crate::uxtheme::ffi;

impl HWND {
	/// [`OpenThemeData`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-openthemedata)
	/// function.
	#[must_use]
	pub fn OpenThemeData(&self, class_list: &str) -> Option<CloseThemeDataGuard> {
		unsafe {
			ffi::OpenThemeData(self.ptr(), WString::from_str(class_list).as_ptr())
				.as_mut()
				.map(|ptr| CloseThemeDataGuard::new(HTHEME::from_ptr(ptr)))
		}
	}

	/// [`SetWindowTheme`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-setwindowtheme)
	/// function.
	pub fn SetWindowTheme(&self, sub_app_name: &str, sub_id_list: Option<&str>) -> HrResult<()> {
		ok_to_hrresult(unsafe {
			ffi::SetWindowTheme(
				self.ptr(),
				WString::from_str(sub_app_name).as_ptr(),
				WString::from_opt_str(sub_id_list).as_ptr(),
			)
		})
	}
}
