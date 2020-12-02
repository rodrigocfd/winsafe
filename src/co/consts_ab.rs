decl!(ACCELF, u8,
	"[`ACCELL`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-accel)
	`fVirt`.");
impl ACCELF {
	val!(NONE, 0);
	val!(VIRTKEY, 1);
	val!(SHIFT, 0x04);
	val!(CONTROL, 0x08);
	val!(ALT, 0x10);
}

decl!(ACCESS_RIGHTS, u32,
	"[`RegOpenKeyEx`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw)
	`samDesired`.");
impl ACCESS_RIGHTS {
	val!(DELETE, 0x00010000);
	val!(READ_CONTROL, 0x00020000);
	val!(WRITE_DAC, 0x00040000);
	val!(WRITE_OWNER, 0x00080000);
	val!(SYNCHRONIZE, 0x00100000);
}

decl!(ADRF, u32,
	"[`NMTVASYNCDRAW`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/ns-commctrl-nmtvasyncdraw)
	`dwRetFlags`. Don't seem to be defined anywhere, unconfirmed values.");
impl ADRF {
	val!(DRAWSYNC, 0);
	val!(DRAWNOTHING, 1);
	val!(DRAWFALLBACK, 2);
	val!(DRAWIMAGE, 3);
}

decl!(APPCOMMAND, i16,
	"[`WM_APPCOMMAND`](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand)
	commands.");
impl APPCOMMAND {
	val!(BROWSER_BACKWARD, 1);
	val!(BROWSER_FORWARD, 2);
	val!(BROWSER_REFRESH, 3);
	val!(BROWSER_STOP, 4);
	val!(BROWSER_SEARCH, 5);
	val!(BROWSER_FAVORITES, 6);
	val!(BROWSER_HOME, 7);
	val!(VOLUME_MUTE, 8);
	val!(VOLUME_DOWN, 9);
	val!(VOLUME_UP, 10);
	val!(MEDIA_NEXTTRACK, 11);
	val!(MEDIA_PREVIOUSTRACK, 12);
	val!(MEDIA_STOP, 13);
	val!(MEDIA_PLAY_PAUSE, 14);
	val!(LAUNCH_MAIL, 15);
	val!(LAUNCH_MEDIA_SELECT, 16);
	val!(LAUNCH_APP1, 17);
	val!(LAUNCH_APP2, 18);
	val!(BASS_DOWN, 19);
	val!(BASS_BOOST, 20);
	val!(BASS_UP, 21);
	val!(TREBLE_DOWN, 22);
	val!(TREBLE_UP, 23);
	val!(MICROPHONE_VOLUME_MUTE, 24);
	val!(MICROPHONE_VOLUME_DOWN, 25);
	val!(MICROPHONE_VOLUME_UP, 26);
	val!(HELP, 27);
	val!(FIND, 28);
	val!(NEW, 29);
	val!(OPEN, 30);
	val!(CLOSE, 31);
	val!(SAVE, 32);
	val!(PRINT, 33);
	val!(UNDO, 34);
	val!(REDO, 35);
	val!(COPY, 36);
	val!(CUT, 37);
	val!(PASTE, 38);
	val!(REPLY_TO_MAIL, 39);
	val!(FORWARD_MAIL, 40);
	val!(SEND_MAIL, 41);
	val!(SPELL_CHECK, 42);
	val!(DICTATE_OR_COMMAND_CONTROL_TOGGLE, 43);
	val!(MIC_ON_OFF_TOGGLE, 44);
	val!(CORRECTION_LIST, 45);
	val!(MEDIA_PLAY, 46);
	val!(MEDIA_PAUSE, 47);
	val!(MEDIA_RECORD, 48);
	val!(MEDIA_FAST_FORWARD, 49);
	val!(MEDIA_REWIND, 50);
	val!(MEDIA_CHANNEL_UP, 51);
	val!(MEDIA_CHANNEL_DOWN, 52);
	val!(DELETE, 53);
	val!(DWM_FLIP3D, 54);
}

decl!(BCN, i32,
	"Button control
	[notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications),
	sent via
	[WM_NOTIFY](https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify).");
impl BCN {
	priv_val!(FIRST, -1250);

	val!(HOTITEMCHANGE, BCN::FIRST.0 + 0x0001);
	val!(DROPDOWN, BCN::FIRST.0 + 0x0002);
}

decl!(BI, u32,
	"[`BITMAPINFOHEADER`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-bitmapinfoheader)
	`biCompression`.");
impl BI {
	val!(RGB, 0);
	val!(RLE8, 1);
	val!(RLE4, 2);
	val!(BITFIELDS, 3);
	val!(JPEG, 4);
	val!(PNG, 5);
}

decl!(BKMODE, i32,
	"[`SetBkMode`](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-setbkmode)
	`mode`.");
impl BKMODE {
	val!(TRANSPARENT, 1);
	val!(OPAQUE, 2);
}