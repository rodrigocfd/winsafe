const_type! {
	CLSCTX, u32,
	"[`CLSCTX`](https://docs.microsoft.com/en-us/windows/win32/api/wtypesbase/ne-wtypesbase-clsctx)
	enumeration.",
	INPROC_SERVER, 0x1
	INPROC_HANDLER, 0x2
	LOCAL_SERVER, 0x4
	INPROC_SERVER16, 0x8
	REMOTE_SERVER, 0x10
	INPROC_HANDLER16, 0x20
	NO_CODE_DOWNLOAD, 0x400
	NO_CUSTOM_MARSHAL, 0x1000
	ENABLE_CODE_DOWNLOAD, 0x2000
	NO_FAILURE_LOG, 0x4000
	DISABLE_AAA, 0x8000
	ENABLE_AAA, 0x10000
	FROM_DEFAULT_CONTEXT, 0x20000
	ACTIVATE_X86_SERVER, 0x40000
	ACTIVATE_32_BIT_SERVER, Self::ACTIVATE_X86_SERVER.0
	ACTIVATE_64_BIT_SERVER, 0x80000
	ENABLE_CLOAKING, 0x100000
	APPCONTAINER, 0x400000
	ACTIVATE_AAA_AS_IU, 0x800000
	ACTIVATE_ARM32_SERVER, 0x2000000
	PS_DLL, 0x80000000
}

const_type! {
	COINIT, u32,
	"[`CoInitializeEx`](https://docs.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)
	`dwCoInit`.",
	APARTMENTTHREADED, 0x2
	MULTITHREADED, 0x0
	DISABLE_OLE1DDE, 0x4
	SPEED_OVER_MEMORY, 0x8
}

const_type! {
	DLGID, u32,
	"Dialog built-in IDs. These are also returned from
	[`MessageBox`](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxw#return-value).",
	OK, 1
	CANCEL, 2
	ABORT, 3
	RETRY, 4
	IGNORE, 5
	YES, 6
	NO, 7
	TRYAGAIN, 10
	CONTINUE, 11
}