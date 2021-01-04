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

const_type! { GWLP, i32,
	/// [`GetWindowLongPtr`](crate::HWND::GetWindowLongPtr) and
	/// [`SetWindowLongPtr`](crate:HWND::SetWindowLongPtr) `nIndex` (`i32`).
	/// Originally has prefixe `GWL` also.

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

const_type! { ICON_SZ, i32,
	/// [`WM_SETICON`](crate::msg::WmSetIcon) icon size (`i32`). Originally has
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
