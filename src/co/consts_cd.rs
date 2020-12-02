decl!(DLGID, u32,
	"Dialogs built-in IDs. These are also returned from
	[`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw#return-value).");
impl DLGID {
	val!(DLGID, OK, 1);
	val!(DLGID, CANCEL, 2);
	val!(DLGID, ABORT, 3);
	val!(DLGID, RETRY, 4);
	val!(DLGID, IGNORE, 5);
	val!(DLGID, YES, 6);
	val!(DLGID, NO, 7);
	val!(DLGID, TRYAGAIN, 10);
	val!(DLGID, CONTINUE, 11);
}