const_type! { WA, u16,
	/// [`WM_ACTIVATE`](crate::msg::wm::Activate)
	/// activation state (`u16`).
	->
	INACTIVE, 0
	ACTIVE, 1
	CLICKACTIVE, 2
}

const_type! { WDA, u32,
	/// [`GetWindowDisplayAffinity`](crate::HWND::GetWindowDisplayAffinity) and
	/// [`SetWindowDisplayAffinity`](crate::HWND::SetWindowDisplayAffinity)
	/// `dwAffinity` (`u32`).
	->
	NONE, 0x00000000
	MONITOR, 0x00000001
	EXCLUDEFROMCAPTURE, 0x00000011
}

const_type! { WH, i32,
	/// [`SetWindowsHookEx`](crate::HHOOK::SetWindowsHookEx) `idHook` (`i32`).
	->
	MSGFILTER, -1
	JOURNALRECORD, 0
	JOURNALPLAYBACK, 1
	KEYBOARD, 2
	GETMESSAGE, 3
	CALLWNDPROC, 4
	CBT, 5
	SYSMSGFILTER, 6
	MOUSE, 7
	DEBUG, 9
	SHELL, 10
	FOREGROUNDIDLE, 11
	CALLWNDPROCRET, 12
	KEYBOARD_LL, 13
	MOUSE_LL, 14
}

const_type! { WIN32, u16,
	/// [`_WIN32`](https://docs.microsoft.com/en-us/windows/win32/winprog/using-the-windows-headers)
	/// version definitions (`u16`).
	->
	WINNT_NT4, 0x0400
	WINNT_WIN2K, 0x0500
	WINNT_WINXP, 0x0501
	WINNT_WS03, 0x0502
	WINNT_WIN6, 0x0600
	WINNT_VISTA, 0x0600
	WINNT_WS08, 0x0600
	WINNT_LONGHORN, 0x0600
	WINNT_WIN7, 0x0601
	WINNT_WIN8, 0x0602
	WINNT_WINBLUE, 0x0603
	WINNT_WINTHRESHOLD, 0x0a00
	WINNT_WIN10, 0x0a00

	IE_IE20, 0x0200
	IE_IE30, 0x0300
	IE_IE302, 0x0302
	IE_IE40, 0x0400
	IE_IE401, 0x0401
	IE_IE50, 0x0500
	IE_IE501, 0x0501
	IE_IE55, 0x0550
	IE_IE60, 0x0600
	IE_IE60SP1, 0x0601
	IE_IE60SP2, 0x0603
	IE_IE70, 0x0700
	IE_IE80, 0x0800
	IE_IE90, 0x0900
	IE_IE100, 0x0A00
	IE_IE110, 0x0A00

	IE_NT4, Self::IE_IE20.0
	IE_NT4SP1, Self::IE_IE20.0
	IE_NT4SP2, Self::IE_IE20.0
	IE_NT4SP3, Self::IE_IE302.0
	IE_NT4SP4, Self::IE_IE401.0
	IE_NT4SP5, Self::IE_IE401.0
	IE_NT4SP6, Self::IE_IE50.0
	IE_WIN98, Self::IE_IE401.0
	IE_WIN98SE, Self::IE_IE50.0
	IE_WINME, Self::IE_IE55.0
	IE_WIN2K, Self::IE_IE501.0
	IE_WIN2KSP1, Self::IE_IE501.0
	IE_WIN2KSP2, Self::IE_IE501.0
	IE_WIN2KSP3, Self::IE_IE501.0
	IE_WIN2KSP4, Self::IE_IE501.0
	IE_XP, Self::IE_IE60.0
	IE_XPSP1, Self::IE_IE60SP1.0
	IE_XPSP2, Self::IE_IE60SP2.0
	IE_WS03, 0x0602
	IE_WS03SP1, Self::IE_IE60SP2.0
	IE_WIN6, Self::IE_IE70.0
	IE_LONGHORN, Self::IE_IE70.0
	IE_WIN7, Self::IE_IE80.0
	IE_WIN8, Self::IE_IE100.0
	IE_WINBLUE, Self::IE_IE100.0
	IE_WINTHRESHOLD, Self::IE_IE110.0
	IE_WIN10, Self::IE_IE110.0
}

const_type! { WM, u32,
	/// Window message codes (`u32`).
	///
	/// Control-specific messages codes have their own types, which are
	/// convertible to `WM`.
	->
	NULL, 0x0000
	CREATE, 0x0001
	DESTROY, 0x0002
	MOVE, 0x0003
	SIZE, 0x0005
	ACTIVATE, 0x0006
	SETFOCUS, 0x0007
	KILLFOCUS, 0x0008
	ENABLE, 0x000a
	SETREDRAW, 0x000b
	SETTEXT, 0x000c
	GETTEXT, 0x000d
	GETTEXTLENGTH, 0x000e
	PAINT, 0x000f
	CLOSE, 0x0010
	QUERYENDSESSION, 0x0011
	QUERYOPEN, 0x0013
	ENDSESSION, 0x0016
	QUIT, 0x0012
	ERASEBKGND, 0x0014
	SYSCOLORCHANGE, 0x0015
	SHOWWINDOW, 0x0018
	WININICHANGE, 0x001a
	DEVMODECHANGE, 0x001b
	ACTIVATEAPP, 0x001c
	FONTCHANGE, 0x001d
	TIMECHANGE, 0x001e
	CANCELMODE, 0x001f
	SETCURSOR, 0x0020
	MOUSEACTIVATE, 0x0021
	CHILDACTIVATE, 0x0022
	QUEUESYNC, 0x0023
	GETMINMAXINFO, 0x0024
	PAINTICON, 0x0026
	ICONERASEBKGND, 0x0027
	NEXTDLGCTL, 0x0028
	SPOOLERSTATUS, 0x002a
	DRAWITEM, 0x002b
	MEASUREITEM, 0x002c
	DELETEITEM, 0x002d
	VKEYTOITEM, 0x002e
	CHARTOITEM, 0x002f
	SETFONT, 0x0030
	GETFONT, 0x0031
	SETHOTKEY, 0x0032
	GETHOTKEY, 0x0033
	QUERYDRAGICON, 0x0037
	COMPAREITEM, 0x0039
	GETOBJECT, 0x003d
	COPYDATA, 0x004a
	COMPACTING, 0x0041
	COMMNOTIFY, 0x0044
	WINDOWPOSCHANGING, 0x0046
	WINDOWPOSCHANGED, 0x0047
	POWER, 0x0048
	NOTIFY, 0x004e
	INPUTLANGCHANGEREQUEST, 0x0050
	INPUTLANGCHANGE, 0x0051
	TCARD, 0x0052
	HELP, 0x0053
	USERCHANGED, 0x0054
	NOTIFYFORMAT, 0x0055
	CONTEXTMENU, 0x007b
	STYLECHANGING, 0x007c
	STYLECHANGED, 0x007d
	DISPLAYCHANGE, 0x007e
	GETICON, 0x007f
	SETICON, 0x0080
	NCCREATE, 0x0081
	NCDESTROY, 0x0082
	NCCALCSIZE, 0x0083
	NCHITTEST, 0x0084
	NCPAINT, 0x0085
	NCACTIVATE, 0x0086
	GETDLGCODE, 0x0087
	SYNCPAINT, 0x0088
	NCMOUSEMOVE, 0x00a0
	NCLBUTTONDOWN, 0x00a1
	NCLBUTTONUP, 0x00a2
	NCLBUTTONDBLCLK, 0x00a3
	NCRBUTTONDOWN, 0x00a4
	NCRBUTTONUP, 0x00a5
	NCRBUTTONDBLCLK, 0x00a6
	NCMBUTTONDOWN, 0x00a7
	NCMBUTTONUP, 0x00a8
	NCMBUTTONDBLCLK, 0x00a9
	NCXBUTTONDOWN, 0x00ab
	NCXBUTTONUP, 0x00ac
	NCXBUTTONDBLCLK, 0x00ad
	INPUT_DEVICE_CHANGE, 0x00fe
	INPUT, 0x00ff
	KEYFIRST, 0x0100
	KEYDOWN, 0x0100
	KEYUP, 0x0101
	CHAR, 0x0102
	DEADCHAR, 0x0103
	SYSKEYDOWN, 0x0104
	SYSKEYUP, 0x0105
	SYSCHAR, 0x0106
	SYSDEADCHAR, 0x0107
	UNICHAR, 0x0109
	KEYLAST, 0x0109
	IME_STARTCOMPOSITION, 0x010d
	IME_ENDCOMPOSITION, 0x010e
	IME_COMPOSITION, 0x010f
	IME_KEYLAST, 0x010f
	INITDIALOG, 0x0110
	COMMAND, 0x0111
	SYSCOMMAND, 0x0112
	TIMER, 0x0113
	HSCROLL, 0x0114
	VSCROLL, 0x0115
	INITMENU, 0x0116
	INITMENUPOPUP, 0x0117
	GESTURE, 0x0119
	GESTURENOTIFY, 0x011a
	MENUSELECT, 0x011f
	MENUCHAR, 0x0120
	ENTERIDLE, 0x0121
	MENURBUTTONUP, 0x0122
	MENUDRAG, 0x0123
	MENUGETOBJECT, 0x0124
	UNINITMENUPOPUP, 0x0125
	MENUCOMMAND, 0x0126
	CHANGEUISTATE, 0x0127
	UPDATEUISTATE, 0x0128
	QUERYUISTATE, 0x0129
	CTLCOLORMSGBOX, 0x0132
	CTLCOLOREDIT, 0x0133
	CTLCOLORLISTBOX, 0x0134
	CTLCOLORBTN, 0x0135
	CTLCOLORDLG, 0x0136
	CTLCOLORSCROLLBAR, 0x0137
	CTLCOLORSTATIC, 0x0138
	MN_GETHMENU, 0x01e1
	MOUSEFIRST, 0x0200
	MOUSEMOVE, 0x0200
	LBUTTONDOWN, 0x0201
	LBUTTONUP, 0x0202
	LBUTTONDBLCLK, 0x0203
	RBUTTONDOWN, 0x0204
	RBUTTONUP, 0x0205
	RBUTTONDBLCLK, 0x0206
	MBUTTONDOWN, 0x0207
	MBUTTONUP, 0x0208
	MBUTTONDBLCLK, 0x0209
	MOUSEHWHEEL, 0x020e
	XBUTTONDOWN, 0x020b
	XBUTTONUP, 0x020c
	XBUTTONDBLCLK, 0x020d
	MOUSELAST, 0x020e
	PARENTNOTIFY, 0x0210
	ENTERMENULOOP, 0x0211
	EXITMENULOOP, 0x0212
	NEXTMENU, 0x0213
	SIZING, 0x0214
	CAPTURECHANGED, 0x0215
	MOVING, 0x0216
	POWERBROADCAST, 0x0218
	DEVICECHANGE, 0x0219
	MDICREATE, 0x0220
	MDIDESTROY, 0x0221
	MDIACTIVATE, 0x0222
	MDIRESTORE, 0x0223
	MDINEXT, 0x0224
	MDIMAXIMIZE, 0x0225
	MDITILE, 0x0226
	MDICASCADE, 0x0227
	MDIICONARRANGE, 0x0228
	MDIGETACTIVE, 0x0229
	MDISETMENU, 0x0230
	ENTERSIZEMOVE, 0x0231
	EXITSIZEMOVE, 0x0232
	DROPFILES, 0x0233
	MDIREFRESHMENU, 0x0234
	POINTERDEVICECHANGE, 0x0238
	POINTERDEVICEINRANGE, 0x0239
	POINTERDEVICEOUTOFRANGE, 0x023a
	TOUCH, 0x0240
	NCPOINTERUPDATE, 0x0241
	NCPOINTERDOWN, 0x0242
	NCPOINTERUP, 0x0243
	POINTERUPDATE, 0x0245
	POINTERDOWN, 0x0246
	POINTERUP, 0x0247
	POINTERENTER, 0x0249
	POINTERLEAVE, 0x024a
	POINTERACTIVATE, 0x024b
	POINTERCAPTURECHANGED, 0x024c
	TOUCHHITTESTING, 0x024d
	POINTERWHEEL, 0x024e
	POINTERHWHEEL, 0x024f
	DM_POINTERHITTEST, 0x0250
	POINTERROUTEDTO, 0x0251
	POINTERROUTEDAWAY, 0x0252
	POINTERROUTEDRELEASED, 0x0253
	IME_SETCONTEXT, 0x0281
	IME_NOTIFY, 0x0282
	IME_CONTROL, 0x0283
	IME_COMPOSITIONFULL, 0x0284
	IME_SELECT, 0x0285
	IME_CHAR, 0x0286
	IME_REQUEST, 0x0288
	IME_KEYDOWN, 0x0290
	IME_KEYUP, 0x0291
	MOUSEHOVER, 0x02a1
	MOUSELEAVE, 0x02a3
	NCMOUSEHOVER, 0x02a0
	NCMOUSELEAVE, 0x02a2
	WTSSESSION_CHANGE, 0x02b1
	TABLET_FIRST, 0x02c0
	TABLET_LAST, 0x02df
	DPICHANGED, 0x02e0
	DPICHANGED_BEFOREPARENT, 0x02e2
	DPICHANGED_AFTERPARENT, 0x02e3
	GETDPISCALEDSIZE, 0x02e4
	CUT, 0x0300
	COPY, 0x0301
	PASTE, 0x0302
	CLEAR, 0x0303
	UNDO, 0x0304
	RENDERFORMAT, 0x0305
	RENDERALLFORMATS, 0x0306
	DESTROYCLIPBOARD, 0x0307
	DRAWCLIPBOARD, 0x0308
	PAINTCLIPBOARD, 0x0309
	VSCROLLCLIPBOARD, 0x030a
	SIZECLIPBOARD, 0x030b
	ASKCBFORMATNAME, 0x030c
	CHANGECBCHAIN, 0x030d
	HSCROLLCLIPBOARD, 0x030e
	QUERYNEWPALETTE, 0x030f
	PALETTEISCHANGING, 0x0310
	PALETTECHANGED, 0x0311
	HOTKEY, 0x0312
	PRINT, 0x0317
	PRINTCLIENT, 0x0318
	APPCOMMAND, 0x0319
	THEMECHANGED, 0x031a
	CLIPBOARDUPDATE, 0x031d
	DWMCOMPOSITIONCHANGED, 0x031e
	DWMNCRENDERINGCHANGED, 0x031f
	DWMCOLORIZATIONCOLORCHANGED, 0x0320
	DWMWINDOWMAXIMIZEDCHANGE, 0x0321
	DWMSENDICONICTHUMBNAIL, 0x0323
	DWMSENDICONICLIVEPREVIEWBITMAP, 0x0326
	GETTITLEBARINFOEX, 0x033f
	HANDHELDFIRST, 0x0358
	HANDHELDLAST, 0x035f
	AFXFIRST, 0x0360
	AFXLAST, 0x037f
	PENWINFIRST, 0x0380
	PENWINLAST, 0x038f
	APP, 0x8000
	USER, 0x0400
}

const_type! { WMPN, u16,
	/// [`WM_PARENTNOFITY`](crate::msg::wm::ParentNotify) event (`u16`).
	->
	CREATE, WM::CREATE.0 as u16
	DESTROY, WM::DESTROY.0 as u16
	LBUTTONDOWN, WM::LBUTTONDOWN.0 as u16
	MBUTTONDOWN, WM::MBUTTONDOWN.0 as u16
	RBUTTONDOWN, WM::RBUTTONDOWN.0 as u16
	XBUTTONDOWN, WM::XBUTTONDOWN.0 as u16
	POINTERDOWN, WM::POINTERDOWN.0 as u16
}

const_type! { WMSZ, u8,
	/// [`WM_SIZING`](crate::msg::wm::Sizing) window edge (`u8`).
	->
	LEFT, 1
	RIGHT, 2
	TOP, 3
	TOPLEFT, 4
	TOPRIGHT, 5
	BOTTOM, 6
	BOTTOMLEFT, 7
	BOTTOMRIGHT, 8
}

const_type! { WPF, u32,
	/// [`WINDOWPLACEMENT`](crate::WINDOWPLACEMENT) `flags` (`u32`).
	->
	SETMINPOSITION, 0x0001
	RESTORETOMAXIMIZED, 0x0002
	ASYNCWINDOWPLACEMENT, 0x0004
}

const_type! { WS, u32,
	/// Window
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles)
	/// (`u32`).
	->
	/// The window is an overlapped window. An overlapped window has a title bar
	/// and a border. Same as the `WS_TILED` style.
	OVERLAPPED, 0x00000000
	/// The window is a pop-up window. This style cannot be used with the
	/// `WS_CHILD` style.
	POPUP, 0x80000000
	/// The window is a child window. A window with this style cannot have a menu
	/// bar. This style cannot be used with the `WS_POPUP` style.
	CHILD, 0x40000000
	/// The window is initially minimized. Same as the `WS_ICONIC` style.
	MINIMIZE, 0x20000000
	/// The window is initially visible. This style can be turned on and off by
	/// using the [`ShowWindow`](crate::HWND::ShowWindow) or
	/// [`SetWindowPos`](crate::HWND::SetWindowPos) function.
	VISIBLE, 0x10000000
	/// The window is initially disabled. A disabled window cannot receive input
	/// from the user. To change this after a window has been created, use the
	/// [`EnableWindow`](crate::HWND::EnableWindow) function.
	DISABLED, 0x08000000
	/// Clips child windows relative to each other; that is, when a particular
	/// child window receives a [`WM_PAINT`](crate::msg::wm::Paint) message, the
	/// `WS_CLIPSIBLINGS` style clips all other overlapping child windows out of
	/// the region of the child window to be updated. If `WS_CLIPSIBLINGS` is not
	/// specified and child windows overlap, it is possible, when drawing within
	/// the client area of a child window, to draw within the client area of a
	/// neighboring child window.
	CLIPSIBLINGS, 0x04000000
	/// Excludes the area occupied by child windows when drawing occurs within
	/// the parent window. This style is used when creating the parent window.
	CLIPCHILDREN, 0x02000000
	/// The window is initially maximized.
	MAXIMIZE, 0x01000000
	/// The window has a title bar (includes the `WS_BORDER` style).
	CAPTION, 0x00c00000
	/// The window has a thin-line border.
	BORDER, 0x00800000
	/// The window has a border of a style typically used with dialog boxes. A
	/// window with this style cannot have a title bar.
	DLGFRAME, 0x00400000
	/// The window has a vertical scroll bar.
	VSCROLL, 0x00200000
	/// The window has a horizontal scroll bar.
	HSCROLL, 0x00100000
	/// The window has a window menu on its title bar. The `WS_CAPTION` style
	/// must also be specified.
	SYSMENU, 0x00080000
	/// The window has a sizing border. Same as the `WS_SIZEBOX` style.
	THICKFRAME, 0x00040000
	/// The window is the first control of a group of controls. The group
	/// consists of this first control and all controls defined after it, up to
	/// the next control with the `WS_GROUP` style. The first control in each
	/// group usually has the `WS_TABSTOP` style so that the user can move from
	/// group to group. The user can subsequently change the keyboard focus from
	/// one control in the group to the next control in the group by using the
	/// direction keys.
	///
	/// You can turn this style on and off to change dialog box navigation. To
	/// change this style after a window has been created, use the
	/// [`SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) function.
	GROUP, 0x00020000
	/// The window is a control that can receive the keyboard focus when the user
	/// presses the TAB key. Pressing the TAB key changes the keyboard focus to
	/// the next control with the `WS_TABSTOP` style.
	///
	/// You can turn this style on and off to change dialog box navigation. To
	/// change this style after a window has been created, use the
	/// [`SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) function.
	/// For user-created windows and modeless dialogs to work with tab stops,
	/// alter the message loop to call the
	/// [`IsDialogMessage`](crate::HWND::IsDialogMessage) function.
	TABSTOP, 0x00010000
	/// The window has a minimize button. Cannot be combined with the
	/// [`WS_EX_CONTEXTHELP`](crate::co::WS_EX::CONTEXTHELP) style. The
	/// `WS_SYSMENU` style must also be specified.
	MINIMIZEBOX, 0x00020000
	/// The window has a maximize button. Cannot be combined with the
	/// [`WS_EX_CONTEXTHELP`](crate::co::WS_EX::CONTEXTHELP) style. The
	/// `WS_SYSMENU` style must also be specified.
	MAXIMIZEBOX, 0x00010000
	/// The window is an overlapped window. An overlapped window has a title bar
	/// and a border. Same as the `WS_OVERLAPPED` style.
	TILED, Self::OVERLAPPED.0
	/// The window is initially minimized. Same as the `WS_MINIMIZE` style.
	ICONIC, Self::MINIMIZE.0
	/// The window has a sizing border. Same as the `WS_THICKFRAME` style.
	SIZEBOX, Self::THICKFRAME.0
	/// The window is an overlapped window. Same as the `WS_OVERLAPPEDWINDOW`
	/// style.
	TILEDWINDOW, Self::OVERLAPPEDWINDOW.0
	/// The window is an overlapped window. Same as the `WS_TILEDWINDOW` style.
	OVERLAPPEDWINDOW, Self::OVERLAPPED.0 | Self::CAPTION.0 | Self::SYSMENU.0 | Self::THICKFRAME.0 | Self::MINIMIZEBOX.0 | Self::MAXIMIZEBOX.0
	/// The window is a pop-up window. This style cannot be used with the
	/// `WS_CHILD` style.
	POPUPWINDOW, Self::POPUP.0 | Self::BORDER.0 | Self::SYSMENU.0
	/// Same as the `WS_CHILD` style.
	CHILDWINDOW, Self::CHILD.0
}

const_type! { WS_EX, u32,
	/// Extended window
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles)
	/// (`u32`).
	->
	/// The window has a double border; the window can, optionally, be created
	/// with a title bar by specifying the [`WS_CAPTION`](crate::co::WS::CAPTION)
	/// style in the dwStyle parameter.
	DLGMODALFRAME, 0x00000001
	/// The child window created with this style does not send the
	/// [`WM_PARENTNOTIFY`](crate::msg::wm::ParentNotify) message to its parent
	/// window when it is created or destroyed.
	NOPARENTNOTIFY, 0x00000004
	/// The window should be placed above all non-topmost windows and should stay
	/// above them, even when the window is deactivated. To add or remove this
	/// style, use the [`SetWindowPos`](crate::HWND::SetWindowPos) function.
	TOPMOST, 0x00000008
	/// The window accepts drag-drop files.
	ACCEPTFILES, 0x00000010
	/// The window should not be painted until siblings beneath the window (that
	/// were created by the same thread) have been painted. The window appears
	/// transparent because the bits of underlying sibling windows have already
	/// been painted.
	///
	/// To achieve transparency without these restrictions, use the
	/// [`SetWindowRgn`](crate::HWND::SetWindowRgn) function.
	TRANSPARENT, 0x00000020
	/// The window is a MDI child window.
	MDICHILD, 0x00000040
	/// The window is intended to be used as a floating toolbar. A tool window
	/// has a title bar that is shorter than a normal title bar, and the window
	/// title is drawn using a smaller font. A tool window does not appear in the
	/// taskbar or in the dialog that appears when the user presses ALT+TAB. If a
	/// tool window has a system menu, its icon is not displayed on the title
	/// bar. However, you can display the system menu by right-clicking or by
	/// typing ALT+SPACE.
	TOOLWINDOW, 0x00000080
	/// The window has a border with a raised edge.
	WINDOWEDGE, 0x00000100
	/// The window has a border with a sunken edge.
	CLIENTEDGE, 0x00000200
	/// The title bar of the window includes a question mark. When the user
	/// clicks the question mark, the cursor changes to a question mark with a
	/// pointer. If the user then clicks a child window, the child receives a
	/// [`WM_HELP`](crate::msg::wm::Help) message. The child window should pass
	/// the message to the parent window procedure, which should call the
	/// [`WinHelp`](crate::HWND::WinHelp) function using the `HELP_WM_HELP`
	/// command. The Help application displays a pop-up window that typically
	/// contains help for the child window.
	///
	/// `WS_EX_CONTEXTHELP` cannot be used with the
	/// [`WS_MAXIMIZEBOX`](crate::co::WS::MAXIMIZEBOX) or
	/// [`WS_MINIMIZEBOX`](crate::co::WS::MINIMIZEBOX) styles.
	CONTEXTHELP, 0x00000400
	/// The window has generic "right-aligned" properties. This depends on the
	/// window class. This style has an effect only if the shell language is
	/// Hebrew, Arabic, or another language that supports reading-order
	/// alignment; otherwise, the style is ignored.
	///
	/// Using the `WS_EX_RIGHT` style for static or edit controls has the same
	/// effect as using the [`SS_RIGHT`](crate::co::SS::RIGHT) or
	/// [`ES_RIGHT`](crate::co::ES::RIGHT) style, respectively. Using this style
	/// with button controls has the same effect as using
	/// [`BS_RIGHT`](crate::co::BS::RIGHT) and
	/// [`BS_RIGHTBUTTON`](crate::co::BS::RIGHTBUTTON) styles.
	RIGHT, 0x00001000
	/// The window has generic left-aligned properties. This is the default.
	LEFT, 0x00000000
	/// If the shell language is Hebrew, Arabic, or another language that
	/// supports reading-order alignment, the window text is displayed using
	/// right-to-left reading-order properties. For other languages, the style is
	/// ignored.
	RTLREADING, 0x00002000
	/// The window text is displayed using left-to-right reading-order
	/// properties. This is the default.
	LTRREADING, 0x00000000
	/// If the shell language is Hebrew, Arabic, or another language that
	/// supports reading order alignment, the vertical scroll bar (if present) is
	/// to the left of the client area. For other languages, the style is ignored.
	LEFTSCROLLBAR, 0x00004000
	/// The vertical scroll bar (if present) is to the right of the client area.
	/// This is the default.
	RIGHTSCROLLBAR, 0x00000000
	/// The window itself contains child windows that should take part in dialog
	/// box navigation. If this style is specified, the dialog manager recurses
	/// into children of this window when performing navigation operations such
	/// as handling the TAB key, an arrow key, or a keyboard mnemonic.
	CONTROLPARENT, 0x00010000
	/// The window has a three-dimensional border style intended to be used for
	/// items that do not accept user input.
	STATICEDGE, 0x00020000
	/// Forces a top-level window onto the taskbar when the window is visible.
	APPWINDOW, 0x00040000
	/// The window is an overlapped window.
	OVERLAPPEDWINDOW, Self::WINDOWEDGE.0 | Self::CLIENTEDGE.0
	/// The window is palette window, which is a modeless dialog box that
	/// presents an array of commands.
	PALETTEWINDOW, Self::WINDOWEDGE.0 | Self::TOOLWINDOW.0 | Self::TOPMOST.0
	/// The window is a layered window. This style cannot be used if the window
	/// has a class style of either [`CS_OWNDC`](crate::co::CS::OWNDC) or
	/// [`CS_CLASSDC`](crate::co::CS::CLASSDC).
	///
	/// Windows 8: The `WS_EX_LAYERED` style is supported for top-level windows
	/// and child windows. Previous Windows versions support `WS_EX_LAYERED` only
	/// for top-level windows.
	LAYERED, 0x00080000
	/// The window does not pass its window layout to its child windows.
	NOINHERITLAYOUT, 0x00100000
	/// The window does not render to a redirection surface. This is for windows
	/// that do not have visible content or that use mechanisms other than
	/// surfaces to provide their visual.
	NOREDIRECTIONBITMAP, 0x00200000
	/// If the shell language is Hebrew, Arabic, or another language that
	/// supports reading order alignment, the horizontal origin of the window is
	/// on the right edge. Increasing horizontal values advance to the left.
	LAYOUTRTL, 0x00400000
	/// Paints all descendants of a window in bottom-to-top painting order using
	/// double-buffering. Bottom-to-top painting order allows a descendent window
	/// to have translucency (alpha) and transparency (color-key) effects, but
	/// only if the descendent window also has the `WS_EX_TRANSPARENT` bit set.
	/// Double-buffering allows the window and its descendents to be painted
	/// without flicker. This cannot be used if the window has a class style of
	/// either [`CS_OWNDC`](crate::co::CS::OWNDC) or
	/// [`CS_CLASSDC`](crate::co::CS::CLASSDC).
	///
	/// Windows 2000: This style is not supported.
	COMPOSITED, 0x02000000
	/// A top-level window created with this style does not become the foreground
	/// window when the user clicks it. The system does not bring this window to
	/// the foreground when the user minimizes or closes the foreground window.
	///
	/// The window should not be activated through programmatic access or via
	/// keyboard navigation by accessible technology, such as Narrator.
	///
	/// To activate the window, use the SetActiveWindow or
	/// [`SetForegroundWindow`](crate::HWND::SetForegroundWindow) function.
	///
	/// The window does not appear on the taskbar by default. To force the window
	/// to appear on the taskbar, use the `WS_EX_APPWINDOW` style.
	NOACTIVATE, 0x08000000
}

const_type! { WVR, u32,
	/// [`WM_NCCALCSIZE`](crate::msg::wm::NcCalcSize)
	/// return flags (`u32`).
	->
	ZERO, 0
	ALIGNTOP, 0x0010
	ALIGNLEFT, 0x0020
	ALIGNBOTTOM, 0x0040
	ALIGNRIGHT, 0x0080
	HREDRAW, 0x0100
	VREDRAW, 0x0200
	REDRAW, Self::HREDRAW.0 | Self::VREDRAW.0
	VALIDRECTS, 0x0400
}
