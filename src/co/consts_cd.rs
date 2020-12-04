ty_const!(CLSCTX, u32,
	"[`CLSCTX`](https://docs.microsoft.com/en-us/windows/win32/api/wtypesbase/ne-wtypesbase-clsctx)
	enumeration.");
impl CLSCTX {
	val!(INPROC_SERVER, 0x1);
	val!(INPROC_HANDLER, 0x2);
	val!(LOCAL_SERVER, 0x4);
	val!(INPROC_SERVER16, 0x8);
	val!(REMOTE_SERVER, 0x10);
	val!(INPROC_HANDLER16, 0x20);
	val!(NO_CODE_DOWNLOAD, 0x400);
	val!(NO_CUSTOM_MARSHAL, 0x1000);
	val!(ENABLE_CODE_DOWNLOAD, 0x2000);
	val!(NO_FAILURE_LOG, 0x4000);
	val!(DISABLE_AAA, 0x8000);
	val!(ENABLE_AAA, 0x10000);
	val!(FROM_DEFAULT_CONTEXT, 0x20000);
	val!(ACTIVATE_X86_SERVER, 0x40000);
	val!(ACTIVATE_32_BIT_SERVER, CLSCTX::ACTIVATE_X86_SERVER.0);
	val!(ACTIVATE_64_BIT_SERVER, 0x80000);
	val!(ENABLE_CLOAKING, 0x100000);
	val!(APPCONTAINER, 0x400000);
	val!(ACTIVATE_AAA_AS_IU, 0x800000);
	val!(ACTIVATE_ARM32_SERVER, 0x2000000);
	val!(PS_DLL, 0x80000000);
}

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