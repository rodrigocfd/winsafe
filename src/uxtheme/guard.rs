use crate::prelude::Handle;
use crate::uxtheme;
use crate::uxtheme::decl::HTHEME;

handle_guard! { HthemeGuard: HTHEME;
	uxtheme::ffi::CloseThemeData;
	/// RAII implementation for [`HTHEME`](crate::HTHEME) which automatically calls
	/// [`CloseThemeData`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-closethemedata)
	/// when the object goes out of scope.
}
