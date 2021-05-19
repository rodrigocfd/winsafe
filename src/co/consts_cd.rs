#![allow(non_upper_case_globals)]

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
	/// `CLSCTX_ENABLE_CODE_DOWNLOAD`.
	NO_CODE_DOWNLOAD, 0x400
	/// Specify if you want the activation to fail if it uses custom marshalling.
	NO_CUSTOM_MARSHAL, 0x1000
	/// Enables the downloading of code from the directory service or the
	/// Internet. This flag cannot be set at the same time as
	/// `CLSCTX_NO_CODE_DOWNLOAD`.
	ENABLE_CODE_DOWNLOAD, 0x2000
	/// The `CLSCTX_NO_FAILURE_LOG` can be used to override the logging of
	/// failures in
	/// [`CoCreateInstanceEx`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstanceex).
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
	/// Control-specific notification codes have their own types, which are
	/// convertible to `CMD`.
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
	/// [`MultiByteToWideChar`](crate::MultiByteToWideChar) `CodePage` (`u32`).
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
}

pub_struct_const! { CREATE, u32,
	/// Process creation
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
	/// (`u32`).
	=>
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

pub_struct_const! { DDL, u16,
	/// [`CB_DIR`](crate::msg::cb::Dir) and [`LB_DIR`](crate::msg::lb::Dir)
	/// attributes (`u16`).
	=>
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

pub_struct_const! { DIB, u32,
	/// [`LOGBRUSH`](crate::LOGBRUSH) `lbColor` (`u32`).
	=>
	/// The color table consists of an array of 16-bit indexes into the
	/// currently realized logical palette.
	RGB_COLORS, 0
	/// The color table contains literal RGB values.
	PAL_COLORS, 1
}

pub_struct_const! { DISPOSITION, u32,
	/// [`CreateFile`](crate::HFILE::CreateFile) `dwCreationDisposition` (`u32`).
	/// Originally has no prefix.
	=>
	/// Creates a new file, only if it does not already exist.
	///
	/// If the specified file exists, the function fails and the last-error code
	/// is set to [`ERROR::FILE_EXISTS`](crate::co::ERROR::FILE_EXISTS).
	///
	/// If the specified file does not exist and is a valid path to a writable
	/// location, a new file is created.
	CREATE_NEW, 1
	/// Creates a new file, always.
	///
	/// If the specified file exists and is writable, the function overwrites
	/// the file, the function succeeds, and last-error code is set to
	/// [`ERROR::ALREADY_EXISTS`](crate::co::ERROR::ALREADY_EXISTS).
	///
	/// If the specified file does not exist and is a valid path, a new file is
	/// created, the function succeeds, and the last-error code is set to
	/// [`ERROR::SUCCESS`](crate::co::ERROR::SUCCESS).
	CREATE_ALWAYS, 2
	/// Opens a file or device, only if it exists.
	///
	/// If the specified file or device does not exist, the function fails and
	/// the last-error code is set to
	/// [`ERROR::FILE_NOT_FOUND`](crate::co::ERROR::FILE_NOT_FOUND).
	OPEN_EXISTING, 3
	/// Opens a file, always.
	///
	/// If the specified file exists, the function succeeds and the last-error
	/// code is set to
	/// [`ERROR::ALREADY_EXISTS`](crate::co::ERROR::ALREADY_EXISTS).
	///
	/// If the specified file does not exist and is a valid path to a writable
	/// location, the function creates a file and the last-error code is set to
	/// [`ERROR::SUCCESS`](crate::co::ERROR::SUCCESS).
	OPEN_ALWAYS, 4
	/// Opens a file and truncates it so that its size is zero bytes, only if it
	/// exists.
	///
	/// If the specified file does not exist, the function fails and the
	/// last-error code is set to
	/// [`ERROR::FILE_NOT_FOUND`](crate::co::ERROR::FILE_NOT_FOUND).
	///
	/// The calling process must open the file with the
	/// [`GENERIC::WRITE`](crate::co::GENERIC::WRITE) bit set as part of the
	/// `dwDesiredAccess` parameter.
	TRUNCATE_EXISTING, 5
}

pub_struct_const! { DLGID, i32,
	/// Dialog built-in IDs (`i32`). These are also returned from
	/// [`MessageBox`](crate::HWND::MessageBox).
	=>
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

pub_struct_const_wm! { DTM,
	/// Date and time picker control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	FIRST, 0x1000
	=>
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

pub_struct_const_nm! { DTN,
	/// Date and time picker control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -740
	FIRST2, -753
	=>
	CLOSEUP, Self::FIRST2.0
	DATETIMECHANGE, Self::FIRST2.0 - 6
	DROPDOWN, Self::FIRST2.0 - 1
	FORMAT, Self::FIRST2.0 - 3
	FORMATQUERY, Self::FIRST.0 - 3
	USERSTRING, Self::FIRST.0 - 5
	WMKEYDOWN, Self::FIRST.0 - 4
}

pub_struct_const_ws! { DTS,
	/// Date and time picker control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/date-and-time-picker-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	UPDOWN, 0x0001
	SHOWNONE, 0x0002
	SHORTDATEFORMAT, 0x0000
	LONGDATEFORMAT, 0x0004
	SHORTDATECENTURYFORMAT, 0x000c
	TIMEFORMAT, 0x0009
	APPCANPARSE, 0x0010
	RIGHTALIGN, 0x0020
}
