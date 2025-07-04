use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;

/// Variant parameter for:
///
/// * [`HWND::DwmGetWindowAttribute`](crate::HWND::DwmGetWindowAttribute)
/// * [`HWND::DwmSetWindowAttribute`](crate::HWND::DwmSetWindowAttribute)
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DwmAttr {
	NcRenderingEnabled(bool),
	NcRenderingPolicy(co::DWMNCRENDERINGPOLICY),
	TransitionsForceDisabled(bool),
	AllowNcPaint(bool),
	CaptionButtonBounds(RECT),
	NonClientRtlLayout(bool),
	ForceIconicRepresentation(bool),
	Flip3dPolicy(co::DWMFLIP3DWINDOWPOLICY),
	ExtendedFrameBounds(RECT),
	HasIconicBitmap(bool),
	DisallowPeek(bool),
	ExcludedFromPeek(bool),
	Cloak(bool),
	Cloaked(co::DWM_CLOAKED),
	FreezeRepresentation(bool),
	PassiveUpdateMode(bool),
	/// Since Windows 11 Build 22000.
	UseHostBackdropBrush(bool),
	/// Since Windows 11 Build 22000.
	UseImmersiveDarkMode(bool),
	/// Since Windows 11 Build 22000.
	WindowCornerPreference(co::DWMWCP),
	/// Since Windows 11 Build 22000.
	BorderColor(COLORREF),
	/// Since Windows 11 Build 22000.
	CaptionColor(COLORREF),
	/// Since Windows 11 Build 22000.
	TextColor(COLORREF),
	/// Since Windows 11 Build 22000.
	VisibleFrameBorderThickness(u32),
	/// Since Windows 11 Build 22621.
	SystemBackdropType(co::DWMSBT),
}

impl DwmAttr {
	/// Returns the correspondent [`co::DWMWA`](crate::co::DWMWA) flag.
	#[must_use]
	pub const fn flag(&self) -> co::DWMWA {
		match self {
			Self::NcRenderingEnabled(_) => co::DWMWA::NCRENDERING_ENABLED,
			Self::NcRenderingPolicy(_) => co::DWMWA::NCRENDERING_POLICY,
			Self::TransitionsForceDisabled(_) => co::DWMWA::TRANSITIONS_FORCEDISABLED,
			Self::AllowNcPaint(_) => co::DWMWA::ALLOW_NCPAINT,
			Self::CaptionButtonBounds(_) => co::DWMWA::CAPTION_BUTTON_BOUNDS,
			Self::NonClientRtlLayout(_) => co::DWMWA::NONCLIENT_RTL_LAYOUT,
			Self::ForceIconicRepresentation(_) => co::DWMWA::FORCE_ICONIC_REPRESENTATION,
			Self::Flip3dPolicy(_) => co::DWMWA::FLIP3D_POLICY,
			Self::ExtendedFrameBounds(_) => co::DWMWA::EXTENDED_FRAME_BOUNDS,
			Self::HasIconicBitmap(_) => co::DWMWA::HAS_ICONIC_BITMAP,
			Self::DisallowPeek(_) => co::DWMWA::DISALLOW_PEEK,
			Self::ExcludedFromPeek(_) => co::DWMWA::EXCLUDED_FROM_PEEK,
			Self::Cloak(_) => co::DWMWA::CLOAK,
			Self::Cloaked(_) => co::DWMWA::CLOAKED,
			Self::FreezeRepresentation(_) => co::DWMWA::FREEZE_REPRESENTATION,
			Self::PassiveUpdateMode(_) => co::DWMWA::PASSIVE_UPDATE_MODE,
			Self::UseHostBackdropBrush(_) => co::DWMWA::USE_HOSTBACKDROPBRUSH,
			Self::UseImmersiveDarkMode(_) => co::DWMWA::USE_IMMERSIVE_DARK_MODE,
			Self::WindowCornerPreference(_) => co::DWMWA::WINDOW_CORNER_PREFERENCE,
			Self::BorderColor(_) => co::DWMWA::BORDER_COLOR,
			Self::CaptionColor(_) => co::DWMWA::CAPTION_COLOR,
			Self::TextColor(_) => co::DWMWA::TEXT_COLOR,
			Self::VisibleFrameBorderThickness(_) => co::DWMWA::VISIBLE_FRAME_BORDER_THICKNESS,
			Self::SystemBackdropType(_) => co::DWMWA::SYSTEMBACKDROP_TYPE,
		}
	}

	/// Returns a pointer to the inner object.
	#[must_use]
	pub(in crate::dwm) const fn ptr(
		&self,
		buf_u32: &mut u32,
		buf_rc: &mut RECT,
	) -> *const std::ffi::c_void {
		match self {
			Self::NcRenderingEnabled(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::NcRenderingPolicy(f) => Self::ptr_of_u32(f.raw(), buf_u32),
			Self::TransitionsForceDisabled(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::AllowNcPaint(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::CaptionButtonBounds(rc) => Self::ptr_of_rc(*rc, buf_rc),
			Self::NonClientRtlLayout(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::ForceIconicRepresentation(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::Flip3dPolicy(f) => Self::ptr_of_u32(f.raw(), buf_u32),
			Self::ExtendedFrameBounds(rc) => Self::ptr_of_rc(*rc, buf_rc),
			Self::HasIconicBitmap(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::DisallowPeek(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::ExcludedFromPeek(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::Cloak(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::Cloaked(f) => Self::ptr_of_u32(f.raw(), buf_u32),
			Self::FreezeRepresentation(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::PassiveUpdateMode(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::UseHostBackdropBrush(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::UseImmersiveDarkMode(b) => Self::ptr_of_bool(*b, buf_u32),
			Self::WindowCornerPreference(f) => Self::ptr_of_u32(f.raw(), buf_u32),
			Self::BorderColor(c) => Self::ptr_of_u32(c.raw(), buf_u32),
			Self::CaptionColor(c) => Self::ptr_of_u32(c.raw(), buf_u32),
			Self::TextColor(c) => Self::ptr_of_u32(c.raw(), buf_u32),
			Self::VisibleFrameBorderThickness(n) => Self::ptr_of_u32(*n, buf_u32),
			Self::SystemBackdropType(f) => Self::ptr_of_u32(f.raw(), buf_u32),
		}
	}

	#[must_use]
	const fn ptr_of_bool(val: bool, buf_u32: &mut u32) -> *const std::ffi::c_void {
		Self::ptr_of_u32(if val { 1 } else { 0 }, buf_u32)
	}

	#[must_use]
	const fn ptr_of_u32(val: u32, buf_u32: &mut u32) -> *const std::ffi::c_void {
		*buf_u32 = val;
		pcvoid(buf_u32)
	}

	#[must_use]
	const fn ptr_of_rc(val: RECT, buf_rc: &mut RECT) -> *const std::ffi::c_void {
		*buf_rc = val;
		pcvoid(buf_rc)
	}

	/// Returns the size of the inner object.
	#[must_use]
	pub(in crate::dwm) const fn sz(&self) -> u32 {
		use std::mem::size_of;
		match self {
			Self::NcRenderingEnabled(_) => size_of::<bool>() as _,
			Self::NcRenderingPolicy(_) => size_of::<co::DWMNCRENDERINGPOLICY>() as _,
			Self::TransitionsForceDisabled(_) => size_of::<bool>() as _,
			Self::AllowNcPaint(_) => size_of::<bool>() as _,
			Self::CaptionButtonBounds(_) => size_of::<RECT>() as _,
			Self::NonClientRtlLayout(_) => size_of::<bool>() as _,
			Self::ForceIconicRepresentation(_) => size_of::<bool>() as _,
			Self::Flip3dPolicy(_) => size_of::<co::DWMFLIP3DWINDOWPOLICY>() as _,
			Self::ExtendedFrameBounds(_) => size_of::<RECT>() as _,
			Self::HasIconicBitmap(_) => size_of::<bool>() as _,
			Self::DisallowPeek(_) => size_of::<bool>() as _,
			Self::ExcludedFromPeek(_) => size_of::<bool>() as _,
			Self::Cloak(_) => size_of::<bool>() as _,
			Self::Cloaked(_) => size_of::<co::DWM_CLOAKED>() as _,
			Self::FreezeRepresentation(_) => size_of::<bool>() as _,
			Self::PassiveUpdateMode(_) => size_of::<bool>() as _,
			Self::UseHostBackdropBrush(_) => size_of::<bool>() as _,
			Self::UseImmersiveDarkMode(_) => size_of::<bool>() as _,
			Self::WindowCornerPreference(_) => size_of::<co::DWMWCP>() as _,
			Self::BorderColor(_) => size_of::<COLORREF>() as _,
			Self::CaptionColor(_) => size_of::<COLORREF>() as _,
			Self::TextColor(_) => size_of::<COLORREF>() as _,
			Self::VisibleFrameBorderThickness(_) => size_of::<u32>() as _,
			Self::SystemBackdropType(_) => size_of::<co::DWMSBT>() as _,
		}
	}

	/// Constructs the object from a raw value.
	#[must_use]
	pub(in crate::dwm) const fn from_raw(flag: co::DWMWA, buf_u32: u32, buf_rc: RECT) -> Self {
		use co::*;
		match flag {
			DWMWA::NCRENDERING_ENABLED => Self::NcRenderingEnabled(buf_u32 != 0),
			DWMWA::NCRENDERING_POLICY => {
				Self::NcRenderingPolicy(unsafe { co::DWMNCRENDERINGPOLICY::from_raw(buf_u32) })
			},
			DWMWA::TRANSITIONS_FORCEDISABLED => Self::TransitionsForceDisabled(buf_u32 != 0),
			DWMWA::ALLOW_NCPAINT => Self::AllowNcPaint(buf_u32 != 0),
			DWMWA::CAPTION_BUTTON_BOUNDS => Self::CaptionButtonBounds(buf_rc),
			DWMWA::NONCLIENT_RTL_LAYOUT => Self::NonClientRtlLayout(buf_u32 != 0),
			DWMWA::FORCE_ICONIC_REPRESENTATION => Self::ForceIconicRepresentation(buf_u32 != 0),
			DWMWA::FLIP3D_POLICY => {
				Self::Flip3dPolicy(unsafe { co::DWMFLIP3DWINDOWPOLICY::from_raw(buf_u32) })
			},
			DWMWA::EXTENDED_FRAME_BOUNDS => Self::ExtendedFrameBounds(buf_rc),
			DWMWA::HAS_ICONIC_BITMAP => Self::HasIconicBitmap(buf_u32 != 0),
			DWMWA::DISALLOW_PEEK => Self::DisallowPeek(buf_u32 != 0),
			DWMWA::EXCLUDED_FROM_PEEK => Self::ExcludedFromPeek(buf_u32 != 0),
			DWMWA::CLOAK => Self::Cloak(buf_u32 != 0),
			DWMWA::CLOAKED => Self::Cloaked(unsafe { co::DWM_CLOAKED::from_raw(buf_u32) }),
			DWMWA::FREEZE_REPRESENTATION => Self::FreezeRepresentation(buf_u32 != 0),
			DWMWA::PASSIVE_UPDATE_MODE => Self::PassiveUpdateMode(buf_u32 != 0),
			DWMWA::USE_HOSTBACKDROPBRUSH => Self::UseHostBackdropBrush(buf_u32 != 0),
			DWMWA::USE_IMMERSIVE_DARK_MODE => Self::UseImmersiveDarkMode(buf_u32 != 0),
			DWMWA::WINDOW_CORNER_PREFERENCE => {
				Self::WindowCornerPreference(unsafe { co::DWMWCP::from_raw(buf_u32) })
			},
			DWMWA::BORDER_COLOR => Self::BorderColor(unsafe { COLORREF::from_raw(buf_u32) }),
			DWMWA::CAPTION_COLOR => Self::CaptionColor(unsafe { COLORREF::from_raw(buf_u32) }),
			DWMWA::TEXT_COLOR => Self::TextColor(unsafe { COLORREF::from_raw(buf_u32) }),
			DWMWA::VISIBLE_FRAME_BORDER_THICKNESS => Self::VisibleFrameBorderThickness(buf_u32),
			DWMWA::SYSTEMBACKDROP_TYPE => {
				Self::SystemBackdropType(unsafe { co::DWMSBT::from_raw(buf_u32) })
			},
			_ => panic!("Invalid DWMWA."),
		}
	}

	/// Returns the pointer to the data and its size, according to the given
	/// attribute flag.
	#[must_use]
	pub(in crate::dwm) const fn ptr_sz_of_flag(
		flag: co::DWMWA,
		buf_u32: &mut u32,
		buf_rc: &mut RECT,
	) -> (*mut std::ffi::c_void, u32) {
		use {co::*, std::mem::size_of};
		match flag {
			DWMWA::NCRENDERING_ENABLED => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::NCRENDERING_POLICY => (pvoid(buf_u32), size_of::<DWMNCRENDERINGPOLICY>() as _),
			DWMWA::TRANSITIONS_FORCEDISABLED => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::ALLOW_NCPAINT => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::CAPTION_BUTTON_BOUNDS => (pvoid(buf_rc), size_of::<RECT>() as _),
			DWMWA::NONCLIENT_RTL_LAYOUT => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::FORCE_ICONIC_REPRESENTATION => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::FLIP3D_POLICY => (pvoid(buf_u32), size_of::<DWMFLIP3DWINDOWPOLICY>() as _),
			DWMWA::EXTENDED_FRAME_BOUNDS => (pvoid(buf_rc), size_of::<RECT>() as _),
			DWMWA::HAS_ICONIC_BITMAP => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::DISALLOW_PEEK => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::EXCLUDED_FROM_PEEK => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::CLOAK => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::CLOAKED => (pvoid(buf_u32), size_of::<DWM_CLOAKED>() as _),
			DWMWA::FREEZE_REPRESENTATION => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::PASSIVE_UPDATE_MODE => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::USE_HOSTBACKDROPBRUSH => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::USE_IMMERSIVE_DARK_MODE => (pvoid(buf_u32), size_of::<bool>() as _),
			DWMWA::WINDOW_CORNER_PREFERENCE => (pvoid(buf_u32), size_of::<DWMWCP>() as _),
			DWMWA::BORDER_COLOR => (pvoid(buf_u32), size_of::<COLORREF>() as _),
			DWMWA::CAPTION_COLOR => (pvoid(buf_u32), size_of::<COLORREF>() as _),
			DWMWA::TEXT_COLOR => (pvoid(buf_u32), size_of::<COLORREF>() as _),
			DWMWA::VISIBLE_FRAME_BORDER_THICKNESS => (pvoid(buf_u32), size_of::<u32>() as _),
			DWMWA::SYSTEMBACKDROP_TYPE => (pvoid(buf_u32), size_of::<DWMSBT>() as _),
			_ => panic!("Invalid DWMWA."),
		}
	}

	/// If the enum contains a `bool`, returns it; otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// content, its general use is discouraged.
	///
	/// # Panics
	///
	/// Panics if the value is not `bool`.
	#[must_use]
	pub const fn unwrap_bool(&self) -> bool {
		match self {
			Self::NcRenderingEnabled(b) => *b,
			Self::TransitionsForceDisabled(b) => *b,
			Self::AllowNcPaint(b) => *b,
			Self::NonClientRtlLayout(b) => *b,
			Self::ForceIconicRepresentation(b) => *b,
			Self::HasIconicBitmap(b) => *b,
			Self::DisallowPeek(b) => *b,
			Self::ExcludedFromPeek(b) => *b,
			Self::Cloak(b) => *b,
			Self::FreezeRepresentation(b) => *b,
			Self::PassiveUpdateMode(b) => *b,
			Self::UseHostBackdropBrush(b) => *b,
			Self::UseImmersiveDarkMode(b) => *b,
			_ => panic!("DwmAttr is not a bool."),
		}
	}

	/// If the enum contains a [`RECT`](crate::RECT), returns it; otherwise
	/// panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// content, its general use is discouraged.
	///
	/// # Panics
	///
	/// Panics if the value is not `RECT`.
	#[must_use]
	pub const fn unwrap_rect(&self) -> RECT {
		match self {
			Self::CaptionButtonBounds(rc) => *rc,
			Self::ExtendedFrameBounds(rc) => *rc,
			_ => panic!("DwmAttr is not a RECT."),
		}
	}

	/// If the enum contains a [`COLORREF`](crate::COLORREF), returns it;
	/// otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// content, its general use is discouraged.
	///
	/// # Panics
	///
	/// Panics if the value is not `COLORREF`.
	#[must_use]
	pub const fn unwrap_colorref(&self) -> COLORREF {
		match self {
			Self::BorderColor(c) => *c,
			Self::CaptionColor(c) => *c,
			Self::TextColor(c) => *c,
			_ => panic!("DwmAttr is not a COLORREF."),
		}
	}

	/// If the enum contains an `u32`, returns it; otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// content, its general use is discouraged.
	///
	/// # Panics
	///
	/// Panics if the value is not `u32`.
	#[must_use]
	pub const fn unwrap_u32(&self) -> u32 {
		match self {
			Self::VisibleFrameBorderThickness(n) => *n,
			_ => panic!("DwmAttr is not an u32."),
		}
	}

	/// If the enum contains a
	/// [`co::DWMNCRENDERINGPOLICY`](crate::co::DWMNCRENDERINGPOLICY),
	/// ([`DwmAttr::NcRenderingPolicy`](crate::DwmAttr::NcRenderingPolicy)),
	/// returns it; otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// content, its general use is discouraged.
	///
	/// # Panics
	///
	/// Panics if the value is not `co::DWMNCRENDERINGPOLICY`.
	#[must_use]
	pub const fn unwrap_ncrenderingpolicy(&self) -> co::DWMNCRENDERINGPOLICY {
		match self {
			Self::NcRenderingPolicy(f) => *f,
			_ => panic!("DwmAttr is not a co::DWMNCRENDERINGPOLICY."),
		}
	}

	/// If the enum contains a
	/// [`co::DWMFLIP3DWINDOWPOLICY`](crate::co::DWMFLIP3DWINDOWPOLICY)
	/// ([`DwmAttr::Flip3dPolicy`](crate::DwmAttr::Flip3dPolicy)), returns it;
	/// otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// content, its general use is discouraged.
	///
	/// # Panics
	///
	/// Panics if the value is not `co::DWMFLIP3DWINDOWPOLICY`.
	#[must_use]
	pub const fn unwrap_flip3dpolicy(&self) -> co::DWMFLIP3DWINDOWPOLICY {
		match self {
			Self::Flip3dPolicy(f) => *f,
			_ => panic!("DwmAttr is not a co::DWMFLIP3DWINDOWPOLICY."),
		}
	}

	/// If the enum contains a [`co::DWM_CLOAKED`](crate::co::DWM_CLOAKED)
	/// ([`DwmAttr::Cloaked`](crate::DwmAttr::Cloaked)), returns it; otherwise
	/// panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// content, its general use is discouraged.
	///
	/// # Panics
	///
	/// Panics if the value is not `co::DWM_CLOAKED`.
	#[must_use]
	pub const fn unwrap_cloaked(&self) -> co::DWM_CLOAKED {
		match self {
			Self::Cloaked(f) => *f,
			_ => panic!("DwmAttr is not a co::DWM_CLOAKED."),
		}
	}

	/// If the enum contains a [`co::DWMWCP`](crate::co::DWMWCP)
	/// ([`DwmAttr::WindowCornerPreference`](crate::DwmAttr::WindowCornerPreference)),
	/// returns it; otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// content, its general use is discouraged.
	///
	/// # Panics
	///
	/// Panics if the value is not `co::DWMWCP`.
	#[must_use]
	pub const fn unwrap_wcp(&self) -> co::DWMWCP {
		match self {
			Self::WindowCornerPreference(f) => *f,
			_ => panic!("DwmAttr is not a co::DWMWCP."),
		}
	}

	/// If the enum contains a [`co::DWMSBT`](crate::co::DWMSBT)
	/// ([`DwmAttr::SystemBackdropType`](crate::DwmAttr::SystemBackdropType)),,
	/// returns it; otherwise panics.
	///
	/// This is a syntactic sugar method to be used when you are sure of the
	/// content, its general use is discouraged.
	///
	/// # Panics
	///
	/// Panics if the value is not `co::DWMSBT`.
	#[must_use]
	pub const fn unwrap_sbt(&self) -> co::DWMSBT {
		match self {
			Self::SystemBackdropType(f) => *f,
			_ => panic!("DwmAttr is not a co::DWMSBT."),
		}
	}
}
