#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::co::WM;

pub_struct_const_wm! { CB,
	/// Combo box control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	FIRST, 0x1700
	=>
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

pub_struct_const_wm! { CBEM,
	/// ComboBoxEx control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-comboboxex-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	=>
	SETIMAGELIST, WM::USER.0 + 2
	GETIMAGELIST, WM::USER.0 + 3
	DELETEITEM, CB::DELETESTRING.0
	GETCOMBOCONTROL, WM::USER.0 + 6
	GETEDITCONTROL, WM::USER.0 + 7
	SETEXTENDEDSTYLE, WM::USER.0 + 14
	GETEXTENDEDSTYLE, WM::USER.0 + 9
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
	HASEDITCHANGED, WM::USER.0 + 10
	INSERTITEM, WM::USER.0 + 11
	SETITEM, WM::USER.0 + 12
	GETITEM, WM::USER.0 + 13
}

pub_struct_const_wsex! { CBES_EX,
	/// Extended combo box
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/comboboxex-control-extended-styles)
	/// (`u32`), convertible to [`WS_EX`](crate::co::WS_EX).
	=>
	NOEDITIMAGE, 0x0000_0001
	NOEDITIMAGEINDENT, 0x0000_0002
	PATHWORDBREAKPROC, 0x0000_0004
	NOSIZELIMIT, 0x0000_0008
	CASESENSITIVE, 0x0000_0010
	TEXTENDELLIPSIS, 0x0000_0020
}

pub_struct_const_cmd! { CBN,
	/// Combo box control `WM_COMMAND`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-combobox-control-reference-notifications)
	/// (`u16`), convertible to [`CMD`](crate::co::CMD).
	=>
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

pub_struct_const_ws! { CBS,
	/// Combo box control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/combo-box-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
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

pub_struct_const! { CC, u32,
	/// [`CHOOSECOLOR`](crate::CHOOSECOLOR) `Flags` (`u32`).
	=>
	/// Causes the dialog box to use the color specified in the `rgbResult`
	/// member as the initial color selection.
	RGBINIT, 0x0000_0001
	/// Causes the dialog box to display the additional controls that allow the
	/// user to create custom colors. If this flag is not set, the user must
	/// click the Define Custom Color button to display the custom color
	/// controls.
	FULLOPEN, 0x0000_0002
	/// Disables the Define Custom Color button.
	PREVENTFULLOPEN, 0x0000_0004
	/// Causes the dialog box to display the Help button. The `hwndOwner` member
	/// must specify the window to receive the `HELPMSGSTRING` registered
	/// messages that the dialog box sends when the user clicks the Help button.
	SHOWHELP, 0x0000_0008
	/// Enables the hook procedure specified in the `lpfnHook` member of this
	/// structure. This flag is used only to initialize the dialog box.
	ENABLEHOOK, 0x0000_0010
	/// The `hInstance` and `lpTemplateName` members specify a dialog box
	/// template to use in place of the default template. This flag is used only
	/// to initialize the dialog box.
	ENABLETEMPLATE, 0x0000_0020
	/// The `hInstance` member identifies a data block that contains a preloaded
	/// dialog box template. The system ignores the `lpTemplateName` member if
	/// this flag is specified. This flag is used only to initialize the dialog
	/// box.
	ENABLETEMPLATEHANDLE, 0x0000_0040
	/// Causes the dialog box to display only solid colors in the set of basic
	/// colors.
	SOLIDCOLOR, 0x0000_0080
	/// Causes the dialog box to display all available colors in the set of
	/// basic colors.
	ANYCOLOR, 0x0000_0100
}

pub_struct_const_wm! { CCM,
	/// Generic common controls
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/common-controls-intro)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	FIRST, 0x2000
	=>
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

pub_struct_const! { CDDS, u32,
	/// [`NMCUSTOMDRAW`](crate::NMCUSTOMDRAW) `dwDrawStage` (`u32`).
	=>
	PREPAINT, 0x0000_0001
	POSTPAINT, 0x0000_0002
	PREERASE, 0x0000_0003
	POSTERASE, 0x0000_0004
	ITEM, 0x0001_0000
	ITEMPREPAINT, Self::ITEM.0 | Self::PREPAINT.0
	ITEMPOSTPAINT, Self::ITEM.0 | Self::POSTPAINT.0
	ITEMPREERASE, Self::ITEM.0 | Self::PREERASE.0
	ITEMPOSTERASE, Self::ITEM.0 | Self::POSTERASE.0
	SUBITEM, 0x0002_0000
}

pub_struct_const! { CDERR, u32,
	/// Common dialog box
	/// [error codes](https://docs.microsoft.com/en-us/windows/win32/api/commdlg/nf-commdlg-commdlgextendederror).
	/// Also includes `PDERR`, `CFERR, `FNERR` and `FRERR` prefixes.
	=>
	/// None of the actual values (zero).
	NoValue, 0
	DIALOGFAILURE, 0xffff
	FINDRESFAILURE, 0x0006
	INITIALIZATION, 0x0002
	LOADRESFAILURE, 0x0007
	LOADSTRFAILURE, 0x0005
	LOCKRESFAILURE, 0x0008
	MEMALLOCFAILURE, 0x0009
	MEMLOCKFAILURE, 0x000a
	NOHINSTANCE, 0x0004
	NOHOOK, 0x000b
	NOTEMPLATE, 0x0003
	REGISTERMSGFAIL, 0x000c
	STRUCTSIZE, 0x0001
	PD_CREATEICFAILURE, 0x100a
	PD_DEFAULTDIFFERENT, 0x100c
	PD_DNDMMISMATCH, 0x1009
	PD_GETDEVMODEFAIL, 0x1005
	PD_INITFAILURE, 0x1006
	PD_LOADDRVFAILURE, 0x1004
	PD_NODEFAULTPRN, 0x1008
	PD_NODEVICES, 0x1007
	PD_PARSEFAILURE, 0x1002
	PD_PRINTERNOTFOUND, 0x100b
	PD_RETDEFFAILURE, 0x1003
	PD_SETUPFAILURE, 0x1001
	CF_MAXLESSTHANMIN, 0x2002
	CF_NOFONTS, 0x2001
	FN_BUFFERTOOSMALL, 0x3003
	FN_INVALIDFILENAME, 0x3002
	FN_SUBCLASSFAILURE, 0x3001
	FR_BUFFERLENGTHZERO, 0x4001
}

pub_struct_const! { CDIS, u32,
	/// [`NMCUSTOMDRAW`](crate::NMCUSTOMDRAW) `uItemState` (`u32`).
	=>
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

pub_struct_const! { CDRF, u32,
	/// [`NM_CUSTOMDRAW`](https://docs.microsoft.com/en-us/windows/win32/controls/nm-customdraw)
	/// return value (`u32`).
	=>
	DODEFAULT, 0x0000_0000
	NEWFONT, 0x0000_0002
	SKIPDEFAULT, 0x0000_0004
	DOERASE, 0x0000_0008
	SKIPPOSTPAINT, 0x0000_0100
	NOTIFYPOSTPAINT, 0x0000_0010
	NOTIFYITEMDRAW, 0x0000_0020
	NOTIFYSUBITEMDRAW, 0x0000_0020
	NOTIFYPOSTERASE, 0x0000_0040
}
impl From<CDRF> for isize {
	fn from(v: CDRF) -> Self {
		v.0 as isize
	}
}

pub_struct_const! { CDS, u32,
	/// [`ChangeDisplaySettings`](crate::ChangeDisplaySettings) `dwFlags`
	/// (`u32`).
	=>
	DYNAMICALLY, 0
	FULLSCREEN, 0x0000_0004
	GLOBAL, 0x0000_0008
	NORESET, 0x1000_0000
	RESET, 0x40000_000
	SET_PRIMARY, 0x0000_0010
	TEST, 0x0000_0002
	UPDATEREGISTRY, 0x0000_0001
}

pub_struct_const! { CF, u32,
	/// Standard clipboard
	/// [formats](https://docs.microsoft.com/en-us/windows/win32/dataxchg/standard-clipboard-formats)
	/// (`u32`).
	=>
	TEXT, 1
	BITMAP, 2
	METAFILEPICT, 3
	SYLK, 4
	DIF, 5
	TIFF, 6
	OEMTEXT, 7
	DIB, 8
	PALETTE, 9
	PENDATA, 10
	RIFF, 11
	WAVE, 12
	UNICODETEXT, 13
	ENHMETAFILE, 14
	HDROP, 15
	LOCALE, 16
	DIBV5, 17
	OWNERDISPLAY, 0x0080
	DSPTEXT, 0x0081
	DSPBITMAP, 0x0082
	DSPMETAFILEPICT, 0x0083
	DSPENHMETAFILE, 0x008e
	PRIVATEFIRST, 0x0200
	PRIVATELAST, 0x02ff
	GDIOBJFIRST, 0x0300
	GDIOBJLAST, 0x03ff
}

pub_struct_const! { CHARSET, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfCharset` (`u8`).
	=>
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

pub_struct_const! { CLIP, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfClipPrecision` (`u8`).
	=>
	DEFAULT_PRECIS, 0
	CHARACTER_PRECIS, 1
	STROKE_PRECIS, 2
	MASK, 0xf
	LH_ANGLES, 1 << 4
	TT_ALWAYS, 2 << 4
	DFA_DISABLE, 4 << 4
	EMBEDDED, 8 << 4
}

pub_struct_const! { CLR, u32,
	/// [`IMAGELISTDRAWPARAMS`](crate::IMAGELISTDRAWPARAMS) `rgbFg` (`u32`).
	=>
	CLR_NONE, 0xffff_ffff
	DEFAULT, 0xff00_0000
}

pub_struct_const! { CLSCTX, u32,
	/// [`CLSCTX`](https://docs.microsoft.com/en-us/windows/win32/api/wtypesbase/ne-wtypesbase-clsctx)
	/// enumeration (`u32`).
	=>
	/// The code that creates and manages objects of this class is a DLL that
	/// runs in the same process as the caller of the function specifying the
	/// class context.
	INPROC_SERVER, 0x1
	/// The code that manages objects of this class is an in-process handler.
	/// This is a DLL that runs in the client process and implements client-side
	/// structures of this class when instances of the class are accessed
	/// remotely.
	INPROC_HANDLER, 0x2
	/// The EXE code that creates and manages objects of this class runs on same
	/// machine but is loaded in a separate process space.
	LOCAL_SERVER, 0x4
	/// A remote context. The `LocalServer32` or `LocalService` code that creates
	/// and manages objects of this class is run on a different computer.
	REMOTE_SERVER, 0x10
	/// Disables the downloading of code from the directory service or the
	/// Internet. This flag cannot be set at the same time as
	/// `CLSCTX::ENABLE_CODE_DOWNLOAD`.
	NO_CODE_DOWNLOAD, 0x400
	/// Specify if you want the activation to fail if it uses custom marshalling.
	NO_CUSTOM_MARSHAL, 0x1000
	/// Enables the downloading of code from the directory service or the
	/// Internet. This flag cannot be set at the same time as
	/// `CLSCTX::NO_CODE_DOWNLOAD`.
	ENABLE_CODE_DOWNLOAD, 0x2000
	/// The `CLSCTX::NO_FAILURE_LOG` can be used to override the logging of
	/// failures in [`CoCreateInstanceEx`](crate::CoCreateInstanceEx).
	NO_FAILURE_LOG, 0x4000
	/// Disables activate-as-activator (AAA) activations for this activation only.
	DISABLE_AAA, 0x8000
	/// Enables activate-as-activator (AAA) activations for this activation only.
	ENABLE_AAA, 0x1_0000
	/// Begin this activation from the default context of the current apartment.
	FROM_DEFAULT_CONTEXT, 0x2_0000
	/// Activate or connect to a 32-bit version of the server; fail if one is not
	/// registered.
	ACTIVATE_X86_SERVER, 0x4_0000
	/// Activate or connect to a 32-bit version of the server; fail if one is not
	/// registered.
	ACTIVATE_32_BIT_SERVER, Self::ACTIVATE_X86_SERVER.0
	/// Activate or connect to a 64 bit version of the server; fail if one is not
	/// registered.
	ACTIVATE_64_BIT_SERVER, 0x8_0000
	/// Specify this flag for Interactive User activation behavior for
	/// As-Activator servers.
	ACTIVATE_AAA_AS_IU, 0x80_0000
	/// (No official docs for this entry.)
	ACTIVATE_ARM32_SERVER, 0x200_0000
}

pub_struct_const! { CMD, u16,
	/// [`WM_COMMAND`](crate::msg::wm::Command) notification codes (`u16`).
	///
	/// **Note:** Control-specific notification codes have their own types,
	/// which are convertible to `CMD`.
	=>
	Menu, 0
	Accelerator, 1
}

pub_struct_const! { COINIT, u32,
	/// [`COINIT`](https://docs.microsoft.com/en-us/windows/win32/api/objbase/ne-objbase-coinit)
	/// enumeration (`u32`).
	=>
	/// Initializes the thread for apartment-threaded object concurrency.
	APARTMENTTHREADED, 0x2
	/// Initializes the thread for multithreaded object concurrency.
	MULTITHREADED, 0x0
	/// Disables DDE for OLE1 support.
	DISABLE_OLE1DDE, 0x4
	/// Increase memory usage in an attempt to increase performance.
	SPEED_OVER_MEMORY, 0x8
}

pub_struct_const! { COLOR, i32,
	/// System
	/// [colors](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor)
	/// (`i32`).
	=>
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
	C3DDKSHADOW, 21
	C3DLIGHT, 22
	INFOTEXT, 23
	INFOBK, 24
	HOTLIGHT, 26
	GRADIENTACTIVECAPTION, 27
	GRADIENTINACTIVECAPTION, 28
	MENUHILIGHT, 29
	MENUBAR, 30
	DESKTOP, Self::BACKGROUND.0
	C3DFACE, Self::BTNFACE.0
	C3DSHADOW, Self::BTNSHADOW.0
	C3DHIGHLIGHT, Self::BTNHIGHLIGHT.0
	C3DHILIGHT, Self::BTNHIGHLIGHT.0
	BTNHILIGHT, Self::BTNHIGHLIGHT.0
}

pub_struct_const! { CP, u32,
	/// [`WideCharToMultiByte`](crate::WideCharToMultiByte) and
	/// [`MultiByteToWideChar`](crate::MultiByteToWideChar) `CodePage`
	/// [identifiers](https://docs.microsoft.com/en-us/windows/win32/intl/code-page-identifiers)
	/// (`u32`).
	=>
	/// The system default Windows ANSI code page.
	ACP, 0
	/// The current system OEM code page.
	OEMCP, 1
	/// The current system Macintosh code page.
	MACCP, 2
	/// The Windows ANSI code page for the current thread.
	THREAD_ACP, 3
	/// Symbol code page (42).
	SYMBOL, 42
	/// UTF-7. Use this value only when forced by a 7-bit transport mechanism.
	/// Use of UTF-8 is preferred. With this value set, `lpDefaultChar` and
	/// `lpUsedDefaultChar` must be set to null.
	UTF7, 65000
	/// UTF-8. With this value set, `lpDefaultChar` and `lpUsedDefaultChar` must
	/// be set to null.
	UTF8, 65001

	/// ANSI Central European; Central European (Windows).
	WINDOWS_1250, 1250
	/// ANSI Cyrillic; Cyrillic (Windows).
	WINDOWS_1251, 1251
	/// ANSI Latin 1; Western European (Windows).
	WINDOWS_1252, 1252
	/// ANSI Greek; Greek (Windows).
	WINDOWS_1253, 1253
	/// ANSI Turkish; Turkish (Windows).
	WINDOWS_1254, 1254
	/// ANSI Hebrew; Hebrew (Windows).
	WINDOWS_1255, 1255
	/// ANSI Arabic; Arabic (Windows).
	WINDOWS_1256, 1256
	/// ANSI Baltic; Baltic (Windows).
	WINDOWS_1257, 1257
	/// ANSI/OEM Vietnamese; Vietnamese (Windows).
	WINDOWS_1258, 1258
}

pub_struct_const! { CREATE, u32,
	/// Process creation
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
	/// (`u32`).
	=>
	/// None of the actual values (zero).
	NoValue, 0
	BREAKAWAY_FROM_JOB, 0x0100_0000
	DEFAULT_ERROR_MODE, 0x0400_0000
	NEW_CONSOLE, 0x0000_0010
	NEW_PROCESS_GROUP, 0x0000_0200
	NO_WINDOW, 0x0800_0000
	PROTECTED_PROCESS, 0x0004_0000
	PRESERVE_CODE_AUTHZ_LEVEL, 0x0200_0000
	SECURE_PROCESS, 0x0040_0000
	SEPARATE_WOW_VDM, 0x0000_0800
	SHARED_WOW_VDM, 0x0000_1000
	SUSPENDED, 0x0000_0004
	UNICODE_ENVIRONMENT, 0x0000_0400
	/// Originally has no `CREATE` prefix.
	DEBUG_ONLY_THIS_PROCESS, 0x0000_0002
	/// Originally has no `CREATE` prefix.
	DEBUG_PROCESS, 0x0000_0001
	/// Originally has no `CREATE` prefix.
	DETACHED_PROCESS, 0x0000_0008
	/// Originally has no `CREATE` prefix.
	EXTENDED_STARTUPINFO_PRESENT, 0x0008_0000
	/// Originally has no `CREATE` prefix.
	INHERIT_PARENT_AFFINITY, 0x0001_0000
}

pub_struct_const! { CS, u32,
	/// Window class
	/// [`styles`](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-class-styles)
	/// (`u32`).
	=>
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
