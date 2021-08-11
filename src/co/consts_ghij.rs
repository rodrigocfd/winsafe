#![allow(non_camel_case_types)]

use crate::co::{CCM, WM};

pub_struct_const! { GA, u32,
	/// [`HWND::GetAncestor`](crate::HWND::GetAncestor) `gaFlags` (`u32`).
	=>
	/// Retrieves the parent window. This does not include the owner, as it does
	/// with the [`HWND::GetParent`](crate::HWND::GetParent) function.
	PARENT, 1
	/// Retrieves the root window by walking the chain of parent windows.
	///
	/// Returns the
	/// [closest](https://groups.google.com/a/chromium.org/g/chromium-dev/c/Hirr_DkuZdw/m/N0pSoJBhAAAJ)
	/// parent with [`WS::OVERLAPPED`](crate::co::WS::OVERLAPPED) or
	/// [`WS::POPUP`](crate::co::WS::POPUP).
	ROOT, 2
	/// Retrieves the owned root window by walking the chain of parent and owner
	/// windows returned by [`HWND::GetParent`](crate::HWND::GetParent).
	///
	/// Returns the
	/// [furthest](https://groups.google.com/a/chromium.org/g/chromium-dev/c/Hirr_DkuZdw/m/N0pSoJBhAAAJ)
	/// parent with [`WS::OVERLAPPED`](crate::co::WS::OVERLAPPED) or
	/// [`WS::POPUP`](crate::co::WS::POPUP), which usually is the main
	/// application window.
	ROOTOWNER, 3
}

pub_struct_const! { GCLP, i32,
	/// [`HWND::GetClassLongPtr`](crate::HWND::GetClassLongPtr) `nIndex`
	/// (`i32`). Originally has prefixes `GCW` and `GCL` also.
	=>
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

pub_struct_const! { GDC, i32,
	/// [`HDC::GetDeviceCaps`](crate::HDC::GetDeviceCaps) `index` (`i32`).
	/// Originally has no prefix.
	=>
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

pub_struct_const! { GDT, u32,
	/// [`NMDATETIMECHANGE`](crate::NMDATETIMECHANGE) and
	/// [`NMDATETIMESTRING`](crate::NMDATETIMESTRING) `dwFlags` (`u32`).
	=>
	VALID, 0
	NONE, 1
}

pub_struct_const! { GDTR, u32,
	/// [`dtm::GetRange`](crate::msg::dtm::GetRange) return value (`u32`).
	=>
	MIN, 0x0001
	MAX, 0x0002
}

pub_struct_const! { GENERIC, u32,
	/// Generic access rights
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/secauthz/generic-access-rights)
	/// (`u32`).
	=>
	/// Read access.
	READ, 0x8000_0000
	/// Write access.
	WRITE, 0x4000_0000
	/// Execute access.
	EXECUTE, 0x2000_0000
	/// All possible access rights.
	ALL, 0x1000_0000
}

pub_struct_const! { GM, i32,
	/// [`HDC::SetGraphicsMode`](crate::HDC::SetGraphicsMode) `iMode` (`i32`).
	=>
	COMPATIBLE, 1
	ADVANCED, 2
}

pub_struct_const! { GMDI, u32,
	/// [`HMENU::GetMenuDefaultItem`](crate::HMENU::GetMenuDefaultItem)
	/// `gmdiFlags` (`u32`).
	=>
	USEDISABLED, 0x0001
	GOINTOPOPUPS, 0x0002
}

pub_struct_const! { GMEM, u32,
	/// [`HGLOBAL::GlobalAlloc`](crate::HGLOBAL::GlobalAlloc) `uFlags` (`u32`).
	=>
	FIXED, 0x0000
	MOVEABLE, 0x0002
	ZEROINIT, 0x0040
	GHND, Self::MOVEABLE.0 | Self::ZEROINIT.0
	GPTR, Self::FIXED.0 | Self::ZEROINIT.0
}

pub_struct_const! { GR, u32,
	/// [`HPROCESS::GetGuiResources`](crate::HPROCESS::GetGuiResources)
	/// `uiFlags` (`u32`).
	=>
	GDIOBJECTS, 0
	GDIOBJECTS_PEAK, 2
	USEROBJECTS, 1
	USEROBJECTS_PEAK, 4
}

pub_struct_const! { GUI, u32,
	/// [`GUITHREADINFO`](crate::GUITHREADINFO) `flags` (`u32`).
	=>
	CARETBLINKING, 0x0000_0001
	INMENUMODE, 0x0000_0004
	INMOVESIZE, 0x0000_0002
	POPUPMENUMODE, 0x0000_00010
	SYSTEMMENUMODE, 0x0000_0008
}

pub_struct_const! { GW, u32,
	/// [`HWND::GetWindow`](crate::HWND::GetWindow) `uCmd` (`u32`).
	=>
	HWNDFIRST, 0
	HWNDLAST, 1
	HWNDNEXT, 2
	HWNDPREV, 3
	OWNER, 4
	CHILD, 5
	ENABLEDPOPUP, 6
	MAX, 6
}

pub_struct_const! { GWL_C, i8,
	/// [`wm::StyleChanged`](crate::msg::wm::StyleChanged) and
	/// [`wm::StyleChanging`](crate::msg::wm::StyleChanging) change (`i8`).
	/// Originally has `GWL` prefix.
	=>
	EXSTYLE, -20
	STYLE, -16
}

pub_struct_const! { GWLP, i32,
	/// [`HWND::GetWindowLongPtr`](crate::HWND::GetWindowLongPtr) and
	/// [`HWND::SetWindowLongPtr`](crate::HWND::SetWindowLongPtr) `nIndex`
	/// (`i32`). Originally has prefix `GWL` also.
	=>
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

pub_struct_const_wm! { HDM,
	/// Header control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	FIRST, 0x1200
	=>
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

pub_struct_const_nm! { HDN,
	/// Header control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -300
	=>
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

pub_struct_const_ws! { HDS, u32,
	/// Header control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/header-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
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

pub_struct_const! { HELPINFO, i32,
	/// [`HELPINFO`](crate::HELPINFO) `iContextType` (`i32`).
	=>
	WINDOW, 0x0001
	MENUITEM, 0x0002
}

pub_struct_const! { HELPW, u32,
	/// [`HWND::WinHelp`](crate::HWND::WinHelp) `uCommand` (`u32`).
	=>
	CONTEXT, 0x0001
	QUIT, 0x0002
	INDEX, 0x0003
	CONTENTS, 0x0003
	HELPONHELP, 0x0004
	SETINDEX, 0x0005
	SETCONTENTS, 0x0005
	CONTEXTPOPUP, 0x0008
	FORCEFILE, 0x0009
	KEY, 0x0101
	COMMAND, 0x0102
	PARTIALKEY, 0x0105
	MULTIKEY, 0x0201
	SETWINPOS, 0x0203
	CONTEXTMENU, 0x000a
	FINDER, 0x000b
	WM_HELP, 0x000c
	SETPOPUP_POS, 0x000d
	TCARD, 0x8000
	TCARD_DATA, 0x0010
	TCARD_OTHER_CALLER, 0x0011
}

pub_struct_const! { HICF, u32,
	/// [NMBCHOTITEM](crate::NMBCHOTITEM) `dwFlags` (`u32`).
	=>
	OTHER, 0x0000_0000
	ARROWKEYS, 0x0000_0002
	ACCELERATOR, 0x0000_0004
	DUPACCEL, 0x0000_0008
	ENTERING, 0x0000_0010
	LEAVING, 0x0000_0020
	RESELECT, 0x0000_0040
	LMOUSE, 0x0000_0080
	TOGGLEDROPDOWN, 0x0000_0100
}

pub_struct_const! { HWND_PLACE, isize,
	/// [`HWND::SetWindowPos`](crate::HWND::SetWindowPos) `hWndInsertAfter`
	/// (`isize`).
	=>
	TOP, 0
	BOTTOM, 1
	TOPMOST, -1
	NOTOPMOST, -2
}

pub_struct_const! { HS, i32,
	/// [`HBRUSH::CreateHatchBrush`](crate::HBRUSH::CreateHatchBrush) `iHatch`
	/// (`i32`).
	=>
	/// Horizontal hatch: `-----`.
	HORIZONTAL, 0
	/// Vertical hatch: `|||||`.
	VERTICAL, 1
	/// 45-degree downward left-to-right hatch: `\\\\\`.
	FDIAGONAL, 2
	/// 45-degree upward left-to-right hatch: `/////`.
	BDIAGONAL, 3
	/// Horizontal and vertical crosshatch: `+++++`.
	CROSS, 4
	/// 45-degree crosshatch: `xxxxx`.
	DIAGCROSS, 5
}

pub_struct_const! { HT, u16,
	/// [`wm::NcHitTest`](crate::msg::wm::NcHitTest),
	/// [`wm::SetCursor`](crate::msg::wm::SetCursor) `hit_test` (`u16`).
	=>
	BORDER, 18
	BOTTOM, 15
	BOTTOMLEFT, 16
	BOTTOMRIGHT, 17
	CAPTION, 2
	CLIENT, 1
	CLOSE, 20
	ERROR, -2i16 as u16
	GROWBOX, 4
	HELP, 21
	HSCROLL, 6
	LEFT, 10
	MENU, 5
	MAXBUTTON, 9
	MINBUTTON, 8
	NOWHERE, 0
	REDUCE, 8
	RIGHT, 11
	SIZE, 4
	SYSMENU, 3
	TOP, 12
	TOPLEFT, 13
	TOPRIGHT, 14
	TRANSPARENT, -1i16 as u16
	VSCROLL, 7
	ZOOM, 9
}

pub_struct_const! { IDB, usize,
	/// [`TBADDBITMAP`](crate::TBADDBITMAP) `nID` (`usize`).
	=>
	STD_SMALL_COLOR, 0
	STD_LARGE_COLOR, 1
	VIEW_SMALL_COLOR, 4
	VIEW_LARGE_COLOR, 5
	HIST_SMALL_COLOR, 8
	HIST_LARGE_COLOR, 9
	HIST_NORMAL, 12
	HIST_HOT, 13
	HIST_DISABLED, 14
	HIST_PRESSED, 15
}

pub_struct_const! { ICON_SZ, u8,
	/// [`wm::SetIcon`](crate::msg::wm::SetIcon) icon size (`u8`). Originally
	/// has `ICON` prefix.
	=>
	SMALL, 0
	BIG, 1
}

pub_struct_const! { IDC, isize,
	/// [`HINSTANCE::LoadCursor`](crate::HINSTANCE::LoadCursor) `lpCursorName`
	/// (`isize`).
	=>
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

pub_struct_const! { IDI, isize,
	/// [`HINSTANCE::LoadIcon`](crate::HINSTANCE::LoadIcon) `lpIconName`
	/// (`isize`).
	=>
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

pub_struct_const! { ILC, u32,
	/// [`HIMAGELIST::Create`](crate::HIMAGELIST::Create) `flags` (`u32`).
	=>
	/// Use a mask. The image list contains two bitmaps, one of which is a
	/// monochrome bitmap used as a mask. If this value is not included, the
	/// image list contains only one bitmap.
	MASK, 0x0000_0001
	/// Use the default behavior if none of the other `ILC::COLORx` flags is
	/// specified. Typically, the default is `ILC::COLOR4`, but for older
	/// display drivers, the default is `ILC::COLORDDB`.
	COLOR, 0x0000_0000
	/// Use a device-dependent bitmap.
	COLORDDB, 0x0000_00fe
	/// Use a 4-bit (16-color) device-independent bitmap (DIB) section as the
	/// bitmap for the image list.
	COLOR4, 0x0000_0004
	/// Use an 8-bit DIB section. The colors used for the color table are the
	/// same colors as the halftone palette.
	COLOR8, 0x0000_0008
	/// Use a 16-bit (32/64k-color) DIB section.
	COLOR16, 0x0000_0010
	/// Use a 24-bit DIB section.
	COLOR24, 0x0000_0018
	/// Use a 32-bit DIB section.
	COLOR32, 0x0000_0020
	/// Mirror the icons contained, if the process is mirrored.
	MIRROR, 0x0000_2000
	/// Causes the mirroring code to mirror each item when inserting a set of
	/// images, versus the whole strip.
	PERITEMMIRROR, 0x0000_8000
	/// Windows Vista and later. Imagelist should accept smaller than set images
	/// and apply original size based on image added.
	ORIGINALSIZE, 0x0001_0000
}

pub_struct_const! { ILD, u32,
	/// [`IMAGELISTDRAWFLAGS`](https://docs.microsoft.com/en-us/windows/win32/controls/imagelistdrawflags)
	/// enumeration (`u32`).
	=>
	NORMAL, 0x0000_0000
	TRANSPARENT, 0x0000_0001
	MASK, 0x0000_0010
	IMAGE, 0x0000_0020
	ROP, 0x0000_0040
	BLEND25, 0x0000_0002
	BLEND50, 0x0000_0004
	OVERLAYMASK, 0x0000_0f00
	PRESERVEALPHA, 0x0000_1000
	SCALE, 0x0000_2000
	DPISCALE, 0x0000_4000
	ASYNC, 0x0000_8000
	SELECTED, Self::BLEND50.0
	FOCUS, Self::BLEND25.0
	BLEND, Self::BLEND50.0
}

pub_struct_const! { ILS, u32,
	/// [`IMAGELISTSTATEFLAGS`](https://docs.microsoft.com/en-us/windows/win32/controls/imageliststateflags)
	/// enumeration (`u32`).
	=>
	NORMAL, 0x0000_0000
	GLOW, 0x0000_0001
	SHADOW, 0x0000_0002
	SATURATE, 0x0000_0004
	ALPHA, 0x0000_0008
}

pub_struct_const! { IMAGE_TYPE, u8,
	/// [`bm::GetImage`](crate::msg::bm::GetImage) `img_type` (`u8`). Originally
	/// has `IMAGE` prefix.
	=>
	BITMAP, 0
	ICON, 1
}

pub_struct_const_wm! { IPM,
	/// IP address control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-ip-address-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	=>
	CLEARADDRESS, WM::USER.0 + 100
	SETADDRESS, WM::USER.0 + 101
	GETADDRESS, WM::USER.0 + 102
	SETRANGE, WM::USER.0 + 103
	SETFOCUS, WM::USER.0 + 104
	ISBLANK, WM::USER.0 + 105
}

pub_struct_const_nm! { IPN,
	/// IP address control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-ip-address-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -860
	=>
	FIELDCHANGED, Self::FIRST.0 - 0
}
