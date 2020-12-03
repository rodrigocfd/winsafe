ty_const!(DLGID, u32,
	"Dialog built-in IDs. These are also returned from
	[`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw#return-value).");
impl DLGID {
	val!(OK, 1);
	val!(CANCEL, 2);
	val!(ABORT, 3);
	val!(RETRY, 4);
	val!(IGNORE, 5);
	val!(YES, 6);
	val!(NO, 7);
	val!(TRYAGAIN, 10);
	val!(CONTINUE, 11);
}