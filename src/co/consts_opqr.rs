use crate::co;

const_type! { OUT_PRECIS, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfOutPrecision`.

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

const_type! { PITCH, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfPitchAndFamily`, used with
	/// [`FF`](crate::co::FF).

	DEFAULT, 0
	FIXED, 1
	VARIABLE, 2
}
impl PITCH {
	/// Composes [`LOGFONT`](crate::LOGFONT) `lfPitchAndFamily`.
	pub fn add_family(&mut self, family: co::FF) -> &PITCH {
		self.0 |= family.0;
		self
	}
}

const_type! { PM, u32,
	/// [`PeekMessage`](crate::PeekMessage) `wRemoveMsg`.

	NOREMOVE, 0x0000
	REMOVE, 0x0001
	NOYIELD, 0x0002

	QS_INPUT, QS::INPUT.0 << 16
	QS_POSTMESSAGE, (QS::POSTMESSAGE.0 | QS::HOTKEY.0 | QS::TIMER.0) << 16
	QS_PAINT, QS::PAINT.0 << 16
	QS_SENDMESSAGE, QS::SENDMESSAGE.0 << 16
}

const_type! { QUALITY, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfQuality`.

	DEFAULT, 0
	DRAFT, 1
	PROOF, 2
	NONANTIALIASED, 3
	ANTIALIASED, 4
	CLEARTYPE, 5
	CLEARTYPE_NATURAL, 6
}

const_type! { QS, u32,
	/// [`GetQueueStatus`](crate::GetQueueStatus) `flags`.

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

const_type! { REGION, i32,
	/// [`GetUpdateRgn`](crate::HWND::GetUpdateRgn) and
	/// [`GetWindowRgn`](crate::HWND::GetWindowRgn) return value.

	NULL, 1
	SIMPLE, 2
	COMPLEX, 3
}