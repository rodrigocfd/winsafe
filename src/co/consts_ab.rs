#![allow(non_camel_case_types)]

pub_struct_const! { ACCELF, u8,
	/// [`ACCELL`](crate::ACCEL) `fVirt` (`u8`). Originally has `F` prefix.
	=>
	/// The `key` member specifies a virtual-key code. If this flag is not
	/// specified, key is assumed to specify a character code.
	VIRTKEY, 1
	/// The SHIFT key must be held down when the accelerator key is pressed.
	SHIFT, 0x04
	/// The CTRL key must be held down when the accelerator key is pressed.
	CONTROL, 0x08
	/// The ALT key must be held down when the accelerator key is pressed.
	ALT, 0x10
}

pub_struct_const! { ACCESS_RIGHTS, u32,
	/// Standard access rights
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)
	/// (`u32`). Originally has no prefix.
	=>
	DELETE, 0x0001_0000
	READ_CONTROL, 0x0002_0000
	WRITE_DAC, 0x0004_0000
	WRITE_OWNER, 0x0008_0000
	SYNCHRONIZE, 0x0010_0000
}

pub_struct_const! { AD, i32,
	/// [`HDC::SetArcDirection`](crate::HDC::SetArcDirection) `dir` (`i32`).
	=>
	COUNTERCLOCKWISE, 1
	CLOCKWISE, 2
}

pub_struct_const! { ADRF, u32,
	/// [`NMTVASYNCDRAW`](crate::NMTVASYNCDRAW) `dwRetFlags` (`u32`). Don't seem
	/// to be defined anywhere, unconfirmed values.
	=>
	DRAWSYNC, 0
	DRAWNOTHING, 1
	DRAWFALLBACK, 2
	DRAWIMAGE, 3
}

pub_struct_const! { APPCOMMAND, u16,
	/// [`wm::AppCommand`](crate::msg::wm::AppCommand) commands (`u16`).
	=>
	BROWSER_BACKWARD, 1
	BROWSER_FORWARD, 2
	BROWSER_REFRESH, 3
	BROWSER_STOP, 4
	BROWSER_SEARCH, 5
	BROWSER_FAVORITES, 6
	BROWSER_HOME, 7
	VOLUME_MUTE, 8
	VOLUME_DOWN, 9
	VOLUME_UP, 10
	MEDIA_NEXTTRACK, 11
	MEDIA_PREVIOUSTRACK, 12
	MEDIA_STOP, 13
	MEDIA_PLAY_PAUSE, 14
	LAUNCH_MAIL, 15
	LAUNCH_MEDIA_SELECT, 16
	LAUNCH_APP1, 17
	LAUNCH_APP2, 18
	BASS_DOWN, 19
	BASS_BOOST, 20
	BASS_UP, 21
	TREBLE_DOWN, 22
	TREBLE_UP, 23
	MICROPHONE_VOLUME_MUTE, 24
	MICROPHONE_VOLUME_DOWN, 25
	MICROPHONE_VOLUME_UP, 26
	HELP, 27
	FIND, 28
	NEW, 29
	OPEN, 30
	CLOSE, 31
	SAVE, 32
	PRINT, 33
	UNDO, 34
	REDO, 35
	COPY, 36
	CUT, 37
	PASTE, 38
	REPLY_TO_MAIL, 39
	FORWARD_MAIL, 40
	SEND_MAIL, 41
	SPELL_CHECK, 42
	DICTATE_OR_COMMAND_CONTROL_TOGGLE, 43
	MIC_ON_OFF_TOGGLE, 44
	CORRECTION_LIST, 45
	MEDIA_PLAY, 46
	MEDIA_PAUSE, 47
	MEDIA_RECORD, 48
	MEDIA_FAST_FORWARD, 49
	MEDIA_REWIND, 50
	MEDIA_CHANNEL_UP, 51
	MEDIA_CHANNEL_DOWN, 52
	DELETE, 53
	DWM_FLIP3D, 54
}

pub_struct_const_nm! { BCN,
	/// Button control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -1250
	=>
	HOTITEMCHANGE, Self::FIRST.0 + 0x0001
	DROPDOWN, Self::FIRST.0 + 0x0002
}

pub_struct_const! { BCSIF, u32,
	/// [`BUTTON_SPLITINFO`](crate::BUTTON_SPLITINFO) `mask` (`u32`).
	=>
	GLYPH, 0x0001
	IMAGE, 0x0002
	STYLE, 0x0004
	SIZE, 0x0008
}

pub_struct_const! { BCSS, u32,
	/// [`BUTTON_SPLITINFO`](crate::BUTTON_SPLITINFO) `uSplitStyle` (`u32`).
	=>
	NOSPLIT, 0x0001
	STRETCH, 0x0002
	ALIGNLEFT, 0x0004
	IMAGE, 0x0008
}

pub_struct_const! { BI, u32,
	/// [`BITMAPINFOHEADER`](crate::BITMAPINFOHEADER) `biCompression` (`u32`).
	=>
	RGB, 0
	RLE8, 1
	RLE4, 2
	BITFIELDS, 3
	JPEG, 4
	PNG, 5
}

pub_struct_const! { BIA, u32,
	/// [`BUTTON_IMAGELIST`](crate::BUTTON_IMAGELIST) `uAlign` (`u32`).
	/// Originally has `BUTTON_IMAGELIST_ALIGN_` prefix.
	=>
	LEFT, 0
	RIGHT, 1
	TOP, 2
	BOTTOM, 3
	CENTER, 4
}

pub_struct_const! { BKMODE, i32,
	/// [`HDC::SetBkMode`](crate::HDC::SetBkMode) `mode` (`i32`).
	=>
	TRANSPARENT, 1
	OPAQUE, 2
}

pub_struct_const_wm! { BM,
	/// Button control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	///
	/// Also includes constants originally with `BCM` prefix.
	=>
	FIRST, 0x1600
	=>
	/// Originally has `BCM` prefix.
	GETIDEALSIZE, Self::FIRST.0 + 0x0001
	/// Originally has `BCM` prefix.
	SETIMAGELIST, Self::FIRST.0 + 0x0002
	/// Originally has `BCM` prefix.
	GETIMAGELIST, Self::FIRST.0 + 0x0003
	/// Originally has `BCM` prefix.
	SETTEXTMARGIN, Self::FIRST.0 + 0x0004
	/// Originally has `BCM` prefix.
	GETTEXTMARGIN, Self::FIRST.0 + 0x0005
	/// Originally has `BCM` prefix.
	SETDROPDOWNSTATE, Self::FIRST.0 + 0x0006
	/// Originally has `BCM` prefix.
	SETSPLITINFO, Self::FIRST.0 + 0x0007
	/// Originally has `BCM` prefix.
	GETSPLITINFO, Self::FIRST.0 + 0x0008
	/// Originally has `BCM` prefix.
	SETNOTE, Self::FIRST.0 + 0x0009
	/// Originally has `BCM` prefix.
	GETNOTE, Self::FIRST.0 + 0x000a
	/// Originally has `BCM` prefix.
	GETNOTELENGTH, Self::FIRST.0 + 0x000b
	/// Originally has `BCM` prefix.
	SETSHIELD, Self::FIRST.0 + 0x000c

	GETCHECK, 0x00f0
	SETCHECK, 0x00f1
	GETSTATE, 0x00f2
	SETSTATE, 0x00f3
	SETSTYLE, 0x00f4
	CLICK, 0x00f5
	GETIMAGE, 0x00f6
	SETIMAGE, 0x00f7
	SETDONTCLICK, 0x00f8
}

pub_struct_const_cmd! { BN,
	/// Button control `WM_COMMAND`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications)
	/// (`u16`), convertible to [`CMD`](crate::co::CMD).
	=>
	CLICKED, 0
	PAINT, 1
	HILITE, 2
	UNHILITE, 3
	DISABLE, 4
	DOUBLECLICKED, 5
	PUSHED, Self::HILITE.0
	UNPUSHED, Self::UNHILITE.0
	DBLCLK, Self::DOUBLECLICKED.0
	SETFOCUS, 6
	KILLFOCUS, 7
}

pub_struct_const_ws! { BS, u32,
	/// Button control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/button-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	PUSHBUTTON, 0x0000_0000
	DEFPUSHBUTTON, 0x0000_0001
	CHECKBOX, 0x0000_0002
	AUTOCHECKBOX, 0x0000_0003
	RADIOBUTTON, 0x0000_0004
	R3STATE, 0x0000_0005
	AUTO3STATE, 0x0000_0006
	GROUPBOX, 0x0000_0007
	USERBUTTON, 0x0000_0008
	AUTORADIOBUTTON, 0x0000_0009
	PUSHBOX, 0x0000_000a
	OWNERDRAW, 0x0000_000b
	TYPEMASK, 0x0000_000f
	LEFTTEXT, 0x0000_0020
	TEXT, 0x0000_0000
	ICON, 0x0000_0040
	BITMAP, 0x0000_0080
	LEFT, 0x0000_0100
	RIGHT, 0x0000_0200
	CENTER, 0x0000_0300
	TOP, 0x0000_0400
	BOTTOM, 0x0000_0800
	VCENTER, 0x0000_0c00
	PUSHLIKE, 0x0000_1000
	MULTILINE, 0x0000_2000
	NOTIFY, 0x0000_4000
	FLAT, 0x0000_8000
	RIGHTBUTTON, Self::LEFTTEXT.0
}

pub_struct_const! { BSS, u32,
	/// [`LOGBRUSH`](crate::LOGBRUSH) `lbStyle` (`u32`). Originally has `BS`
	/// prefix.
	=>
	SOLID, 0
	NULL, 1
	HOLLOW, Self::NULL.0
	HATCHED, 2
	PATTERN, 3
	INDEXED, 4
	DIBPATTERN, 5
	DIBPATTERNPT, 6
	PATTERN8X8, 7
	DIBPATTERN8X8, 8
	MONOPATTERN, 9
}

pub_struct_const! { BST, u32,
	/// [`bm::GetCheck`](crate::msg::bm::GetCheck) return value (`u32`).
	=>
	UNCHECKED, 0x0000
	CHECKED, 0x0001
	INDETERMINATE, 0x0002
	PUSHED, 0x0004
	FOCUS, 0x0008
}

pub_struct_const_ws! { BTNS, u8,
	/// Toolbar control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/toolbar-control-and-button-styles)
	/// (`u8`), convertible to [`WS`](crate::co::WS).
	=>
	BUTTON, 0x00
	SEP, 0x01
	CHECK, 0x02
	GROUP, 0x04
	CHECKGROUP, Self::GROUP.0 | Self::CHECK.0
	DROPDOWN, 0x08
	AUTOSIZE, 0x10
	NOPREFIX, 0x20
	SHOWTEXT, 0x40
	WHOLEDROPDOWN, 0x80
}
