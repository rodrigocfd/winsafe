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