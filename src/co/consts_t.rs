use crate::co::{CCM, WM};

pub_struct_const! { TA, u32,
	/// [`SetTextAlign`](crate::HDC::SetTextAlign) `align` (`u32`). Also includes
	/// constants with `VTA` prefix.
	=>
	NOUPDATECP, 0
	UPDATECP, 1
	LEFT, 0
	RIGHT, 2
	CENTER, 6
	TOP, 0
	BOTTOM, 8
	BASELINE, 24
	RTLREADING, 256
}

pub_struct_const! { TB, i32,
	/// [`NMTRBTHUMBPOSCHANGING`](crate::NMTRBTHUMBPOSCHANGING) `nReason`
	/// (`i32`).
	=>
	LINEUP, 0
	LINEDOWN, 1
	PAGEUP, 2
	PAGEDOWN, 3
	THUMBPOSITION, 4
	THUMBTRACK, 5
	TOP, 6
	BOTTOM, 7
	ENDTRACK, 8
}

pub_struct_const_wm! { TBM,
	/// Toolbar control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-toolbar-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM). Originally has `TB`
	/// prefix.
	=>
	=>
	ADDBITMAP, WM::USER.0 + 19
	ADDBUTTONS, WM::USER.0 + 68
	ADDSTRING, WM::USER.0 + 77
	AUTOSIZE, WM::USER.0 + 33
	BUTTONCOUNT, WM::USER.0 + 24
	BUTTONSTRUCTSIZE, WM::USER.0 + 30
	CHANGEBITMAP, WM::USER.0 + 43
	CHECKBUTTON, WM::USER.0 + 2
	COMMANDTOINDEX, WM::USER.0 + 25
	CUSTOMIZE, WM::USER.0 + 27
	DELETEBUTTON, WM::USER.0 + 22
	ENABLEBUTTON, WM::USER.0 + 1
	GETANCHORHIGHLIGHT, WM::USER.0 + 74
	GETBITMAP, WM::USER.0 + 44
	GETBITMAPFLAGS, WM::USER.0 + 41
	GETBUTTON, WM::USER.0 + 23
	GETBUTTONINFO, WM::USER.0 + 63
	GETBUTTONSIZE, WM::USER.0 + 58
	GETBUTTONTEXT, WM::USER.0 + 75
	GETCOLORSCHEME, CCM::GETCOLORSCHEME.0
	GETDISABLEDIMAGELIST, WM::USER.0 + 55
	GETEXTENDEDSTYLE, WM::USER.0 + 85
	GETHOTIMAGELIST, WM::USER.0 + 53
	GETHOTITEM, WM::USER.0 + 71
	GETIDEALSIZE, WM::USER.0 + 99
	GETIMAGELIST, WM::USER.0 + 49
	GETIMAGELISTCOUNT, WM::USER.0 + 98
	GETINSERTMARK, WM::USER.0 + 79
	GETINSERTMARKCOLOR, WM::USER.0 + 89
	GETITEMDROPDOWNRECT, WM::USER.0 + 103
	GETITEMRECT, WM::USER.0 + 29
	GETMAXSIZE, WM::USER.0 + 83
	GETMETRICS, WM::USER.0 + 101
	GETOBJECT, WM::USER.0 + 62
	GETPADDING, WM::USER.0 + 86
	GETPRESSEDIMAGELIST, WM::USER.0 + 105
	GETRECT, WM::USER.0 + 51
	GETROWS, WM::USER.0 + 40
	GETSTATE, WM::USER.0 + 18
	GETSTRING, WM::USER.0 + 91
	GETSTYLE, WM::USER.0 + 57
	GETTEXTROWS, WM::USER.0 + 61
	GETTOOLTIPS, WM::USER.0 + 35
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
	HASACCELERATOR, WM::USER.0 + 95
	HIDEBUTTON, WM::USER.0 + 4
	HITTEST, WM::USER.0 + 69
	INDETERMINATE, WM::USER.0 + 5
	INSERTBUTTON, WM::USER.0 + 67
	INSERTMARKHITTEST, WM::USER.0 + 81
	ISBUTTONCHECKED, WM::USER.0 + 10
	ISBUTTONENABLED, WM::USER.0 + 9
	ISBUTTONHIDDEN, WM::USER.0 + 12
	ISBUTTONHIGHLIGHTED, WM::USER.0 + 14
	ISBUTTONINDETERMINATE, WM::USER.0 + 13
	ISBUTTONPRESSED, WM::USER.0 + 11
	LOADIMAGES, WM::USER.0 + 50
	MAPACCELERATOR, WM::USER.0 + 90
	MARKBUTTON, WM::USER.0 + 6
	MOVEBUTTON, WM::USER.0 + 82
	PRESSBUTTON, WM::USER.0 + 3
	REPLACEBITMAP, WM::USER.0 + 46
	SAVERESTORE, WM::USER.0 + 76
	SETANCHORHIGHLIGHT, WM::USER.0 + 73
	SETBITMAPSIZE, WM::USER.0 + 32
	SETBOUNDINGSIZE, WM::USER.0 + 93
	SETBUTTONINFO, WM::USER.0 + 64
	SETBUTTONSIZE, WM::USER.0 + 31
	SETBUTTONWIDTH, WM::USER.0 + 59
	SETCMDID, WM::USER.0 + 42
	SETCOLORSCHEME, CCM::SETCOLORSCHEME.0
	SETDISABLEDIMAGELIST, WM::USER.0 + 54
	SETDRAWTEXTFLAGS, WM::USER.0 + 70
	SETEXTENDEDSTYLE, WM::USER.0 + 84
	SETHOTIMAGELIST, WM::USER.0 + 52
	SETHOTITEM, WM::USER.0 + 72
	SETHOTITEM2, WM::USER.0 + 94
	SETIMAGELIST, WM::USER.0 + 48
	SETINDENT, WM::USER.0 + 47
	SETINSERTMARK, WM::USER.0 + 80
	SETINSERTMARKCOLOR, WM::USER.0 + 88
	SETLISTGAP, WM::USER.0 + 96
	SETMAXTEXTROWS, WM::USER.0 + 60
	SETMETRICS, WM::USER.0 + 102
	SETPADDING, WM::USER.0 + 87
	SETPARENT, WM::USER.0 + 37
	SETPRESSEDIMAGELIST, WM::USER.0 + 104
	SETREDRAWTEXTFLAGS, WM::USER.0 + 70
	SETROWS, WM::USER.0 + 39
	SETSTATE, WM::USER.0 + 17
	SETSTYLE, WM::USER.0 + 56
	SETTOOLTIPS, WM::USER.0 + 36
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	SETWINDOWTHEME, CCM::SETWINDOWTHEME.0
}

pub_struct_const_nm! { TBN,
	/// Toolbar control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-toolbar-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -700
	=>
	BEGINADJUST, Self::FIRST.0 - 3
	BEGINDRAG, Self::FIRST.0 - 1
	CUSTHELP, Self::FIRST.0 - 9
	DELETINGBUTTON, Self::FIRST.0 - 15
	DRAGOUT, Self::FIRST.0 - 14
	DRAGOVER, Self::FIRST.0 - 27
	DROPDOWN, Self::FIRST.0 - 10
	DUPACCELERATOR, Self::FIRST.0 - 25
	ENDADJUST, Self::FIRST.0 - 4
	ENDDRAG, Self::FIRST.0 - 2
	GETBUTTONINFO, Self::FIRST.0 - 20
	GETDISPINFO, Self::FIRST.0 - 17
	GETINFOTIP, Self::FIRST.0 - 19
	GETOBJECT, Self::FIRST.0 - 12
	HOTITEMCHANGE, Self::FIRST.0 - 13
	INITCUSTOMIZE, Self::FIRST.0 - 23
	MAPACCELERATOR, Self::FIRST.0 - 28
	QUERYDELETE, Self::FIRST.0 - 7
	QUERYINSERT, Self::FIRST.0 - 6
	RESET, Self::FIRST.0 - 5
	RESTORE, Self::FIRST.0 - 21
	SAVE, Self::FIRST.0 - 22
	TOOLBARCHANGE, Self::FIRST.0 - 8
	WRAPACCELERATOR, Self::FIRST.0 - 26
	WRAPHOTITEM, Self::FIRST.0 - 24
}

pub_struct_const_ws! { TBS,
	/// Trackbar control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/trackbar-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	AUTOTICKS, 0x0001
	VERT, 0x0002
	HORZ, 0x0000
	TOP, 0x0004
	BOTTOM, 0x0000
	LEFT, 0x0004
	RIGHT, 0x0000
	BOTH, 0x0008
	NOTICKS, 0x0010
	ENABLESELRANGE, 0x0020
	FIXEDLENGTH, 0x0040
	NOTHUMB, 0x0080
	TOOLTIPS, 0x0100
	REVERSED, 0x0200
	DOWNISLEFT, 0x0400
	NOTIFYBEFOREMOVE, 0x0800
	TRANSPARENTBKGND, 0x1000
}

pub_struct_const! { TCIS, u32,
	/// Tab control item
	/// [states](https://docs.microsoft.com/en-us/windows/win32/controls/tab-control-item-states)
	/// (`u32`).
	=>
	BUTTONPRESSED, 0x0001
	HIGHLIGHTED, 0x0002
}

pub_struct_const_wm! { TCM,
	/// Tab control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tab-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	FIRST, 0x1300
	=>
	GETIMAGELIST, Self::FIRST.0 + 2
	SETIMAGELIST, Self::FIRST.0 + 3
	GETITEMCOUNT, Self::FIRST.0 + 4
	GETITEM, Self::FIRST.0 + 60
	SETITEM, Self::FIRST.0 + 61
	INSERTITEM, Self::FIRST.0 + 62
	DELETEITEM, Self::FIRST.0 + 8
	DELETEALLITEMS, Self::FIRST.0 + 9
	GETITEMRECT, Self::FIRST.0 + 10
	GETCURSEL, Self::FIRST.0 + 11
	SETCURSEL, Self::FIRST.0 + 12
	HITTEST, Self::FIRST.0 + 13
	SETITEMEXTRA, Self::FIRST.0 + 14
	ADJUSTRECT, Self::FIRST.0 + 40
	SETITEMSIZE, Self::FIRST.0 + 41
	REMOVEIMAGE, Self::FIRST.0 + 42
	SETPADDING, Self::FIRST.0 + 43
	GETROWCOUNT, Self::FIRST.0 + 44
	GETTOOLTIPS, Self::FIRST.0 + 45
	SETTOOLTIPS, Self::FIRST.0 + 46
	GETCURFOCUS, Self::FIRST.0 + 47
	SETCURFOCUS, Self::FIRST.0 + 48
	SETMINTABWIDTH, Self::FIRST.0 + 49
	DESELECTALL, Self::FIRST.0 + 50
	HIGHLIGHTITEM, Self::FIRST.0 + 51
	SETEXTENDEDSTYLE, Self::FIRST.0 + 52
	GETEXTENDEDSTYLE, Self::FIRST.0 + 53
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
}

pub_struct_const_nm! { TCN,
	/// Tab control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tab-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -550
	=>
	FOCUSCHANGE, Self::FIRST.0 - 4
	GETOBJECT, Self::FIRST.0 - 3
	KEYDOWN, Self::FIRST.0 - 0
	SELCHANGE, Self::FIRST.0 - 1
	SELCHANGING, Self::FIRST.0 - 2
}

pub_struct_const_ws! { TCS,
	/// Tab control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/tab-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	SCROLLOPPOSITE, 0x0001
	BOTTOM, 0x0002
	RIGHT, 0x0002
	MULTISELECT, 0x0004
	FLATBUTTONS, 0x0008
	FORCEICONLEFT, 0x0010
	FORCELABELLEFT, 0x0020
	HOTTRACK, 0x0040
	VERTICAL, 0x0080
	TABS, 0x0000
	BUTTONS, 0x0100
	SINGLELINE, 0x0000
	MULTILINE, 0x0200
	RIGHTJUSTIFY, 0x0000
	FIXEDWIDTH, 0x0400
	RAGGEDRIGHT, 0x0800
	FOCUSONBUTTONDOWN, 0x1000
	OWNERDRAWFIXED, 0x2000
	TOOLTIPS, 0x4000
	FOCUSNEVER, 0x8000
}

pub_struct_const_wsex! { TCS_EX,
	/// Extended tab control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/tab-control-extended-styles)
	/// (`u32`), convertible to [`WS_EX`](crate::co::WS_EX).
	=>
	FLATSEPARATORS, 0x0000_0001
	REGISTERDROP, 0x0000_0002
}

pub_struct_const! { TD_ICON, isize,
	/// [`TaskDialog`](crate::HWND::TaskDialog) `pszIcon` and
	/// [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) `pszMainIcon` (`isize`).
	/// Originally has `TD` prefix and `ICON` suffix.
	=>
	WARNING, -1
	ERROR, -2
	INFORMATION, -3
	SHIELD, -4
}

pub_struct_const! { TDCBF, i32,
	/// [`TaskDialog`](crate::HWND::TaskDialog) and
	/// [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) `dwCommonButtons` (`i32`).
	/// Originally has `TDCBF` prefix and `BUTTON` suffix.
	=>
	OK, 0x0001
	YES, 0x0002
	NO, 0x0004
	CANCEL, 0x0008
	RETRY, 0x0010
	CLOSE, 0x0020
}

pub_struct_const! { TDF, u32,
	/// [`TASKDIALOGCONFIG`](crate::TASKDIALOGCONFIG) `dwFlags` (`i32`).
	=>
	ENABLE_HYPERLINKS, 0x0001
	USE_HICON_MAIN, 0x0002
	USE_HICON_FOOTER, 0x0004
	ALLOW_DIALOG_CANCELLATION, 0x0008
	USE_COMMAND_LINKS, 0x0010
	USE_COMMAND_LINKS_NO_ICON, 0x0020
	EXPAND_FOOTER_AREA, 0x0040
	EXPANDED_BY_DEFAULT, 0x0080
	VERIFICATION_FLAG_CHECKED, 0x0100
	SHOW_PROGRESS_BAR, 0x0200
	SHOW_MARQUEE_PROGRESS_BAR, 0x0400
	CALLBACK_TIMER, 0x0800
	POSITION_RELATIVE_TO_WINDOW, 0x1000
	RTL_LAYOUT, 0x2000
	NO_DEFAULT_RADIO_BUTTON, 0x4000
	CAN_BE_MINIMIZED, 0x8000
	NO_SET_FOREGROUND, 0x0001_0000
	SIZE_TO_CONTENT, 0x0100_0000
}

pub_struct_const! { TH32CS, u32,
	/// [`CreateToolhelp32Snapshot`](crate::HPROCESSLIST) `dwFlags` (`u32`).
	=>
	SNAPHEAPLIST, 0x0000_0001
	SNAPPROCESS, 0x0000_0002
	SNAPTHREAD, 0x0000_0004
	SNAPMODULE, 0x0000_0008
	SNAPMODULE32, 0x0000_0010
	SNAPALL, Self::SNAPHEAPLIST.0 | Self::SNAPPROCESS.0 | Self::SNAPTHREAD.0 | Self::SNAPMODULE.0
	INHERIT, 0x8000_0000
}

pub_struct_const! { THREAD_CREATE, u32,
	/// [`CreateThread`](crate::HTHREAD::CreateThread) `dwFlags` (`u32`).
	/// Originally has no prefix.
	=>
	/// Originally just a zero.
	RUN_IMMEDIATELY, 0
	CREATE_SUSPENDED, 0x0000_0004
	STACK_SIZE_PARAM_IS_A_RESERVATION, 0x0001_0000
}

pub_struct_const! { TME, u32,
	/// [`TrackMouseEvent`](crate::TrackMouseEvent) `dwFlags` (`u32`).
	=>
	CANCEL, 0x8000_0000
	HOVER, 0x0000_0001
	LEAVE, 0x0000_0002
	NONCLIENT, 0x0000_0010
	QUERY, 0x4000_0000
}

pub_struct_const! { TPM, u32,
	/// [`TrackPopupMenu`](crate::HMENU::TrackPopupMenu) `uFlags` (`u32`).
	=>
	LEFTBUTTON, 0x0000
	RIGHTBUTTON, 0x0002
	LEFTALIGN, 0x0000
	CENTERALIGN, 0x0004
	RIGHTALIGN, 0x0008
	TOPALIGN, 0x0000
	VCENTERALIGN, 0x0010
	BOTTOMALIGN, 0x0020
	HORIZONTAL, 0x0000
	VERTICAL, 0x0040
	NONOTIFY, 0x0080
	RETURNCMD, 0x0100
	RECURSE, 0x0001
	HORPOSANIMATION, 0x0400
	HORNEGANIMATION, 0x0800
	VERPOSANIMATION, 0x1000
	VERNEGANIMATION, 0x2000
	NOANIMATION, 0x4000
	LAYOUTRTL, 0x8000
	WORKAREA, 0x10000
}

pub_struct_const_wm! { TRBM,
	/// Trackbar control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM). Originally has `TBM`
	/// prefix.
	=>
	=>
	GETPOS, WM::USER.0
	GETRANGEMIN, WM::USER.0 + 1
	GETRANGEMAX, WM::USER.0 + 2
	GETTIC, WM::USER.0 + 3
	SETTIC, WM::USER.0 + 4
	SETPOS, WM::USER.0 + 5
	SETRANGE, WM::USER.0 + 6
	SETRANGEMIN, WM::USER.0 + 7
	SETRANGEMAX, WM::USER.0 + 8
	CLEARTICS, WM::USER.0 + 9
	SETSEL, WM::USER.0 + 10
	SETSELSTART, WM::USER.0 + 11
	SETSELEND, WM::USER.0 + 12
	GETPTICS, WM::USER.0 + 14
	GETTICPOS, WM::USER.0 + 15
	GETNUMTICS, WM::USER.0 + 16
	GETSELSTART, WM::USER.0 + 17
	GETSELEND, WM::USER.0 + 18
	CLEARSEL, WM::USER.0 + 19
	SETTICFREQ, WM::USER.0 + 20
	SETPAGESIZE, WM::USER.0 + 21
	GETPAGESIZE, WM::USER.0 + 22
	SETLINESIZE, WM::USER.0 + 23
	GETLINESIZE, WM::USER.0 + 24
	GETTHUMBRECT, WM::USER.0 + 25
	GETCHANNELRECT, WM::USER.0 + 26
	SETTHUMBLENGTH, WM::USER.0 + 27
	GETTHUMBLENGTH, WM::USER.0 + 28
	SETTOOLTIPS, WM::USER.0 + 29
	GETTOOLTIPS, WM::USER.0 + 30
	SETTIPSIDE, WM::USER.0 + 31
	SETBUDDY, WM::USER.0 + 32
	GETBUDDY, WM::USER.0 + 33
	SETPOSNOTIFY, WM::USER.0 + 34
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
}

pub_struct_const_nm! { TRBN,
	/// Trackbar control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-trackbar-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -1501
	=>
	THUMBPOSCHANGING, Self::FIRST.0 - 1
}

pub_struct_const! { TVE, u32,
	/// [`TVM_EXPAND`](crate::msg::tvm::Expand) `action` (`u32`).
	=>
	COLLAPSE, 0x0001
	EXPAND, 0x0002
	TOGGLE, 0x0003
	EXPANDPARTIAL, 0x4000
	COLLAPSERESET, 0x8000
}

pub_struct_const! { TVGN, u32,
	/// [`TVM_GETNEXTITEM`](crate::msg::tvm::GetNextItem) `which` (`u32`).
	=>
	ROOT, 0x0000
	NEXT, 0x0001
	PREVIOUS, 0x0002
	PARENT, 0x0003
	CHILD, 0x0004
	FIRSTVISIBLE, 0x0005
	NEXTVISIBLE, 0x0006
	PREVIOUSVISIBLE, 0x0007
	DROPHILITE, 0x0008
	CARET, 0x0009
	LASTVISIBLE, 0x000a
	NEXTSELECTED, 0x000b
	/// Originally has no `TVGN` prefix.
	TVSI_NOSINGLEEXPAND, 0x8000
}

pub_struct_const! { TVI, isize,
	/// [`TVINSERTSTRUCT`](crate::TVINSERTSTRUCT) `hInsertAfter` (`isize`).
	=>
	ROOT, -0x10000
	FIRST, -0x0ffff
	LAST, -0x0fffe
	SORT, -0x0fffd
}

pub_struct_const! { TVIF, u32,
	/// [`TVITEM`](crate::TVITEM) `mask` (`u32`).
	=>
	TEXT, 0x0001
	IMAGE, 0x0002
	PARAM, 0x0004
	STATE, 0x0008
	HANDLE, 0x0010
	SELECTEDIMAGE, 0x0020
	CHILDREN, 0x0040
	INTEGRAL, 0x0080
	STATEEX, 0x0100
	EXPANDEDIMAGE, 0x0200
}

pub_struct_const! { TVIS, u32,
	/// Tree view item
	/// [states](https://docs.microsoft.com/en-us/windows/win32/controls/tree-view-control-item-states)
	/// (`u32`)
	=>
	SELECTED, 0x0002
	CUT, 0x0004
	DROPHILITED, 0x0008
	BOLD, 0x0010
	EXPANDED, 0x0020
	EXPANDEDONCE, 0x0040
	EXPANDPARTIAL, 0x0080
	OVERLAYMASK, 0x0f00
	STATEIMAGEMASK, 0xf000
	USERMASK, 0xf000
}

pub_struct_const! { TVIS_EX, u32,
	/// [`TVITEMEX`](crate::TVITEMEX) `uStateEx` (`u32`).
	=>
	DISABLED, 0x0002
	FLAT, 0x0001
	/// This value is not declared in any header, it may not be accurate.
	HWND, 0x0000
}

pub_struct_const_wm! { TVM,
	/// Tree view control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	FIRST, 0x1100
	=>
	INSERTITEM, Self::FIRST.0 + 50
	DELETEITEM, Self::FIRST.0 + 1
	EXPAND, Self::FIRST.0 + 2
	GETITEMRECT, Self::FIRST.0 + 4
	GETCOUNT, Self::FIRST.0 + 5
	GETINDENT, Self::FIRST.0 + 6
	SETINDENT, Self::FIRST.0 + 7
	GETIMAGELIST, Self::FIRST.0 + 8
	SETIMAGELIST, Self::FIRST.0 + 9
	GETNEXTITEM, Self::FIRST.0 + 10
	SELECTITEM, Self::FIRST.0 + 11
	GETITEM, Self::FIRST.0 + 62
	SETITEM, Self::FIRST.0 + 63
	EDITLABEL, Self::FIRST.0 + 65
	GETEDITCONTROL, Self::FIRST.0 + 15
	GETVISIBLECOUNT, Self::FIRST.0 + 16
	HITTEST, Self::FIRST.0 + 17
	CREATEDRAGIMAGE, Self::FIRST.0 + 18
	SORTCHILDREN, Self::FIRST.0 + 19
	ENSUREVISIBLE, Self::FIRST.0 + 20
	SORTCHILDRENCB, Self::FIRST.0 + 21
	ENDEDITLABELNOW, Self::FIRST.0 + 22
	GETISEARCHSTRING, Self::FIRST.0 + 64
	SETTOOLTIPS, Self::FIRST.0 + 24
	GETTOOLTIPS, Self::FIRST.0 + 25
	SETINSERTMARK, Self::FIRST.0 + 26
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
	SETITEMHEIGHT, Self::FIRST.0 + 27
	GETITEMHEIGHT, Self::FIRST.0 + 28
	SETBKCOLOR, Self::FIRST.0 + 29
	SETTEXTCOLOR, Self::FIRST.0 + 30
	GETBKCOLOR, Self::FIRST.0 + 31
	GETTEXTCOLOR, Self::FIRST.0 + 32
	SETSCROLLTIME, Self::FIRST.0 + 33
	GETSCROLLTIME, Self::FIRST.0 + 34
	SETINSERTMARKCOLOR, Self::FIRST.0 + 37
	GETINSERTMARKCOLOR, Self::FIRST.0 + 38
	SETBORDER, Self::FIRST.0 + 35
	GETITEMSTATE, Self::FIRST.0 + 39
	SETLINECOLOR, Self::FIRST.0 + 40
	GETLINECOLOR, Self::FIRST.0 + 41
	MAPACCIDTOHTREEITEM, Self::FIRST.0 + 42
	MAPHTREEITEMTOACCID, Self::FIRST.0 + 43
	SETEXTENDEDSTYLE, Self::FIRST.0 + 44
	GETEXTENDEDSTYLE, Self::FIRST.0 + 45
	SETAUTOSCROLLINFO, Self::FIRST.0 + 59
	SETHOT, Self::FIRST.0 + 58
	GETSELECTEDCOUNT, Self::FIRST.0 + 70
	SHOWINFOTIP, Self::FIRST.0 + 71
	GETITEMPARTRECT, Self::FIRST.0 + 72
}

pub_struct_const_nm! { TVN,
	/// Tree view control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-tree-view-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -400
	=>
	SELCHANGING, Self::FIRST.0 - 50
	SELCHANGED, Self::FIRST.0 - 51
	GETDISPINFO, Self::FIRST.0 - 52
	SETDISPINFO, Self::FIRST.0 - 53
	ITEMEXPANDING, Self::FIRST.0 - 54
	ITEMEXPANDED, Self::FIRST.0 - 55
	BEGINDRAG, Self::FIRST.0 - 56
	BEGINRDRAG, Self::FIRST.0 - 57
	DELETEITEM, Self::FIRST.0 - 58
	BEGINLABELEDIT, Self::FIRST.0 - 59
	ENDLABELEDIT, Self::FIRST.0 - 60
	KEYDOWN, Self::FIRST.0 - 12
	GETINFOTIP, Self::FIRST.0 - 14
	SINGLEEXPAND, Self::FIRST.0 - 15
	ITEMCHANGING, Self::FIRST.0 - 17
	ITEMCHANGED, Self::FIRST.0 - 19
	ASYNCDRAW, Self::FIRST.0 - 20
}

pub_struct_const_ws! { TVS,
	/// Tree view control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/tree-view-control-window-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	HASBUTTONS, 0x0001
	HASLINES, 0x0002
	LINESATROOT, 0x0004
	EDITLABELS, 0x0008
	DISABLEDRAGDROP, 0x0010
	SHOWSELALWAYS, 0x0020
	RTLREADING, 0x0040
	NOTOOLTIPS, 0x0080
	CHECKBOXES, 0x0100
	TRACKSELECT, 0x0200
	SINGLEEXPAND, 0x0400
	INFOTIP, 0x0800
	FULLROWSELECT, 0x1000
	NOSCROLL, 0x2000
	NONEVENHEIGHT, 0x4000
	NOHSCROLL, 0x8000
}

pub_struct_const_wsex! { TVS_EX,
	/// Extended tree view control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/tree-view-control-window-extended-styles)
	/// (`u32`), convertible to [`WS_EX`](crate::co::WS_EX).
	=>
	NONE, 0
	NOSINGLECOLLAPSE, 0x0001
	MULTISELECT, 0x0002
	DOUBLEBUFFER, 0x0004
	NOINDENTSTATE, 0x0008
	RICHTOOLTIP, 0x0010
	AUTOHSCROLL, 0x0020
	FADEINOUTEXPANDOS, 0x0040
	PARTIALCHECKBOXES, 0x0080
	EXCLUSIONCHECKBOXES, 0x0100
	DIMMEDCHECKBOXES, 0x0200
	DRAWIMAGEASYNC, 0x0400
}

pub_struct_const! { TVSIL, u8,
	/// [`TVM_GETIMAGELIST`](crate::msg::tvm::GetImageList) and
	/// [`TVM_SETIMAGELIST`](crate::msg::tvm::SetImageList) `kind`.
	=>
	NORMAL, 0
	STATE, 2
}
