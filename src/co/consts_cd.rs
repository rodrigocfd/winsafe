ty_const!(COINIT, u32,
	"[`CoInitializeEx`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)
	`dwCoInit`.");
impl COINIT {
	val!(APARTMENTTHREADED, 0x2);
	val!(MULTITHREADED, 0x0);
	val!(DISABLE_OLE1DDE, 0x4);
	val!(SPEED_OVER_MEMORY, 0x8);
}

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