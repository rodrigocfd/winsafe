use crate::co::{CMD, WS};

const_type_cmd! { EN,
	/// Edit control [`WM_COMMAND`]
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-notifications)
	/// (`u16`), convertible to [`CMD`](crate::co::CMD).

	SETFOCUS, 0x0100
	KILLFOCUS, 0x0200
	CHANGE, 0x0300
	UPDATE, 0x0400
	ERRSPACE, 0x0500
	MAXTEXT, 0x0501
	HSCROLL, 0x0601
	VSCROLL, 0x0602
	ALIGN_LTR_EC, 0x0700
	ALIGN_RTL_EC, 0x0701
	BEFORE_PASTE, 0x0800
	AFTER_PASTE, 0x0801
}

const_type! { EMF, u32,
	/// [`NMLVEMPTYMARKUP`](crate::NMLVEMPTYMARKUP) `dwFlags` (`u32`).

	LEFT, 0x00000000
	CENTERED, 0x00000001
}

const_type! { ENDSESSION, u32,
	/// [`WM_ENDSESSION`](crate::msg::WmEndSession) event (`u32`).

	RESTARTORSHUTDOWN, 0
	CLOSEAPP, 0x00000001
	CRITICAL, 0x40000000
	LOGOFF, 0x80000000
}

const_type_ws! { ES,
	/// Edit control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/edit-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).

	LEFT, 0x0000
	CENTER, 0x0001
	RIGHT, 0x0002
	MULTILINE, 0x0004
	UPPERCASE, 0x0008
	LOWERCASE, 0x0010
	PASSWORD, 0x0020
	AUTOVSCROLL, 0x0040
	AUTOHSCROLL, 0x0080
	NOHIDESEL, 0x0100
	OEMCONVERT, 0x0400
	READONLY, 0x0800
	WANTRETURN, 0x1000
	NUMBER, 0x2000
}

const_type! { FAPPCOMMAND, u16,
	/// [`WM_APPCOMMAND`](crate::msg::WmAppCommand) input event (`u16`).

	MOUSE, 0x8000
	KEY, 0
	OEM, 0x1000
}

const_type! { FF, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfPitchAndFamily` (`u8`), used with
	/// [`PITCH`](crate::co::PITCH).

	DONTCARE, 0 << 4
	ROMAN, 1 << 4
	SWISS, 2 << 4
	MODERN, 3 << 4
	SCRIPT, 4 << 4
	DECORATIVE, 5 << 4
}

const_type! { FORMAT_MESSAGE, u32,
	/// [`FormatMessage`](crate::co::ERROR::FormatMessage) `dwFlags` (`u32`).

	ALLOCATE_BUFFER, 0x00000100
	ARGUMENT_ARRAY, 0x00002000
	FROM_HMODULE, 0x00000800
	FROM_STRING, 0x00000400
	FROM_SYSTEM, 0x00001000
	IGNORE_INSERTS, 0x00000200
	MAX_WIDTH_MASK, 0x000000ff
}

const_type! { FW, u32,
	/// [`LOGFONT`](crate::LOGFONT) `lfWeight` (`u32`).

	DONTCARE, 0
	THIN, 100
	EXTRALIGHT, 200
	ULTRALIGHT, Self::EXTRALIGHT.0
	LIGHT, 300
	NORMAL, 400
	REGULAR, 400
	MEDIUM, 500
	SEMIBOLD, 600
	DEMIBOLD, Self::SEMIBOLD.0
	BOLD, 700
	EXTRABOLD, 800
	ULTRABOLD, Self::EXTRABOLD.0
	HEAVY, 900
	BLACK, Self::HEAVY.0
}
