use crate::co::CCM;

pub_struct_const! { MB, u32,
	/// [`MessageBox`](crate::HWND::MessageBox) `uType` (`u32`).
	=>
	/// The message box contains three push buttons: Abort, Retry, and Ignore.
	ABORTRETRYIGNORE, 0x0000_0002
	/// The message box contains three push buttons: Cancel, Try Again,
	/// Continue. Use this message box type instead of
	/// [`MB::ABORTRETRYIGNORE`](crate::co::MB::ABORTRETRYIGNORE).
	CANCELTRYCONTINUE, 0x0000_0006
	/// Adds a Help button to the message box. When the user clicks the Help
	/// button or presses F1, the system sends a
	/// [`WM_HELP`](crate::msg::wm::Help) message to the owner.
	HELP, 0x0000_4000
	/// The message box contains one push button: OK. This is the default.
	OK, 0x0000_0000
	/// The message box contains two push buttons: OK and Cancel.
	OKCANCEL, 0x0000_0001
	/// The message box contains two push buttons: Retry and Cancel.
	RETRYCANCEL, 0x0000_0005
	/// The message box contains two push buttons: Yes and No.
	YESNO, 0x0000_0004
	/// The message box contains three push buttons: Yes, No, and Cancel.
	YESNOCANCEL, 0x0000_0003

	/// An exclamation-point icon appears in the message box.
	ICONEXCLAMATION, 0x0000_0030
	/// An exclamation-point icon appears in the message box.
	ICONWARNING, Self::ICONEXCLAMATION.0
	/// An icon consisting of a lowercase letter i in a circle appears in the
	/// message box.
	ICONINFORMATION, 0x0000_0040
	/// An icon consisting of a lowercase letter i in a circle appears in the
	/// message box.
	ICONASTERISK, Self::ICONINFORMATION.0
	/// A question-mark icon appears in the message box. The question-mark
	/// message icon is no longer recommended because it does not clearly
	/// represent a specific type of message and because the phrasing of a
	/// message as a question could apply to any message type. In addition,
	/// users can confuse the message symbol question mark with Help
	/// information. Therefore, do not use this question mark message symbol in
	/// your message boxes. The system continues to support its inclusion only
	/// for backward compatibility.
	ICONQUESTION, 0x0000_0020
	/// A stop-sign icon appears in the message box.
	ICONSTOP, Self::ICONERROR.0
	/// A stop-sign icon appears in the message box.
	ICONERROR, 0x0000_0010
	/// A stop-sign icon appears in the message box.
	ICONHAND, Self::ICONERROR.0

	/// The first button is the default button. `MB::DEFBUTTON1` is the default
	/// unless [`MB::DEFBUTTON2`](crate::co::MB::DEFBUTTON2),
	/// [`MB::DEFBUTTON3`](crate::co::MB::DEFBUTTON3), or
	/// [`MB::DEFBUTTON4`](crate::co::MB::DEFBUTTON4) is specified.
	DEFBUTTON1, 0x0000_0000
	/// The second button is the default button.
	DEFBUTTON2, 0x0000_0100
	/// The third button is the default button.
	DEFBUTTON3, 0x0000_0200
	/// The fourth button is the default button.
	DEFBUTTON4, 0x0000_0300

	/// The user must respond to the message box before continuing work in the
	/// window identified by the hWnd parameter. However, the user can move to
	/// the windows of other threads and work in those windows.
	///
	/// Depending on the hierarchy of windows in the application, the user may
	/// be able to move to other windows within the thread. All child windows of
	/// the parent of the message box are automatically disabled, but pop-up
	/// windows are not.
	///
	/// `MB::APPLMODAL` is the default if neither
	/// [`MB::SYSTEMMODAL`](crate::co::MB::SYSTEMMODAL) nor
	/// [`MB::TASKMODAL`](crate::co::MB::TASKMODAL) is specified.
	APPLMODAL, 0x0000_0000
	/// Same as [`MB::APPLMODAL`](crate::co::MB::APPLMODAL) except that the
	/// message box has the [`WS_EX::TOPMOST`](crate::co::WS_EX::TOPMOST) style.
	/// Use system-modal message boxes to notify the user of serious,
	/// potentially damaging errors that require immediate attention (for
	/// example, running out of memory). This flag has no effect on the user's
	/// ability to interact with windows other than those associated with hWnd.
	SYSTEMMODAL, 0x0000_1000
	/// Same as [`MB::APPLMODAL`](crate::co::MB::APPLMODAL) except that all the
	/// top-level windows belonging to the current thread are disabled if the
	/// hWnd parameter is NULL. Use this flag when the calling application or
	/// library does not have a window handle available but still needs to
	/// prevent input to other windows in the calling thread without suspending
	/// other threads.
	TASKMODAL, 0x0000_2000

	/// Same as desktop of the interactive window station. For more information,
	/// see
	/// [Window Stations](https://docs.microsoft.com/en-us/windows/win32/winstation/window-stations).
	///
	/// If the current input desktop is not the default desktop,
	/// [`MessageBox`](crate::HWND::MessageBox) does not return until the user
	/// switches to the default desktop.
	DEFAULT_DESKTOP_ONLY, 0x0002_0000
	/// The text is right-justified.
	RIGHT, 0x0008_0000
	/// Displays message and caption text using right-to-left reading order on
	/// Hebrew and Arabic systems.
	RTLREADING, 0x0010_0000
	/// The message box becomes the foreground window. Internally, the system
	/// calls the [`SetForegroundWindow`](crate::HWND::SetForegroundWindow)
	/// function for the message box.
	SETFOREGROUND, 0x0001_0000
	/// The message box is created with the
	/// [`WS_EX::TOPMOST`](crate::co::WS_EX::TOPMOST) window style.
	TOPMOST, 0x0004_0000
	/// The caller is a service notifying the user of an event. The function
	/// displays a message box on the current active desktop, even if there is
	/// no user logged on to the computer.
	///
	/// Terminal Services: If the calling thread has an impersonation token, the
	/// function directs the message box to the session specified in the
	/// impersonation token.
	///
	/// If this flag is set, the `hWnd` parameter must be NULL. This is so that
	/// the message box can appear on a desktop other than the desktop
	/// corresponding to the `hWnd`.
	///
	/// For information on security considerations in regard to using this flag,
	/// see
	/// [Interactive Services](https://docs.microsoft.com/en-us/windows/win32/services/interactive-services).
	/// In particular, be aware that this flag can produce interactive content
	/// on a locked desktop and should therefore be used for only a very limited
	/// set of scenarios, such as resource exhaustion.
	SERVICE_NOTIFICATION, 0x0020_0000
}

pub_struct_const! { MBC, u32,
	/// [`MultiByteToWideChar`](crate::MultiByteToWideChar) `dwFlags` (`u32`).
	/// Originally has `MB` prefix.
	=>
	COMPOSITE, 0x0000_0002
	ERR_INVALID_CHARS, 0x0000_0008
	PRECOMPOSED, 0x0000_0001
	USEGLYPHCHARS, 0x0000_0004
}

pub_struct_const_wm! { MCM,
	/// Month calendar control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-month-calendar-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	FIRST, 0x1000
	=>
	GETCURSEL, Self::FIRST.0 + 1
	SETCURSEL, Self::FIRST.0 + 2
	GETMAXSELCOUNT, Self::FIRST.0 + 3
	SETMAXSELCOUNT, Self::FIRST.0 + 4
	GETSELRANGE, Self::FIRST.0 + 5
	SETSELRANGE, Self::FIRST.0 + 6
	GETMONTHRANGE, Self::FIRST.0 + 7
	SETDAYSTATE, Self::FIRST.0 + 8
	GETMINREQRECT, Self::FIRST.0 + 9
	SETCOLOR, Self::FIRST.0 + 10
	GETCOLOR, Self::FIRST.0 + 11
	SETTODAY, Self::FIRST.0 + 12
	GETTODAY, Self::FIRST.0 + 13
	HITTEST, Self::FIRST.0 + 14
	SETFIRSTDAYOFWEEK, Self::FIRST.0 + 15
	GETFIRSTDAYOFWEEK, Self::FIRST.0 + 16
	GETRANGE, Self::FIRST.0 + 17
	SETRANGE, Self::FIRST.0 + 18
	GETMONTHDELTA, Self::FIRST.0 + 19
	SETMONTHDELTA, Self::FIRST.0 + 20
	GETMAXTODAYWIDTH, Self::FIRST.0 + 21
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
	GETCURRENTVIEW, Self::FIRST.0 + 22
	GETCALENDARCOUNT, Self::FIRST.0 + 23
	GETCALENDARGRIDINFO, Self::FIRST.0 + 24
	GETCALID, Self::FIRST.0 + 27
	SETCALID, Self::FIRST.0 + 28
	SIZERECTTOMIN, Self::FIRST.0 + 29
	SETCALENDARBORDER, Self::FIRST.0 + 30
	GETCALENDARBORDER, Self::FIRST.0 + 31
	SETCURRENTVIEW, Self::FIRST.0 + 32
}

pub_struct_const! { MCMV, u32,
	/// [`NMVIEWCHANGE`](crate::NMVIEWCHANGE) `dwOldView` and `dwNewView` (`u32`).
	=>
	MONTH, 0
	YEAR, 1
	DECADE, 2
	CENTURY, 3
}

pub_struct_const_nm! { MCN,
	/// Month calendar control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-month-calendar-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -746
	=>
	SELECT, Self::FIRST.0
	GETDAYSTATE, Self::FIRST.0 - 1
	SELCHANGE, Self::FIRST.0 - 3
	VIEWCHANGE, Self::FIRST.0 - 4
}

pub_struct_const! { MCSC, u8,
	/// [`DTM_GETMCCOLOR`](crate::msg::dtm::GetMcColor) color (`u8`).
	=>
	BACKGROUND, 0
	TEXT, 1
	TITLEBK, 2
	TITLETEXT, 3
	MONTHBK, 4
	TRAILINGTEXT, 5
}

pub_struct_const_ws! { MCS,
	/// Month calendar control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/month-calendar-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	NONE, 0
	DAYSTATE, 0x0001
	MULTISELECT, 0x0002
	WEEKNUMBERS, 0x0004
	NOTODAYCIRCLE, 0x0008
	NOTODAY, 0x0010
	NOTRAILINGDATES, 0x0040
	SHORTDAYSOFWEEK, 0x0080
	NOSELCHANGEONNAV, 0x0100
}

pub_struct_const! { MF, u32,
	/// [`AppendMenu`](crate::HMENU::AppendMenu) and
	/// [`InsertMenu`](crate::HMENU::InsertMenu) `uFlags` (`u32`).
	=>
	INSERT, 0x0000_0000
	CHANGE, 0x0000_0080
	APPEND, 0x0000_0100
	DELETE, 0x0000_0200
	REMOVE, 0x0000_1000
	BYCOMMAND, 0x0000_0000
	BYPOSITION, 0x0000_0400
	SEPARATOR, 0x0000_0800
	ENABLED, 0x0000_0000
	GRAYED, 0x0000_0001
	DISABLED, 0x0000_0002
	UNCHECKED, 0x0000_0000
	CHECKED, 0x0000_0008
	USECHECKBITMAPS, 0x0000_0200
	STRING, 0x0000_0000
	BITMAP, 0x0000_0004
	OWNERDRAW, 0x0000_0100
	POPUP, 0x0000_0010
	MENUBARBREAK, 0x0000_0020
	MENUBREAK, 0x0000_0040
	UNHILITE, 0x0000_0000
	HILITE, 0x0000_0080
	DEFAULT, 0x0000_1000
	SYSMENU, 0x0000_2000
	HELP, 0x0000_4000
	RIGHTJUSTIFY, 0x0000_4000
	MOUSESELECT, 0x0000_8000
}

pub_struct_const! { MFS, u32,
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fState` (`u32`).
	=>
	GRAYED, 0x0000_0003
	DISABLED, MFS::GRAYED.0
	CHECKED, MF::CHECKED.0
	HILITE, MF::HILITE.0
	ENABLED, MF::ENABLED.0
	UNCHECKED, MF::UNCHECKED.0
	UNHILITE, MF::UNHILITE.0
	DEFAULT, MF::DEFAULT.0
}

pub_struct_const! { MFT, u32,
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fType` (`u32`).
	=>
	STRING, MF::STRING.0
	BITMAP, MF::BITMAP.0
	MENUBARBREAK, MF::MENUBARBREAK.0
	MENUBREAK, MF::MENUBREAK.0
	OWNERDRAW, MF::OWNERDRAW.0
	RADIOCHECK, 0x00000200
	SEPARATOR, MF::SEPARATOR.0
	RIGHTORDER, 0x00002000
	RIGHTJUSTIFY, MF::RIGHTJUSTIFY.0
}

pub_struct_const! { MIM, u32,
	/// [`MENUINFO`](crate::MENUINFO) `fMask` (`u32`).
	=>
	MAXHEIGHT, 0x0000_0001
	BACKGROUND, 0x0000_0002
	HELPID, 0x0000_0004
	MENUDATA, 0x0000_0008
	STYLE, 0x0000_0010
	APPLYTOSUBMENUS, 0x8000_0000
}

pub_struct_const! { MIIM, u32,
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fMask` (`u32`).
	=>
	MAXHEIGHT, 0x0000_0001
	BACKGROUND, 0x0000_0002
	HELPID, 0x0000_0004
	MENUDATA, 0x0000_0008
	STYLE, 0x0000_0010
	APPLYTOSUBMENUS, 0x8000_0000
}

pub_struct_const! { MK, u16,
	/// [`WM_LBUTTONDOWN`](crate::msg::wm::LButtonDown) (and similar) virtual
	/// keys (`u16`).
	=>
	LBUTTON, 0x0001
	RBUTTON, 0x0002
	SHIFT, 0x0004
	CONTROL, 0x0008
	MBUTTON, 0x0010
	XBUTTON1, 0x0020
	XBUTTON2, 0x0040
}

pub_struct_const! { MND, u8,
	/// [`WM_MENUDRAG`](crate::msg::wm::MenuDrag) return value.
	=>
	CONTINUE, 0
	ENDMENU, 1
}

pub_struct_const! { MNS, u32,
	/// [`MENUINFO`](crate::MENUINFO) `dwStyle` (`u32`).
	=>
	NOCHECK, 0x8000_0000
	MODELESS, 0x4000_0000
	DRAGDROP, 0x2000_0000
	AUTODISMISS, 0x1000_0000
	NOTIFYBYPOS, 0x0800_0000
	CHECKORBMP, 0x0400_0000
}

pub_struct_const! { MSGF, u8,
	/// [`WM_ENTERIDLE`](crate::msg::wm::EnterIdle) reason (`u8`).
	=>
	DIALOGBOX, 0
	MENU, 2
}

pub_struct_const! { NIF, u32,
	/// [`NOTIFYICONDATA`](crate::NOTIFYICONDATA) `uFlags` (`u32`).
	=>
	MESSAGE, 0x0000_0001
	ICON, 0x0000_0002
	TIP, 0x0000_0004
	STATE, 0x0000_0008
	INFO, 0x0000_0010
	GUID, 0x0000_0020
	REALTIME, 0x0000_0040
	SHOWTIP, 0x0000_0080
}

pub_struct_const! { NIIF, u32,
	/// [`NOTIFYICONDATA`](crate::NOTIFYICONDATA) `dwInfoFlags` (`u32`).
	=>
	NONE, 0x0000_0000
	INFO, 0x0000_0001
	WARNING, 0x0000_0002
	ERROR, 0x0000_0003
	USER, 0x0000_0004
	NOSOUND, 0x0000_0010
	LARGE_ICON, 0x0000_0020
	RESPECT_QUIET_TIME, 0x0000_0080
}

pub_struct_const! { NIM, u32,
	/// [`Shell_NotifyIcon`](crate::Shell_NotifyIcon) `dwMessage` (`u32`).
	=>
	ADD, 0x0000_0000
	MODIFY, 0x0000_0001
	DELETE, 0x0000_0002
	SETFOCUS, 0x0000_0003
	SETVERSION, 0x0000_0004
}

pub_struct_const! { NIS, u32,
	/// [`NOTIFYICONDATA`](crate::NOTIFYICONDATA) `dwState` and `dwStateFlags`
	/// (`u32`).
	=>
	HIDDEN, 0x0000_0001
	SHAREDICON, 0x0000_0002
}

pub_struct_const! { NM, i32,
	/// [`WM_NOTIFY`](crate::msg::wm::Notify) notification codes (`i32`).
	///
	/// Control-specific notification codes have their own types, which are
	/// convertible to `NM`.
	=>
	OUTOFMEMORY, Self::FIRST.0 - 1
	CLICK, Self::FIRST.0 - 2
	DBLCLK, Self::FIRST.0 - 3
	RETURN, Self::FIRST.0 - 4
	RCLICK, Self::FIRST.0 - 5
	RDBLCLK, Self::FIRST.0 - 6
	SETFOCUS, Self::FIRST.0 - 7
	KILLFOCUS, Self::FIRST.0 - 8
	CUSTOMDRAW, Self::FIRST.0 - 12
	HOVER, Self::FIRST.0 - 13
	NCHITTEST, Self::FIRST.0 - 14
	KEYDOWN, Self::FIRST.0 - 15
	RELEASEDCAPTURE, Self::FIRST.0 - 16
	SETCURSOR, Self::FIRST.0 - 17
	CHAR, Self::FIRST.0 - 18
	TOOLTIPSCREATED, Self::FIRST.0 - 19
	LDOWN, Self::FIRST.0 - 20
	RDOWN, Self::FIRST.0 - 21
	THEMECHANGED, Self::FIRST.0 - 22
}
impl_const_values! { NM,
	FIRST, 0
}
