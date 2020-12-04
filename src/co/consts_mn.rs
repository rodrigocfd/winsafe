const_type!(MB, u32,
	"[MessageBox](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw)
	`uType`.");
impl MB {
	const_val!(ABORTRETRYIGNORE, 0x00000002);
	const_val!(CANCELTRYCONTINUE, 0x00000006);
	const_val!(HELP, 0x00004000);
	const_val!(OK, 0x00000000);
	const_val!(OKCANCEL, 0x00000001);
	const_val!(RETRYCANCEL, 0x00000005);
	const_val!(YESNO, 0x00000004);
	const_val!(YESNOCANCEL, 0x00000003);

	const_val!(ICONEXCLAMATION, 0x00000030);
	const_val!(ICONWARNING, MB::ICONEXCLAMATION.0);
	const_val!(ICONINFORMATION, 0x00000040);
	const_val!(ICONASTERISK, MB::ICONINFORMATION.0);
	const_val!(ICONQUESTION, 0x00000020);
	const_val!(ICONSTOP, MB::ICONERROR.0);
	const_val!(ICONERROR, 0x00000010);
	const_val!(ICONHAND, MB::ICONERROR.0);

	const_val!(DEFBUTTON1, 0x00000000);
	const_val!(DEFBUTTON2, 0x00000100);
	const_val!(DEFBUTTON3, 0x00000200);
	const_val!(DEFBUTTON4, 0x00000300);

	const_val!(APPLMODAL, 0x00000000);
	const_val!(SYSTEMMODAL, 0x00001000);
	const_val!(TASKMODAL, 0x00002000);

	const_val!(DEFAULT_DESKTOP_ONLY, 0x00020000);
	const_val!(RIGHT, 0x00080000);
	const_val!(RTLREADING, 0x00100000);
	const_val!(SETFOREGROUND, 0x00010000);
	const_val!(TOPMOST, 0x00040000);
	const_val!(SERVICE_NOTIFICATION, 0x00200000);
}

const_type!(NM, i32,
	"Common control
	[notifications](https://docs.microsoft.com/en-us/windows/win32/controls/common-control-reference#notifications).");
impl NM {
	const FIRST: Self = Self(0);

	const_val!(OUTOFMEMORY, Self::FIRST.0 - 1);
	const_val!(CLICK, Self::FIRST.0 - 2);
	const_val!(DBLCLK, Self::FIRST.0 - 3);
	const_val!(RETURN, Self::FIRST.0 - 4);
	const_val!(RCLICK, Self::FIRST.0 - 5);
	const_val!(RDBLCLK, Self::FIRST.0 - 6);
	const_val!(SETFOCUS, Self::FIRST.0 - 7);
	const_val!(KILLFOCUS, Self::FIRST.0 - 8);
	const_val!(CUSTOMDRAW, Self::FIRST.0 - 12);
	const_val!(HOVER, Self::FIRST.0 - 13);
	const_val!(NCHITTEST, Self::FIRST.0 - 14);
	const_val!(KEYDOWN, Self::FIRST.0 - 15);
	const_val!(RELEASEDCAPTURE, Self::FIRST.0 - 16);
	const_val!(SETCURSOR, Self::FIRST.0 - 17);
	const_val!(CHAR, Self::FIRST.0 - 18);
	const_val!(TOOLTIPSCREATED, Self::FIRST.0 - 19);
	const_val!(LDOWN, Self::FIRST.0 - 20);
	const_val!(RDOWN, Self::FIRST.0 - 21);
	const_val!(THEMECHANGED, Self::FIRST.0 - 22);
}