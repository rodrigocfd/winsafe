const_type!(CLSCTX, u32,
	"[`CLSCTX`](https://docs.microsoft.com/en-us/windows/win32/api/wtypesbase/ne-wtypesbase-clsctx)
	enumeration.");
impl CLSCTX {
	const_val!(INPROC_SERVER, 0x1);
	const_val!(INPROC_HANDLER, 0x2);
	const_val!(LOCAL_SERVER, 0x4);
	const_val!(INPROC_SERVER16, 0x8);
	const_val!(REMOTE_SERVER, 0x10);
	const_val!(INPROC_HANDLER16, 0x20);
	const_val!(NO_CODE_DOWNLOAD, 0x400);
	const_val!(NO_CUSTOM_MARSHAL, 0x1000);
	const_val!(ENABLE_CODE_DOWNLOAD, 0x2000);
	const_val!(NO_FAILURE_LOG, 0x4000);
	const_val!(DISABLE_AAA, 0x8000);
	const_val!(ENABLE_AAA, 0x10000);
	const_val!(FROM_DEFAULT_CONTEXT, 0x20000);
	const_val!(ACTIVATE_X86_SERVER, 0x40000);
	const_val!(ACTIVATE_32_BIT_SERVER, Self::ACTIVATE_X86_SERVER.0);
	const_val!(ACTIVATE_64_BIT_SERVER, 0x80000);
	const_val!(ENABLE_CLOAKING, 0x100000);
	const_val!(APPCONTAINER, 0x400000);
	const_val!(ACTIVATE_AAA_AS_IU, 0x800000);
	const_val!(ACTIVATE_ARM32_SERVER, 0x2000000);
	const_val!(PS_DLL, 0x80000000);
}

const_type!(COINIT, u32,
	"[`CoInitializeEx`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)
	`dwCoInit`.");
impl COINIT {
	const_val!(APARTMENTTHREADED, 0x2);
	const_val!(MULTITHREADED, 0x0);
	const_val!(DISABLE_OLE1DDE, 0x4);
	const_val!(SPEED_OVER_MEMORY, 0x8);
}

const_type!(DLGID, u32,
	"Dialog built-in IDs. These are also returned from
	[`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw#return-value).");
impl DLGID {
	const_val!(OK, 1);
	const_val!(CANCEL, 2);
	const_val!(ABORT, 3);
	const_val!(RETRY, 4);
	const_val!(IGNORE, 5);
	const_val!(YES, 6);
	const_val!(NO, 7);
	const_val!(TRYAGAIN, 10);
	const_val!(CONTINUE, 11);
}