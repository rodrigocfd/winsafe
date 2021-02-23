#![allow(non_upper_case_globals)]

use crate::co::{NM, WM, WS, WS_EX};

const_type_wm! { CB,
	/// Combo box control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).

	SETMINVISIBLE, Self::FIRST.0 + 1
	GETMINVISIBLE, Self::FIRST.0 + 2
	SETCUEBANNER, Self::FIRST.0 + 3
	GETCUEBANNER, Self::FIRST.0 + 4

	GETEDITSEL, 0x0140
	LIMITTEXT, 0x0141
	SETEDITSEL, 0x0142
	ADDSTRING, 0x0143
	DELETESTRING, 0x0144
	DIR, 0x0145
	GETCOUNT, 0x0146
	GETCURSEL, 0x0147
	GETLBTEXT, 0x0148
	GETLBTEXTLEN, 0x0149
	INSERTSTRING, 0x014a
	RESETCONTENT, 0x014b
	FINDSTRING, 0x014c
	SELECTSTRING, 0x014d
	SETCURSEL, 0x014e
	SHOWDROPDOWN, 0x014f
	GETITEMDATA, 0x0150
	SETITEMDATA, 0x0151
	GETDROPPEDCONTROLRECT, 0x0152
	SETITEMHEIGHT, 0x0153
	GETITEMHEIGHT, 0x0154
	SETEXTENDEDUI, 0x0155
	GETEXTENDEDUI, 0x0156
	GETDROPPEDSTATE, 0x0157
	FINDSTRINGEXACT, 0x0158
	SETLOCALE, 0x0159
	GETLOCALE, 0x015a
	GETTOPINDEX, 0x015b
	SETTOPINDEX, 0x015c
	GETHORIZONTALEXTENT, 0x015d
	SETHORIZONTALEXTENT, 0x015e
	GETDROPPEDWIDTH, 0x015f
	SETDROPPEDWIDTH, 0x0160
	INITSTORAGE, 0x0161
	GETCOMBOBOXINFO, 0x0164
}
const_type_priv_values! { CB
	FIRST, 0x1700
}

const_type_cmd! { CBN,
	/// Combo box control `WM_COMMAND`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-notifications)
	/// (`u16`), convertible to [`CMD`](crate::co::CMD).

	ERRSPACE, (0 - 1) as u16
	SELCHANGE, 1
	DBLCLK, 2
	SETFOCUS, 3
	KILLFOCUS, 4
	EDITCHANGE, 5
	EDITUPDATE, 6
	DROPDOWN, 7
	CLOSEUP, 8
	SELENDOK, 9
	SELENDCANCEL, 10
}

const_type_ws! { CBS,
	/// Combo box control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/combo-box-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).

	SIMPLE, 0x0001
	DROPDOWN, 0x0002
	DROPDOWNLIST, 0x0003
	OWNERDRAWFIXED, 0x0010
	OWNERDRAWVARIABLE, 0x0020
	AUTOHSCROLL, 0x0040
	OEMCONVERT, 0x0080
	SORT, 0x0100
	HASSTRINGS, 0x0200
	NOINTEGRALHEIGHT, 0x0400
	DISABLENOSCROLL, 0x0800
	UPPERCASE, 0x2000
	LOWERCASE, 0x4000
}

const_type_wsex! { CBES_EX,
	/// Extended combo box
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/comboboxex-control-extended-styles)
	/// (`u32`), convertible to [`WS_EX`](crate::co::WS_EX).

	NOEDITIMAGE, 0x00000001
	NOEDITIMAGEINDENT, 0x00000002
	PATHWORDBREAKPROC, 0x00000004
	NOSIZELIMIT, 0x00000008
	CASESENSITIVE, 0x00000010
	TEXTENDELLIPSIS, 0x00000020
}

const_type_wm! { CCM,
	/// Generic common controls
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/common-controls-intro)
	/// (`u32`), convertible to [`WM`](crate::co::WM).

	SETBKCOLOR, Self::FIRST.0 + 1
	SETCOLORSCHEME, Self::FIRST.0 + 2
	GETCOLORSCHEME, Self::FIRST.0 + 3
	GETDROPTARGET, Self::FIRST.0 + 4
	SETUNICODEFORMAT, Self::FIRST.0 + 5
	GETUNICODEFORMAT, Self::FIRST.0 + 6
	SETVERSION, Self::FIRST.0 + 0x7
	GETVERSION, Self::FIRST.0 + 0x8
	SETNOTIFYWINDOW, Self::FIRST.0 + 0x9
	SETWINDOWTHEME, Self::FIRST.0 + 0xb
	DPISCALE, Self::FIRST.0 + 0xc
}
const_type_priv_values! { CCM
	FIRST, 0x2000
}

const_type! { CDDS, u32,
	/// [`NMCUSTOMDRAW`](crate::NMCUSTOMDRAW) `dwDrawStage` (`u32`).

	PREPAINT, 0x00000001
	POSTPAINT, 0x00000002
	PREERASE, 0x00000003
	POSTERASE, 0x00000004
	ITEM, 0x00010000
	ITEMPREPAINT, Self::ITEM.0 | Self::PREPAINT.0
	ITEMPOSTPAINT, Self::ITEM.0 | Self::POSTPAINT.0
	ITEMPREERASE, Self::ITEM.0 | Self::PREERASE.0
	ITEMPOSTERASE, Self::ITEM.0 | Self::POSTERASE.0
	SUBITEM, 0x00020000
}

const_type! { CDIS, u32,
	/// [`NMCUSTOMDRAW`](crate::NMCUSTOMDRAW) `uItemState` (`u32`).

	SELECTED, 0x0001
	GRAYED, 0x0002
	DISABLED, 0x0004
	CHECKED, 0x0008
	FOCUS, 0x0010
	DEFAULT, 0x0020
	HOT, 0x0040
	MARKED, 0x0080
	INDETERMINATE, 0x0100
	SHOWKEYBOARDCUES, 0x0200
	NEARHOT, 0x0400
	OTHERSIDEHOT, 0x0800
	DROPHILITED, 0x1000
}

const_type! { CDRF, u32,
	/// [`NM_CUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-customdraw)
	/// return value (`u32`).

	DODEFAULT, 0x00000000
	NEWFONT, 0x00000002
	SKIPDEFAULT, 0x00000004
	DOERASE, 0x00000008
	SKIPPOSTPAINT, 0x00000100
	NOTIFYPOSTPAINT, 0x00000010
	NOTIFYITEMDRAW, 0x00000020
	NOTIFYSUBITEMDRAW, 0x00000020
	NOTIFYPOSTERASE, 0x00000040
}
impl From<CDRF> for isize {
	fn from(v: CDRF) -> Self {
		v.0 as isize
	}
}

const_type! { CHARSET, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfCharset` (`u8`).

	ANSI, 0
	DEFAULT, 1
	SYMBOL, 2
	SHIFTJIS, 128
	HANGEUL, 129
	HANGUL, 129
	GB2312, 134
	CHINESEBIG5, 136
	OEM, 255
	JOHAB, 130
	HEBREW, 177
	ARABIC, 178
	GREEK, 161
	TURKISH, 162
	VIETNAMESE, 163
	THAI, 222
	EASTEUROPE, 238
	RUSSIAN, 204
	MAC, 77
	BALTIC, 186
}

const_type! { CLIP, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfClipPrecision` (`u8`).

	DEFAULT_PRECIS, 0
	CHARACTER_PRECIS, 1
	STROKE_PRECIS, 2
	MASK, 0xf
	LH_ANGLES, 1 << 4
	TT_ALWAYS, 2 << 4
	DFA_DISABLE, 4 << 4
	EMBEDDED, 8 << 4
}

const_type! { CLR, u32,
	/// [`IMAGELISTDRAWPARAMS`](crate::IMAGELISTDRAWPARAMS) `rgbFg` (`u32`).

	CLR_NONE, 0xffffffff
	DEFAULT, 0xff000000
}

const_type! { CLSCTX, u32,
	/// [`CLSCTX`](https://docs.microsoft.com/en-us/windows/win32/api/wtypesbase/ne-wtypesbase-clsctx)
	/// enumeration (`u32`).

	INPROC_SERVER, 0x1
	INPROC_HANDLER, 0x2
	LOCAL_SERVER, 0x4
	INPROC_SERVER16, 0x8
	REMOTE_SERVER, 0x10
	INPROC_HANDLER16, 0x20
	NO_CODE_DOWNLOAD, 0x400
	NO_CUSTOM_MARSHAL, 0x1000
	ENABLE_CODE_DOWNLOAD, 0x2000
	NO_FAILURE_LOG, 0x4000
	DISABLE_AAA, 0x8000
	ENABLE_AAA, 0x10000
	FROM_DEFAULT_CONTEXT, 0x20000
	ACTIVATE_X86_SERVER, 0x40000
	ACTIVATE_32_BIT_SERVER, Self::ACTIVATE_X86_SERVER.0
	ACTIVATE_64_BIT_SERVER, 0x80000
	ENABLE_CLOAKING, 0x100000
	APPCONTAINER, 0x400000
	ACTIVATE_AAA_AS_IU, 0x800000
	ACTIVATE_ARM32_SERVER, 0x2000000
	PS_DLL, 0x80000000
}

const_type! { CMD, u16,
	/// [`WM_COMMAND`](crate::msg::WmCommand) notification codes (`u16`).
	///
	/// Control-specific notification codes have their own types, which are
	/// convertible to `CMD`.

	Menu, 0
	Accelerator, 1
}

const_type! { COINIT, u32,
	/// [`CoInitializeEx`](crate::CoInitializeEx) `dwCoInit` (`u32`).

	APARTMENTTHREADED, 0x2
	MULTITHREADED, 0x0
	DISABLE_OLE1DDE, 0x4
	SPEED_OVER_MEMORY, 0x8
}

const_type! { COLOR, u32,
	/// System
	/// [colors](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
	/// (`u32`).

	SCROLLBAR, 0
	BACKGROUND, 1
	ACTIVECAPTION, 2
	INACTIVECAPTION, 3
	MENU, 4
	WINDOW, 5
	WINDOWFRAME, 6
	MENUTEXT, 7
	WINDOWTEXT, 8
	CAPTIONTEXT, 9
	ACTIVEBORDER, 10
	INACTIVEBORDER, 11
	APPWORKSPACE, 12
	HIGHLIGHT, 13
	HIGHLIGHTTEXT, 14
	BTNFACE, 15
	BTNSHADOW, 16
	GRAYTEXT, 17
	BTNTEXT, 18
	INACTIVECAPTIONTEXT, 19
	BTNHIGHLIGHT, 20
	_3DDKSHADOW, 21
	_3DLIGHT, 22
	INFOTEXT, 23
	INFOBK, 24
	HOTLIGHT, 26
	GRADIENTACTIVECAPTION, 27
	GRADIENTINACTIVECAPTION, 28
	MENUHILIGHT, 29
	MENUBAR, 30
	DESKTOP, Self::BACKGROUND.0
	_3DFACE, Self::BTNFACE.0
	_3DSHADOW, Self::BTNSHADOW.0
	_3DHIGHLIGHT, Self::BTNHIGHLIGHT.0
	_3DHILIGHT, Self::BTNHIGHLIGHT.0
	BTNHILIGHT, Self::BTNHIGHLIGHT.0
}

const_type! { CS, u32,
	/// Window class
	/// [`styles`](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles)
	/// (`u32`).

	VREDRAW, 0x0001
	HREDRAW, 0x0002
	DBLCLKS, 0x0008
	OWNDC, 0x0020
	CLASSDC, 0x0040
	PARENTDC, 0x0080
	NOCLOSE, 0x0200
	SAVEBITS, 0x0800
	BYTEALIGNCLIENT, 0x1000
	BYTEALIGNWINDOW, 0x2000
	GLOBALCLASS, 0x4000
	IME, 0x00010000
	DROPSHADOW, 0x00020000
}

const_type! { DDL, u16,
	/// [`CB_DIR`](crate::msg::CbDir) attributes (`u16`).

	READWRITE, 0x0000
	READONLY, 0x0001
	HIDDEN, 0x0002
	SYSTEM, 0x0004
	DIRECTORY, 0x0010
	ARCHIVE, 0x0020
	POSTMSGS, 0x2000
	DRIVES, 0x4000
	EXCLUSIVE, 0x8000
}

const_type! { DLGID, u16,
	/// Dialog built-in IDs (`u16`). These are also returned from
	/// [`MessageBox`](crate::HWND::MessageBox).

	OK, 1
	CANCEL, 2
	ABORT, 3
	RETRY, 4
	IGNORE, 5
	YES, 6
	NO, 7
	TRYAGAIN, 10
	CONTINUE, 11
}

const_type_wm! { DTM,
	/// Date and time picker control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).

	GETSYSTEMTIME, Self::FIRST.0 + 1
	SETSYSTEMTIME, Self::FIRST.0 + 2
	GETRANGE, Self::FIRST.0 + 3
	SETRANGE, Self::FIRST.0 + 4
	SETFORMAT, Self::FIRST.0 + 50
	SETMCCOLOR, Self::FIRST.0 + 6
	GETMCCOLOR, Self::FIRST.0 + 7
	GETMONTHCAL, Self::FIRST.0 + 8
	SETMCFONT, Self::FIRST.0 + 9
	GETMCFONT, Self::FIRST.0 + 10
	SETMCSTYLE, Self::FIRST.0 + 11
	GETMCSTYLE, Self::FIRST.0 + 12
	CLOSEMONTHCAL, Self::FIRST.0 + 13
	GETDATETIMEPICKERINFO, Self::FIRST.0 + 14
	GETIDEALSIZE, Self::FIRST.0 + 15
}
const_type_priv_values! { DTM
	FIRST, 0x1000
}

const_type_nm! { DTN,
	/// Date and time picker control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).

	CLOSEUP, Self::FIRST2.0
	DATETIMECHANGE, Self::FIRST2.0 - 6
	DROPDOWN, Self::FIRST2.0 - 1
	FORMAT, Self::FIRST2.0 - 3
	FORMATQUERY, Self::FIRST.0 - 3
	USERSTRING, Self::FIRST.0 - 5
	WMKEYDOWN, Self::FIRST.0 - 4
}
const_type_priv_values! { DTN
	FIRST, -740
	FIRST2, -753
}

const_type_ws! { DTS,
	/// Date and time picker control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/date-and-time-picker-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).

	UPDOWN, 0x0001
	SHOWNONE, 0x0002
	SHORTDATEFORMAT, 0x0000
	LONGDATEFORMAT, 0x0004
	SHORTDATECENTURYFORMAT, 0x000c
	TIMEFORMAT, 0x0009
	APPCANPARSE, 0x0010
	RIGHTALIGN, 0x0020
}
