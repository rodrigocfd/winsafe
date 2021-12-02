#![allow(non_camel_case_types, non_upper_case_globals)]

const_bitflag! { DDL: u16;
	/// [`cb::Dir`](crate::msg::cb::Dir) and [`lb::Dir`](crate::msg::lb::Dir)
	/// attributes (`u16`).
	=>
	=>
	READWRITE 0x0000
	READONLY 0x0001
	HIDDEN 0x0002
	SYSTEM 0x0004
	DIRECTORY 0x0010
	ARCHIVE 0x0020
	POSTMSGS 0x2000
	DRIVES 0x4000
	EXCLUSIVE 0x8000
}

const_ordinary! { DIB: u32;
	/// [`LOGBRUSH`](crate::LOGBRUSH) `lbColor` (`u32`).
	=>
	=>
	/// The color table consists of an array of 16-bit indexes into the
	/// currently realized logical palette.
	RGB_COLORS 0
	/// The color table contains literal RGB values.
	PAL_COLORS 1
}

const_ordinary! { DISP_CHANGE: i32;
	/// [`ChangeDisplaySettings`](crate::ChangeDisplaySettings) return value
	/// (`u32`).
	=>
	=>
	SUCCESSFUL 0
	RESTART 1
	FAILED -1
	BADMODE -2
	NOTUPDATED -3
	BADFLAGS -4
	BADPARAM -5
	BADDUALVIEW -6
}

const_ordinary! { DISPOSITION: u32;
	/// [`HFILE::CreateFile`](crate::HFILE::CreateFile) `creation_disposition`
	/// (`u32`).
	///
	/// Originally has no prefix.
	=>
	=>
	/// Creates a new file only if it does not already exist.
	///
	/// If the specified file exists the function fails and the last-error code
	/// is set to [`ERROR::FILE_EXISTS`](crate::co::ERROR::FILE_EXISTS).
	///
	/// If the specified file does not exist and is a valid path to a writable
	/// location a new file is created.
	CREATE_NEW 1
	/// Creates a new file always.
	///
	/// If the specified file exists and is writable the function overwrites
	/// the file the function succeeds and last-error code is set to
	/// [`ERROR::ALREADY_EXISTS`](crate::co::ERROR::ALREADY_EXISTS).
	///
	/// If the specified file does not exist and is a valid path a new file is
	/// created the function succeeds and the last-error code is set to
	/// [`ERROR::SUCCESS`](crate::co::ERROR::SUCCESS).
	CREATE_ALWAYS 2
	/// Opens a file or device only if it exists.
	///
	/// If the specified file or device does not exist the function fails and
	/// the last-error code is set to
	/// [`ERROR::FILE_NOT_FOUND`](crate::co::ERROR::FILE_NOT_FOUND).
	OPEN_EXISTING 3
	/// Opens a file always.
	///
	/// If the specified file exists the function succeeds and the last-error
	/// code is set to
	/// [`ERROR::ALREADY_EXISTS`](crate::co::ERROR::ALREADY_EXISTS).
	///
	/// If the specified file does not exist and is a valid path to a writable
	/// location the function creates a file and the last-error code is set to
	/// [`ERROR::SUCCESS`](crate::co::ERROR::SUCCESS).
	OPEN_ALWAYS 4
	/// Opens a file and truncates it so that its size is zero bytes only if it
	/// exists.
	///
	/// If the specified file does not exist the function fails and the
	/// last-error code is set to
	/// [`ERROR::FILE_NOT_FOUND`](crate::co::ERROR::FILE_NOT_FOUND).
	///
	/// The calling process must open the file with the
	/// [`GENERIC::WRITE`](crate::co::GENERIC::WRITE) bit set as part of the
	/// `dwDesiredAccess` parameter.
	TRUNCATE_EXISTING 5
}

const_ordinary! { DLGC: u16;
	/// [`wm::GetDlgCode`](crate::msg::wm::GetDlgCode) return value (`u16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	BUTTON 0x2000
	DEFPUSHBUTTON 0x0010
	HASSETSEL 0x0008
	RADIOBUTTON 0x0040
	STATIC 0x0100
	UNDEFPUSHBUTTON 0x0020
	WANTALLKEYS 0x0004
	WANTARROWS 0x0001
	WANTCHARS 0x0080
	WANTMESSAGE 0x0004
	WANTTAB 0x0002
}

const_ordinary! { DLGID: u16;
	/// Dialog built-in IDs (`u16`). These are also returned from
	/// [`HWND::MessageBox`](crate::HWND::MessageBox) and
	/// [`HWND::TaskDialog`](crate::HWND::TaskDialog).
	=>
	=>
	OK 1
	CANCEL 2
	ABORT 3
	RETRY 4
	IGNORE 5
	YES 6
	NO 7
	TRYAGAIN 10
	CONTINUE 11
}

const_bitflag! { DM: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmFields` (`u32`).
	=>
	=>
	ORIENTATION 0x0000_0001
	PAPERSIZE 0x0000_0002
	PAPERLENGTH 0x0000_0004
	PAPERWIDTH 0x0000_0008
	SCALE 0x0000_0010
	POSITION 0x0000_0020
	NUP 0x0000_0040
	DISPLAYORIENTATION 0x0000_0080
	COPIES 0x0000_0100
	DEFAULTSOURCE 0x0000_0200
	PRINTQUALITY 0x0000_0400
	COLOR 0x0000_0800
	DUPLEX 0x0000_1000
	YRESOLUTION 0x0000_2000
	TTOPTION 0x0000_4000
	COLLATE 0x0000_8000
	FORMNAME 0x0001_0000
	LOGPIXELS 0x0002_0000
	BITSPERPEL 0x0004_0000
	PELSWIDTH 0x0008_0000
	PELSHEIGHT 0x0010_0000
	DISPLAYFLAGS 0x0020_0000
	DISPLAYFREQUENCY 0x0040_0000
	ICMMETHOD 0x0080_0000
	ICMINTENT 0x0100_0000
	MEDIATYPE 0x0200_0000
	DITHERTYPE 0x0400_0000
	PANNINGWIDTH 0x0800_0000
	PANNINGHEIGHT 0x1000_0000
	DISPLAYFIXEDOUTPUT 0x2000_0000
}

const_ordinary! { DMBIN: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmDefaultSource` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	UPPER 1
	ONLYONE 1
	LOWER 2
	MIDDLE 3
	MANUAL 4
	ENVELOPE 5
	ENVMANUAL 6
	AUTO 7
	TRACTOR 8
	SMALLFMT 9
	LARGEFMT 10
	LARGECAPACITY 11
	CASSETTE 14
	FORMSOURCE 15
	LAST Self::FORMSOURCE.0
	/// Device-specific bins start here.
	USER 256
}

const_ordinary! { DMCOLOR: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmColor` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	MONOCHROME 1
	COLOR 2
}

const_bitflag! { DMDISPLAYFLAGS: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmDisplayFlags` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	INTERLACED 0x0000_0002
	TEXTMODE 0x0000_0004
}

const_ordinary! { DMICMMETHOD: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmICMMethod` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// ICM disabled.
	NONE 1
	/// ICM handled by system.
	SYSTEM 2
	/// ICM handled by driver.
	DRIVER 3
	/// ICM handled by device.
	DEVICE 4
	/// Device-specific intents start here.
	USER 256
}

const_ordinary! { DMDFO: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmDisplayFixedOutput` (`u32`).
	=>
	=>
	DEFAULT 0
	STRETCH 1
	CENTER 2
}

const_ordinary! { DMDITHER: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmDitherType` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// No dithering.
	NONE 1
	/// Dither with a coarse brush.
	COARSE 2
	/// Dither with a fine brush.
	FINE 3
	/// LineArt dithering.
	LINEART 4
	/// LineArt dithering.
	ERRORDIFFUSION 5
	/// LineArt dithering.
	RESERVED6 6
	/// LineArt dithering.
	RESERVED7 7
	/// LineArt dithering.
	RESERVED8 8
	/// LineArt dithering.
	RESERVED9 9
	/// Device does grayscaling.
	GRAYSCALE 10
	/// Device-specific dithers start here.
	USER 256
}

const_ordinary! { DMDO: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmDisplayOrientation` (`u32`).
	=>
	=>
	DEFAULT 0
	D90 1
	D180 2
	D270 3
}

const_ordinary! { DMDUP: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmDuplex` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	SIMPLEX 1
	VERTICAL 2
	HORIZONTAL 3
}

const_ordinary! { DMICM: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmICMIntent` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Maximize color saturation.
	SATURATE 1
	/// Maximize color contrast.
	CONTRAST 2
	/// Use specific color metric.
	COLORIMETRIC 3
	/// Use specific color metric.
	ABS_COLORIMETRIC 4
	/// Device-specific intents start here.
	USER 256
}

const_ordinary! { DMMEDIA: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmMediaType` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Standard paper.
	STANDARD 1
	/// Transparency.
	TRANSPARENCY 2
	/// Glossy paper.
	GLOSSY 3
	/// Device-specific media start here.
	USER 256
}

const_ordinary! { DMNUP: u32;
	/// [`DEVMODE`](crate::DEVMODE) `dmNup` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	SYSTEM 1
	ONEUP 2
}

const_ordinary! { DMORIENT: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmOrientation` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	PORTRAIT 1
	LANDSCAPE 2
}

const_ordinary! { DMPAPER: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmPaperSize` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Letter 8 1/2 x 11 in.
	LETTER 1
	/// Letter Small 8 1/2 x 11 in.
	LETTERSMALL 2
	/// Tabloid 11 x 17 in.
	TABLOID 3
	/// Ledger 17 x 11 in.
	LEDGER 4
	/// Legal 8 1/2 x 14 in.
	LEGAL 5
	/// Statement 5 1/2 x 8 1/2 in.
	STATEMENT 6
	/// Executive 7 1/4 x 10 1/2 in.
	EXECUTIVE 7
	/// A3 297 x 420 mm.
	A3 8
	/// A4 210 x 297 mm.
	A4 9
	/// A4 Small 210 x 297 mm.
	A4SMALL 10
	/// A5 148 x 210 mm.
	A5 11
	/// B4 (JIS) 250 x 354.
	B4 12
	/// B5 (JIS) 182 x 257 mm.
	B5 13
	/// Folio 8 1/2 x 13 in.
	FOLIO 14
	/// Quarto 215 x 275 mm.
	QUARTO 15
	/// 10x14 in.
	P10X14 16
	/// 11x17 in.
	P11X17 17
	/// Note 8 1/2 x 11 in.
	NOTE 18
	/// Envelope #9 3 7/8 x 8 7/8.
	ENV_9 19
	/// Envelope #10 4 1/8 x 9 1/2.
	ENV_10 20
	/// Envelope #11 4 1/2 x 10 3/8.
	ENV_11 21
	/// Envelope #12 4 \276 x 11.
	ENV_12 22
	/// Envelope #14 5 x 11 1/2.
	ENV_14 23
	/// C size sheet.
	CSHEET 24
	/// D size sheet.
	DSHEET 25
	/// E size sheet.
	ESHEET 26
	/// Envelope DL 110 x 220mm.
	ENV_DL 27
	/// Envelope C5 162 x 229 mm.
	ENV_C5 28
	/// Envelope C3 324 x 458 mm.
	ENV_C3 29
	/// Envelope C4 229 x 324 mm.
	ENV_C4 30
	/// Envelope C6 114 x 162 mm.
	ENV_C6 31
	/// Envelope C65 114 x 229 mm.
	ENV_C65 32
	/// Envelope B4 250 x 353 mm.
	ENV_B4 33
	/// Envelope B5 176 x 250 mm.
	ENV_B5 34
	/// Envelope B6 176 x 125 mm.
	ENV_B6 35
	/// Envelope 110 x 230 mm.
	ENV_ITALY 36
	/// Envelope Monarch 3.875 x 7.5 in.
	ENV_MONARCH 37
	/// 6 3/4 Envelope 3 5/8 x 6 1/2 in.
	ENV_PERSONAL 38
	/// US Std Fanfold 14 7/8 x 11 in.
	FANFOLD_US 39
	/// German Std Fanfold 8 1/2 x 12 in.
	FANFOLD_STD_GERMAN 40
	/// German Legal Fanfold 8 1/2 x 13 in.
	FANFOLD_LGL_GERMAN 41
	/// B4 (ISO) 250 x 353 mm.
	ISO_B4 42
	/// Japanese Postcard 100 x 148 mm.
	JAPANESE_POSTCARD 43
	/// 9 x 11 in.
	P9X11 44
	/// 10 x 11 in.
	P10X11 45
	/// 15 x 11 in.
	P15X11 46
	/// Envelope Invite 220 x 220 mm.
	ENV_INVITE 47
	/// Letter Extra 9 275 x 12 in.
	LETTER_EXTRA 50
	/// Legal Extra 9 275 x 15 in.
	LEGAL_EXTRA 51
	/// Tabloid Extra 11.69 x 18 in.
	TABLOID_EXTRA 52
	/// A4 Extra 9.27 x 12.69 in.
	A4_EXTRA 53
	/// Letter Transverse 8 275 x 11 in.
	LETTER_TRANSVERSE 54
	/// A4 Transverse 210 x 297 mm.
	A4_TRANSVERSE 55
	/// Letter Extra Transverse 9\275 x 12 in.
	LETTER_EXTRA_TRANSVERSE 56
	/// SuperA/SuperA/A4 227 x 356 mm.
	A_PLUS 57
	/// SuperB/SuperB/A3 305 x 487 mm.
	B_PLUS 58
	/// Letter Plus 8.5 x 12.69 in.
	ETTER_PLUS 59
	/// A4 Plus 210 x 330 mm.
	A4_PLUS 60
	/// A5 Transverse 148 x 210 mm.
	A5_TRANSVERSE 61
	/// B5 (JIS) Transverse 182 x 257 mm.
	B5_TRANSVERSE 62
	/// A3 Extra 322 x 445 mm.
	A3_EXTRA 63
	/// A5 Extra 174 x 235 mm.
	A5_EXTRA 64
	/// B5 (ISO) Extra 201 x 276 mm.
	B5_EXTRA 65
	/// A2 420 x 594 mm.
	A2 66
	/// A3 Transverse 297 x 420 mm.
	A3_TRANSVERSE 67
	/// A3 Extra Transverse 322 x 445 mm.
	A3_EXTRA_TRANSVERSE 68
	/// Japanese Double Postcard 200 x 148 mm.
	DBL_JAPANESE_POSTCARD 69
	/// A6 105 x 148 mm.
	A6 70
	/// Japanese Envelope Kaku #2.
	JENV_KAKU2 71
	/// Japanese Envelope Kaku #3.
	JENV_KAKU3 72
	/// Japanese Envelope Chou #3.
	JENV_CHOU3 73
	/// Japanese Envelope Chou #4.
	JENV_CHOU4 74
	/// Letter Rotated 11 x 8 1/2 11 in.
	LETTER_ROTATED 75
	/// A3 Rotated 420 x 297 mm.
	A3_ROTATED 76
	/// A4 Rotated 297 x 210 mm.
	A4_ROTATED 77
	/// A5 Rotated 210 x 148 mm.
	A5_ROTATED 78
	/// B4 (JIS) Rotated 364 x 257 mm.
	B4_JIS_ROTATED 79
	/// B5 (JIS) Rotated 257 x 182 mm.
	B5_JIS_ROTATED 80
	/// Japanese Postcard Rotated 148 x 100 mm.
	JAPANESE_POSTCARD_ROTATED 81
	/// Double Japanese Postcard Rotated 148 x 200 mm.
	DBL_JAPANESE_POSTCARD_ROTATED 82
	/// A6 Rotated 148 x 105 mm.
	A6_ROTATED 83
	/// Japanese Envelope Kaku #2 Rotated.
	JENV_KAKU2_ROTATED 84
	/// Japanese Envelope Kaku #3 Rotated.
	JENV_KAKU3_ROTATED 85
	/// Japanese Envelope Chou #3 Rotated.
	JENV_CHOU3_ROTATED 86
	/// Japanese Envelope Chou #4 Rotated.
	JENV_CHOU4_ROTATED 87
	/// B6 (JIS) 128 x 182 mm.
	B6_JIS 88
	/// B6 (JIS) Rotated 182 x 128 mm.
	B6_JIS_ROTATED 89
	/// 12 x 11 in.
	P12X11 90
	/// Japanese Envelope You #4.
	JENV_YOU4 91
	/// Japanese Envelope You #4 Rotated.
	JENV_YOU4_ROTATED 92
	/// PRC 16K 146 x 215 mm.
	P16K 93
	/// PRC 32K 97 x 151 mm.
	P32K 94
	/// PRC 32K (Big) 97 x 151 mm.
	P32KBIG 95
	/// PRC Envelope #1 102 x 165 mm.
	PENV_1 96
	/// PRC Envelope #2 102 x 176 mm.
	PENV_2 97
	/// PRC Envelope #3 125 x 176 mm.
	PENV_3 98
	/// PRC Envelope #4 110 x 208 mm.
	PENV_4 99
	/// PRC Envelope #5 110 x 220 mm.
	PENV_5 100
	/// PRC Envelope #6 120 x 230 mm.
	PENV_6 101
	/// PRC Envelope #7 160 x 230 mm.
	PENV_7 102
	/// PRC Envelope #8 120 x 309 mm.
	PENV_8 103
	/// PRC Envelope #9 229 x 324 mm.
	PENV_9 104
	/// PRC Envelope #10 324 x 458 mm.
	PENV_10 105
	/// PRC 16K Rotated.
	P16K_ROTATED 106
	/// PRC 32K Rotated.
	P32K_ROTATED 107
	/// PRC 32K(Big) Rotated.
	P32KBIG_ROTATED 108
	/// PRC Envelope #1 Rotated 165 x 102 mm.
	PENV_1_ROTATED 109
	/// PRC Envelope #2 Rotated 176 x 102 mm.
	PENV_2_ROTATED 110
	/// PRC Envelope #3 Rotated 176 x 125 mm.
	PENV_3_ROTATED 111
	/// PRC Envelope #4 Rotated 208 x 110 mm.
	PENV_4_ROTATED 112
	/// PRC Envelope #5 Rotated 220 x 110 mm.
	PENV_5_ROTATED 113
	/// PRC Envelope #6 Rotated 230 x 120 mm.
	PENV_6_ROTATED 114
	/// PRC Envelope #7 Rotated 230 x 160 mm.
	PENV_7_ROTATED 115
	/// PRC Envelope #8 Rotated 309 x 120 mm.
	PENV_8_ROTATED 116
	/// PRC Envelope #9 Rotated 324 x 229 mm.
	PENV_9_ROTATED 117
	/// PRC Envelope #10 Rotated 458 x 324 mm.
	PENV_10_ROTATED 118
	/// Other papers start here.
	USER 256
}

const_ordinary! { DMRES: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmPrintQuality` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	DRAFT -1
	LOW -2
	MEDIUM -3
	HIGH -4
}

const_ordinary! { DMTT: i16;
	/// [`DEVMODE`](crate::DEVMODE) `dmTTOption` (`i16`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// Print TT fonts as graphics.
	BITMAP 1
	/// Download TT fonts as soft fonts.
	DOWNLOAD 2
	/// Substitude device fonts for TT fonts.
	SUBDEV 3
	/// Download TT fonts as outline soft fonts.
	DOWNLOAD_OUTLINE 4
}

const_wm! { DTM;
	/// Date and time picker control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-messages)
	/// (`u32`).
	=>
	FIRST 0x1000
	=>
	GETSYSTEMTIME Self::FIRST.0 + 1
	SETSYSTEMTIME Self::FIRST.0 + 2
	GETRANGE Self::FIRST.0 + 3
	SETRANGE Self::FIRST.0 + 4
	SETFORMAT Self::FIRST.0 + 50
	SETMCCOLOR Self::FIRST.0 + 6
	GETMCCOLOR Self::FIRST.0 + 7
	GETMONTHCAL Self::FIRST.0 + 8
	SETMCFONT Self::FIRST.0 + 9
	GETMCFONT Self::FIRST.0 + 10
	SETMCSTYLE Self::FIRST.0 + 11
	GETMCSTYLE Self::FIRST.0 + 12
	CLOSEMONTHCAL Self::FIRST.0 + 13
	GETDATETIMEPICKERINFO Self::FIRST.0 + 14
	GETIDEALSIZE Self::FIRST.0 + 15
}

const_nm! { DTN;
	/// Date and time picker control `WM_NOTIFY`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-date-and-time-picker-control-reference-notifications)
	/// (`i32`).
	=>
	FIRST -740
	FIRST2 -753
	=>
	CLOSEUP Self::FIRST2.0
	DATETIMECHANGE Self::FIRST2.0 - 6
	DROPDOWN Self::FIRST2.0 - 1
	FORMAT Self::FIRST2.0 - 3
	FORMATQUERY Self::FIRST.0 - 3
	USERSTRING Self::FIRST.0 - 5
	WMKEYDOWN Self::FIRST.0 - 4
}

const_ws! { DTS: u32;
	/// Date and time picker control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/date-and-time-picker-control-styles)
	/// (`u32`).
	=>
	=>
	UPDOWN 0x0001
	SHOWNONE 0x0002
	SHORTDATEFORMAT 0x0000
	LONGDATEFORMAT 0x0004
	SHORTDATECENTURYFORMAT 0x000c
	TIMEFORMAT 0x0009
	APPCANPARSE 0x0010
	RIGHTALIGN 0x0020
}
