ty_const!(FORMAT_MESSAGE, u32,
	"[`FormatMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
	`dwFlags`.");
impl FORMAT_MESSAGE {
	val!(ALLOCATE_BUFFER, 0x00000100);
	val!(ARGUMENT_ARRAY, 0x00002000);
	val!(FROM_HMODULE, 0x00000800);
	val!(FROM_STRING, 0x00000400);
	val!(FROM_SYSTEM, 0x00001000);
	val!(IGNORE_INSERTS, 0x00000200);
	val!(MAX_WIDTH_MASK, 0x000000ff);
}