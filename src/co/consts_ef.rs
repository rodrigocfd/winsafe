use crate::co::{CMD, WM, WS};

const_type_wm! { EM,
	/// Edit control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	->
	GETSEL, 0x00b0
	SETSEL, 0x00b1
	GETRECT, 0x00b2
	SETRECT, 0x00b3
	SETRECTNP, 0x00b4
	SCROLL, 0x00b5
	LINESCROLL, 0x00b6
	SCROLLCARET, 0x00b7
	GETMODIFY, 0x00b8
	SETMODIFY, 0x00b9
	GETLINECOUNT, 0x00ba
	LINEINDEX, 0x00bb
	SETHANDLE, 0x00bc
	GETHANDLE, 0x00bd
	GETTHUMB, 0x00be
	LINELENGTH, 0x00c1
	REPLACESEL, 0x00c2
	GETLINE, 0x00c4
	LIMITTEXT, 0x00c5
	CANUNDO, 0x00c6
	UNDO, 0x00c7
	FMTLINES, 0x00c8
	LINEFROMCHAR, 0x00c9
	SETTABSTOPS, 0x00cb
	SETPASSWORDCHAR, 0x00cc
	EMPTYUNDOBUFFER, 0x00cd
	GETFIRSTVISIBLELINE, 0x00ce
	SETREADONLY, 0x00cf
	SETWORDBREAKPROC, 0x00d0
	GETWORDBREAKPROC, 0x00d1
	GETPASSWORDCHAR, 0x00d2
	SETMARGINS, 0x00d3
	GETMARGINS, 0x00d4
	SETLIMITTEXT, Self::LIMITTEXT.0
	GETLIMITTEXT, 0x00d5
	POSFROMCHAR, 0x00d6
	CHARFROMPOS, 0x00d7
	SETIMESTATUS, 0x00d8
	GETIMESTATUS, 0x00d9
	ENABLEFEATURE, 0x00da
}

const_type_cmd! { EN,
	/// Edit control `WM_COMMAND`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-notifications)
	/// (`u16`), convertible to [`CMD`](crate::co::CMD).
	->
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
	->
	LEFT, 0x00000000
	CENTERED, 0x00000001
}

const_type! { ENDSESSION, u32,
	/// [`WM_ENDSESSION`](crate::msg::wm::EndSession) event (`u32`).
	->
	RESTARTORSHUTDOWN, 0
	CLOSEAPP, 0x00000001
	CRITICAL, 0x40000000
	LOGOFF, 0x80000000
}

const_type_ws! { ES,
	/// Edit control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/edit-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	->
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
	/// [`WM_APPCOMMAND`](crate::msg::wm::AppCommand) input event (`u16`).
	->
	MOUSE, 0x8000
	KEY, 0
	OEM, 0x1000
}

const_type! { FF, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfPitchAndFamily` (`u8`), used with
	/// [`PITCH`](crate::co::PITCH).
	->
	DONTCARE, 0 << 4
	ROMAN, 1 << 4
	SWISS, 2 << 4
	MODERN, 3 << 4
	SCRIPT, 4 << 4
	DECORATIVE, 5 << 4
}

const_type! { FORMAT_MESSAGE, u32,
	/// [`FormatMessage`](crate::co::ERROR::FormatMessage) `dwFlags` (`u32`).
	->
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
	->
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
