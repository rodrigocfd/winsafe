decl!(MB, u32,
	"[MessageBox](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	`uType`.");
impl MB {
	val!(MB, ABORTRETRYIGNORE, 0x00000002);
	val!(MB, CANCELTRYCONTINUE, 0x00000006);
	val!(MB, HELP, 0x00004000);
	val!(MB, OK, 0x00000000);
	val!(MB, OKCANCEL, 0x00000001);
	val!(MB, RETRYCANCEL, 0x00000005);
	val!(MB, YESNO, 0x00000004);
	val!(MB, YESNOCANCEL, 0x00000003);

	val!(MB, ICONEXCLAMATION, 0x00000030);
	val!(MB, ICONWARNING, MB::ICONEXCLAMATION.0);
	val!(MB, ICONINFORMATION, 0x00000040);
	val!(MB, ICONASTERISK, MB::ICONINFORMATION.0);
	val!(MB, ICONQUESTION, 0x00000020);
	val!(MB, ICONSTOP, MB::ICONERROR.0);
	val!(MB, ICONERROR, 0x00000010);
	val!(MB, ICONHAND, MB::ICONERROR.0);

	val!(MB, DEFBUTTON1, 0x00000000);
	val!(MB, DEFBUTTON2, 0x00000100);
	val!(MB, DEFBUTTON3, 0x00000200);
	val!(MB, DEFBUTTON4, 0x00000300);

	val!(MB, APPLMODAL, 0x00000000);
	val!(MB, SYSTEMMODAL, 0x00001000);
	val!(MB, TASKMODAL, 0x00002000);

	val!(MB, DEFAULT_DESKTOP_ONLY, 0x00020000);
	val!(MB, RIGHT, 0x00080000);
	val!(MB, RTLREADING, 0x00100000);
	val!(MB, SETFOREGROUND, 0x00010000);
	val!(MB, TOPMOST, 0x00040000);
	val!(MB, SERVICE_NOTIFICATION, 0x00200000);
}

decl!(NM, i32,
	"Common control
	[notifications](https://docs.microsoft.com/en-us/windows/win32/controls/common-control-reference#notifications).");
impl NM {
	priv_val!(NM, FIRST, 0);

	val!(NM, OUTOFMEMORY, NM::FIRST.0 - 1);
	val!(NM, CLICK, NM::FIRST.0 - 2);
	val!(NM, DBLCLK, NM::FIRST.0 - 3);
	val!(NM, RETURN, NM::FIRST.0 - 4);
	val!(NM, RCLICK, NM::FIRST.0 - 5);
	val!(NM, RDBLCLK, NM::FIRST.0 - 6);
	val!(NM, SETFOCUS, NM::FIRST.0 - 7);
	val!(NM, KILLFOCUS, NM::FIRST.0 - 8);
	val!(NM, CUSTOMDRAW, NM::FIRST.0 - 12);
	val!(NM, HOVER, NM::FIRST.0 - 13);
	val!(NM, NCHITTEST, NM::FIRST.0 - 14);
	val!(NM, KEYDOWN, NM::FIRST.0 - 15);
	val!(NM, RELEASEDCAPTURE, NM::FIRST.0 - 16);
	val!(NM, SETCURSOR, NM::FIRST.0 - 17);
	val!(NM, CHAR, NM::FIRST.0 - 18);
	val!(NM, TOOLTIPSCREATED, NM::FIRST.0 - 19);
	val!(NM, LDOWN, NM::FIRST.0 - 20);
	val!(NM, RDOWN, NM::FIRST.0 - 21);
	val!(NM, THEMECHANGED, NM::FIRST.0 - 22);
}