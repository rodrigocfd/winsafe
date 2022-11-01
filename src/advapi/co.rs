#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::co::{ACCESS_RIGHTS, STANDARD_RIGHTS};

const_bitflag! { KEY: u32: "advapi";
	/// [`HKEY::RegOpenKeyEx`](crate::prelude::advapi_Hkey::RegOpenKeyEx)
	/// `access_rights` (`u32`).
	=>
	=>
	QUERY_VALUE 0x0001
	SET_VALUE 0x0002
	CREATE_SUB_KEY 0x0004
	ENUMERATE_SUB_KEYS 0x0008
	NOTIFY 0x0010
	CREATE_LINK 0x0020
	WOW64_32KEY 0x0200
	WOW64_64KEY 0x0100
	WOW64_RES 0x0300
	READ (STANDARD_RIGHTS::READ.0 | Self::QUERY_VALUE.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
	WRITE (STANDARD_RIGHTS::WRITE.0 | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
	EXECUTE Self::READ.0 & !ACCESS_RIGHTS::SYNCHRONIZE.0
	ALL_ACCESS (STANDARD_RIGHTS::ALL.0 | Self::QUERY_VALUE.0 | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0 | Self::CREATE_LINK.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
}

const_ordinary! { REG: u32: "advapi";
	/// Registry
	/// [value types](https://learn.microsoft.com/en-us/windows/win32/sysinfo/registry-value-types)
	/// (`u32`).
	=>
	=>
	NONE 0
	SZ 1
	EXPAND_SZ 2
	BINARY 3
	DWORD 4
	DWORD_LITTLE_ENDIAN 4
	DWORD_BIG_ENDIAN 5
	LINK 6
	MULTI_SZ 7
	RESOURCE_LIST 8
	FULL_RESOURCE_DESCRIPTOR 9
	RESOURCE_REQUIREMENTS_LIST 10
	QWORD 11
	QWORD_LITTLE_ENDIAN 11
}

const_bitflag! { REG_OPTION: u32: "advapi";
	/// [`HKEY::RegOpenKeyEx`](crate::prelude::advapi_Hkey::RegOpenKeyEx)
	/// `options` (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	RESERVED 0x0000_0000
	NON_VOLATILE 0x0000_0000
	VOLATILE 0x0000_0001
	CREATE_LINK 0x0000_0002
	BACKUP_RESTORE 0x0000_0004
	OPEN_LINK 0x0000_0008
}

const_bitflag! { RRF: u32: "advapi";
	/// [`HKEY::GetValue`](crate::prelude::advapi_Hkey::GetValue) `dwFlags`
	/// (`u32`).
	=>
	=>
	RT_REG_NONE 0x0000_0001
	RT_REG_SZ 0x0000_0002
	RT_REG_EXPAND_SZ 0x0000_0004
	RT_REG_BINARY 0x0000_0008
	RT_REG_DWORD 0x0000_0010
	RT_REG_MULTI_SZ 0x0000_0020
	RT_REG_QWORD 0x0000_0040
	RT_DWORD Self::RT_REG_BINARY.0 | Self::RT_REG_DWORD.0
	RT_QWORD Self::RT_REG_BINARY.0 | Self::RT_REG_QWORD.0
	RT_ANY 0x0000_ffff

	SUBKEY_WOW6464KEY 0x0001_0000
	SUBKEY_WOW6432KEY 0x0002_0000
	WOW64_MASK 0x0003_0000

	NOEXPAND 0x1000_0000
	ZEROONFAILURE 0x2000_0000
}
