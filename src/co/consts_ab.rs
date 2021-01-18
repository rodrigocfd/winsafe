use std::u32;

use crate::co::WS;

const_type! { ACCELF, u8,
	/// [`ACCELL`](crate::ACCEL) `fVirt` (`u8`).

	VIRTKEY, 1
	SHIFT, 0x04
	CONTROL, 0x08
	ALT, 0x10
}

const_type! { ACCESS_RIGHTS, u32,
	/// [`RegOpenKeyEx`](crate::HKEY::RegOpenKeyEx) `samDesired` (`u32`).
	/// Originally has no prefix.

	DELETE, 0x00010000
	READ_CONTROL, 0x00020000
	WRITE_DAC, 0x00040000
	WRITE_OWNER, 0x00080000
	SYNCHRONIZE, 0x00100000
}

const_type! { ADRF, u32,
	/// [`NMTVASYNCDRAW`](crate::NMTVASYNCDRAW) `dwRetFlags` (`u32`). Don't seem
	/// to be defined anywhere, unconfirmed values.

	DRAWSYNC, 0
	DRAWNOTHING, 1
	DRAWFALLBACK, 2
	DRAWIMAGE, 3
}

const_type! { APPCOMMAND, u16,
	/// [`WM_APPCOMMAND`](crate::msg::WmAppCommand) commands (`u16`).

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

const_type! { BCSIF, u32,
	/// [`BUTTON_SPLITINFO`](crate::BUTTON_SPLITINFO) `mask` (`u32`).

	GLYPH, 0x0001
	IMAGE, 0x0002
	STYLE, 0x0004
	SIZE, 0x0008
}

const_type! { BCSS, u32,
	/// [`BUTTON_SPLITINFO`](crate::BUTTON_SPLITINFO) `uSplitStyle` (`u32`).

	NOSPLIT, 0x0001
	STRETCH, 0x0002
	ALIGNLEFT, 0x0004
	IMAGE, 0x0008
}

const_type! { BI, u32,
	/// [`BITMAPINFOHEADER`](crate::BITMAPINFOHEADER) `biCompression` (`u32`).

	RGB, 0
	RLE8, 1
	RLE4, 2
	BITFIELDS, 3
	JPEG, 4
	PNG, 5
}

const_type! { BIA, u32,
	/// [`BUTTON_IMAGELIST`](crate::BUTTON_IMAGELIST) `uAlign` (`u32`).
	/// Originally has `BUTTON_IMAGELIST_ALIGN_` prefix.

	LEFT, 0
	RIGHT, 1
	TOP, 2
	BOTTOM, 3
	CENTER, 4
}

const_type! { BKMODE, i32,
	/// [`SetBkMode`](crate::HDC::SetBkMode) `mode` (`i32`).

	TRANSPARENT, 1
	OPAQUE, 2
}

const_type! { BM, u32,
	/// Button control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-messages)
	/// (`u32`).

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

const_type! { BS, u32,
	/// Button control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/button-styles)
	/// (`u32`).

	PUSHBUTTON, 0x00000000
	DEFPUSHBUTTON, 0x00000001
	CHECKBOX, 0x00000002
	AUTOCHECKBOX, 0x00000003
	RADIOBUTTON, 0x00000004
	R3STATE, 0x00000005
	AUTO3STATE, 0x00000006
	GROUPBOX, 0x00000007
	USERBUTTON, 0x00000008
	AUTORADIOBUTTON, 0x00000009
	PUSHBOX, 0x0000000a
	OWNERDRAW, 0x0000000b
	TYPEMASK, 0x0000000f
	LEFTTEXT, 0x00000020
	TEXT, 0x00000000
	ICON, 0x00000040
	BITMAP, 0x00000080
	LEFT, 0x00000100
	RIGHT, 0x00000200
	CENTER, 0x00000300
	TOP, 0x00000400
	BOTTOM, 0x00000800
	VCENTER, 0x00000c00
	PUSHLIKE, 0x00001000
	MULTILINE, 0x00002000
	NOTIFY, 0x00004000
	FLAT, 0x00008000
	RIGHTBUTTON, Self::LEFTTEXT.0
}
impl From<BS> for WS {
	fn from(v: BS) -> Self {
		Self(v.0)
	}
}

const_type! { BST, u32,
	/// [`BM_GETCHECK`](crate::msg::BmGetCheck) return value (`u32`).

	UNCHECKED, 0x0000
	CHECKED, 0x0001
	INDETERMINATE, 0x0002
	PUSHED, 0x0004
	FOCUS, 0x0008
}
