#![allow(non_snake_case)]

use crate::uxtheme;

/// [`IsThemeActive`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemeactive)
/// static method.
#[must_use]
pub fn IsThemeActive() -> bool {
	unsafe { uxtheme::ffi::IsThemeActive() != 0 }
}

/// [`IsAppThemed`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isappthemed)
/// static method.
#[must_use]
pub fn IsAppThemed() -> bool {
	unsafe { uxtheme::ffi::IsAppThemed() != 0 }
}

/// [`IsCompositionActive`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-iscompositionactive)
/// static method.
#[must_use]
pub fn IsCompositionActive() -> bool {
	unsafe { uxtheme::ffi::IsCompositionActive() != 0 }
}

/// [`IsThemeDialogTextureEnabled`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemedialogtextureenabled)
/// static method.
///
/// **Note:** This function doesn't exist in x32.
#[cfg(target_pointer_width = "64")]
#[must_use]
pub fn IsThemeDialogTextureEnabled() -> bool {
	unsafe { uxtheme::ffi::IsThemeDialogTextureEnabled() != 0 }
}
