decl!(MB, u32,
	"[MessageBox](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	`uType`.");
impl MB {
	val!(ABORTRETRYIGNORE, 0x00000002);
	val!(CANCELTRYCONTINUE, 0x00000006);
	val!(HELP, 0x00004000);
	val!(OK, 0x00000000);
	val!(OKCANCEL, 0x00000001);
	val!(RETRYCANCEL, 0x00000005);
	val!(YESNO, 0x00000004);
	val!(YESNOCANCEL, 0x00000003);

	val!(ICONEXCLAMATION, 0x00000030);
	val!(ICONWARNING, MB::ICONEXCLAMATION.0);
	val!(ICONINFORMATION, 0x00000040);
	val!(ICONASTERISK, MB::ICONINFORMATION.0);
	val!(ICONQUESTION, 0x00000020);
	val!(ICONSTOP, MB::ICONERROR.0);
	val!(ICONERROR, 0x00000010);
	val!(ICONHAND, MB::ICONERROR.0);

	val!(DEFBUTTON1, 0x00000000);
	val!(DEFBUTTON2, 0x00000100);
	val!(DEFBUTTON3, 0x00000200);
	val!(DEFBUTTON4, 0x00000300);

	val!(APPLMODAL, 0x00000000);
	val!(SYSTEMMODAL, 0x00001000);
	val!(TASKMODAL, 0x00002000);

	val!(DEFAULT_DESKTOP_ONLY, 0x00020000);
	val!(RIGHT, 0x00080000);
	val!(RTLREADING, 0x00100000);
	val!(SETFOREGROUND, 0x00010000);
	val!(TOPMOST, 0x00040000);
	val!(SERVICE_NOTIFICATION, 0x00200000);
}

decl!(NM, i32,
	"Common control
	[notifications](https://docs.microsoft.com/en-us/windows/win32/controls/common-control-reference#notifications).");
impl NM {
	priv_val!(FIRST, 0);

	val!(OUTOFMEMORY, NM::FIRST.0 - 1);
	val!(CLICK, NM::FIRST.0 - 2);
	val!(DBLCLK, NM::FIRST.0 - 3);
	val!(RETURN, NM::FIRST.0 - 4);
	val!(RCLICK, NM::FIRST.0 - 5);
	val!(RDBLCLK, NM::FIRST.0 - 6);
	val!(SETFOCUS, NM::FIRST.0 - 7);
	val!(KILLFOCUS, NM::FIRST.0 - 8);
	val!(CUSTOMDRAW, NM::FIRST.0 - 12);
	val!(HOVER, NM::FIRST.0 - 13);
	val!(NCHITTEST, NM::FIRST.0 - 14);
	val!(KEYDOWN, NM::FIRST.0 - 15);
	val!(RELEASEDCAPTURE, NM::FIRST.0 - 16);
	val!(SETCURSOR, NM::FIRST.0 - 17);
	val!(CHAR, NM::FIRST.0 - 18);
	val!(TOOLTIPSCREATED, NM::FIRST.0 - 19);
	val!(LDOWN, NM::FIRST.0 - 20);
	val!(RDOWN, NM::FIRST.0 - 21);
	val!(THEMECHANGED, NM::FIRST.0 - 22);
}