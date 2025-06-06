#![allow(non_camel_case_types)]

use crate::co::*;

const_bitflag! { FORM: u32;
	/// [`FORM_INFO_1`](crate::FORM_INFO_1) and
	/// [`FORM_INFO_2`](crate::FORM_INFO_2) `Flags` (`u32`).
	=>
	USER 0x0000_0000
	BUILTIN 0x0000_0001
	PRINTER 0x0000_0002
}

const_bitflag! { PRINTER_ACCESS: u32;
	/// [`PRINTER_DEFAULTS`](crate::PRINTER_DEFAULTS) `DesiredAccess` (`u32`).
	=>
	ADMINISTER 0x0000_0004
	USE 0x0000_0008
	MANAGE_LIMITED 0x0000_0040
	ALL_ACCESS (STANDARD_RIGHTS::REQUIRED.raw() | Self::ADMINISTER.0 | Self::USE.0)
	READ (STANDARD_RIGHTS::READ.raw() | Self::USE.0)
	WRITE (STANDARD_RIGHTS::WRITE.raw() | Self::USE.0)
	EXECUTE (STANDARD_RIGHTS::EXECUTE.raw() | Self::USE.0)
}

const_bitflag! { PRINTER_ATTRIBUTE_2: u32;
	/// [`PRINTER_INFO_2`](crate::PRINTER_INFO_2) `Attributes` (`u32`).
	=>
	DIRECT 0x0000_0002
	DO_COMPLETE_FIRST 0x0000_0200
	ENABLE_DEVQ 0x0000_0080
	HIDDEN 0x0000_0020
	KEEPPRINTEDJOBS 0x0000_0100
	LOCAL 0x0000_0040
	NETWORK 0x0000_0010
	PUBLISHED 0x0000_2000
	QUEUED 0x0000_0001
	RAW_ONLY 0x0000_1000
	SHARED 0x0000_0008
	FAX 0x0000_4000
	FRIENDLY_NAME 0x0010_0000
	MACHINE 0x0008_0000
	PUSHED_USER 0x0002_0000
	PUSHED_MACHINE 0x0004_0000
	TS 0x0000_8000
}

const_ordinary! { PRINTER_ATTRIBUTE_4: u32;
	/// [`PRINTER_INFO_4`](crate::PRINTER_INFO_4) `Attributes` (`u32`).
	=>
	LOCAL PRINTER_ATTRIBUTE_2::LOCAL.0
	NETWORK PRINTER_ATTRIBUTE_2::NETWORK.0
}

const_bitflag! { PRINTER_CONNECTION: u32;
	/// [`PRINTER_CONNECTION_INFO_1`](crate::PRINTER_CONNECTION_INFO_1)
	/// `dwFlags` (`u32`).
	=>
	MISMATCH 0x0000_0020
	NO_UI 0x0000_0040
}

const_bitflag! { PRINTER_ENUM: u32;
	/// [`EnumPrinters2`](crate::EnumPrinters2) and
	/// [`EnumPrinters4`](crate::EnumPrinters4) `flags` (`u32`).
	=>
	LOCAL 0x0000_0002
	NAME 0x0000_0008
	SHARED 0x0000_0020
	CONNECTIONS 0x0000_0004
	NETWORK 0x0000_0040
	REMOTE 0x0000_0010
	CATEGORY_3D 0x0400_0000
	CATEGORY_ALL 0x0200_0000
}

const_bitflag! { PRINTER_OPTION: u32;
	/// [`PRINTER_OPTION`](https://learn.microsoft.com/en-us/windows/win32/printdocs/printer-option-flags)
	/// enumeration (`u32`).
	=>
	NO_CACHE 1
	CACHE 2
	CLIENT_CHANGE 4
	NO_CLIENT_DATA 8
}

const_bitflag! { PRINTER_STATUS: u32;
	/// [`PRINTER_INFO_2`](crate::PRINTER_INFO_2) `status` (`u32`).
	=>
	PAUSED 0x0000_0001
	ERROR 0x0000_0002
	PENDING_DELETION 0x0000_0004
	PAPER_JAM 0x0000_0008
	PAPER_OUT 0x0000_0010
	MANUAL_FEED 0x0000_0020
	PAPER_PROBLEM 0x0000_0040
	OFFLINE 0x0000_0080
	IO_ACTIVE 0x0000_0100
	BUSY 0x0000_0200
	PRINTING 0x0000_0400
	OUTPUT_BIN_FULL 0x0000_0800
	NOT_AVAILABLE 0x0000_1000
	WAITING 0x0000_2000
	PROCESSING 0x0000_4000
	INITIALIZING 0x0000_8000
	WARMING_UP 0x0001_0000
	TONER_LOW 0x0002_0000
	NO_TONER 0x0004_0000
	PAGE_PUNT 0x0008_0000
	USER_INTERVENTION 0x0010_0000
	OUT_OF_MEMORY 0x0020_0000
	DOOR_OPEN 0x0040_0000
	SERVER_UNKNOWN 0x0080_0000
	POWER_SAVE 0x0100_0000
	SERVER_OFFLINE 0x0200_0000
	DRIVER_UPDATE_NEEDED 0x0400_0000
}

const_bitflag! { STRING_FORM: u32;
	/// [`FORM_INFO_2`](crate::FORM_INFO_2) `StringType` (`u32`).
	///
	/// Originally has `STRING` prefix.
	=>
	NONE 0x0000_0001
	MUIDLL 0x0000_0002
	LANGPAIR 0x0000_0004
}
