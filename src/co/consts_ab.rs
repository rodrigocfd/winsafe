const_type!(ACCELF, u8,
	"[`ACCELL`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-accel)
	`fVirt`.");
impl ACCELF {
	const_val!(NONE, 0);
	const_val!(VIRTKEY, 1);
	const_val!(SHIFT, 0x04);
	const_val!(CONTROL, 0x08);
	const_val!(ALT, 0x10);
}

const_type!(ACCESS_RIGHTS, u32,
	"[`RegOpenKeyEx`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw)
	`samDesired`.");
impl ACCESS_RIGHTS {
	const_val!(DELETE, 0x00010000);
	const_val!(READ_CONTROL, 0x00020000);
	const_val!(WRITE_DAC, 0x00040000);
	const_val!(WRITE_OWNER, 0x00080000);
	const_val!(SYNCHRONIZE, 0x00100000);
}

const_type!(ADRF, u32,
	"[`NMTVASYNCDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvasyncdraw)
	`dwRetFlags`. Don't seem to be defined anywhere, unconfirmed values.");
impl ADRF {
	const_val!(DRAWSYNC, 0);
	const_val!(DRAWNOTHING, 1);
	const_val!(DRAWFALLBACK, 2);
	const_val!(DRAWIMAGE, 3);
}

const_type!(APPCOMMAND, i16,
	"[`WM_APPCOMMAND`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand)
	commands.");
impl APPCOMMAND {
	const_val!(BROWSER_BACKWARD, 1);
	const_val!(BROWSER_FORWARD, 2);
	const_val!(BROWSER_REFRESH, 3);
	const_val!(BROWSER_STOP, 4);
	const_val!(BROWSER_SEARCH, 5);
	const_val!(BROWSER_FAVORITES, 6);
	const_val!(BROWSER_HOME, 7);
	const_val!(VOLUME_MUTE, 8);
	const_val!(VOLUME_DOWN, 9);
	const_val!(VOLUME_UP, 10);
	const_val!(MEDIA_NEXTTRACK, 11);
	const_val!(MEDIA_PREVIOUSTRACK, 12);
	const_val!(MEDIA_STOP, 13);
	const_val!(MEDIA_PLAY_PAUSE, 14);
	const_val!(LAUNCH_MAIL, 15);
	const_val!(LAUNCH_MEDIA_SELECT, 16);
	const_val!(LAUNCH_APP1, 17);
	const_val!(LAUNCH_APP2, 18);
	const_val!(BASS_DOWN, 19);
	const_val!(BASS_BOOST, 20);
	const_val!(BASS_UP, 21);
	const_val!(TREBLE_DOWN, 22);
	const_val!(TREBLE_UP, 23);
	const_val!(MICROPHONE_VOLUME_MUTE, 24);
	const_val!(MICROPHONE_VOLUME_DOWN, 25);
	const_val!(MICROPHONE_VOLUME_UP, 26);
	const_val!(HELP, 27);
	const_val!(FIND, 28);
	const_val!(NEW, 29);
	const_val!(OPEN, 30);
	const_val!(CLOSE, 31);
	const_val!(SAVE, 32);
	const_val!(PRINT, 33);
	const_val!(UNDO, 34);
	const_val!(REDO, 35);
	const_val!(COPY, 36);
	const_val!(CUT, 37);
	const_val!(PASTE, 38);
	const_val!(REPLY_TO_MAIL, 39);
	const_val!(FORWARD_MAIL, 40);
	const_val!(SEND_MAIL, 41);
	const_val!(SPELL_CHECK, 42);
	const_val!(DICTATE_OR_COMMAND_CONTROL_TOGGLE, 43);
	const_val!(MIC_ON_OFF_TOGGLE, 44);
	const_val!(CORRECTION_LIST, 45);
	const_val!(MEDIA_PLAY, 46);
	const_val!(MEDIA_PAUSE, 47);
	const_val!(MEDIA_RECORD, 48);
	const_val!(MEDIA_FAST_FORWARD, 49);
	const_val!(MEDIA_REWIND, 50);
	const_val!(MEDIA_CHANNEL_UP, 51);
	const_val!(MEDIA_CHANNEL_DOWN, 52);
	const_val!(DELETE, 53);
	const_val!(DWM_FLIP3D, 54);
}

const_type!(BCN, i32,
	"Button control
	[notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications),
	sent via
	[WM_NOTIFY](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify).");
impl BCN {
	priv_const_val!(FIRST, -1250);

	const_val!(HOTITEMCHANGE, BCN::FIRST.0 + 0x0001);
	const_val!(DROPDOWN, BCN::FIRST.0 + 0x0002);
}

const_type!(BI, u32,
	"[`BITMAPINFOHEADER`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader)
	`biCompression`.");
impl BI {
	const_val!(RGB, 0);
	const_val!(RLE8, 1);
	const_val!(RLE4, 2);
	const_val!(BITFIELDS, 3);
	const_val!(JPEG, 4);
	const_val!(PNG, 5);
}

const_type!(BKMODE, i32,
	"[`SetBkMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	`mode`.");
impl BKMODE {
	const_val!(TRANSPARENT, 1);
	const_val!(OPAQUE, 2);
}