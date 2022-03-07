#![allow(non_camel_case_types, non_upper_case_globals)]

const_bitflag! { ACCESS_RIGHTS: u32: "kernel";
	/// Standard access rights
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)
	/// (`u32`).
	///
	/// Originally has no prefix.
	=>
	=>
	DELETE 0x0001_0000
	READ_CONTROL 0x0002_0000
	WRITE_DAC 0x0004_0000
	WRITE_OWNER 0x0008_0000
	SYNCHRONIZE 0x0010_0000
}

const_ordinary! { CP: u16: "kernel";
	/// [`WideCharToMultiByte`](crate::WideCharToMultiByte) and
	/// [`MultiByteToWideChar`](crate::MultiByteToWideChar) `code_page`
	/// [identifiers](https://docs.microsoft.com/en-us/windows/win32/intl/code-page-identifiers)
	/// (`u16`).
	///
	/// Originally these functions receive an `u32` parameter, but these are in
	/// fact `u16` constants.
	=>
	=>
	/// The system default Windows ANSI code page.
	ACP 0
	/// The current system OEM code page.
	OEMCP 1
	/// The current system Macintosh code page.
	MACCP 2
	/// The Windows ANSI code page for the current thread.
	THREAD_ACP 3
	/// Symbol code page (42).
	SYMBOL 42

	/// Unicode UTF-16 little endian byte order (BMP of ISO 10646); available
	/// only to managed applications.
	UTF16 1200
	/// Unicode UTF-16 big endian byte order; available only to managed
	/// applications.
	UNICODE_FFFE 1201
	/// ANSI Central European; Central European (Windows).
	WINDOWS_1250 1250
	/// ANSI Cyrillic; Cyrillic (Windows).
	WINDOWS_1251 1251
	/// ANSI Latin 1; Western European (Windows).
	WINDOWS_1252 1252
	/// ANSI Greek; Greek (Windows).
	WINDOWS_1253 1253
	/// ANSI Turkish; Turkish (Windows).
	WINDOWS_1254 1254
	/// ANSI Hebrew; Hebrew (Windows).
	WINDOWS_1255 1255
	/// ANSI Arabic; Arabic (Windows).
	WINDOWS_1256 1256
	/// ANSI Baltic; Baltic (Windows).
	WINDOWS_1257 1257
	/// ANSI/OEM Vietnamese; Vietnamese (Windows).
	WINDOWS_1258 1258
	/// Korean (Johab).
	JOHAB 1361
	/// MAC Roman; Western European (Mac).
	MACINTOSH 10000

	/// Unicode (UTF-7).
	///
	/// Conversion functions: use this value only when forced by a 7-bit
	/// transport mechanism. Use of UTF-8 is preferred. With this value set,
	/// `lpDefaultChar` and `lpUsedDefaultChar` must be set to null.
	UTF7 65000
	/// Unicode (UTF-8).
	///
	/// Conversion functions: with this value set `lpDefaultChar` and
	/// `lpUsedDefaultChar` must be set to null.
	UTF8 65001
}

const_bitflag! { CREATE: u32: "kernel";
	/// Process creation
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
	/// (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	BREAKAWAY_FROM_JOB 0x0100_0000
	DEFAULT_ERROR_MODE 0x0400_0000
	NEW_CONSOLE 0x0000_0010
	NEW_PROCESS_GROUP 0x0000_0200
	NO_WINDOW 0x0800_0000
	PROTECTED_PROCESS 0x0004_0000
	PRESERVE_CODE_AUTHZ_LEVEL 0x0200_0000
	SECURE_PROCESS 0x0040_0000
	SEPARATE_WOW_VDM 0x0000_0800
	SHARED_WOW_VDM 0x0000_1000
	SUSPENDED 0x0000_0004
	UNICODE_ENVIRONMENT 0x0000_0400
	/// Originally has no `CREATE` prefix.
	DEBUG_ONLY_THIS_PROCESS 0x0000_0002
	/// Originally has no `CREATE` prefix.
	DEBUG_PROCESS 0x0000_0001
	/// Originally has no `CREATE` prefix.
	DETACHED_PROCESS 0x0000_0008
	/// Originally has no `CREATE` prefix.
	EXTENDED_STARTUPINFO_PRESENT 0x0008_0000
	/// Originally has no `CREATE` prefix.
	INHERIT_PARENT_AFFINITY 0x0001_0000
}

const_ordinary! { DISPOSITION: u32: "kernel";
	/// [`HFILE::CreateFile`](crate::prelude::KernelHfile::CreateFile)
	/// `creation_disposition` (`u32`).
	///
	/// Originally has no prefix.
	=>
	=>
	/// Creates a new file only if it does not already exist.
	///
	/// If the specified file exists the function fails and the last-error code
	/// is set to [`ERROR::FILE_EXISTS`](crate::co::ERROR::FILE_EXISTS).
	///
	/// If the specified file does not exist and is a valid path to a writable
	/// location a new file is created.
	CREATE_NEW 1
	/// Creates a new file always.
	///
	/// If the specified file exists and is writable the function overwrites
	/// the file the function succeeds and last-error code is set to
	/// [`ERROR::ALREADY_EXISTS`](crate::co::ERROR::ALREADY_EXISTS).
	///
	/// If the specified file does not exist and is a valid path a new file is
	/// created the function succeeds and the last-error code is set to
	/// [`ERROR::SUCCESS`](crate::co::ERROR::SUCCESS).
	CREATE_ALWAYS 2
	/// Opens a file or device only if it exists.
	///
	/// If the specified file or device does not exist the function fails and
	/// the last-error code is set to
	/// [`ERROR::FILE_NOT_FOUND`](crate::co::ERROR::FILE_NOT_FOUND).
	OPEN_EXISTING 3
	/// Opens a file always.
	///
	/// If the specified file exists the function succeeds and the last-error
	/// code is set to
	/// [`ERROR::ALREADY_EXISTS`](crate::co::ERROR::ALREADY_EXISTS).
	///
	/// If the specified file does not exist and is a valid path to a writable
	/// location the function creates a file and the last-error code is set to
	/// [`ERROR::SUCCESS`](crate::co::ERROR::SUCCESS).
	OPEN_ALWAYS 4
	/// Opens a file and truncates it so that its size is zero bytes only if it
	/// exists.
	///
	/// If the specified file does not exist the function fails and the
	/// last-error code is set to
	/// [`ERROR::FILE_NOT_FOUND`](crate::co::ERROR::FILE_NOT_FOUND).
	///
	/// The calling process must open the file with the
	/// [`GENERIC::WRITE`](crate::co::GENERIC::WRITE) bit set as part of the
	/// `dwDesiredAccess` parameter.
	TRUNCATE_EXISTING 5
}

const_bitflag! { FILE_ATTRIBUTE: u32: "kernel";
	/// File attribute
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants),
	/// also other flags from
	/// [`HFILE::CreateFile`](crate::prelude::KernelHfile::CreateFile)
	/// `flags_and_attrs` (`u32`).
	=>
	=>
	READONLY 0x0000_0001
	HIDDEN 0x0000_0002
	SYSTEM 0x0000_0004
	DIRECTORY 0x0000_0010
	ARCHIVE 0x0000_0020
	DEVICE 0x0000_0040
	NORMAL 0x0000_0080
	TEMPORARY 0x0000_0100
	SPARSE_FILE 0x0000_0200
	REPARSE_POINT 0x0000_0400
	COMPRESSED 0x0000_0800
	OFFLINE 0x000_01000
	NOT_CONTENT_INDEXED 0x0000_2000
	ENCRYPTED 0x0000_4000
	INTEGRITY_STREAM 0x0000_8000
	VIRTUAL 0x0001_0000
	NO_SCRUB_DATA 0x0002_0000
	EA 0x0004_0000
	PINNED 0x0008_0000
	UNPINNED 0x0010_0000
	RECALL_ON_OPEN 0x0004_0000
	RECALL_ON_DATA_ACCESS 0x0040_0000

	FLAG_WRITE_THROUGH 0x8000_0000
	FLAG_OVERLAPPED 0x4000_0000
	FLAG_NO_BUFFERING 0x2000_0000
	FLAG_RANDOM_ACCESS 0x1000_0000
	FLAG_SEQUENTIAL_SCAN 0x0800_0000
	FLAG_DELETE_ON_CLOSE 0x0400_0000
	FLAG_BACKUP_SEMANTICS 0x0200_0000
	FLAG_POSIX_SEMANTICS 0x0100_0000
	FLAG_SESSION_AWARE 0x0080_0000
	FLAG_OPEN_REPARSE_POINT 0x0020_0000
	FLAG_OPEN_NO_RECALL 0x0010_0000
	FLAG_FIRST_PIPE_INSTANCE 0x0008_0000

	SECURITY_ANONYMOUS 0 << 16
	SECURITY_IDENTIFICATION 1 << 16
	SECURITY_IMPERSONATION 2 << 16
	SECURITY_DELEGATION 3 << 16
	SECURITY_CONTEXT_TRACKING 0x0004_0000
	SECURITY_EFFECTIVE_ONLY 0x0008_0000
}

const_bitflag! { FILE_MAP: u32: "kernel";
	/// [`HFILEMAP::MapViewOfFile`](crate::prelude::KernelHfilemap::MapViewOfFile)
	/// `desired_access` (`u32`).
	=>
	=>
	ALL_ACCESS SECTION::ALL_ACCESS.0
	READ SECTION::MAP_READ.0
	WRITE SECTION::MAP_WRITE.0

	COPY 0x0000_0001
	EXECUTE SECTION::MAP_EXECUTE_EXPLICIT.0
	LARGE_PAGES 0x2000_0000
	TARGETS_INVALID 0x4000_0000
}

const_bitflag! { FILE_SHARE: u32: "kernel";
	/// [`HFILE::CreateFile`](crate::prelude::KernelHfile::CreateFile)
	/// `share_mode` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	READ 0x0000_0001
	WRITE 0x0000_0002
	DELETE 0x0000_0004
}

const_ordinary! { FILE_STARTING_POINT: u32: "kernel";
	/// [`HFILE::SetFilePointerEx`](crate::prelude::KernelHfile::SetFilePointerEx)
	/// `move_method` (`u32`).
	///
	/// Originally has `FILE` prefix.
	=>
	=>
	/// The starting point is zero or the beginning of the file. If this flag is
	/// specified then the `liDistanceToMove` parameter is interpreted as an
	/// unsigned value.
	BEGIN 0
	/// The start point is the current value of the file pointer.
	CURRENT 1
	/// The starting point is the current end-of-file position.
	END 2
}

const_ordinary! { FILE_TYPE: u32: "kernel";
	/// [`HFILE::GetFileType`](crate::prelude::KernelHfile::GetFileType) return
	/// value (`u32`).
	=>
	=>
	/// The specified file is a character file typically an LPT device or a
	/// console.
	CHAR 0x0002
	/// The specified file is a disk file.
	DISK 0x0001
	/// The specified file is a socket a named pipe or an anonymous pipe.
	PIPE 0x0003
	/// Unused.
	REMOTE 0x8000
	/// Either the type of the specified file is unknown or the function
	/// failed.
	UNKNOWN 0x0000
}

const_bitflag! { FILE_VOL: u32: "kernel";
	/// [`GetVolumeInformation`](crate::GetVolumeInformation) `flags` (`u32`).
	///
	/// Originally has `FILE` prefix.
	=>
	=>
	CASE_PRESERVED_NAMES 0x0000_0002
	CASE_SENSITIVE_SEARCH 0x0000_0001
	DAX_VOLUME 0x2000_0000
	FILE_COMPRESSION 0x0000_0010
	NAMED_STREAMS 0x0004_0000
	PERSISTENT_ACLS 0x0000_0008
	READ_ONLY_VOLUME 0x0008_0000
	SEQUENTIAL_WRITE_ONCE 0x0010_0000
	SUPPORTS_ENCRYPTION 0x0002_0000
	SUPPORTS_EXTENDED_ATTRIBUTES 0x0080_0000
	SUPPORTS_HARD_LINKS 0x0040_0000
	SUPPORTS_OBJECT_IDS 0x0001_0000
	SUPPORTS_OPEN_BY_FILE_ID 0x0100_0000
	SUPPORTS_REPARSE_POINTS 0x0000_0080
	SUPPORTS_SPARSE_FILES 0x0000_0040
	SUPPORTS_TRANSACTIONS 0x0020_0000
	SUPPORTS_USN_JOURNAL 0x0200_0000
	UNICODE_ON_DISK 0x0000_0004
	VOLUME_IS_COMPRESSED 0x0000_8000
	VOLUME_QUOTAS 0x0000_0020
	SUPPORTS_BLOCK_REFCOUNTING 0x0800_0000
}

const_ordinary! { FIRMWARE_TYPE: u32: "kernel";
	/// [`FIRMWARE_TYPE`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-firmware_type)
	/// enumeration (`u32`).
	=>
	=>
	Unknown 0
	Bios 1
	Uefi 2
}

const_bitflag! { FORMAT_MESSAGE: u32: "kernel";
	/// [`FormatMessage`](crate::prelude::FormattedError::FormatMessage)
	/// `dwFlags` (`u32`).
	=>
	=>
	ALLOCATE_BUFFER 0x0000_0100
	ARGUMENT_ARRAY 0x0000_2000
	FROM_HMODULE 0x0000_0800
	FROM_STRING 0x0000_0400
	FROM_SYSTEM 0x0000_1000
	IGNORE_INSERTS 0x0000_0200
	MAX_WIDTH_MASK 0x0000_00ff
}

const_bitflag! { GENERIC: u32: "kernel";
	/// Generic access rights
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/secauthz/generic-access-rights)
	/// (`u32`).
	=>
	=>
	/// Read access.
	READ 0x8000_0000
	/// Write access.
	WRITE 0x4000_0000
	/// Execute access.
	EXECUTE 0x2000_0000
	/// All possible access rights.
	ALL 0x1000_0000
}

const_bitflag! { GMEM: u32: "kernel";
	/// [`HGLOBAL::GlobalAlloc`](crate::prelude::KernelHglobal::GlobalAlloc) and
	/// [`HGLOBAL::GlobalReAlloc`](crate::prelude::KernelHglobal::GlobalReAlloc)
	/// `flags` (`u32`).
	=>
	=>
	FIXED 0x0000
	MOVEABLE 0x0002
	ZEROINIT 0x0040
	GHND Self::MOVEABLE.0 | Self::ZEROINIT.0
	GPTR Self::FIXED.0 | Self::ZEROINIT.0
}

const_bitflag! { GR: u32: "kernel";
	/// [`HPROCESS::GetGuiResources`](crate::prelude::KernelHprocess::GetGuiResources)
	/// `flags` (`u32`).
	=>
	=>
	GDIOBJECTS 0
	GDIOBJECTS_PEAK 2
	USEROBJECTS 1
	USEROBJECTS_PEAK 4
}

const_ordinary! { LANG: u16: "kernel";
	/// Language
	/// [identifier](https://docs.microsoft.com/en-us/windows/win32/intl/language-identifier-constants-and-strings)
	/// (`u16`).
	=>
	=>
	NEUTRAL 0x00
	INVARIANT 0x7f
	AFRIKAANS 0x36
	ALBANIAN 0x1c
	ALSATIAN 0x84
	AMHARIC 0x5e
	ARABIC 0x01
	ARMENIAN 0x2b
	ASSAMESE 0x4d
	AZERI 0x2c
	AZERBAIJANI 0x2c
	BANGLA 0x45
	BASHKIR 0x6d
	BASQUE 0x2d
	BELARUSIAN 0x23
	BENGALI 0x45
	BRETON 0x7e
	BOSNIAN 0x1a
	BOSNIAN_NEUTRAL 0x781a
	BULGARIAN 0x02
	CATALAN 0x03
	CENTRAL_KURDISH 0x92
	CHEROKEE 0x5c
	CHINESE 0x04
	CHINESE_SIMPLIFIED 0x04
	CHINESE_TRADITIONAL 0x7c04
	CORSICAN 0x83
	CROATIAN 0x1a
	CZECH 0x05
	DANISH 0x06
	DARI 0x8c
	DIVEHI 0x65
	DUTCH 0x13
	ENGLISH 0x09
	ESTONIAN 0x25
	FAEROESE 0x38
	FARSI 0x29
	FILIPINO 0x64
	FINNISH 0x0b
	FRENCH 0x0c
	FRISIAN 0x62
	FULAH 0x67
	GALICIAN 0x56
	GEORGIAN 0x37
	GERMAN 0x07
	GREEK 0x08
	GREENLANDIC 0x6f
	GUJARATI 0x47
	HAUSA 0x68
	HAWAIIAN 0x75
	HEBREW 0x0d
	HINDI 0x39
	HUNGARIAN 0x0e
	ICELANDIC 0x0f
	IGBO 0x70
	INDONESIAN 0x21
	INUKTITUT 0x5d
	IRISH 0x3c
	ITALIAN 0x10
	JAPANESE 0x11
	KANNADA 0x4b
	KASHMIRI 0x60
	KAZAK 0x3f
	KHMER 0x53
	KICHE 0x86
	KINYARWANDA 0x87
	KONKANI 0x57
	KOREAN 0x12
	KYRGYZ 0x40
	LAO 0x54
	LATVIAN 0x26
	LITHUANIAN 0x27
	LOWER_SORBIAN 0x2e
	LUXEMBOURGISH 0x6e
	MACEDONIAN 0x2f
	MALAY 0x3e
	MALAYALAM 0x4c
	MALTESE 0x3a
	MANIPURI 0x58
	MAORI 0x81
	MAPUDUNGUN 0x7a
	MARATHI 0x4e
	MOHAWK 0x7c
	MONGOLIAN 0x50
	NEPALI 0x61
	NORWEGIAN 0x14
	OCCITAN 0x82
	ODIA 0x48
	ORIYA 0x48
	PASHTO 0x63
	PERSIAN 0x29
	POLISH 0x15
	PORTUGUESE 0x16
	PULAR 0x67
	PUNJABI 0x46
	QUECHUA 0x6b
	ROMANIAN 0x18
	ROMANSH 0x17
	RUSSIAN 0x19
	SAKHA 0x85
	SAMI 0x3b
	SANSKRIT 0x4f
	SCOTTISH_GAELIC 0x91
	SERBIAN 0x1a
	SERBIAN_NEUTRAL 0x7c1a
	SINDHI 0x59
	SINHALESE 0x5b
	SLOVAK 0x1b
	SLOVENIAN 0x24
	SOTHO 0x6c
	SPANISH 0x0a
	SWAHILI 0x41
	SWEDISH 0x1d
	SYRIAC 0x5a
	TAJIK 0x28
	TAMAZIGHT 0x5f
	TAMIL 0x49
	TATAR 0x44
	TELUGU 0x4a
	THAI 0x1e
	TIBETAN 0x51
	TIGRIGNA 0x73
	TIGRINYA 0x73
	TSWANA 0x32
	TURKISH 0x1f
	TURKMEN 0x42
	UIGHUR 0x80
	UKRAINIAN 0x22
	UPPER_SORBIAN 0x2e
	URDU 0x20
	UZBEK 0x43
	VALENCIAN 0x03
	VIETNAMESE 0x2a
	WELSH 0x52
	WOLOF 0x88
	XHOSA 0x34
	YAKUT 0x85
	YI 0x78
	YORUBA 0x6a
	ZULU 0x35
}

const_bitflag! { MBC: u32: "kernel";
	/// [`MultiByteToWideChar`](crate::MultiByteToWideChar) `flags` (`u32`).
	///
	/// Originally has `MB` prefix.
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	COMPOSITE 0x0000_0002
	ERR_INVALID_CHARS 0x0000_0008
	PRECOMPOSED 0x0000_0001
	USEGLYPHCHARS 0x0000_0004
}

const_ordinary! { PAGE: u32: "kernel";
	/// [`HFILE::CreateFileMapping`](crate::prelude::KernelHfile::CreateFileMapping)
	/// `protect` (`u32`).
	=>
	=>
	/// Allows views to be mapped for read-only copy-on-write or execute
	/// access.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) and
	/// [`GENERIC::EXECUTE`](crate::co::GENERIC::EXECUTE) access rights.
	EXECUTE_READ 0x20
	/// Allows views to be mapped for read-only copy-on-write read/write or
	/// execute access.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ),
	/// [`GENERIC::WRITE`](crate::co::GENERIC::WRITE) and
	/// [`GENERIC::EXECUTE`](crate::co::GENERIC::EXECUTE) access rights.
	EXECUTE_READWRITE 0x40
	/// Allows views to be mapped for read-only copy-on-write or execute
	/// access. This value is equivalent to PAGE_EXECUTE_READ.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) and
	/// [`GENERIC::EXECUTE`](crate::co::GENERIC::EXECUTE) access rights.
	EXECUTE_WRITECOPY 0x80
	/// Allows views to be mapped for read-only or copy-on-write access. An
	/// attempt to write to a specific region results in an access violation.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) access right.
	READONLY 0x02
	/// Allows views to be mapped for read-only copy-on-write or read/write
	/// access.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) and
	/// [`GENERIC::WRITE`](crate::co::GENERIC::WRITE) access rights.
	READWRITE 0x04
	/// Allows views to be mapped for read-only or copy-on-write access. This
	/// value is equivalent to `PAGE::READONLY`.
	///
	/// The file handle must be created with the
	/// [`GENERIC::READ`](crate::co::GENERIC::READ) access right.
	WRITECOPY 0x08

	SEC_COMMIT 0x800_0000
	SEC_IMAGE 0x100_0000
	SEC_IMAGE_NO_EXECUTE 0x1100_0000
	SEC_LARGE_PAGES 0x8000_0000
	SEC_NOCACHE 0x1000_0000
	SEC_RESERVE 0x400_0000
	SEC_WRITECOMBINE 0x4000_0000
}

const_bitflag! { PROCESS: u32: "kernel";
	/// Process
	/// [security and access rights](https://docs.microsoft.com/en-us/windows/win32/procthread/process-security-and-access-rights)
	/// (`u32`).
	=>
	=>
	DELETE ACCESS_RIGHTS::DELETE.0
	READ_CONTROL ACCESS_RIGHTS::READ_CONTROL.0
	SYNCHRONIZE ACCESS_RIGHTS::SYNCHRONIZE.0
	WRITE_DAC ACCESS_RIGHTS::WRITE_DAC.0
	WRITE_OWNER ACCESS_RIGHTS::WRITE_OWNER.0

	ALL_ACCESS STANDARD_RIGHTS::REQUIRED.0 | ACCESS_RIGHTS::SYNCHRONIZE.0 | 0xffff
	CREATE_PROCESS 0x0080
	CREATE_THREAD 0x0002
	DUP_HANDLE 0x0040
	QUERY_INFORMATION 0x0400
	QUERY_LIMITED_INFORMATION 0x1000
	SET_INFORMATION 0x0200
	SET_QUOTA 0x0100
	SUSPEND_RESUME 0x0800
	TERMINATE 0x0001
	WM_OPERATION 0x0008
	VM_READ 0x0010
	VM_WRITE 0x0020
}

const_ordinary! { PROCESSOR: u32: "kernel";
	/// [`SYSTEM_INFO`](crate::SYSTEM_INFO) `dwProcessorType` (`u32`).
	=>
	=>
	INTEL_386 386
	INTEL_486 486
	INTEL_PENTIUM 586
	INTEL_IA64 2200
	AMD_X8664 8664
	MIPS_R4000 4000
	ALPHA_21064 21064
	PPC_601 601
	PPC_603 603
	PPC_604 604
	PPC_620 620
	HITACHI_SH3 10003
	HITACHI_SH3E 10004
	HITACHI_SH4 10005
	MOTOROLA_821 821
	SHx_SH3 103
	SHx_SH4 104
	STRONGARM 2577
	ARM720 1824
	ARM820 2080
	ARM920 2336
	ARM_7TDMI 70001
	OPTIL 0x494f
}

const_ordinary! { PROCESS_NAME: u32: "kernel";
	/// [`QueryFullProcessImageName`](crate::prelude::KernelHprocess::QueryFullProcessImageName)
	/// flags (`u32`).
	=>
	=>
	/// The name should use the Win32 path format.
	WIN32 0
	/// The name should use the native system path format.
	NATIVE 0x0000_0001
}

const_ordinary! { PROCESSOR_ARCHITECTURE: u16: "kernel";
	/// [`SYSTEM_INFO`](crate::SYSTEM_INFO) `wProcessorArchitecture` (`u16`).
	=>
	=>
	INTEL 0
	MIPS 1
	ALPHA 2
	PPC 3
	SHX 4
	ARM 5
	IA64 6
	ALPHA64 7
	MSIL 8
	AMD64 9
	IA32_ON_WIN64 10
	NEUTRAL 11
	ARM64 12
	ARM32_ON_WIN64 13
	IA32_ON_ARM64 14
	UNKNOWN 0xffff
}

const_bitflag! { REPLACEFILE: u32: "kernel";
	/// [`ReplaceFile`](crate::ReplaceFile) `flags` (`u32`).
	=>
	=>
	WRITE_THROUGH 0x0000_0001
	IGNORE_MERGE_ERRORS 0x0000_0002
	IGNORE_ACL_ERRORS 0x0000_0004
}

const_ordinary! { RT: u16: "kernel";
	/// Predefined resource
	/// [types](https://docs.microsoft.com/en-us/windows/win32/menurc/resource-types)
	/// (`u16`).
	=>
	=>
	ACCELERATOR 9
	ANICURSOR 21
	ANIICON 22
	BITMAP 2
	CURSOR 1
	DIALOG 5
	DLGINCLUDE 17
	FONT 8
	FONTDIR 7
	GROUP_CURSOR 12
	GROUP_ICON 14
	HTML 23
	ICON 3
	MANIFEST 24
	MENU 4
	MESSAGETABLE 11
	PLUGPLAY 19
	RCDATA 10
	STRING 6
	VERSION 16
	VXD 20
}

const_ordinary! { SCS: u32: "kernel";
	/// [`GetBinaryType`](crate::GetBinaryType) return value (`u32`).
	=>
	=>
	W_32BIT_BINARY 0
	DOS_BINARY 1
	WOW_BINARY 2
	PIF_BINARY 3
	POSIX_BINARY 4
	OS216_BINARY 5
	W_64BIT_BINARY 6
}

const_bitflag! { SECTION: u32: "kernel";
	/// Composes [`FILE_MAP`](crate::co::FILE_MAP) (`u32`).
	=>
	=>
	QUERY 0x0001
	MAP_WRITE 0x0002
	MAP_READ 0x0004
	MAP_EXECUTE 0x0008
	EXTEND_SIZE 0x0010
	MAP_EXECUTE_EXPLICIT 0x0020
	ALL_ACCESS STANDARD_RIGHTS::REQUIRED.0 | Self::QUERY.0 | Self::MAP_WRITE.0 | Self::MAP_EXECUTE.0 | Self::EXTEND_SIZE.0
}

const_bitflag! { SECURITY_IMPERSONATION: u32: "kernel";
	/// [`SECURITY_IMPERSONATION_LEVEL`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-security_impersonation_level)
	/// enumeration (`u32`).
	=>
	=>
	Anonymous 0
	Identification 1
	Impersonation 2
	Delegation 3
}

const_ordinary! { SORT: u16: "kernel";
	/// Sort order
	/// [identifiers](https://docs.microsoft.com/en-us/windows/win32/intl/sort-order-identifiers)
	/// (`u16`).
	=>
	=>
	DEFAULT 0x0
	INVARIANT_MATH 0x1
	JAPANESE_XJIS 0x0
	JAPANESE_UNICODE 0x1
	JAPANESE_RADICALSTROKE 0x4
	CHINESE_BIG5 0x0
	CHINESE_PRCP 0x0
	CHINESE_UNICODE 0x1
	CHINESE_PRC 0x2
	CHINESE_BOPOMOFO 0x3
	CHINESE_RADICALSTROKE 0x4
	KOREAN_KSC 0x0
	KOREAN_UNICODE 0x1
	GERMAN_PHONE_BOOK 0x1
	HUNGARIAN_DEFAULT 0x0
	HUNGARIAN_TECHNICAL 0x1
	GEORGIAN_TRADITIONAL 0x0
	GEORGIAN_MODERN 0x1
}

const_bitflag! { STANDARD_RIGHTS: u32: "kernel";
	/// Standard access rights
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/secauthz/standard-access-rights)
	/// (`u32`).
	=>
	=>
	REQUIRED 0x000_f0000
	READ ACCESS_RIGHTS::READ_CONTROL.0
	WRITE ACCESS_RIGHTS::READ_CONTROL.0
	EXECUTE ACCESS_RIGHTS::READ_CONTROL.0
	ALL 0x001_f0000
}

const_bitflag! { STARTF: u32: "kernel";
	/// [`HPROCESS::CreateProcess`](crate::prelude::KernelHprocess::CreateProcess)
	/// `dwFlags` (`u32`).
	=>
	=>
	FORCEONFEEDBACK 0x0000_0040
	FORCEOFFFEEDBACK 0x0000_0080
	PREVENTPINNING 0x0000_2000
	RUNFULLSCREEN 0x0000_0020
	TITLEISAPPID 0x0000_1000
	TITLEISLINKNAME 0x0000_0800
	UNTRUSTEDSOURCE 0x0000_8000
	USECOUNTCHARS 0x0000_0008
	USEFILLATTRIBUTE 0x0000_0010
	USEHOTKEY 0x0000_0200
	USEPOSITION 0x0000_0004
	USESHOWWINDOW 0x0000_0001
	USESIZE 0x0000_0002
	USESTDHANDLES 0x0000_0100
}

const_ordinary! { SUBLANG: u16: "kernel";
	/// Sublanguage
	/// [identifier](https://docs.microsoft.com/en-us/windows/win32/intl/language-identifier-constants-and-strings)
	/// (`u16`).
	=>
	=>
	NEUTRAL 0x00
	DEFAULT 0x01
	SYS_DEFAULT 0x02
	CUSTOM_DEFAULT 0x03
	CUSTOM_UNSPECIFIED 0x04
	UI_CUSTOM_DEFAULT 0x05
	AFRIKAANS_SOUTH_AFRICA 0x01
	ALBANIAN_ALBANIA 0x01
	ALSATIAN_FRANCE 0x01
	AMHARIC_ETHIOPIA 0x01
	ARABIC_SAUDI_ARABIA 0x01
	ARABIC_IRAQ 0x02
	ARABIC_EGYPT 0x03
	ARABIC_LIBYA 0x04
	ARABIC_ALGERIA 0x05
	ARABIC_MOROCCO 0x06
	ARABIC_TUNISIA 0x07
	ARABIC_OMAN 0x08
	ARABIC_YEMEN 0x09
	ARABIC_SYRIA 0x0a
	ARABIC_JORDAN 0x0b
	ARABIC_LEBANON 0x0c
	ARABIC_KUWAIT 0x0d
	ARABIC_UAE 0x0e
	ARABIC_BAHRAIN 0x0f
	ARABIC_QATAR 0x10
	ARMENIAN_ARMENIA 0x01
	ASSAMESE_INDIA 0x01
	AZERI_LATIN 0x01
	AZERI_CYRILLIC 0x02
	AZERBAIJANI_AZERBAIJAN_LATIN 0x01
	AZERBAIJANI_AZERBAIJAN_CYRILLIC 0x02
	BANGLA_INDIA 0x01
	BANGLA_BANGLADESH 0x02
	BASHKIR_RUSSIA 0x01
	BASQUE_BASQUE 0x01
	BELARUSIAN_BELARUS 0x01
	BENGALI_INDIA 0x01
	BENGALI_BANGLADESH 0x02
	BOSNIAN_BOSNIA_HERZEGOVINA_LATIN 0x05
	BOSNIAN_BOSNIA_HERZEGOVINA_CYRILLIC 0x08
	BRETON_FRANCE 0x01
	BULGARIAN_BULGARIA 0x01
	CATALAN_CATALAN 0x01
	CENTRAL_KURDISH_IRAQ 0x01
	CHEROKEE_CHEROKEE 0x01
	CHINESE_TRADITIONAL 0x01
	CHINESE_SIMPLIFIED 0x02
	CHINESE_HONGKONG 0x03
	CHINESE_SINGAPORE 0x04
	CHINESE_MACAU 0x05
	CORSICAN_FRANCE 0x01
	CZECH_CZECH_REPUBLIC 0x01
	CROATIAN_CROATIA 0x01
	CROATIAN_BOSNIA_HERZEGOVINA_LATIN 0x04
	DANISH_DENMARK 0x01
	DARI_AFGHANISTAN 0x01
	DIVEHI_MALDIVES 0x01
	DUTCH 0x01
	DUTCH_BELGIAN 0x02
	ENGLISH_US 0x01
	ENGLISH_UK 0x02
	ENGLISH_AUS 0x03
	ENGLISH_CAN 0x04
	ENGLISH_NZ 0x05
	ENGLISH_EIRE 0x06
	ENGLISH_SOUTH_AFRICA 0x07
	ENGLISH_JAMAICA 0x08
	ENGLISH_CARIBBEAN 0x09
	ENGLISH_BELIZE 0x0a
	ENGLISH_TRINIDAD 0x0b
	ENGLISH_ZIMBABWE 0x0c
	ENGLISH_PHILIPPINES 0x0d
	ENGLISH_INDIA 0x10
	ENGLISH_MALAYSIA 0x11
	ENGLISH_SINGAPORE 0x12
	ESTONIAN_ESTONIA 0x01
	FAEROESE_FAROE_ISLANDS 0x01
	FILIPINO_PHILIPPINES 0x01
	FINNISH_FINLAND 0x01
	FRENCH 0x01
	FRENCH_BELGIAN 0x02
	FRENCH_CANADIAN 0x03
	FRENCH_SWISS 0x04
	FRENCH_LUXEMBOURG 0x05
	FRENCH_MONACO 0x06
	FRISIAN_NETHERLANDS 0x01
	FULAH_SENEGAL 0x02
	GALICIAN_GALICIAN 0x01
	GEORGIAN_GEORGIA 0x01
	GERMAN 0x01
	GERMAN_SWISS 0x02
	GERMAN_AUSTRIAN 0x03
	GERMAN_LUXEMBOURG 0x04
	GERMAN_LIECHTENSTEIN 0x05
	GREEK_GREECE 0x01
	GREENLANDIC_GREENLAND 0x01
	GUJARATI_INDIA 0x01
	HAUSA_NIGERIA_LATIN 0x01
	HAWAIIAN_US 0x01
	HEBREW_ISRAEL 0x01
	HINDI_INDIA 0x01
	HUNGARIAN_HUNGARY 0x01
	ICELANDIC_ICELAND 0x01
	IGBO_NIGERIA 0x01
	INDONESIAN_INDONESIA 0x01
	INUKTITUT_CANADA 0x01
	INUKTITUT_CANADA_LATIN 0x02
	IRISH_IRELAND 0x02
	ITALIAN 0x01
	ITALIAN_SWISS 0x02
	JAPANESE_JAPAN 0x01
	KANNADA_INDIA 0x01
	KASHMIRI_SASIA 0x02
	KASHMIRI_INDIA 0x02
	KAZAK_KAZAKHSTAN 0x01
	KHMER_CAMBODIA 0x01
	KICHE_GUATEMALA 0x01
	KINYARWANDA_RWANDA 0x01
	KONKANI_INDIA 0x01
	KOREAN 0x01
	KYRGYZ_KYRGYZSTAN 0x01
	LAO_LAO 0x01
	LATVIAN_LATVIA 0x01
	LITHUANIAN 0x01
	LOWER_SORBIAN_GERMANY 0x02
	LUXEMBOURGISH_LUXEMBOURG 0x01
	MACEDONIAN_MACEDONIA 0x01
	MALAY_MALAYSIA 0x01
	MALAY_BRUNEI_DARUSSALAM 0x02
	MALAYALAM_INDIA 0x01
	MALTESE_MALTA 0x01
	MAORI_NEW_ZEALAND 0x01
	MAPUDUNGUN_CHILE 0x01
	MARATHI_INDIA 0x01
	MOHAWK_MOHAWK 0x01
	MONGOLIAN_CYRILLIC_MONGOLIA 0x01
	MONGOLIAN_PRC 0x02
	NEPALI_INDIA 0x02
	NEPALI_NEPAL 0x01
	NORWEGIAN_BOKMAL 0x01
	NORWEGIAN_NYNORSK 0x02
	OCCITAN_FRANCE 0x01
	ODIA_INDIA 0x01
	ORIYA_INDIA 0x01
	PASHTO_AFGHANISTAN 0x01
	PERSIAN_IRAN 0x01
	POLISH_POLAND 0x01
	PORTUGUESE 0x02
	PORTUGUESE_BRAZILIAN 0x01
	PULAR_SENEGAL 0x02
	PUNJABI_INDIA 0x01
	PUNJABI_PAKISTAN 0x02
	QUECHUA_BOLIVIA 0x01
	QUECHUA_ECUADOR 0x02
	QUECHUA_PERU 0x03
	ROMANIAN_ROMANIA 0x01
	ROMANSH_SWITZERLAND 0x01
	RUSSIAN_RUSSIA 0x01
	SAKHA_RUSSIA 0x01
	SAMI_NORTHERN_NORWAY 0x01
	SAMI_NORTHERN_SWEDEN 0x02
	SAMI_NORTHERN_FINLAND 0x03
	SAMI_LULE_NORWAY 0x04
	SAMI_LULE_SWEDEN 0x05
	SAMI_SOUTHERN_NORWAY 0x06
	SAMI_SOUTHERN_SWEDEN 0x07
	SAMI_SKOLT_FINLAND 0x08
	SAMI_INARI_FINLAND 0x09
	SANSKRIT_INDIA 0x01
	SCOTTISH_GAELIC 0x01
	SERBIAN_BOSNIA_HERZEGOVINA_LATIN 0x06
	SERBIAN_BOSNIA_HERZEGOVINA_CYRILLIC 0x07
	SERBIAN_MONTENEGRO_LATIN 0x0b
	SERBIAN_MONTENEGRO_CYRILLIC 0x0c
	SERBIAN_SERBIA_LATIN 0x09
	SERBIAN_SERBIA_CYRILLIC 0x0a
	SERBIAN_CROATIA 0x01
	SERBIAN_LATIN 0x02
	SERBIAN_CYRILLIC 0x03
	SINDHI_INDIA 0x01
	SINDHI_PAKISTAN 0x02
	SINDHI_AFGHANISTAN 0x02
	SINHALESE_SRI_LANKA 0x01
	SOTHO_NORTHERN_SOUTH_AFRICA 0x01
	SLOVAK_SLOVAKIA 0x01
	SLOVENIAN_SLOVENIA 0x01
	SPANISH 0x01
	SPANISH_MEXICAN 0x02
	SPANISH_MODERN 0x03
	SPANISH_GUATEMALA 0x04
	SPANISH_COSTA_RICA 0x05
	SPANISH_PANAMA 0x06
	SPANISH_DOMINICAN_REPUBLIC 0x07
	SPANISH_VENEZUELA 0x08
	SPANISH_COLOMBIA 0x09
	SPANISH_PERU 0x0a
	SPANISH_ARGENTINA 0x0b
	SPANISH_ECUADOR 0x0c
	SPANISH_CHILE 0x0d
	SPANISH_URUGUAY 0x0e
	SPANISH_PARAGUAY 0x0f
	SPANISH_BOLIVIA 0x10
	SPANISH_EL_SALVADOR 0x11
	SPANISH_HONDURAS 0x12
	SPANISH_NICARAGUA 0x13
	SPANISH_PUERTO_RICO 0x14
	SPANISH_US 0x15
	SWAHILI_KENYA 0x01
	SWEDISH 0x01
	SWEDISH_FINLAND 0x02
	SYRIAC_SYRIA 0x01
	TAJIK_TAJIKISTAN 0x01
	TAMAZIGHT_ALGERIA_LATIN 0x02
	TAMAZIGHT_MOROCCO_TIFINAGH 0x04
	TAMIL_INDIA 0x01
	TAMIL_SRI_LANKA 0x02
	TATAR_RUSSIA 0x01
	TELUGU_INDIA 0x01
	THAI_THAILAND 0x01
	TIBETAN_PRC 0x01
	TIGRIGNA_ERITREA 0x02
	TIGRINYA_ERITREA 0x02
	TIGRINYA_ETHIOPIA 0x01
	TSWANA_BOTSWANA 0x02
	TSWANA_SOUTH_AFRICA 0x01
	TURKISH_TURKEY 0x01
	TURKMEN_TURKMENISTAN 0x01
	UIGHUR_PRC 0x01
	UKRAINIAN_UKRAINE 0x01
	UPPER_SORBIAN_GERMANY 0x01
	URDU_PAKISTAN 0x01
	URDU_INDIA 0x02
	UZBEK_LATIN 0x01
	UZBEK_CYRILLIC 0x02
	VALENCIAN_VALENCIA 0x02
	VIETNAMESE_VIETNAM 0x01
	WELSH_UNITED_KINGDOM 0x01
	WOLOF_SENEGAL 0x01
	XHOSA_SOUTH_AFRICA 0x01
	YAKUT_RUSSIA 0x01
	YI_PRC 0x01
	YORUBA_NIGERIA 0x01
	ZULU_SOUTH_AFRICA 0x01
}

const_ordinary! { SW: i32: "kernel";
	/// [`HWND::ShowWindow`](crate::prelude::UserHwnd::ShowWindow) `show_cmd`
	/// (`i32`).
	=>
	=>
	/// Hides the window and activates another window.
	HIDE 0
	/// Activates and displays a window. If the window is minimized or
	/// maximized the system restores it to its original size and position. An
	/// application should specify this flag when displaying the window for the
	/// first time.
	SHOWNORMAL 1
	/// Activates the window and displays it as a minimized window.
	SHOWMINIMIZED 2
	/// Activates the window and displays it as a maximized window.
	SHOWMAXIMIZED 3
	/// Displays a window in its most recent size and position. This value is
	/// similar to `SW::SHOWNORMAL` except that the window is not activated.
	SHOWNOACTIVATE 4
	/// Activates the window and displays it in its current size and position.
	SHOW 5
	/// Minimizes the specified window and activates the next top-level window
	/// in the Z order.
	MINIMIZE 6
	/// Displays the window as a minimized window. This value is similar to
	/// `SW::SHOWMINIMIZED` except the window is not activated.
	SHOWMINNOACTIVE 7
	// Displays the window in its current size and position. This value is
	// similar to `SW::SHOW` except that the window is not activated.
	SHOWNA 8
	/// Activates and displays the window. If the window is minimized or
	/// maximized the system restores it to its original size and position. An
	/// application should specify this flag when restoring a minimized window.
	RESTORE 9
	/// Sets the show state based on the SW value specified in the
	/// [`STARTUPINFO`](crate::STARTUPINFO) structure passed to the
	/// [`HPROCESS::CreateProcess`](crate::prelude::KernelHprocess::CreateProcess)
	/// function by the program that started the application.
	SHOWDEFAULT 10
	/// Minimizes a window even if the thread that owns the window is not
	/// responding. This flag should only be used when minimizing windows from a
	/// different thread.
	FORCEMINIMIZE 11
}

const_bitflag! { TH32CS: u32: "kernel";
	/// [`HPROCESSLIST::CreateToolhelp32Snapshot`](crate::HPROCESSLIST) `flags`
	/// (`u32`).
	=>
	=>
	SNAPHEAPLIST 0x0000_0001
	SNAPPROCESS 0x0000_0002
	SNAPTHREAD 0x0000_0004
	SNAPMODULE 0x0000_0008
	SNAPMODULE32 0x0000_0010
	SNAPALL Self::SNAPHEAPLIST.0 | Self::SNAPPROCESS.0 | Self::SNAPTHREAD.0 | Self::SNAPMODULE.0
	INHERIT 0x8000_0000
}

const_bitflag! { THREAD_CREATE: u32: "kernel";
	/// [`HTHREAD::CreateThread`](crate::prelude::KernelHthread::CreateThread)
	/// `flags` (`u32`). Originally has no prefix.
	=>
	=>
	/// Originally just a zero.
	RUN_IMMEDIATELY 0
	CREATE_SUSPENDED 0x0000_0004
	STACK_SIZE_PARAM_IS_A_RESERVATION 0x0001_0000
}

const_bitflag! { TOKEN: u32: "kernel";
	/// [Token access rights](https://docs.microsoft.com/en-us/windows/win32/secauthz/access-rights-for-access-token-objects).
	=>
	=>
	DELETE ACCESS_RIGHTS::DELETE.0
	READ_CONTROL ACCESS_RIGHTS::READ_CONTROL.0
	WRITE_DAC ACCESS_RIGHTS::WRITE_DAC.0
	WRITE_OWNER ACCESS_RIGHTS::WRITE_OWNER.0

	ASSIGN_PRIMARY 0x0001
	DUPLICATE 0x0002
	IMPERSONATE 0x0004
	QUERY 0x0008
	QUERY_SOURCE 0x0010
	ADJUST_PRIVILEGES 0x0020
	ADJUST_GROUPS 0x0040
	ADJUST_DEFAULT 0x0080
	ADJUST_SESSIONID 0x0100
	ALL_ACCESS_P STANDARD_RIGHTS::REQUIRED.0 | Self::ASSIGN_PRIMARY.0 | Self::DUPLICATE.0 | Self::IMPERSONATE.0 | Self::QUERY.0 | Self::QUERY_SOURCE.0 | Self::ADJUST_PRIVILEGES.0 | Self::ADJUST_GROUPS.0 | Self::ADJUST_DEFAULT.0
	ALL_ACCESS Self::ALL_ACCESS_P.0 | Self::ADJUST_SESSIONID.0
	READ STANDARD_RIGHTS::READ.0 | Self::QUERY.0
	WRITE STANDARD_RIGHTS::WRITE.0 | Self::ADJUST_PRIVILEGES.0 | Self::ADJUST_GROUPS.0 | Self::ADJUST_DEFAULT.0
	EXECUTE STANDARD_RIGHTS::EXECUTE.0
	TRUST_CONSTRAINT_MASK STANDARD_RIGHTS::READ.0 | Self::QUERY.0 | Self::QUERY_SOURCE.0
	ACCESS_PSEUDO_HANDLE Self::QUERY.0 | Self::QUERY_SOURCE.0
}

const_ordinary! { VER_COND: u8: "kernel";
	/// [`VerSetConditionMask`](crate::VerSetConditionMask) `condition` (`u8`).
	=>
	=>
	EQUAL 1
	GREATER 2
	GREATER_EQUAL 3
	LESS 4
	LESS_EQUAL 5
	AND 6
	OR 7
	CONDITION_MASK 7
}

const_bitflag! { VER_MASK: u32: "kernel";
	/// [`VerifyVersionInfo`](crate::VerifyVersionInfo) and
	/// [`VerSetConditionMask`](crate::VerSetConditionMask) `type_mask` (`u32`).
	=>
	=>
	MINORVERSION 0x000_0001
	MAJORVERSION 0x000_0002
	BUILDNUMBER 0x000_0004
	PLATFORMID 0x000_0008
	SERVICEPACKMINOR 0x000_0010
	SERVICEPACKMAJOR 0x000_0020
	SUITENAME 0x000_0040
	PRODUCT_TYPE 0x000_0080
}

const_ordinary! { VER_NT: u8: "kernel";
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `wProductType` (`u8`).
	=>
	=>
	WORKSTATION 0x000_0001
	DOMAIN_CONTROLLER 0x000_0002
	SERVER 0x000_0003
}

const_bitflag! { VER_SUITE: u16: "kernel";
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `wSuiteMask` (`u16`).
	=>
	=>
	SMALLBUSINESS 0x0001
	ENTERPRISE 0x0002
	BACKOFFICE 0x0004
	COMMUNICATIONS 0x0008
	TERMINAL 0x0010
	SMALLBUSINESS_RESTRICTED 0x0020
	EMBEDDEDNT 0x0040
	DATACENTER 0x0080
	SINGLEUSERTS 0x0100
	PERSONAL 0x0200
	BLADE 0x0400
	EMBEDDED_RESTRICTED 0x0800
	SECURITY_APPLIANCE 0x1000
	STORAGE_SERVER 0x2000
	COMPUTE_SERVER 0x4000
	WH_SERVER 0x8000
	//MULTIUSERTS 0x00020000 // Win32 bug, truncated to zero as u16
}

const_ordinary! { VER_PLATFORM: u32: "kernel";
	/// [`OSVERSIONINFOEX`](crate::OSVERSIONINFOEX) `dwPlatformId` (`u32`).
	=>
	=>
	WIN32s 0
	WIN32_WINDOWS 1
	WIN32_NT 2
}

const_ordinary! { WAIT: u32: "kernel";
	/// [`HPROCESS::WaitForSingleObject`](crate::prelude::KernelHprocess::WaitForSingleObject)
	/// return value (`u32`).
	=>
	=>
	ABANDONED 0x0000_0080
	OBJECT_0 0x0000_0000
	TIMEOUT 0x0000_0102
	FAILED 0xffff_ffff
}

const_bitflag! { WC: u32: "kernel";
	/// [`WideCharToMultiByte`](crate::WideCharToMultiByte) `flags` (`u32`).
	=>
	=>
	COMPOSITECHECK 0x0000_0200
	ERR_INVALID_CHARS 0x0000_0080
	NO_BEST_FIT_CHARS 0x0000_0400
	DEFAULTCHAR 0x0000_0040
	DISCARDNS 0x0000_0010
	SEPCHARS 0x0000_0020
}

const_ordinary! { WIN32: u16: "kernel";
	/// [`_WIN32`](https://docs.microsoft.com/en-us/windows/win32/winprog/using-the-windows-headers)
	/// version definitions (`u16`).
	=>
	=>
	WINNT_NT4 0x0400
	WINNT_WIN2K 0x0500
	WINNT_WINXP 0x0501
	WINNT_WS03 0x0502
	WINNT_WIN6 0x0600
	WINNT_VISTA 0x0600
	WINNT_WS08 0x0600
	WINNT_LONGHORN 0x0600
	WINNT_WIN7 0x0601
	WINNT_WIN8 0x0602
	WINNT_WINBLUE 0x0603
	WINNT_WINTHRESHOLD 0x0a00
	WINNT_WIN10 0x0a00

	IE_IE20 0x0200
	IE_IE30 0x0300
	IE_IE302 0x0302
	IE_IE40 0x0400
	IE_IE401 0x0401
	IE_IE50 0x0500
	IE_IE501 0x0501
	IE_IE55 0x0550
	IE_IE60 0x0600
	IE_IE60SP1 0x0601
	IE_IE60SP2 0x0603
	IE_IE70 0x0700
	IE_IE80 0x0800
	IE_IE90 0x0900
	IE_IE100 0x0A00
	IE_IE110 0x0A00

	IE_NT4 Self::IE_IE20.0
	IE_NT4SP1 Self::IE_IE20.0
	IE_NT4SP2 Self::IE_IE20.0
	IE_NT4SP3 Self::IE_IE302.0
	IE_NT4SP4 Self::IE_IE401.0
	IE_NT4SP5 Self::IE_IE401.0
	IE_NT4SP6 Self::IE_IE50.0
	IE_WIN98 Self::IE_IE401.0
	IE_WIN98SE Self::IE_IE50.0
	IE_WINME Self::IE_IE55.0
	IE_WIN2K Self::IE_IE501.0
	IE_WIN2KSP1 Self::IE_IE501.0
	IE_WIN2KSP2 Self::IE_IE501.0
	IE_WIN2KSP3 Self::IE_IE501.0
	IE_WIN2KSP4 Self::IE_IE501.0
	IE_XP Self::IE_IE60.0
	IE_XPSP1 Self::IE_IE60SP1.0
	IE_XPSP2 Self::IE_IE60SP2.0
	IE_WS03 0x0602
	IE_WS03SP1 Self::IE_IE60SP2.0
	IE_WIN6 Self::IE_IE70.0
	IE_LONGHORN Self::IE_IE70.0
	IE_WIN7 Self::IE_IE80.0
	IE_WIN8 Self::IE_IE100.0
	IE_WINBLUE Self::IE_IE100.0
	IE_WINTHRESHOLD Self::IE_IE110.0
	IE_WIN10 Self::IE_IE110.0
}
