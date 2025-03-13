#![allow(non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::oleaut::vts::*;

#[repr(C)]
pub struct IActionVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Id: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Id: fn(COMPTR, PCSTR) -> HRES,
	pub get_Type: fn(COMPTR, *mut u32) -> HRES,
}

#[repr(C)]
pub struct IActionCollectionVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Count: fn(COMPTR, *mut i32) -> HRES,
	pub get_Item: fn(COMPTR, i32, *mut COMPTR) -> HRES,
	pub get__NewEnum: fn(COMPTR, *mut COMPTR) -> HRES,
	pub get_XmlText: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_XmlText: fn(COMPTR, PCSTR) -> HRES,
	pub Create: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub Remove: fn(COMPTR, VARIANT) -> HRES,
	pub Clear: fn(COMPTR) -> HRES,
	pub get_Context: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Context: fn(COMPTR, PCSTR) -> HRES,
}

#[repr(C)]
pub struct IBootTriggerVT {
	pub ITriggerVT: ITriggerVT,
	pub get_Delay: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Delay: fn(COMPTR, PCSTR) -> HRES,
}

#[repr(C)]
pub struct IComHandlerActionVT {
	pub IAction: IActionVT,
	pub get_ClassId: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_ClassId: fn(COMPTR, PCSTR) -> HRES,
	pub get_Data: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Data: fn(COMPTR, PCSTR) -> HRES,
}

#[repr(C)]
pub struct IDailyTriggerVT {
	pub ITriggerVT: ITriggerVT,
	pub get_DaysInterval: fn(COMPTR, *mut i16) -> HRES,
	pub put_DaysInterval: fn(COMPTR, i16) -> HRES,
	pub get_RandomDelay: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_RandomDelay: fn(COMPTR, PCSTR) -> HRES,
}

#[repr(C)]
pub struct IEmailActionVT {
	pub IAction: IActionVT,
	pub get_Server: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Server: fn(COMPTR, PCSTR) -> HRES,
	pub get_Subject: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Subject: fn(COMPTR, PCSTR) -> HRES,
	pub get_To: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_To: fn(COMPTR, PCSTR) -> HRES,
	pub get_Cc: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Cc: fn(COMPTR, PCSTR) -> HRES,
	pub get_Bcc: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Bcc: fn(COMPTR, PCSTR) -> HRES,
	pub get_ReplyTo: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_ReplyTo: fn(COMPTR, PCSTR) -> HRES,
	pub get_From: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_From: fn(COMPTR, PCSTR) -> HRES,
	pub get_HeaderFields: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_HeaderFields: fn(COMPTR, COMPTR) -> HRES,
	pub get_Body: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Body: fn(COMPTR, PCSTR) -> HRES,
	pub get_Attachments: fn(COMPTR, PVOID) -> HRES,
	pub put_Attachments: fn(COMPTR, PCVOID) -> HRES,
}

#[repr(C)]
pub struct IEventTriggerVT {
	pub ITriggerVT: ITriggerVT,
	pub get_Subscription: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Subscription: fn(COMPTR, PCSTR) -> HRES,
	pub get_Delay: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Delay: fn(COMPTR, PCSTR) -> HRES,
	pub get_ValueQueries: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_ValueQueries: fn(COMPTR, COMPTR) -> HRES,
}

#[repr(C)]
pub struct IExecActionVT {
	pub IAction: IActionVT,
	pub get_Path: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Path: fn(COMPTR, PCSTR) -> HRES,
	pub get_Arguments: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Arguments: fn(COMPTR, PCSTR) -> HRES,
	pub get_WorkingDirectory: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_WorkingDirectory: fn(COMPTR, PCSTR) -> HRES,
}

#[repr(C)]
pub struct ILogonTriggerVT {
	pub ITriggerVT: ITriggerVT,
	pub get_Delay: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Delay: fn(COMPTR, PCSTR) -> HRES,
	pub get_UserId: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_UserId: fn(COMPTR, PCSTR) -> HRES,
}

#[repr(C)]
pub struct IRegisteredTaskVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Name: fn(COMPTR, *mut PSTR) -> HRES,
	pub get_Path: fn(COMPTR, *mut PSTR) -> HRES,
	pub get_State: fn(COMPTR, *mut u32) -> HRES,
	pub get_Enabled: fn(COMPTR, *mut i16) -> HRES,
	pub put_Enabled: fn(COMPTR, i16) -> HRES,
	pub Run: fn(COMPTR, VARIANT, *mut COMPTR) -> HRES,
	pub RunEx: fn(COMPTR, VARIANT, i32, i32, PCSTR, *mut COMPTR) -> HRES,
	pub GetInstances: fn(COMPTR, i32, *mut COMPTR) -> HRES,
	pub get_LastRunTime: fn(COMPTR, *mut f64) -> HRES,
	pub get_LastTaskResult: fn(COMPTR, *mut i32) -> HRES,
	pub get_NumberOfMissedRuns: fn(COMPTR, *mut i32) -> HRES,
	pub get_NextRunTime: fn(COMPTR, *mut f64) -> HRES,
	pub get_Definition: fn(COMPTR, *mut COMPTR) -> HRES,
	pub get_Xml: fn(COMPTR, *mut PSTR) -> HRES,
	pub GetSecurityDescriptor: fn(COMPTR, i32, *mut PSTR) -> HRES,
	pub SetSecurityDescriptor: fn(COMPTR, PCSTR, i32) -> HRES,
	pub Stop: fn(COMPTR, i32) -> HRES,
	pub GetRunTimes: fn(COMPTR, PCVOID, PCVOID, *mut u32, PVOID) -> HRES,
}

#[repr(C)]
pub struct IRegistrationInfoVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Description: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Description: fn(COMPTR, PSTR) -> HRES,
	pub get_Author: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Author: fn(COMPTR, PSTR) -> HRES,
	pub get_Version: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Version: fn(COMPTR, PSTR) -> HRES,
	pub get_Date: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Date: fn(COMPTR, PSTR) -> HRES,
	pub get_Documentation: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Documentation: fn(COMPTR, PSTR) -> HRES,
	pub get_XmlText: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_XmlText: fn(COMPTR, PSTR) -> HRES,
	pub get_URI: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_URI: fn(COMPTR, PSTR) -> HRES,
	pub get_SecurityDescriptor: fn(COMPTR, *mut VARIANT) -> HRES,
	pub put_SecurityDescriptor: fn(COMPTR, VARIANT) -> HRES,
	pub get_Source: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Source: fn(COMPTR, PSTR) -> HRES,
}

#[repr(C)]
pub struct ITaskDefinitionVT {
	pub IDispatchVT: IDispatchVT,
	pub get_RegistrationInfo: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_RegistrationInfo: fn(COMPTR, COMPTR) -> HRES,
	pub get_Triggers: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_Triggers: fn(COMPTR, COMPTR) -> HRES,
	pub get_Settings: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_Settings: fn(COMPTR, COMPTR) -> HRES,
	pub get_Data: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Data: fn(COMPTR, PCSTR) -> HRES,
	pub get_Principal: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_Principal: fn(COMPTR, COMPTR) -> HRES,
	pub get_Actions: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_Actions: fn(COMPTR, COMPTR) -> HRES,
	pub get_XmlText: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_XmlText: fn(COMPTR, PCSTR) -> HRES,
}

#[repr(C)]
pub struct ITaskFolderVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Name: fn(COMPTR, *mut PSTR) -> HRES,
	pub get_Path: fn(COMPTR, *mut PSTR) -> HRES,
	pub GetFolder: fn(COMPTR, PCSTR, *mut COMPTR) -> HRES,
	pub GetFolders: fn(COMPTR, i32, *mut COMPTR) -> HRES,
	pub CreateFolder: fn(COMPTR, PCSTR, VARIANT, *mut COMPTR) -> HRES,
	pub DeleteFolder: fn(COMPTR, PCSTR, i32) -> HRES,
	pub GetTask: fn(COMPTR, PCSTR, *mut COMPTR) -> HRES,
	pub GetTasks: fn(COMPTR, i32, *mut COMPTR) -> HRES,
	pub DeleteTask: fn(COMPTR, PCSTR, i32) -> HRES,
	pub RegisterTask:
		fn(COMPTR, PCSTR, PCSTR, i32, VARIANT, VARIANT, u32, VARIANT, *mut COMPTR) -> HRES,
	pub RegisterTaskDefinition:
		fn(COMPTR, PCSTR, COMPTR, i32, VARIANT, VARIANT, u32, VARIANT, *mut COMPTR) -> HRES,
	pub GetSecurityDescriptor: fn(COMPTR, i32, *mut PSTR) -> HRES,
	pub SetSecurityDescriptor: fn(COMPTR, PCSTR, i32) -> HRES,
}

#[repr(C)]
pub struct ITaskServiceVT {
	pub IDispatchVT: IDispatchVT,
	pub GetFolder: fn(COMPTR, PCSTR, *mut COMPTR) -> HRES,
	pub GetRunningTasks: fn(COMPTR, i32, *mut COMPTR) -> HRES,
	pub NewTask: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub Connect: fn(COMPTR, VARIANT, VARIANT, VARIANT, VARIANT) -> HRES,
	pub get_Connected: fn(COMPTR, *mut i16) -> HRES,
	pub get_TargetServer: fn(COMPTR, *mut PSTR) -> HRES,
	pub get_ConnectedUser: fn(COMPTR, *mut PSTR) -> HRES,
	pub get_ConnectedDomain: fn(COMPTR, *mut PSTR) -> HRES,
	pub get_HighestVersion: fn(COMPTR, *mut u32) -> HRES,
}

#[repr(C)]
pub struct ITriggerVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Type: fn(COMPTR, *mut u32) -> HRES,
	pub get_Id: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_Id: fn(COMPTR, PCSTR) -> HRES,
	pub get_Repetition: fn(COMPTR, *mut COMPTR) -> HRES,
	pub put_Repetition: fn(COMPTR, COMPTR) -> HRES,
	pub get_ExecutionTimeLimit: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_ExecutionTimeLimit: fn(COMPTR, PCSTR) -> HRES,
	pub get_StartBoundary: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_StartBoundary: fn(COMPTR, PCSTR) -> HRES,
	pub get_EndBoundary: fn(COMPTR, *mut PSTR) -> HRES,
	pub put_EndBoundary: fn(COMPTR, PCSTR) -> HRES,
	pub get_Enabled: fn(COMPTR, *mut i16) -> HRES,
	pub put_Enabled: fn(COMPTR, i16) -> HRES,
}

#[repr(C)]
pub struct ITriggerCollectionVT {
	pub IDispatchVT: IDispatchVT,
	pub get_Count: fn(COMPTR, *mut i32) -> HRES,
	pub get_Item: fn(COMPTR, i32, *mut COMPTR) -> HRES,
	pub get__NewEnum: fn(COMPTR, *mut COMPTR) -> HRES,
	pub Create: fn(COMPTR, u32, *mut COMPTR) -> HRES,
	pub Remove: fn(COMPTR, VARIANT) -> HRES,
	pub Clear: fn(COMPTR) -> HRES,
}
