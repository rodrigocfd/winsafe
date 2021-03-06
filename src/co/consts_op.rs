use crate::co::{FF, QS, WS};

const_type! { OCR, u32,
	/// [`SetSystemCursor`](crate::HCURSOR::SetSystemCursor) `id` (`u32`).
	->
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

const_type! { OUT_PRECIS, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfOutPrecision` (`u8`).
	->
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

const_type_ws! { PBS,
	/// Progress bar control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/progress-bar-control-styles)
	/// (`u32`), convertible to [`WS`](crate::co::WS).
	->
	SMOOTH, 0x01
	VERTICAL, 0x04
	MARQUEE, 0x08
	SMOOTHREVERSE, 0x10
}

const_type! { PITCH, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfPitchAndFamily` (`u8`), used with
	/// [`FF`](crate::co::FF).
	->
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

const_type! { PM, u32,
	/// [`PeekMessage`](crate::PeekMessage) `wRemoveMsg` (`u32`).
	->
	NOREMOVE, 0x0000
	REMOVE, 0x0001
	NOYIELD, 0x0002

	QS_INPUT, QS::INPUT.0 << 16
	QS_POSTMESSAGE, (QS::POSTMESSAGE.0 | QS::HOTKEY.0 | QS::TIMER.0) << 16
	QS_PAINT, QS::PAINT.0 << 16
	QS_SENDMESSAGE, QS::SENDMESSAGE.0 << 16
}
