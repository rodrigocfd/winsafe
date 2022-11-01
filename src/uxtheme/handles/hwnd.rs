#![allow(non_camel_case_types, non_snake_case)]

use crate::kernel::decl::WString;
use crate::prelude::Handle;
use crate::user::decl::HWND;
use crate::uxtheme;
use crate::uxtheme::decl::HTHEME;

impl uxtheme_Hwnd for HWND {}

/// This trait is enabled with the `uxtheme` feature, and provides methods for
/// [`HWND`](crate::HWND).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "uxtheme")))]
pub trait uxtheme_Hwnd: Handle {
	/// [`OpenThemeData`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-openthemedata)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HTHEME::CloseThemeData`](crate::prelude::uxtheme_Htheme::CloseThemeData)
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
