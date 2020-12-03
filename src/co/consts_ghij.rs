ty_const!(GW, u32,
	"[`GetWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	`uCmd`.");
impl GW {
	val!(HWNDFIRST, 0);
	val!(HWNDLAST, 1);
	val!(HWNDNEXT, 2);
	val!(HWNDPREV, 3);
	val!(OWNER, 4);
	val!(CHILD, 5);
	val!(ENABLEDPOPUP, 6);
	val!(MAX, 6);
}