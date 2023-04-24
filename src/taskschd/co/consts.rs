#![allow(non_camel_case_types)]

const_bitflag! { TASK_CREATION: u32;
	/// [`TASK_CREATION`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/ne-taskschd-task_creation)
	/// enumeration (`u32`).
	/// 
	/// Originally has `TASK` prefix.
	=>
	=>
	VALIDATE_ONLY 0x1
	CREATE 0x2
	UPDATE 0x4
	CREATE_OR_UPDATE Self::CREATE.0 | Self::UPDATE.0
	DISABL 0x8
	DONT_ADD_PRINCIPAL_ACE 0x10
	IGNORE_REGISTRATION_TRIGGERS 0x20
}

const_bitflag! { TASK_LOGON: u32;
	/// [`TASK_LOGON_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/taskschd/ne-taskschd-task_logon_type)
	/// enumeration (`u32`).
	=>
	=>
	NONE 0
	PASSWORD 1
	S4U 2
	INTERACTIVE_TOKEN 3
	GROUP 4
	SERVICE_ACCOUNT 5
	INTERACTIVE_TOKEN_OR_PASSWORD 6
}
