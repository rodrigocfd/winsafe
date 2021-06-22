#![allow(non_upper_case_globals)]

use crate::co::{CCM, FF, WM};

pub_struct_const! { OCR, u32,
	/// [`SetSystemCursor`](crate::HCURSOR::SetSystemCursor) `id` (`u32`).
	=>
	APPSTARTING, 32650
	NORMAL, 32512
	CROSS, 32515
	HAND, 32649
	HELP, 32651
	IBEAM, 32513
	NO, 32648
	SIZEALL, 32646
	SIZENESW, 32643
	SIZENS, 32645
	SIZENWSE, 32642
	SIZEWE, 32644
	UP, 32516
	WAIT, 32514
}

pub_struct_const! { OUT_PRECIS, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfOutPrecision` (`u8`).
	=>
	DEFAULT, 0
	STRING, 1
	CHARACTER, 2
	STROKE, 3
	TT, 4
	DEVICE, 5
	RASTER, 6
	TT_ONLY, 7
	OUTLINE, 8
	SCREEN_OUTLINE, 9
	PS_ONLY, 10
}

pub_struct_const! { PAGE, u32,
	/// [`CreateFileMapping`](crate::HFILE::CreateFileMapping) `flProtect`
	/// (`u32`).
	=>
	/// Allows views to be mapped for read-only, copy-on-write, or execute
	/// access.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) and
	/// [`GENERIC::EXECUTE`](crate::co::GENERIC::EXECUTE) access rights.
	EXECUTE_READ, 0x20
	/// Allows views to be mapped for read-only, copy-on-write, read/write, or
	/// execute access.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ),
	/// [`GENERIC::WRITE`](crate::co::GENERIC::WRITE), and
	/// [`GENERIC::EXECUTE`](crate::co::GENERIC::EXECUTE) access rights.
	EXECUTE_READWRITE, 0x40
	/// Allows views to be mapped for read-only, copy-on-write, or execute
	/// access. This value is equivalent to PAGE_EXECUTE_READ.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) and
	/// [`GENERIC::EXECUTE`](crate::co::GENERIC::EXECUTE) access rights.
	EXECUTE_WRITECOPY, 0x80
	/// Allows views to be mapped for read-only or copy-on-write access. An
	/// attempt to write to a specific region results in an access violation.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) access right.
	READONLY, 0x02
	/// Allows views to be mapped for read-only, copy-on-write, or read/write
	/// access.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) and
	/// [`GENERIC::WRITE`](crate::co::GENERIC::WRITE) access rights.
	READWRITE, 0x04
	/// Allows views to be mapped for read-only or copy-on-write access. This
	/// value is equivalent to `PAGE::READONLY`.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) access right.
	WRITECOPY, 0x08

	SEC_COMMIT, 0x800_0000
	SEC_IMAGE, 0x100_0000
	SEC_IMAGE_NO_EXECUTE, 0x1100_0000
	SEC_LARGE_PAGES, 0x8000_0000
	SEC_NOCACHE, 0x1000_0000
	SEC_RESERVE, 0x400_0000
	SEC_WRITECOMBINE, 0x4000_0000
}

pub_struct_const_wm! { PBM,
	/// Progress bar control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-progress-bar-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	=>
	SETRANGE, WM::USER.0 + 1
	SETPOS, WM::USER.0 + 2
	DELTAPOS, WM::USER.0 + 3
	SETSTEP, WM::USER.0 + 4
	STEPIT, WM::USER.0 + 5
	SETRANGE32, WM::USER.0 + 6
	GETRANGE, WM::USER.0 + 7
	GETPOS, WM::USER.0 + 8
	SETBARCOLOR, WM::USER.0 + 9
	SETBKCOLOR, CCM::SETBKCOLOR.0
	SETMARQUEE, WM::USER.0 + 10
	GETSTEP, WM::USER.0 + 13
	GETBKCOLOR, WM::USER.0 + 14
	GETBARCOLOR, WM::USER.0 + 15
	SETSTATE, WM::USER.0 + 16
	GETSTATE, WM::USER.0 + 17
}

pub_struct_const_ws! { PBS,
	/// Progress bar control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/progress-bar-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	SMOOTH, 0x01
	VERTICAL, 0x04
	MARQUEE, 0x08
	SMOOTHREVERSE, 0x10
}

pub_struct_const! { PBST, u32,
	/// Progress bar
	/// [states](https://docs.microsoft.com/en-us/windows/win32/controls/pbm-setstate)
	/// (`u32`).
	=>
	NORMAL, 0x0001
	ERROR, 0x0002
	PAUSED, 0x0003
}

pub_struct_const! { PITCH, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfPitchAndFamily` (`u8`), used with
	/// [`FF`](crate::co::FF).
	=>
	DEFAULT, 0
	FIXED, 1
	VARIABLE, 2
}
impl PITCH {
	/// Composes [`LOGFONT`](crate::LOGFONT) `lfPitchAndFamily`.
	pub fn add_family(&mut self, family: FF) -> &PITCH {
		self.0 |= family.0;
		self
	}
}

pub_struct_const! { PM, u32,
	/// [`PeekMessage`](crate::PeekMessage) `wRemoveMsg` (`u32`).
	=>
	NOREMOVE, 0x0000
	REMOVE, 0x0001
	NOYIELD, 0x0002

	QS_INPUT, QS::INPUT.0 << 16
	QS_POSTMESSAGE, (QS::POSTMESSAGE.0 | QS::HOTKEY.0 | QS::TIMER.0) << 16
	QS_PAINT, QS::PAINT.0 << 16
	QS_SENDMESSAGE, QS::SENDMESSAGE.0 << 16
}

pub_struct_const! { PROCESSOR, u32,
	/// [`SYSTEM_INFO`](crate::SYSTEM_INFO) `dwProcessorType` (`u32`).
	=>
	INTEL_386, 386
	INTEL_486, 486
	INTEL_PENTIUM, 586
	INTEL_IA64, 2200
	AMD_X8664, 8664
	MIPS_R4000, 4000
	ALPHA_21064, 21064
	PPC_601, 601
	PPC_603, 603
	PPC_604, 604
	PPC_620, 620
	HITACHI_SH3, 10003
	HITACHI_SH3E, 10004
	HITACHI_SH4, 10005
	MOTOROLA_821, 821
	SHx_SH3, 103
	SHx_SH4, 104
	STRONGARM, 2577
	ARM720, 1824
	ARM820, 2080
	ARM920, 2336
	ARM_7TDMI, 70001
	OPTIL, 0x494f
}

pub_struct_const! { PROCESSOR_ARCHITECTURE, u16,
	/// [`SYSTEM_INFO`](crate::SYSTEM_INFO) `wProcessorArchitecture` (`u16`).
	=>
	INTEL, 0
	MIPS, 1
	ALPHA, 2
	PPC, 3
	SHX, 4
	ARM, 5
	IA64, 6
	ALPHA64, 7
	MSIL, 8
	AMD64, 9
	IA32_ON_WIN64, 10
	NEUTRAL, 11
	ARM64, 12
	ARM32_ON_WIN64, 13
	IA32_ON_ARM64, 14
	UNKNOWN, 0xffff
}

pub_struct_const! { PS, i32,
	/// [`CreatePen`](crate::HPEN::CreatePen) `iStyle` (`i32`).
	=>
	SOLID, 0
	DASH, 1
	DOT, 2
	DASHDOT, 3
	DASHDOTDOT, 4
	NULL, 5
	INSIDEFRAME, 6
}

pub_struct_const! { QUALITY, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfQuality` (`u8`).
	=>
	DEFAULT, 0
	DRAFT, 1
	PROOF, 2
	NONANTIALIASED, 3
	ANTIALIASED, 4
	CLEARTYPE, 5
	CLEARTYPE_NATURAL, 6
}

pub_struct_const! { QS, u32,
	/// [`GetQueueStatus`](crate::GetQueueStatus) `flags` (`u32`).
	=>
	KEY, 0x0001
	MOUSEMOVE, 0x0002
	MOUSEBUTTON, 0x0004
	POSTMESSAGE, 0x0008
	TIMER, 0x0010
	PAINT, 0x0020
	SENDMESSAGE, 0x0040
	HOTKEY, 0x0080
	ALLPOSTMESSAGE, 0x0100
	RAWINPUT, 0x0400
	TOUCH, 0x0800
	POINTER, 0x1000
	MOUSE, Self::MOUSEMOVE.0 | Self::MOUSEBUTTON.0
	INPUT, Self::MOUSE.0 | Self::KEY.0 | Self::RAWINPUT.0 | Self::TOUCH.0 | Self::POINTER.0
	ALLINPUT, Self::INPUT.0 | Self::POSTMESSAGE.0 | Self::TIMER.0 | Self::PAINT.0 | Self::HOTKEY.0 | Self::SENDMESSAGE.0
}

pub_struct_const_wm! { RB,
	/// Rebar control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-rebar-control-reference-messages)
	/// (`u32`), convertible to [`WM`](crate::co::WM).
	=>
	=>
	DELETEBAND, WM::USER.0 + 2
	GETBARINFO, WM::USER.0 + 3
	SETBARINFO, WM::USER.0 + 4
	SETPARENT, WM::USER.0 + 7
	HITTEST, WM::USER.0 + 8
	GETRECT, WM::USER.0 + 9
	INSERTBAND, WM::USER.0 + 10
	SETBANDINFO, WM::USER.0 + 11
	GETBANDCOUNT, WM::USER.0 + 12
	GETROWCOUNT, WM::USER.0 + 13
	GETROWHEIGHT, WM::USER.0 + 14
	IDTOINDEX, WM::USER.0 + 16
	GETTOOLTIPS, WM::USER.0 + 17
	SETTOOLTIPS, WM::USER.0 + 18
	SETBKCOLOR, WM::USER.0 + 19
	GETBKCOLOR, WM::USER.0 + 20
	SETTEXTCOLOR, WM::USER.0 + 21
	GETTEXTCOLOR, WM::USER.0 + 22
	SIZETORECT, WM::USER.0 + 23
	SETCOLORSCHEME, CCM::SETCOLORSCHEME.0
	GETCOLORSCHEME, CCM::GETCOLORSCHEME.0
	BEGINDRAG, WM::USER.0 + 24
	ENDDRAG, WM::USER.0 + 25
	DRAGMOVE, WM::USER.0 + 26
	GETBARHEIGHT, WM::USER.0 + 27
	GETBANDINFO, WM::USER.0 + 28
	MINIMIZEBAND, WM::USER.0 + 30
	MAXIMIZEBAND, WM::USER.0 + 31
	GETDROPTARGET, CCM::GETDROPTARGET.0
	GETBANDBORDERS, WM::USER.0 + 34
	SHOWBAND, WM::USER.0 + 35
	SETPALETTE, WM::USER.0 + 37
	GETPALETTE, WM::USER.0 + 38
	MOVEBAND, WM::USER.0 + 39
	SETUNICODEFORMAT, CCM::SETUNICODEFORMAT.0
	GETUNICODEFORMAT, CCM::GETUNICODEFORMAT.0
	GETBANDMARGINS, WM::USER.0 + 40
	SETWINDOWTHEME, CCM::SETWINDOWTHEME.0
	SETEXTENDEDSTYLE, WM::USER.0 + 41
	GETEXTENDEDSTYLE, WM::USER.0 + 42
	PUSHCHEVRON, WM::USER.0 + 43
	SETBANDWIDTH, WM::USER.0 + 44
}

pub_struct_const_nm! { RBN,
	/// Rebar control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-rebar-control-reference-notifications)
	/// (`i32`), convertible to [`NM`](crate::co::NM).
	=>
	FIRST, -831
	=>
	HEIGHTCHANGE, Self::FIRST.0 - 0
	GETOBJECT, Self::FIRST.0 - 1
	LAYOUTCHANGED, Self::FIRST.0 - 2
	AUTOSIZE, Self::FIRST.0 - 3
	BEGINDRAG, Self::FIRST.0 - 4
	ENDDRAG, Self::FIRST.0 - 5
	DELETINGBAND, Self::FIRST.0 - 6
	DELETEDBAND, Self::FIRST.0 - 7
	CHILDSIZE, Self::FIRST.0 - 8
	CHEVRONPUSHED, Self::FIRST.0 - 10
	SPLITTERDRAG, Self::FIRST.0 - 11
	MINMAX, Self::FIRST.0 - 21
	AUTOBREAK, Self::FIRST.0 - 22
}

pub_struct_const_ws! { RBS,
	/// Rebar control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/rebar-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	=>
	TOOLTIPS, 0x0000_0100
	VARHEIGHT, 0x0000_0200
	BANDBORDERS, 0x0000_0400
	FIXEDORDER, 0x0000_0800
	REGISTERDROP, 0x000_01000
	AUTOSIZE, 0x0000_2000
	VERTICALGRIPPER, 0x0000_4000
	DBLCLKTOGGLE, 0x0000_8000
}

pub_struct_const! { RDW, u32,
	/// [`RedrawWindow`](crate::HWND::RedrawWindow) `flags` (`u32`).
	=>
	INVALIDATE, 0x0001
	INTERNALPAINT, 0x0002
	ERASE, 0x0004
	VALIDATE, 0x0008
	NOINTERNALPAINT, 0x0010
	NOERASE, 0x0020
	NOCHILDREN, 0x0040
	ALLCHILDREN, 0x0080
	UPDATENOW, 0x0100
	ERASENOW, 0x0200
	FRAME, 0x0400
	NOFRAME, 0x0800
}

pub_struct_const! { REG, u32,
	/// Registry
	/// [value types](https://docs.microsoft.com/en-us/windows/win32/sysinfo/registry-value-types)
	/// (`u32`).
	=>
	NONE, 0
	SZ, 1
	EXPAND_SZ, 2
	BINARY, 3
	DWORD, 4
	DWORD_LITTLE_ENDIAN, 4
	DWORD_BIG_ENDIAN, 5
	LINK, 6
	MULTI_SZ, 7
	RESOURCE_LIST, 8
	FULL_RESOURCE_DESCRIPTOR, 9
	RESOURCE_REQUIREMENTS_LIST, 10
	QWORD, 11
	QWORD_LITTLE_ENDIAN, 11
}

pub_struct_const! { REG_OPTION, u32,
	/// [`RegOpenKeyEx`](crate::HKEY::RegOpenKeyEx) `uOptions` (`u32`).
	=>
	RESERVED, 0x0000_0000
	NON_VOLATILE, 0x0000_0000
	VOLATILE, 0x0000_0001
	CREATE_LINK, 0x0000_0002
	BACKUP_RESTORE, 0x0000_0004
	OPEN_LINK, 0x0000_0008
}

pub_struct_const! { REGION, i32,
	/// [`GetUpdateRgn`](crate::HWND::GetUpdateRgn),
	/// [`GetWindowRgn`](crate::HWND::GetWindowRgn) and
	/// [`SelectObjectRgn`](crate::HDC::SelectObjectRgn) return value (`i32`).
	=>
	NULL, 1
	SIMPLE, 2
	COMPLEX, 3
}

pub_struct_const! { ROP, u32,
	/// Raster operation code (`u32`).
	/// [`BitBlt`](crate::HDC::BitBlt) `rop`,
	/// [`PatBlt`](crate::HDC::PatBlt) `rop` and
	/// [`IMAGELISTDRAWPARAMS`](crate::IMAGELISTDRAWPARAMS) `dwRop`.
	/// Originally has no prefix.
	=>
	SRCCOPY, 0x00cc_0020
	SRCPAINT, 0x00ee_0086
	SRCAND, 0x0088_00c6
	SRCINVERT, 0x0066_0046
	SRCERASE,0x0044_0328
	NOTSRCCOPY, 0x0033_0008
	NOTSRCERASE, 0x0011_00a6
	MERGECOPY, 0x00c0_00ca
	MERGEPAINT, 0x00bb_0226
	PATCOPY, 0x00f0_0021
	PATPAINT, 0x00fb_0a09
	PATINVERT, 0x005a_0049
	DSTINVERT, 0x0055_0009
	BLACKNESS, 0x0000_0042
	WHITENESS, 0x00ff_0062
	NOMIRRORBITMAP, 0x8000_0000
	CAPTUREBLT, 0x4000_0000
}

pub_struct_const! { RRF, u32,
	/// [`RegGetValue`](crate::HKEY::RegGetValue) `dwFlags` (`u32`).
	=>
	RT_REG_NONE, 0x0000_0001
	RT_REG_SZ, 0x0000_0002
	RT_REG_EXPAND_SZ, 0x0000_0004
	RT_REG_BINARY, 0x0000_0008
	RT_REG_DWORD, 0x0000_0010
	RT_REG_MULTI_SZ, 0x0000_0020
	RT_REG_QWORD, 0x0000_0040
	RT_DWORD, Self::RT_REG_BINARY.0 | Self::RT_REG_DWORD.0
	RT_QWORD, Self::RT_REG_BINARY.0 | Self::RT_REG_QWORD.0
	RT_ANY, 0x0000_ffff

	SUBKEY_WOW6464KEY, 0x0001_0000
	SUBKEY_WOW6432KEY, 0x0002_0000
	WOW64_MASK, 0x0003_0000

	NOEXPAND, 0x1000_0000
	ZEROONFAILURE, 0x2000_0000
}
