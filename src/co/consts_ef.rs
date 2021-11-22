#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::co::{ACCESS_RIGHTS, SECTION, STANDARD_RIGHTS, WM};

pub_struct_const! { EDS: u32;
	/// [`EnumDisplaySettingsEx`](crate::EnumDisplaySettingsEx) `flags` (`u32`).
	=>
	RAWMODE 0x0000_0002
	ROTATEDMODE 0x0000_0004
}

pub_struct_const! { EIMES: u16;
	/// [`em::GetImeStatus`](crate::msg::em::GetImeStatus) return value (`u16`).
	=>
	GETCOMPSTRATONCE 0x0001
	CANCELCOMPSTRINFOCUS 0x0002
	COMPLETECOMPSTRKILLFOCUS 0x0004
}

pub_struct_const_wm! { EM;
	/// Edit control
	/// [messages](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-messages)
	/// (`u32`) convertible to [`WM`](crate::co::WM).
	=>
	FIRST 0x1500
	=>
	SETCUEBANNER Self::FIRST.0 + 1
	GETCUEBANNER Self::FIRST.0 + 2
	SHOWBALLOONTIP Self::FIRST.0 + 3
	HIDEBALLOONTIP Self::FIRST.0 + 4
	SETHILITE Self::FIRST.0 + 5
	GETHILITE Self::FIRST.0 + 6
	NOSETFOCUS Self::FIRST.0 + 7
	TAKEFOCUS Self::FIRST.0 + 8
	SETEXTENDEDSTYLE Self::FIRST.0 + 10
	GETEXTENDEDSTYLE Self::FIRST.0 + 11
	SETENDOFLINE Self::FIRST.0 + 12
	GETENDOFLINE Self::FIRST.0 + 13
	ENABLESEARCHWEB Self::FIRST.0 + 14
	SEARCHWEB Self::FIRST.0 + 15
	SETCARETINDEX Self::FIRST.0 + 17
	GETCARETINDEX Self::FIRST.0 + 18
	GETZOOM WM::USER.0 + 224
	SETZOOM WM::USER.0 + 225
	FILELINEFROMCHAR Self::FIRST.0 + 19
	FILELINEINDEX Self::FIRST.0 + 20
	FILELINELENGTH Self::FIRST.0 + 21
	GETFILELINE Self::FIRST.0 + 22
	GETFILELINECOUNT Self::FIRST.0 + 23

	GETSEL 0x00b0
	SETSEL 0x00b1
	GETRECT 0x00b2
	SETRECT 0x00b3
	SETRECTNP 0x00b4
	SCROLL 0x00b5
	LINESCROLL 0x00b6
	SCROLLCARET 0x00b7
	GETMODIFY 0x00b8
	SETMODIFY 0x00b9
	GETLINECOUNT 0x00ba
	LINEINDEX 0x00bb
	SETHANDLE 0x00bc
	GETHANDLE 0x00bd
	GETTHUMB 0x00be
	LINELENGTH 0x00c1
	REPLACESEL 0x00c2
	GETLINE 0x00c4
	LIMITTEXT 0x00c5
	CANUNDO 0x00c6
	UNDO 0x00c7
	FMTLINES 0x00c8
	LINEFROMCHAR 0x00c9
	SETTABSTOPS 0x00cb
	SETPASSWORDCHAR 0x00cc
	EMPTYUNDOBUFFER 0x00cd
	GETFIRSTVISIBLELINE 0x00ce
	SETREADONLY 0x00cf
	SETWORDBREAKPROC 0x00d0
	GETWORDBREAKPROC 0x00d1
	GETPASSWORDCHAR 0x00d2
	SETMARGINS 0x00d3
	GETMARGINS 0x00d4
	SETLIMITTEXT Self::LIMITTEXT.0
	GETLIMITTEXT 0x00d5
	POSFROMCHAR 0x00d6
	CHARFROMPOS 0x00d7
	SETIMESTATUS 0x00d8
	GETIMESTATUS 0x00d9
	ENABLEFEATURE 0x00da
}

pub_struct_const! { EMF: u32;
	/// [`NMLVEMPTYMARKUP`](crate::NMLVEMPTYMARKUP) `dwFlags` (`u32`).
	=>
	LEFT 0x0000_0000
	CENTERED 0x0000_0001
}

pub_struct_const_cmd! { EN;
	/// Edit control `WM_COMMAND`
	/// [notifications](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-edit-control-reference-notifications)
	/// (`u16`) convertible to [`CMD`](crate::co::CMD).
	=>
	SETFOCUS 0x0100
	KILLFOCUS 0x0200
	CHANGE 0x0300
	UPDATE 0x0400
	ERRSPACE 0x0500
	MAXTEXT 0x0501
	HSCROLL 0x0601
	VSCROLL 0x0602
	ALIGN_LTR_EC 0x0700
	ALIGN_RTL_EC 0x0701
	BEFORE_PASTE 0x0800
	AFTER_PASTE 0x0801
}

pub_struct_const! { ENDSESSION: u32;
	/// [`wm::EndSession`](crate::msg::wm::EndSession) event (`u32`).
	=>
	RESTARTORSHUTDOWN 0
	CLOSEAPP 0x0000_0001
	CRITICAL 0x4000_0000
	LOGOFF 0x8000_0000
}

pub_struct_const! { ENUM_SETTINGS: u32;
	/// [`EnumDisplaySettingsEx`](crate::EnumDisplaySettingsEx) `mode_num`
	/// (`u32`). Originally with `ENUM` prefix and `SETTINGS` suffix.
	=>
	CURRENT (0 - 1) as u32
	REGISTRY (0 - 2) as u32
}

pub_struct_const_ws! { ES: u32;
	/// Edit control
	/// [styles](https://docs.microsoft.com/en-us/windows/win32/controls/edit-control-styles)
	/// (`u32`) convertible to [`WS`](crate::co::WS).
	=>
	LEFT 0x0000
	CENTER 0x0001
	RIGHT 0x0002
	MULTILINE 0x0004
	UPPERCASE 0x0008
	LOWERCASE 0x0010
	PASSWORD 0x0020
	AUTOVSCROLL 0x0040
	AUTOHSCROLL 0x0080
	NOHIDESEL 0x0100
	OEMCONVERT 0x0400
	READONLY 0x0800
	WANTRETURN 0x1000
	NUMBER 0x2000
}

pub_struct_const! { FAPPCOMMAND: u16;
	/// [`wm::AppCommand`](crate::msg::wm::AppCommand) input event (`u16`).
	=>
	MOUSE 0x8000
	KEY 0
	OEM 0x1000
}

pub_struct_const! { FF: u8;
	/// [`LOGFONT`](crate::LOGFONT) `lfPitchAndFamily` (`u8`) used with
	/// [`PITCH`](crate::co::PITCH).
	=>
	DONTCARE 0 << 4
	ROMAN 1 << 4
	SWISS 2 << 4
	MODERN 3 << 4
	SCRIPT 4 << 4
	DECORATIVE 5 << 4
}

pub_struct_const! { FILE_ATTRIBUTE: u32;
	/// File attribute
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants),
	/// also other flags from [`HFILE::CreateFile`](crate::HFILE::CreateFile)
	/// `flags_and_attrs` (`u32`).
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

pub_struct_const! { FILE_MAP: u32;
	/// [`HFILEMAP::MapViewOfFile`](crate::HFILEMAP::MapViewOfFile)
	/// `desired_access` (`u32`).
	=>
	ALL_ACCESS SECTION::ALL_ACCESS.0
	READ SECTION::MAP_READ.0
	WRITE SECTION::MAP_WRITE.0

	COPY 0x0000_0001
	EXECUTE SECTION::MAP_EXECUTE_EXPLICIT.0
	LARGE_PAGES 0x2000_0000
	TARGETS_INVALID 0x4000_0000
}

pub_struct_const! { FILE_RIGHT: u32;
	/// File access rights
	/// [flags](https://docs.microsoft.com/en-us/windows/win32/fileio/file-access-rights-constants)
	/// (`u32`). Originally has `FILE` prefix.
	=>
	ADD_FILE 0x0002
	ADD_SUBDIRECTORY 0x0004
	ALL_ACCESS STANDARD_RIGHTS::REQUIRED.0 | ACCESS_RIGHTS::SYNCHRONIZE.0 | 0x1ff
	APPEND_DATA 0x0004
	CREATE_PIPE_INSTANCE 0x0004
	DELETE_CHILD 0x0040
	EXECUTE 0x0020
	LIST_DIRECTORY 0x0001
	READ_ATTRIBUTES 0x0080
	READ_DATA 0x0001
	READ_EA 0x0008
	TRAVERSE 0x0020
	WRITE_ATTRIBUTES 0x0100
	WRITE_DATA 0x0002
}

pub_struct_const! { FILE_SHARE: u32;
	/// [`HFILE::CreateFile`](crate::HFILE::CreateFile) `share_mode` (`u32`).
	=>
	/// None of the actual values (zero).
	NoValue 0
	READ 0x0000_0001
	WRITE 0x0000_0002
	DELETE 0x0000_0004
}

pub_struct_const! { FILE_STARTING_POINT: u32;
	/// [`HFILE::SetFilePointerEx`](crate::HFILE::SetFilePointerEx)
	/// `move_method` (`u32`). Originally has `FILE` prefix.
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

pub_struct_const! { FILE_TYPE: u32;
	/// [`HFILE::GetFileType`](crate::HFILE::GetFileType) return value (`u32`).
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

pub_struct_const! { FIRMWARE_TYPE: u32;
	/// [`FIRMWARE_TYPE`](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-firmware_type)
	/// enumeration (`u32`).
	=>
	Unknown 0
	Bios 1
	Uefi 2
}

pub_struct_const! { FO: u32;
	/// [`SHFILEOPSTRUCT`](crate::SHFILEOPSTRUCT) `wFunc` (`u32`).
	=>
	MOVE 0x0001
	COPY 0x0002
	DELETE 0x0003
	RENAME 0x0004
}

pub_struct_const! { FOF: u16;
	/// [`SHFILEOPSTRUCT`](crate::SHFILEOPSTRUCT) `fFlags` (`u16`).
	=>
	MULTIDESTFILES 0x0001
	CONFIRMMOUSE 0x0002
	SILENT 0x0004
	RENAMEONCOLLISION 0x0008
	NOCONFIRMATION 0x0010
	WANTMAPPINGHANDLE 0x0020
	ALLOWUNDO 0x0040
	FILESONLY 0x0080
	SIMPLEPROGRESS 0x0100
	NOCONFIRMMKDIR 0x0200
	NOERRORUI 0x0400
	NOCOPYSECURITYATTRIBS 0x0800
	NORECURSION 0x1000
	NO_CONNECTED_ELEMENTS 0x2000
	WANTNUKEWARNING 0x4000
	NORECURSEREPARSE 0x8000
	NO_UI Self::SILENT.0 | Self::NOCONFIRMATION.0 | Self::NOERRORUI.0 | Self::NOCONFIRMMKDIR.0
}

pub_struct_const! { FORMAT_MESSAGE: u32;
	/// [`FormatMessage`](crate::co::ERROR::FormatMessage) `dwFlags` (`u32`).
	=>
	ALLOCATE_BUFFER 0x0000_0100
	ARGUMENT_ARRAY 0x0000_2000
	FROM_HMODULE 0x0000_0800
	FROM_STRING 0x0000_0400
	FROM_SYSTEM 0x0000_1000
	IGNORE_INSERTS 0x0000_0200
	MAX_WIDTH_MASK 0x0000_00ff
}

pub_struct_const! { FW: u32;
	/// [`HFONT::CreateFont`](crate::HFONT::CreateFont) `weight` and
	/// [`LOGFONT`](crate::LOGFONT) `lfWeight` (`u32`).
	=>
	DONTCARE 0
	THIN 100
	EXTRALIGHT 200
	ULTRALIGHT Self::EXTRALIGHT.0
	LIGHT 300
	NORMAL 400
	REGULAR 400
	MEDIUM 500
	SEMIBOLD 600
	DEMIBOLD Self::SEMIBOLD.0
	BOLD 700
	EXTRABOLD 800
	ULTRABOLD Self::EXTRABOLD.0
	HEAVY 900
	BLACK Self::HEAVY.0
}
