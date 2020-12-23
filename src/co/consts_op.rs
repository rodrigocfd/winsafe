use crate::co;

const_type! { OUT_PRECIS, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfOutPrecision` (`u8`).

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
	/// [`LOGFONT`](crate::LOGFONT) `lfPitchAndFamily` (`u8`), used with
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
	/// [`PeekMessage`](crate::PeekMessage) `wRemoveMsg` (`u32`).

	NOREMOVE, 0x0000
	REMOVE, 0x0001
	NOYIELD, 0x0002

	QS_INPUT, co::QS::INPUT.0 << 16
	QS_POSTMESSAGE, (co::QS::POSTMESSAGE.0 | co::QS::HOTKEY.0 | co::QS::TIMER.0) << 16
	QS_PAINT, co::QS::PAINT.0 << 16
	QS_SENDMESSAGE, co::QS::SENDMESSAGE.0 << 16
}