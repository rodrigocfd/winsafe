#![allow(non_snake_case)]

use crate::kernel::decl::WString;
use crate::prelude::Handle;
use crate::user::decl::HWND;
use crate::uxtheme;
use crate::uxtheme::decl::HTHEME;

impl UxthemeHwnd for HWND {}

/// [`HWND`](crate::HWND) methods from `uxtheme` feature.
#[cfg_attr(docsrs, doc(cfg(feature = "uxtheme")))]
pub trait UxthemeHwnd: Handle {
	/// [`OpenThemeData`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-openthemedata)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HTHEME::CloseThemeData`](crate::prelude::UxthemeHtheme::CloseThemeData)
	/// call.
	#[must_use]
	fn OpenThemeData(self, class_list: &str) -> Option<HTHEME> {
		unsafe {
			uxtheme::ffi::OpenThemeData(
				self.as_ptr(),
				WString::from_str(class_list).as_ptr(),
			).as_mut()
		}.map(|ptr| HTHEME(ptr))
	}
}
