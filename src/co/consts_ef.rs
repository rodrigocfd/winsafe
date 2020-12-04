const_type!(FORMAT_MESSAGE, u32,
	"[`FormatMessage`](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessagew)
	`dwFlags`.");
impl FORMAT_MESSAGE {
	const_val!(ALLOCATE_BUFFER, 0x00000100);
	const_val!(ARGUMENT_ARRAY, 0x00002000);
	const_val!(FROM_HMODULE, 0x00000800);
	const_val!(FROM_STRING, 0x00000400);
	const_val!(FROM_SYSTEM, 0x00001000);
	const_val!(IGNORE_INSERTS, 0x00000200);
	const_val!(MAX_WIDTH_MASK, 0x000000ff);
}