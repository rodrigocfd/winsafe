const_type! {
	/// [`GetWindow`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindow)
	/// `uCmd`.
	GW, u32,

	HWNDFIRST, 0
	HWNDLAST, 1
	HWNDNEXT, 2
	HWNDPREV, 3
	OWNER, 4
	CHILD, 5
	ENABLEDPOPUP, 6
	MAX, 6
}