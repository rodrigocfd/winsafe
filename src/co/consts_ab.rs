decl!(ACCELF, u8,
	"[`ACCELL`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-accel)
	`fVirt`.");
impl ACCELF {
	val!(ACCELF, NONE, 0);
	val!(ACCELF, VIRTKEY, 1);
	val!(ACCELF, SHIFT, 0x04);
	val!(ACCELF, CONTROL, 0x08);
	val!(ACCELF, ALT, 0x10);
}

decl!(ACCESS_RIGHTS, u32,
	"[`RegOpenKeyEx`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw)
	`samDesired`.");
impl ACCESS_RIGHTS {
	val!(ACCESS_RIGHTS, DELETE, 0x00010000);
	val!(ACCESS_RIGHTS, READ_CONTROL, 0x00020000);
	val!(ACCESS_RIGHTS, WRITE_DAC, 0x00040000);
	val!(ACCESS_RIGHTS, WRITE_OWNER, 0x00080000);
	val!(ACCESS_RIGHTS, SYNCHRONIZE, 0x00100000);
}

decl!(ADRF, u32,
	"[`NMTVASYNCDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvasyncdraw)
	`dwRetFlags`. Don't seem to be defined anywhere, unconfirmed values.");
impl ADRF {
	val!(ADRF, DRAWSYNC, 0);
	val!(ADRF, DRAWNOTHING, 1);
	val!(ADRF, DRAWFALLBACK, 2);
	val!(ADRF, DRAWIMAGE, 3);
}

decl!(APPCOMMAND, i16,
	"[`WM_APPCOMMAND`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand)
	commands.");
impl APPCOMMAND {
	val!(APPCOMMAND, BROWSER_BACKWARD, 1);
	val!(APPCOMMAND, BROWSER_FORWARD, 2);
	val!(APPCOMMAND, BROWSER_REFRESH, 3);
	val!(APPCOMMAND, BROWSER_STOP, 4);
	val!(APPCOMMAND, BROWSER_SEARCH, 5);
	val!(APPCOMMAND, BROWSER_FAVORITES, 6);
	val!(APPCOMMAND, BROWSER_HOME, 7);
	val!(APPCOMMAND, VOLUME_MUTE, 8);
	val!(APPCOMMAND, VOLUME_DOWN, 9);
	val!(APPCOMMAND, VOLUME_UP, 10);
	val!(APPCOMMAND, MEDIA_NEXTTRACK, 11);
	val!(APPCOMMAND, MEDIA_PREVIOUSTRACK, 12);
	val!(APPCOMMAND, MEDIA_STOP, 13);
	val!(APPCOMMAND, MEDIA_PLAY_PAUSE, 14);
	val!(APPCOMMAND, LAUNCH_MAIL, 15);
	val!(APPCOMMAND, LAUNCH_MEDIA_SELECT, 16);
	val!(APPCOMMAND, LAUNCH_APP1, 17);
	val!(APPCOMMAND, LAUNCH_APP2, 18);
	val!(APPCOMMAND, BASS_DOWN, 19);
	val!(APPCOMMAND, BASS_BOOST, 20);
	val!(APPCOMMAND, BASS_UP, 21);
	val!(APPCOMMAND, TREBLE_DOWN, 22);
	val!(APPCOMMAND, TREBLE_UP, 23);
	val!(APPCOMMAND, MICROPHONE_VOLUME_MUTE, 24);
	val!(APPCOMMAND, MICROPHONE_VOLUME_DOWN, 25);
	val!(APPCOMMAND, MICROPHONE_VOLUME_UP, 26);
	val!(APPCOMMAND, HELP, 27);
	val!(APPCOMMAND, FIND, 28);
	val!(APPCOMMAND, NEW, 29);
	val!(APPCOMMAND, OPEN, 30);
	val!(APPCOMMAND, CLOSE, 31);
	val!(APPCOMMAND, SAVE, 32);
	val!(APPCOMMAND, PRINT, 33);
	val!(APPCOMMAND, UNDO, 34);
	val!(APPCOMMAND, REDO, 35);
	val!(APPCOMMAND, COPY, 36);
	val!(APPCOMMAND, CUT, 37);
	val!(APPCOMMAND, PASTE, 38);
	val!(APPCOMMAND, REPLY_TO_MAIL, 39);
	val!(APPCOMMAND, FORWARD_MAIL, 40);
	val!(APPCOMMAND, SEND_MAIL, 41);
	val!(APPCOMMAND, SPELL_CHECK, 42);
	val!(APPCOMMAND, DICTATE_OR_COMMAND_CONTROL_TOGGLE, 43);
	val!(APPCOMMAND, MIC_ON_OFF_TOGGLE, 44);
	val!(APPCOMMAND, CORRECTION_LIST, 45);
	val!(APPCOMMAND, MEDIA_PLAY, 46);
	val!(APPCOMMAND, MEDIA_PAUSE, 47);
	val!(APPCOMMAND, MEDIA_RECORD, 48);
	val!(APPCOMMAND, MEDIA_FAST_FORWARD, 49);
	val!(APPCOMMAND, MEDIA_REWIND, 50);
	val!(APPCOMMAND, MEDIA_CHANNEL_UP, 51);
	val!(APPCOMMAND, MEDIA_CHANNEL_DOWN, 52);
	val!(APPCOMMAND, DELETE, 53);
	val!(APPCOMMAND, DWM_FLIP3D, 54);
}

decl!(BCN, i32,
	"Button control
	[notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications),
	sent via
	[WM_NOTIFY](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify).");
impl BCN {
	priv_val!(BCN, FIRST, -1250);

	val!(BCN, HOTITEMCHANGE, BCN::FIRST.0 + 0x0001);
	val!(BCN, DROPDOWN, BCN::FIRST.0 + 0x0002);
}

decl!(BI, u32,
	"[`BITMAPINFOHEADER`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader)
	`biCompression`.");
impl BI {
	val!(BI, RGB, 0);
	val!(BI, RLE8, 1);
	val!(BI, RLE4, 2);
	val!(BI, BITFIELDS, 3);
	val!(BI, JPEG, 4);
	val!(BI, PNG, 5);
}

decl!(BKMODE, i32,
	"[`SetBkMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	`mode`.");
impl BKMODE {
	val!(BKMODE, TRANSPARENT, 1);
	val!(BKMODE, OPAQUE, 2);
}