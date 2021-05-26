use crate::co::{CCM, FF, QS, WM};

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
