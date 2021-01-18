#![allow(non_upper_case_globals)]

use crate::co::WS;

const_type! { CBS, u32,
	/// Combo box control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/combo-box-styles)
	/// (`u32`).

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
impl From<CBS> for WS {
	fn from(v: CBS) -> Self {
		Self(v.0)
	}
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
	/// [`WM_COMMAND`](crate::msg::WmCommand)
	/// notifications  (`u16`) for:
	///
	/// * [Button](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications);
	/// * [ComboBox](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-notifications);
	/// * [Edit](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-notifications);
	/// * [ListBox](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-box-control-reference-notifications).

	Menu, 0
	Accelerator, 1

	BN_CLICKED, 0
	BN_PAINT, 1
	BN_HILITE, 2
	BN_UNHILITE, 3
	BN_DISABLE, 4
	BN_DOUBLECLICKED, 5
	BN_PUSHED, Self::BN_HILITE.0
	BN_UNPUSHED, Self::BN_UNHILITE.0
	BN_DBLCLK, Self::BN_DOUBLECLICKED.0
	BN_SETFOCUS, 6
	BN_KILLFOCUS, 7

	CBN_ERRSPACE, (0 - 1) as u16
	CBN_SELCHANGE, 1
	CBN_DBLCLK, 2
	CBN_SETFOCUS, 3
	CBN_KILLFOCUS, 4
	CBN_EDITCHANGE, 5
	CBN_EDITUPDATE, 6
	CBN_DROPDOWN, 7
	CBN_CLOSEUP, 8
	CBN_SELENDOK, 9
	CBN_SELENDCANCEL, 10

	EN_SETFOCUS, 0x0100
	EN_KILLFOCUS, 0x0200
	EN_CHANGE, 0x0300
	EN_UPDATE, 0x0400
	EN_ERRSPACE, 0x0500
	EN_MAXTEXT, 0x0501
	EN_HSCROLL, 0x0601
	EN_VSCROLL, 0x0602
	EN_ALIGN_LTR_EC, 0x0700
	EN_ALIGN_RTL_EC, 0x0701
	EN_BEFORE_PASTE, 0x0800
	EN_AFTER_PASTE, 0x0801

	LBN_ERRSPACE, (0 -2) as u16
	LBN_SELCHANGE, 1
	LBN_DBLCLK, 2
	LBN_SELCANCEL, 3
	LBN_SETFOCUS, 4
	LBN_KILLFOCUS, 5
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
