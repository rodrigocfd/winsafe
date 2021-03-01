use crate::co::{CCM, NM, WM, WS};

const_type! { GA, u32,
	/// [`GetAncestor`](crate::HWND::GetAncestor) `gaFlags` (`u32`).

	PARENT, 1
	ROOT, 2
	ROOTOWNER, 3
}

const_type! { GCLP, i32,
	/// [`GetClassLongPtr`](crate::HWND::GetClassLongPtr) `nIndex` (`i32`).
	/// Originally has prefixes `GCW` and `GCL` also.

	ATOM, -32
	CBWNDEXTRA, -18
	CBCLSEXTRA, -20
	MENUNAME, -8
	HBRBACKGROUND, -10
	HCURSOR, -12
	HICON, -14
	HMODULE, -16
	WNDPROC, -24
	HICONSM, -34
}

const_type! { GDC, i32,
	/// [`GetDeviceCaps`](crate::HDC::GetDeviceCaps) `index` (`i32`). Originally
	/// has no prefix.

	DRIVERVERSION, 0
	TECHNOLOGY, 2
	HORZSIZE, 4
	VERTSIZE, 6
	HORZRES, 8
	VERTRES, 10
	BITSPIXEL, 12
	PLANES, 14
	NUMBRUSHES, 16
	NUMPENS, 18
	NUMMARKERS, 20
	NUMFONTS, 22
	NUMCOLORS, 24
	PDEVICESIZE, 26
	CURVECAPS, 28
	LINECAPS, 30
	POLYGONALCAPS, 32
	TEXTCAPS, 34
	CLIPCAPS, 36
	RASTERCAPS, 38
	ASPECTX, 40
	ASPECTY, 42
	ASPECTXY, 44
	LOGPIXELSX, 88
	LOGPIXELSY, 90
	SIZEPALETTE, 104
	NUMRESERVED, 106
	COLORRES, 108
	PHYSICALWIDTH, 110
	PHYSICALHEIGHT, 111
	PHYSICALOFFSETX, 112
	PHYSICALOFFSETY, 113
	SCALINGFACTORX, 114
	SCALINGFACTORY, 115
	VREFRESH, 116
	DESKTOPVERTRES, 117
	DESKTOPHORZRES, 118
	BLTALIGNMENT, 119
	SHADEBLENDCAPS, 120
	COLORMGMTCAPS, 121
}

const_type! { GDT, u32,
	/// [`NMDATETIMECHANGE`](crate::NMDATETIMECHANGE) and
	/// [`NMDATETIMESTRING`](crate::NMDATETIMESTRING) `dwFlags` (`u32`).

	VALID, 0
	NONE, 1
}

const_type! { GDTR, u32,
	/// [`DTM_GETRANGE`](crate::msg::dtm::GetRange) return value (`u32`).

	MIN, 0x0001
	MAX, 0x0002
}

const_type! { GW, u32,
	/// [`GetWindow`](crate::HWND::GetWindow) `uCmd` (`u32`).

	HWNDFIRST, 0
	HWNDLAST, 1
	HWNDNEXT, 2
	HWNDPREV, 3
	OWNER, 4
	CHILD, 5
	ENABLEDPOPUP, 6
	MAX, 6
}

const_type! { GWL_C, i8,
	/// [`WM_STYLECHANGED`](crate::msg::wm::StyleChanged) and
	/// [`WM_STYLECHANGING`](crate::msg::wm::StyleChanging) change (`i8`).
	/// Originally has `GWL` prefix.

	EXSTYLE, -20
	STYLE, -16
}

const_type! { GWLP, i32,
	/// [`GetWindowLongPtr`](crate::HWND::GetWindowLongPtr) and
	/// [`SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) `nIndex` (`i32`).
	/// Originally has prefix `GWL` also.

	STYLE, -16
	EXSTYLE, -20
	WNDPROC, -4
	HINSTANCE, -6
	HWNDPARENT, -8
	USERDATA, -21
	ID, -12
	DWLP_DLGPROC, 8 //std::mem::size_of::<isize> as i32 https://github.com/rust-lang/rust/issues/51910
	DWLP_MSGRESULT, 0
	DWLP_USER, Self::DWLP_DLGPROC.0 + 8 //std::mem::size_of::<isize> as i32 https://github.com/rust-lang/rust/issues/51910
}

const_type_wm! { HDM,
	/// Header control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).

	GETITEMCOUNT, Self::FIRST.0 + 0
	INSERTITEM, Self::FIRST.0 + 10
	DELETEITEM, Self::FIRST.0 + 11
	GETITEM, Self::FIRST.0 + 11
	SETITEM, Self::FIRST.0 + 12
	LAYOUT, Self::FIRST.0 + 5
	HITTEST, Self::FIRST.0 + 6
	GETITEMRECT, Self::FIRST.0 + 7
	SETIMAGELIST, Self::FIRST.0 + 8
	GETIMAGELIST, Self::FIRST.0 + 9
	ORDERTOINDEX, Self::FIRST.0 + 15
	CREATEDRAGIMAGE, Self::FIRST.0 + 16
	GETORDERARRAY, Self::FIRST.0 + 17
	SETORDERARRAY, Self::FIRST.0 + 18
	SETHOTDIVIDER, Self::FIRST.0 + 19
	SETBITMAPMARGIN, Self::FIRST.0 + 20
	GETBITMAPMARGIN, Self::FIRST.0 + 21
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
	SETFILTERCHANGETIMEOUT, Self::FIRST.0 + 22
	EDITFILTER, Self::FIRST.0 + 23
	CLEARFILTER, Self::FIRST.0 + 24
	GETITEMDROPDOWNRECT, Self::FIRST.0 + 25
	GETOVERFLOWRECT, Self::FIRST.0 + 26
	GETFOCUSEDITEM, Self::FIRST.0 + 27
	SETFOCUSEDITEM, Self::FIRST.0 + 28
}
const_type_priv_values! { HDM
	FIRST, 0x1200
}

const_type_nm! { HDN,
	/// Header control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).

	ITEMCHANGING, Self::FIRST.0 - 20
	ITEMCHANGED, Self::FIRST.0 - 21
	ITEMCLICK, Self::FIRST.0 - 22
	ITEMDBLCLICK, Self::FIRST.0 - 23
	DIVIDERDBLCLICK, Self::FIRST.0 - 25
	BEGINTRACK, Self::FIRST.0 - 26
	ENDTRACK, Self::FIRST.0 - 27
	TRACK, Self::FIRST.0 - 28
	GETDISPINFO, Self::FIRST.0 - 29
	BEGINDRAG, Self::FIRST.0 - 10
	ENDDRAG, Self::FIRST.0 - 11
	FILTERCHANGE, Self::FIRST.0 - 12
	FILTERBTNCLICK, Self::FIRST.0 - 13
	BEGINFILTEREDIT, Self::FIRST.0 - 14
	ENDFILTEREDIT, Self::FIRST.0 - 15
	ITEMSTATEICONCLICK, Self::FIRST.0 - 16
	ITEMKEYDOWN, Self::FIRST.0 - 17
	DROPDOWN, Self::FIRST.0 - 18
	OVERFLOWCLICK, Self::FIRST.0 - 19
}
const_type_priv_values! { HDN
	FIRST, -300
}

const_type_ws! { HDS,
	/// Header control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/header-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).

	HORZ, 0x0000
	BUTTONS, 0x0002
	HOTTRACK, 0x0004
	HIDDEN, 0x0008
	DRAGDROP, 0x0040
	FULLDRAG, 0x0080
	FILTERBAR, 0x0100
	FLAT, 0x0200
	CHECKBOXES, 0x0400
	NOSIZING, 0x0800
	OVERFLOW, 0x1000
}

const_type! { HICF, u32,
	/// [NMBCHOTITEM](crate::NMBCHOTITEM) `dwFlags` (`u32`).

	OTHER, 0x00000000
	ARROWKEYS, 0x00000002
	ACCELERATOR, 0x00000004
	DUPACCEL, 0x00000008
	ENTERING, 0x00000010
	LEAVING, 0x00000020
	RESELECT, 0x00000040
	LMOUSE, 0x00000080
	TOGGLEDROPDOWN, 0x00000100
}

const_type! { HWND_PLACE, isize,
	/// [`SetWindowPos`](crate::HWND::SetWindowPos) `hWndInsertAfter` (`isize`).

	TOP, 0
	BOTTOM, 1
	TOPMOST, -1
	NOTOPMOST, -2
}

const_type! { ICON_SZ, u8,
	/// [`WM_SETICON`](crate::msg::wm::SetIcon) icon size (`u8`). Originally has
	/// `ICON` prefix.

	SMALL, 0
	BIG, 1
}

const_type! { IDC, usize,
	/// [`LoadCursor`](crate::HINSTANCE::LoadCursor) `lpCursorName` (`usize`).

	ARROW, 32512
	IBEAM, 32513
	WAIT, 32514
	CROSS, 32515
	UPARROW, 32516
	SIZENWSE, 32642
	SIZENESW, 32643
	SIZEWE, 32644
	SIZENS, 32645
	SIZEALL, 32646
	NO, 32648
	HAND, 32649
	APPSTARTING, 32650
	HELP, 32651
	PIN, 32671
	PERSON, 32672
}

const_type! { IDI, usize,
	/// [`LoadIcon`](crate::HINSTANCE::LoadIcon) `lpIconName` (`usize`).

	APPLICATION, 32512
	HAND, 32513
	QUESTION, 32514
	EXCLAMATION, 32515
	ASTERISK, 32516
	WINLOGO, 32517
	SHIELD, 32518
	WARNING, Self::EXCLAMATION.0
	ERROR, Self::HAND.0
	INFORMATION, Self::ASTERISK.0
}

const_type! { ILC, u32,
	/// [`ImageList_Create`](crate::HIMAGELIST::ImageList_Create) `flags` (`u32`).

	MASK, 0x00000001
	COLOR, 0x00000000
	COLORDDB, 0x000000fe
	COLOR4, 0x00000004
	COLOR8, 0x00000008
	COLOR16, 0x00000010
	COLOR24, 0x00000018
	COLOR32, 0x00000020
	MIRROR, 0x00002000
	PERITEMMIRROR, 0x00008000
	ORIGINALSIZE, 0x00010000
	HIGHQUALITYSCALE, 0x00020000
}

const_type! { ILD, u32,
	/// [`IMAGELISTDRAWFLAGS`](https://docs.microsoft.com/en-us/windows/win32/controls/imagelistdrawflags)
	/// enumeration (`u32`).

	NORMAL, 0x00000000
	TRANSPARENT, 0x00000001
	MASK, 0x00000010
	IMAGE, 0x00000020
	ROP, 0x00000040
	BLEND25, 0x00000002
	BLEND50, 0x00000004
	OVERLAYMASK, 0x00000f00
	PRESERVEALPHA, 0x00001000
	SCALE, 0x00002000
	DPISCALE, 0x00004000
	ASYNC, 0x00008000
	SELECTED, Self::BLEND50.0
	FOCUS, Self::BLEND25.0
	BLEND, Self::BLEND50.0
}

const_type! { ILS, u32,
	/// [`IMAGELISTSTATEFLAGS`](https://docs.microsoft.com/en-us/windows/win32/controls/imageliststateflags)
	/// enumeration (`u32`).

	NORMAL, 0x00000000
	GLOW, 0x00000001
	SHADOW, 0x00000002
	SATURATE, 0x00000004
	ALPHA, 0x00000008
}

const_type! { IMAGE_TYPE, u8,
	/// [`BM_GETIMAGE`](crate::msg::bm::GetImage) `img_type` (`u8`). Originally
	/// has `IMAGE` prefix.

	BITMAP, 0
	ICON, 1
}

const_type_wm! { IPM,
	/// IP address control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-ip-address-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).

	CLEARADDRESS, WM::USER.0 + 100
	SETADDRESS, WM::USER.0 + 101
	GETADDRESS, WM::USER.0 + 102
	SETRANGE, WM::USER.0 + 103
	SETFOCUS, WM::USER.0 + 104
	ISBLANK, WM::USER.0 + 105
}

const_type_nm! { IPN,
	/// IP address control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-ip-address-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).

	FIELDCHANGED, Self::FIRST.0 - 0
}
const_type_priv_values! { IPN
	FIRST, -860
}
