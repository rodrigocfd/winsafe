const_type! { EMF, u32,
	/// [`NMLVEMPTYMARKUP`](crate::NMLVEMPTYMARKUP) `dwFlags`.

	LEFT, 0x00000000
	CENTERED, 0x00000001
}

const_type! { FF, u8,
	/// [`LOGFONT`](crate::LOGFONT) `lfPitchAndFamily`, used with
	/// [`PITCH`](crate::co::PITCH).

	DONTCARE, 0 << 4
	ROMAN, 1 << 4
	SWISS, 2 << 4
	MODERN, 3 << 4
	SCRIPT, 4 << 4
	DECORATIVE, 5 << 4
}

const_type! { FORMAT_MESSAGE, u32,
	/// [`FormatMessage`](crate::co::ERROR::FormatMessage) `dwFlags`.

	ALLOCATE_BUFFER, 0x00000100
	ARGUMENT_ARRAY, 0x00002000
	FROM_HMODULE, 0x00000800
	FROM_STRING, 0x00000400
	FROM_SYSTEM, 0x00001000
	IGNORE_INSERTS, 0x00000200
	MAX_WIDTH_MASK, 0x000000ff
}

const_type! { FW, u32,
	/// [`LOGFONT`](crate::LOGFONT) `lfWeight`.

	DONTCARE, 0
	THIN, 100
	EXTRALIGHT, 200
	ULTRALIGHT, Self::EXTRALIGHT.0
	LIGHT, 300
	NORMAL, 400
	REGULAR, 400
	MEDIUM, 500
	SEMIBOLD, 600
	DEMIBOLD, Self::SEMIBOLD.0
	BOLD, 700
	EXTRABOLD, 800
	ULTRABOLD, Self::EXTRABOLD.0
	HEAVY, 900
	BLACK, Self::HEAVY.0
}