#![allow(non_camel_case_types)]

const_bitflag! { DWM_CLOAKED: u32;
	/// [`DWMWA`](crate::co::DWMWA) cloaked flags (`u32`).
	=>
	APP 0x0000_0001
	SHELL 0x0000_0002
	INHERITED 0x0000_0004
}

const_ordinary! { DWM_SIT: u32;
	/// [`DwmSetIconicLivePreviewBitmap`](crate::HWND::DwmSetIconicLivePreviewBitmap)
	/// `sit_flags` (`u32`).
	=>
	DISPLAYFRAME 0x0000_0001
}

const_ordinary! { DWMFLIP3DWINDOWPOLICY: u32;
	/// [`DWMFLIP3DWINDOWPOLICY`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/ne-dwmapi-dwmflip3dwindowpolicy)
	/// enumeration (`u32`).
	=>
	DEFAULT 0
	EXCLUDEBELOW 1
	EXCLUDEABOVE 2
}

const_ordinary! { DWMNCRENDERINGPOLICY: u32;
	/// [`DWMNCRENDERINGPOLICY`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/ne-dwmapi-dwmncrenderingpolicy)
	/// enumeration (`u32`).
	=>
	USEWINDOWSTYLE 0
	DISABLED 1
	ENABLED 2
}

const_ordinary! { DWMSBT: u32;
	/// [`DWMWA`](crate::co::DWMWA) system backdrop type (`u32`).
	=>
	AUTO 0
	NONE 1
	MAINWINDOW 2
	TRANSIENTWINDOW 3
	TABBEDWINDOW 4
}

const_bitflag! { DWMSC: u32;
	/// [`DwmShowContact`](crate::DwmShowContact) `show_contact` (`u32`).
	=>
	NONE 0x0000_0000
	DOWN 0x0000_0001
	UP 0x0000_0002
	DRAG 0x0000_0004
	HOLD 0x0000_0008
	PENBARREL 0x0000_0010
	ALL 0xffff_ffff
}

const_ordinary! { DWMWA: u32;
	/// [`DWMWINDOWATTRIBUTE`](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/ne-dwmapi-dwmwindowattribute)
	/// enumeration (`u32`).
	=>
	NCRENDERING_ENABLED 1
	NCRENDERING_POLICY 2
	TRANSITIONS_FORCEDISABLED 3
	ALLOW_NCPAINT 4
	CAPTION_BUTTON_BOUNDS 5
	NONCLIENT_RTL_LAYOUT 6
	FORCE_ICONIC_REPRESENTATION 7
	FLIP3D_POLICY 8
	EXTENDED_FRAME_BOUNDS 9
	HAS_ICONIC_BITMAP 10
	DISALLOW_PEEK 11
	EXCLUDED_FROM_PEEK 12
	CLOAK 13
	CLOAKED 14
	FREEZE_REPRESENTATION 15
	PASSIVE_UPDATE_MODE 16
	/// Since Windows 11 Build 22000.
	USE_HOSTBACKDROPBRUSH 17
	/// Since Windows 11 Build 22000.
	USE_IMMERSIVE_DARK_MODE 20
	/// Since Windows 11 Build 22000.
	WINDOW_CORNER_PREFERENCE 33
	/// Since Windows 11 Build 22000.
	BORDER_COLOR 34
	/// Since Windows 11 Build 22000.
	CAPTION_COLOR 35
	/// Since Windows 11 Build 22000.
	TEXT_COLOR 36
	/// Since Windows 11 Build 22000.
	VISIBLE_FRAME_BORDER_THICKNESS 37
	/// Since Windows 11 Build 22621.
	SYSTEMBACKDROP_TYPE 38
}

const_ordinary! { DWMWCP: u32;
	/// [`DWMWA`](crate::co::DWMWA) window corner preference (`u32`).
	=>
	DEFAULT 0
	DONOTROUND 1
	ROUND 2
	ROUNDSMALL 3
}
