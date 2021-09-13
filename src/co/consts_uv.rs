#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::co::{CCM, WM};

pub_struct_const_wm! { UDM,
	/// Up-down control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-up-down-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	=>
	SETRANGE, WM::USER.0 + 101
	GETRANGE, WM::USER.0 + 102
	SETPOS, WM::USER.0 + 103
	GETPOS, WM::USER.0 + 104
	SETBUDDY, WM::USER.0 + 105
	GETBUDDY, WM::USER.0 + 106
	SETACCEL, WM::USER.0 + 107
	GETACCEL, WM::USER.0 + 108
	SETBASE, WM::USER.0 + 109
	GETBASE, WM::USER.0 + 110
	SETRANGE32, WM::USER.0 + 111
	GETRANGE32, WM::USER.0 + 112
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
	SETPOS32, WM::USER.0 + 113
	GETPOS32, WM::USER.0 + 114
}

pub_struct_const_nm! { UDN,
	/// Up-down control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-up-down-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -721
	=>
	DELTAPOS, Self::FIRST.0 - 1
}

pub_struct_const_ws! { UDS, u32,
	/// Up-down control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/up-down-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	WRAP, 0x0001
	SETBUDDYINT, 0x0002
	ALIGNRIGHT, 0x0004
	ALIGNLEFT, 0x0008
	AUTOBUDDY, 0x0010
	ARROWKEYS, 0x0020
	HORZ, 0x0040
	NOTHOUSANDS, 0x0080
	HOTTRACK, 0x0100
}

pub_struct_const! { VER_COND, u8,
	/// [`VerSetConditionMask`](crate::VerSetConditionMask) `condition` (`u8`).
	=>
	EQUAL, 1
	GREATER, 2
	GREATER_EQUAL, 3
	LESS, 4
	LESS_EQUAL, 5
	AND, 6
	OR, 7
	CONDITION_MASK, 7
}

pub_struct_const! { VER_MASK, u32,
	/// [`VerifyVersionInfo`](crate::VerifyVersionInfo) and
	/// [`VerSetConditionMask`](crate::VerSetConditionMask) `type_mask` (`u32`).
	=>
	MINORVERSION, 0x000_0001
	MAJORVERSION, 0x000_0002
	BUILDNUMBER, 0x000_0004
	PLATFORMID, 0x000_0008
	SERVICEPACKMINOR, 0x000_0010
	SERVICEPACKMAJOR, 0x000_0020
	SUITENAME, 0x000_0040
	PRODUCT_TYPE, 0x000_0080
}

pub_struct_const! { VER_NT, u8,
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `wProductType` (`u8`).
	=>
	WORKSTATION, 0x000_0001
	DOMAIN_CONTROLLER, 0x000_0002
	SERVER, 0x000_0003
}

pub_struct_const! { VER_PLATFORM, u32,
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `dwPlatformId` (`u32`).
	=>
	WIN32s, 0
	WIN32_WINDOWS, 1
	WIN32_NT, 2
}

pub_struct_const! { VER_SUITE, u16,
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `wSuiteMask` (`u16`).
	=>
	SMALLBUSINESS, 0x0001
	ENTERPRISE, 0x0002
	BACKOFFICE, 0x0004
	COMMUNICATIONS, 0x0008
	TERMINAL, 0x0010
	SMALLBUSINESS_RESTRICTED, 0x0020
	EMBEDDEDNT, 0x0040
	DATACENTER, 0x0080
	SINGLEUSERTS, 0x0100
	PERSONAL, 0x0200
	BLADE, 0x0400
	EMBEDDED_RESTRICTED, 0x0800
	SECURITY_APPLIANCE, 0x1000
	STORAGE_SERVER, 0x2000
	COMPUTE_SERVER, 0x4000
	WH_SERVER, 0x8000
	//MULTIUSERTS, 0x00020000 // Win32 bug, truncated to zero as u16
}

pub_struct_const! { VFT, u32,
	/// [`VS_FIXEDFILEINFO`](crate::VS_FIXEDFILEINFO) `dwFileType` (`u32`).
	=>
	UNKNOWN, 0x0000_0000
	APP, 0x0000_0001
	DLL, 0x0000_0002
	DRV, 0x0000_0003
	FONT, 0x0000_0004
	VXD, 0x0000_0005
	STATIC_LIB, 0x0000_0007
}

pub_struct_const! { VFT2, u32,
	/// [`VS_FIXEDFILEINFO`](crate::VS_FIXEDFILEINFO) `dwFileSubType` (`u32`).
	=>
	UNKNOWN, 0x0000_0000
	DRV_PRINTER, 0x0000_0001
	DRV_KEYBOARD, 0x0000_0002
	DRV_LANGUAGE, 0x0000_0003
	DRV_DISPLAY, 0x0000_0004
	DRV_MOUSE, 0x0000_0005
	DRV_NETWORK, 0x0000_0006
	DRV_SYSTEM, 0x0000_0007
	DRV_INSTALLABLE, 0x0000_0008
	DRV_SOUND, 0x0000_0009
	DRV_COMM, 0x0000_000a
	DRV_INPUTMETHOD, 0x0000_000b
	DRV_VERSIONED_PRINTER, 0x0000_000c

	FONT_RASTER, 0x0000_0001
	FONT_VECTOR, 0x0000_0002
	FONT_TRUETYPE, 0x0000_0003
}

pub_struct_const! { VK, u16,
	/// [Virtual key codes](https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
	/// (`u16`).
	=>
	/// None of the actual values (zero).
	NoValue, 0
	/// Left mouse button.
	LBUTTON, 0x01
	/// Right mouse button.
	RBUTTON, 0x02
	/// Control-break processing.
	CANCEL, 0x03
	/// Middle mouse button (three-button mouse).
	MBUTTON, 0x04
	/// X1 mouse button.
	XBUTTON1, 0x05
	/// X2 mouse button.
	XBUTTON2, 0x06
	/// BACKSPACE key.
	BACK, 0x08
	/// TAB key.
	TAB, 0x09
	/// CLEAR key.
	CLEAR, 0x0c
	/// ENTER key.
	RETURN, 0x0d
	/// SHIFT key.
	SHIFT, 0x10
	/// CTRL key.
	CONTROL, 0x11
	/// ALT key.
	MENU, 0x12
	/// PAUSE key.
	PAUSE, 0x13
	/// CAPS LOCK key.
	CAPITAL, 0x14
	/// IME Kana mode.
	KANA, 0x15
	/// IME Hangul mode.
	HANGUL, 0x15
	/// IME On.
	IME_ON, 0x16
	/// IME Junja mode.
	JUNJA, 0x17
	/// IME final mode.
	FINAL, 0x18
	/// IME Hanja mode.
	HANJA, 0x19
	/// IME Kanji mode.
	KANJI, 0x19
	/// ESC key.
	ESCAPE, 0x1b
	/// IME convert.
	CONVERT, 0x1c
	/// IME nonconvert.
	NONCONVERT, 0x1d
	/// IME accept.
	ACCEPT, 0x1e
	/// IME mode change request.
	MODECHANGE, 0x1f
	/// SPACEBAR key.
	SPACE, 0x20
	/// PAGE UP key.
	PRIOR, 0x21
	/// PAGE DOWN key.
	NEXT, 0x22
	/// END key.
	END, 0x23
	/// HOME key.
	HOME, 0x24
	/// LEFT ARROW key.
	LEFT, 0x25
	/// UP ARROW key.
	UP, 0x26
	/// RIGHT ARROW key.
	RIGHT, 0x27
	/// DOWN ARROW key.
	DOWN, 0x28
	/// SELECT key.
	SELECT, 0x29
	/// PRINT key.
	PRINT, 0x2a
	/// EXECUTE key.
	EXECUTE, 0x2b
	/// PRINT SCREEN key.
	SNAPSHOT, 0x2c
	/// INS key.
	INSERT, 0x2d
	/// DEL key.
	DELETE, 0x2e
	/// HELP key.
	HELP, 0x2f

	/// Number 0 key.
	CHAR_0, 0x30
	/// Number 1 key.
	CHAR_1, 0x31
	/// Number 2 key.
	CHAR_2, 0x32
	/// Number 3 key.
	CHAR_3, 0x33
	/// Number 4 key.
	CHAR_4, 0x34
	/// Number 5 key.
	CHAR_5, 0x35
	/// Number 6 key.
	CHAR_6, 0x36
	/// Number 7 key.
	CHAR_7, 0x37
	/// Number 8 key.
	CHAR_8, 0x38
	/// Number 9 key.
	CHAR_9, 0x39
	/// Character A key.
	CHAR_A, 0x41
	/// Character B key.
	CHAR_B, 0x42
	/// Character C key.
	CHAR_C, 0x43
	/// Character D key.
	CHAR_D, 0x44
	/// Character E key.
	CHAR_E, 0x45
	/// Character F key.
	CHAR_F, 0x46
	/// Character G key.
	CHAR_G, 0x47
	/// Character H key.
	CHAR_H, 0x48
	/// Character I key.
	CHAR_I, 0x49
	/// Character J key.
	CHAR_J, 0x4a
	/// Character K key.
	CHAR_K, 0x4b
	/// Character L key.
	CHAR_L, 0x4c
	/// Character M key.
	CHAR_M, 0x4d
	/// Character N key.
	CHAR_N, 0x4e
	/// Character O key.
	CHAR_O, 0x4f
	/// Character P key.
	CHAR_P, 0x50
	/// Character Q key.
	CHAR_Q, 0x51
	/// Character R key.
	CHAR_R, 0x52
	/// Character S key.
	CHAR_S, 0x53
	/// Character T key.
	CHAR_T, 0x54
	/// Character U key.
	CHAR_U, 0x55
	/// Character V key.
	CHAR_V, 0x56
	/// Character W key.
	CHAR_W, 0x57
	/// Character X key.
	CHAR_X, 0x58
	/// Character Y key.
	CHAR_Y, 0x59
	/// Character Z key.
	CHAR_Z, 0x5a

	/// Left Windows key (Natural keyboard).
	LWIN, 0x5b
	/// Right Windows key (Natural keyboard).
	RWIN, 0x5c
	/// Applications key, context menu (Natural keyboard).
	APPS, 0x5d
	/// Computer Sleep key.
	SLEEP, 0x5f
	/// Numeric keypad 0 key.
	NUMPAD0, 0x60
	/// Numeric keypad 1 key.
	NUMPAD1, 0x61
	/// Numeric keypad 2 key.
	NUMPAD2, 0x62
	/// Numeric keypad 3 key.
	NUMPAD3, 0x63
	/// Numeric keypad 4 key.
	NUMPAD4, 0x64
	/// Numeric keypad 5 key.
	NUMPAD5, 0x65
	/// Numeric keypad 6 key.
	NUMPAD6, 0x66
	/// Numeric keypad 7 key.
	NUMPAD7, 0x67
	/// Numeric keypad 8 key.
	NUMPAD8, 0x68
	/// Numeric keypad 9 key.
	NUMPAD9, 0x69
	/// Numeric keypad multiply key.
	MULTIPLY, 0x6a
	/// Numeric keypad add key.
	ADD, 0x6b
	/// Numeric keypad separator key.
	SEPARATOR, 0x6c
	/// Numeric keypad subtract key.
	SUBTRACT, 0x6d
	/// Numeric keypad decimal key.
	DECIMAL, 0x6e
	/// Numeric keypad divide key.
	DIVIDE, 0x6f
	F1, 0x70
	F2, 0x71
	F3, 0x72
	F4, 0x73
	F5, 0x74
	F6, 0x75
	F7, 0x76
	F8, 0x77
	F9, 0x78
	F10, 0x79
	F11, 0x7a
	F12, 0x7b
	F13, 0x7c
	F14, 0x7d
	F15, 0x7e
	F16, 0x7f
	F17, 0x80
	F18, 0x81
	F19, 0x82
	F20, 0x83
	F21, 0x84
	F22, 0x85
	F23, 0x86
	F24, 0x87
	/// NUM LOCK key.
	NUMLOCK, 0x90
	/// SCROLL LOCK key.
	SCROLL, 0x91
	OEM_NEC_EQUAL, 0x92
	OEM_FJ_JISHO, 0x92
	OEM_FJ_MASSHOU, 0x93
	OEM_FJ_TOUROKU, 0x94
	OEM_FJ_LOYA, 0x95
	OEM_FJ_ROYA, 0x96
	/// Left SHIFT key.
	LSHIFT, 0xa0
	/// Right SHIFT key.
	RSHIFT, 0xa1
	/// Left CONTROL key.
	LCONTROL, 0xa2
	/// Right CONTROL key.
	RCONTROL, 0xa3
	/// Left MENU key.
	LMENU, 0xa4
	/// Right MENU key.
	RMENU, 0xa5
	BROWSER_BACK, 0xa6
	BROWSER_FORWARD, 0xa7
	BROWSER_REFRESH, 0xa8
	BROWSER_STOP, 0xa9
	BROWSER_SEARCH, 0xaa
	BROWSER_FAVORITES, 0xab
	BROWSER_HOME, 0xac
	VOLUME_MUTE, 0xad
	VOLUME_DOWN, 0xae
	VOLUME_UP, 0xaf
	MEDIA_NEXT_TRACK, 0xb0
	MEDIA_PREV_TRACK, 0xb1
	MEDIA_STOP, 0xb2
	MEDIA_PLAY_PAUSE, 0xb3
	LAUNCH_MAIL, 0xb4
	LAUNCH_MEDIA_SELECT, 0xb5
	LAUNCH_APP1, 0xb6
	LAUNCH_APP2, 0xb7
	OEM_1, 0xba
	OEM_PLUS, 0xbb
	OEM_COMMA, 0xbc
	OEM_MINUS, 0xbd
	OEM_PERIOD, 0xbe
	OEM_2, 0xbf
	OEM_3, 0xc0
	OEM_4, 0xdb
	OEM_5, 0xdc
	OEM_6, 0xdd
	OEM_7, 0xde
	OEM_8, 0xdf
	OEM_AX, 0xe1
	OEM_102, 0xe2
	ICO_HELP, 0xe3
	ICO_00, 0xe4
	PROCESSKEY, 0xe5
	ICO_CLEAR, 0xe6
	PACKET, 0xe7
	OEM_RESET, 0xe9
	OEM_JUMP, 0xea
	OEM_PA1, 0xeb
	OEM_PA2, 0xec
	OEM_PA3, 0xed
	OEM_WSCTRL, 0xee
	OEM_CUSEL, 0xef
	OEM_ATTN, 0xf0
	OEM_FINISH, 0xf1
	OEM_COPY, 0xf2
	OEM_AUTO, 0xf3
	OEM_ENLW, 0xf4
	OEM_BACKTAB, 0xf5
	ATTN, 0xf6
	CRSEL, 0xf7
	EXSEL, 0xf8
	EREOF, 0xf9
	PLAY, 0xfa
	ZOOM, 0xfb
	NONAME, 0xfc
	PA1, 0xfd
	OEM_CLEAR, 0xfe
}

pub_struct_const! { VK_DIR, u16,
	/// [`LVFINDINFO`](crate::LVFINDINFO) `vkDirection` (`u16`).
	=>
	PRIOR, 0x21
	NEXT, 0x22
	END, 0x23
	HOME, 0x24
	LEFT, 0x25
	UP, 0x26
	RIGHT, 0x27
	DOWN, 0x28
}

pub_struct_const! { VOS, u32,
	/// [`VS_FIXEDFILEINFO`](crate::VS_FIXEDFILEINFO) `dwFileOS` (`u32`).
	=>
	UNKNOWN, 0x0000_0000
	DOS, 0x0001_0000
	OS216, 0x0002_0000
	OS232, 0x0003_0000
	NT, 0x0004_0000
	WINCE, 0x0005_0000

	_BASE, 0x0000_0000
	_WINDOWS16, 0x0000_0001
	_PM16, 0x0000_0002
	_PM32, 0x0000_0003
	_WINDOWS32, 0x0000_0004

	DOS_WINDOWS16, 0x0001_0001
	DOS_WINDOWS32, 0x0001_0004
	OS216_PM16, 0x0002_0002
	OS232_PM32, 0x0003_0003
	NT_WINDOWS32, 0x0004_0004
}

pub_struct_const! { VS_FF, u32,
	/// [`VS_FIXEDFILEINFO`](crate::VS_FIXEDFILEINFO) `dwFileFlags` (`u32`).
	=>
	DEBUG, 0x0000_0001
	PRERELEASE, 0x0000_0002
	PATCHED, 0x0000_0004
	PRIVATEBUILD, 0x0000_0008
	INFOINFERRED, 0x0000_0010
	SPECIALBUILD, 0x0000_0020
}
