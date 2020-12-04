const_type!(GW, u32,
	"[`GetWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	`uCmd`.");
impl GW {
	const_val!(HWNDFIRST, 0);
	const_val!(HWNDLAST, 1);
	const_val!(HWNDNEXT, 2);
	const_val!(HWNDPREV, 3);
	const_val!(OWNER, 4);
	const_val!(CHILD, 5);
	const_val!(ENABLEDPOPUP, 6);
	const_val!(MAX, 6);
}