#![allow(non_snake_case)]

use crate::ffi::uxtheme;

use crate::co;
use crate::handles::HDC;
use crate::priv_funcs::const_void;
use crate::structs::RECT;

handle_type! {
	/// Handle to a
	/// [theme](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/).
	/// Exposes methods.
	HTHEME
}

impl HTHEME {
	/// [`CloseThemeData`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-closethemedata)
	/// method.
	pub fn CloseThemeData(self) -> Result<(), co::ERROR> {
		match unsafe { uxtheme::CloseThemeData(self.0) } {
			0 => Ok(()),
			err => Err(co::ERROR::from(err)),
		}
	}

	/// [`DrawThemeBackground`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-drawthemebackground)
	/// method.
	pub fn DrawThemeBackground(self,
		hdc: HDC, iPartId: co::VS_PART, iStateId: co::VS_STATE,
		pRect: RECT, pClipRect: RECT) -> Result<(), co::ERROR>
	{
		match unsafe {
			uxtheme::DrawThemeBackground(
				self.0,
				hdc.as_ptr(),
				iPartId.into(),
				iStateId.into(),
				const_void(&pRect),
				const_void(&pClipRect),
			)
		} {
			0 => Ok(()),
			err => Err(co::ERROR::from(err)),
		}
	}

	/// [`GetThemeAppProperties`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemeappproperties)
	/// static method.
	pub fn GetThemeAppProperties() -> co::STAP {
		co::STAP::from(unsafe { uxtheme::GetThemeAppProperties() })
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

	/// [`IsThemeDialogTextureEnabled`](https://docs.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemedialogtextureenabled)
	/// static method.
	pub fn IsThemeDialogTextureEnabled() -> bool {
		unsafe { uxtheme::IsThemeDialogTextureEnabled() != 0 }
	}
}
