#![allow(non_snake_case)]

use crate::decl::*;
use crate::kernel::ffi_types::*;
use crate::macros::*;
use crate::oleaut::vts::*;

com_vtbl! { IActionVT : IDispatchVT
	get_Id(*mut PSTR) -> HRES
	put_Id(PCSTR) -> HRES
	get_Type(*mut u32) -> HRES
}

com_vtbl! { IActionCollectionVT : IDispatchVT
	get_Count(*mut i32) -> HRES
	get_Item(i32, *mut COMPTR) -> HRES
	get__NewEnum(*mut COMPTR) -> HRES
	get_XmlText(*mut PSTR) -> HRES
	put_XmlText(PCSTR) -> HRES
	Create(u32, *mut COMPTR) -> HRES
	Remove(VARIANT) -> HRES
	Clear() -> HRES
	get_Context(*mut PSTR) -> HRES
	put_Context(PCSTR) -> HRES
}

com_vtbl! { IBootTriggerVT: ITriggerVT
	get_Delay(*mut PSTR) -> HRES
	put_Delay(PCSTR) -> HRES
}

com_vtbl! { IComHandlerActionVT : IActionVT
	get_ClassId(*mut PSTR) -> HRES
	put_ClassId(PCSTR) -> HRES
	get_Data(*mut PSTR) -> HRES
	put_Data(PCSTR) -> HRES
}

com_vtbl! { IDailyTriggerVT : ITriggerVT
	get_DaysInterval(*mut i16) -> HRES
	put_DaysInterval(i16) -> HRES
	get_RandomDelay(*mut PSTR) -> HRES
	put_RandomDelay(PCSTR) -> HRES
}

com_vtbl! { IEmailActionVT : IActionVT
	get_Server(*mut PSTR) -> HRES
	put_Server(PCSTR) -> HRES
	get_Subject(*mut PSTR) -> HRES
	put_Subject(PCSTR) -> HRES
	get_To(*mut PSTR) -> HRES
	put_To(PCSTR) -> HRES
	get_Cc(*mut PSTR) -> HRES
	put_Cc(PCSTR) -> HRES
	get_Bcc(*mut PSTR) -> HRES
	put_Bcc(PCSTR) -> HRES
	get_ReplyTo(*mut PSTR) -> HRES
	put_ReplyTo(PCSTR) -> HRES
	get_From(*mut PSTR) -> HRES
	put_From(PCSTR) -> HRES
	get_HeaderFields(*mut COMPTR) -> HRES
	put_HeaderFields(COMPTR) -> HRES
	get_Body(*mut PSTR) -> HRES
	put_Body(PCSTR) -> HRES
	get_Attachments(PVOID) -> HRES
	put_Attachments(PCVOID) -> HRES
}

com_vtbl! { IEventTriggerVT : ITriggerVT
	get_Subscription(*mut PSTR) -> HRES
	put_Subscription(PCSTR) -> HRES
	get_Delay(*mut PSTR) -> HRES
	put_Delay(PCSTR) -> HRES
	get_ValueQueries(*mut COMPTR) -> HRES
	put_ValueQueries(COMPTR) -> HRES
}

com_vtbl! { IExecActionVT : IActionVT
	get_Path(*mut PSTR) -> HRES
	put_Path(PCSTR) -> HRES
	get_Arguments(*mut PSTR) -> HRES
	put_Arguments(PCSTR) -> HRES
	get_WorkingDirectory(*mut PSTR) -> HRES
	put_WorkingDirectory(PCSTR) -> HRES
}

com_vtbl! { ILogonTriggerVT : ITriggerVT
	get_Delay(*mut PSTR) -> HRES
	put_Delay(PCSTR) -> HRES
	get_UserId(*mut PSTR) -> HRES
	put_UserId(PCSTR) -> HRES
}

com_vtbl! { IPrincipalVT : IDispatchVT
	get_Id(*mut PSTR) -> HRES
	put_Id(PCSTR) -> HRES
	get_DisplayName(*mut PSTR) -> HRES
	put_DisplayName(PCSTR) -> HRES
	get_UserId(*mut PSTR) -> HRES
	put_UserId(PCSTR) -> HRES
	get_LogonType(*mut u32) -> HRES
	put_LogonType(u32) -> HRES
	get_GroupId(*mut PSTR) -> HRES
	put_GroupId(PCSTR) -> HRES
	get_RunLevel(*mut u32) -> HRES
	put_RunLevel(u32) -> HRES
}

com_vtbl! { IRegisteredTaskVT : IDispatchVT
	get_Name(*mut PSTR) -> HRES
	get_Path(*mut PSTR) -> HRES
	get_State(*mut u32) -> HRES
	get_Enabled(*mut i16) -> HRES
	put_Enabled(i16) -> HRES
	Run(VARIANT, *mut COMPTR) -> HRES
	RunEx(VARIANT, i32, i32, PCSTR, *mut COMPTR) -> HRES
	GetInstances(i32, *mut COMPTR) -> HRES
	get_LastRunTime(*mut f64) -> HRES
	get_LastTaskResult(*mut i32) -> HRES
	get_NumberOfMissedRuns(*mut i32) -> HRES
	get_NextRunTime(*mut f64) -> HRES
	get_Definition(*mut COMPTR) -> HRES
	get_Xml(*mut PSTR) -> HRES
	GetSecurityDescriptor(i32, *mut PSTR) -> HRES
	SetSecurityDescriptor(PCSTR, i32) -> HRES
	Stop(i32) -> HRES
	GetRunTimes(PCVOID, PCVOID, *mut u32, PVOID) -> HRES
}

com_vtbl! { IRegistrationInfoVT : IDispatchVT
	get_Description(*mut PSTR) -> HRES
	put_Description(PSTR) -> HRES
	get_Author(*mut PSTR) -> HRES
	put_Author(PSTR) -> HRES
	get_Version(*mut PSTR) -> HRES
	put_Version(PSTR) -> HRES
	get_Date(*mut PSTR) -> HRES
	put_Date(PSTR) -> HRES
	get_Documentation(*mut PSTR) -> HRES
	put_Documentation(PSTR) -> HRES
	get_XmlText(*mut PSTR) -> HRES
	put_XmlText(PSTR) -> HRES
	get_URI(*mut PSTR) -> HRES
	put_URI(PSTR) -> HRES
	get_SecurityDescriptor(*mut VARIANT) -> HRES
	put_SecurityDescriptor(VARIANT) -> HRES
	get_Source(*mut PSTR) -> HRES
	put_Source(PSTR) -> HRES
}

com_vtbl! { ITaskDefinitionVT : IDispatchVT
	get_RegistrationInfo(*mut COMPTR) -> HRES
	put_RegistrationInfo(COMPTR) -> HRES
	get_Triggers(*mut COMPTR) -> HRES
	put_Triggers(COMPTR) -> HRES
	get_Settings(*mut COMPTR) -> HRES
	put_Settings(COMPTR) -> HRES
	get_Data(*mut PSTR) -> HRES
	put_Data(PCSTR) -> HRES
	get_Principal(*mut COMPTR) -> HRES
	put_Principal(COMPTR) -> HRES
	get_Actions(*mut COMPTR) -> HRES
	put_Actions(COMPTR) -> HRES
	get_XmlText(*mut PSTR) -> HRES
	put_XmlText(PCSTR) -> HRES
}

com_vtbl! { ITaskFolderVT : IDispatchVT
	get_Name(*mut PSTR) -> HRES
	get_Path(*mut PSTR) -> HRES
	GetFolder(PCSTR, *mut COMPTR) -> HRES
	GetFolders(i32, *mut COMPTR) -> HRES
	CreateFolder(PCSTR, VARIANT, *mut COMPTR) -> HRES
	DeleteFolder(PCSTR, i32) -> HRES
	GetTask(PCSTR, *mut COMPTR) -> HRES
	GetTasks(i32, *mut COMPTR) -> HRES
	DeleteTask(PCSTR, i32) -> HRES
	RegisterTask(PCSTR, PCSTR, i32, VARIANT, VARIANT, u32, VARIANT, *mut COMPTR) -> HRES
	RegisterTaskDefinition(PCSTR, COMPTR, i32, VARIANT, VARIANT, u32, VARIANT, *mut COMPTR) -> HRES
	GetSecurityDescriptor(i32, *mut PSTR) -> HRES
	SetSecurityDescriptor(PCSTR, i32) -> HRES
}

com_vtbl! { ITaskServiceVT : IDispatchVT
	GetFolder(PCSTR, *mut COMPTR) -> HRES
	GetRunningTasks(i32, *mut COMPTR) -> HRES
	NewTask(u32, *mut COMPTR) -> HRES
	Connect(VARIANT, VARIANT, VARIANT, VARIANT) -> HRES
	get_Connected(*mut i16) -> HRES
	get_TargetServer(*mut PSTR) -> HRES
	get_ConnectedUser(*mut PSTR) -> HRES
	get_ConnectedDomain(*mut PSTR) -> HRES
	get_HighestVersion(*mut u32) -> HRES
}

com_vtbl! { ITaskSettingsVT : IDispatchVT
	get_AllowDemandStart(*mut i16) -> HRES
	put_AllowDemandStart(i16) -> HRES
	get_RestartInterval(*mut PSTR) -> HRES
	put_RestartInterval(PSTR) -> HRES
	get_RestartCount(*mut i32) -> HRES
	put_RestartCount(i32) -> HRES
	get_MultipleInstances(*mut u32) -> HRES
	put_MultipleInstances(u32) -> HRES
	get_StopIfGoingOnBatteries(*mut i16) -> HRES
	put_StopIfGoingOnBatteries(i16) -> HRES
	get_DisallowStartIfOnBatteries(*mut i16) -> HRES
	put_DisallowStartIfOnBatteries(i16) -> HRES
	get_AllowHardTerminate(*mut i16) -> HRES
	put_AllowHardTerminate(i16) -> HRES
	get_StartWhenAvailable(*mut i16) -> HRES
	put_StartWhenAvailable(i16) -> HRES
	get_XmlText(*mut PSTR) -> HRES
	put_XmlText(PSTR) -> HRES
	get_RunOnlyIfNetworkAvailable(*mut i16) -> HRES
	put_RunOnlyIfNetworkAvailable(i16) -> HRES
	get_ExecutionTimeLimit(*mut PSTR) -> HRES
	put_ExecutionTimeLimit(PSTR) -> HRES
	get_Enabled(*mut i16) -> HRES
	put_Enabled(i16) -> HRES
	get_DeleteExpiredTaskAfter(*mut PSTR) -> HRES
	put_DeleteExpiredTaskAfter(PSTR) -> HRES
	get_Priority(*mut i32) -> HRES
	put_Priority(i32) -> HRES
	get_Compatibility(*mut u32) -> HRES
	put_Compatibility(u32) -> HRES
	get_Hidden(*mut i16) -> HRES
	put_Hidden(i16) -> HRES
	get_IdleSettings(*mut COMPTR) -> HRES
	put_IdleSettings(COMPTR) -> HRES
	get_RunOnlyIfIdle(*mut i16) -> HRES
	put_RunOnlyIfIdle(i16) -> HRES
	get_WakeToRun(*mut i16) -> HRES
	put_WakeToRun(i16) -> HRES
	get_NetworkSettings(*mut COMPTR) -> HRES
	put_NetworkSettings(COMPTR) -> HRES
}

com_vtbl! { ITriggerVT : IDispatchVT
	get_Type(*mut u32) -> HRES
	get_Id(*mut PSTR) -> HRES
	put_Id(PCSTR) -> HRES
	get_Repetition(*mut COMPTR) -> HRES
	put_Repetition(COMPTR) -> HRES
	get_ExecutionTimeLimit(*mut PSTR) -> HRES
	put_ExecutionTimeLimit(PCSTR) -> HRES
	get_StartBoundary(*mut PSTR) -> HRES
	put_StartBoundary(PCSTR) -> HRES
	get_EndBoundary(*mut PSTR) -> HRES
	put_EndBoundary(PCSTR) -> HRES
	get_Enabled(*mut i16) -> HRES
	put_Enabled(i16) -> HRES
}

com_vtbl! { ITriggerCollectionVT : IDispatchVT
	get_Count(*mut i32) -> HRES
	get_Item(i32, *mut COMPTR) -> HRES
	get__NewEnum(*mut COMPTR) -> HRES
	Create(u32, *mut COMPTR) -> HRES
	Remove(VARIANT) -> HRES
	Clear() -> HRES
}
