#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::co::*;

const_ordinary! { EVENTLOG: u16;
	/// [`HEVENTLOG::ReportEvent`](crate::prelude::advapi_Heventlog::ReportEvent)
	/// `event_type` [`u16`].
	=>
	=>
	SUCCESS 0x0000
	AUDIT_FAILURE 0x0010
	AUDIT_SUCCESS 0x0008
	ERROR_TYPE 0x0001
	INFORMATION_TYPE 0x0004
	WARNING_TYPE 0x0002
}

const_bitflag! { KEY: u32;
	/// [Registry access rights](https://learn.microsoft.com/en-us/windows/win32/sysinfo/registry-key-security-and-access-rights)
	/// (`u32`).
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
	READ (STANDARD_RIGHTS::READ.raw() | Self::QUERY_VALUE.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0) & !ACCESS_RIGHTS::SYNCHRONIZE.raw()
	WRITE (STANDARD_RIGHTS::WRITE.raw() | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0) & !ACCESS_RIGHTS::SYNCHRONIZE.raw()
	EXECUTE Self::READ.0 & !ACCESS_RIGHTS::SYNCHRONIZE.raw()
	ALL_ACCESS (STANDARD_RIGHTS::ALL.raw() | Self::QUERY_VALUE.0 | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0 | Self::CREATE_LINK.0) & !ACCESS_RIGHTS::SYNCHRONIZE.raw()
}

const_ordinary! { REG: u32;
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

const_ordinary! { REG_DISPOSITION: u32;
	/// [`HKEY::RegCreateKeyEx`](crate::prelude::advapi_Hkey::RegCreateKeyEx)
	/// creation disposition (`u32`).
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	/// The key did not exist and was created.
	CREATED_NEW_KEY 0x0000_0001
	/// The key existed and was simply opened without being changed.
	OPENED_EXISTING_KEY 0x0000_0002
}

const_bitflag! { REG_OPTION: u32;
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

const_ordinary! { REG_RESTORE: u32;
	/// Registry restore
	/// [`flags`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regrestorekeyw)
	/// (`u32`).
	///
	/// Originally has `REG` prefix.
	=>
	=>
	FORCE_RESTORE 0x0000_0008
	WHOLE_HIVE_VOLATILE 0x0000_0001
}

const_ordinary! { REG_SAVE: u32;
	/// Registry save
	/// [`flags`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsavekeyexw)
	/// (`u32`).
	///
	/// Originally has `REG` prefix.
	=>
	=>
	STANDARD_FORMAT 1
	LATEST_FORMAT 2
	NO_COMPRESSION 4
}

const_ordinary! { RID: u32;
	/// The
	/// [portion](https://learn.microsoft.com/en-us/windows/win32/secgloss/r-gly)
	/// of a [`SID`](crate::SID) that identifies a user or group in relation to
	/// the authority that issued the `SID`.
	///
	/// Originally has `RID` suffix.
	=>
	=>
	SECURITY_NULL 0x0000_0000
	SECURITY_WORLD 0x0000_0000
	SECURITY_LOCAL 0x0000_0000
	SECURITY_LOCAL_LOGON 0x0000_0001
	SECURITY_CREATOR_OWNER 0x0000_0000
	SECURITY_CREATOR_GROUP 0x0000_0001
	SECURITY_CREATOR_OWNER_SERVER 0x0000_0002
	SECURITY_CREATOR_GROUP_SERVER 0x0000_0003
	SECURITY_CREATOR_OWNER_RIGHTS 0x0000_0004

	SECURITY_DIALUP 0x0000_0001
	SECURITY_NETWORK 0x0000_0002
	SECURITY_BATCH 0x0000_0003
	SECURITY_INTERACTIVE 0x0000_0004
	SECURITY_LOGON_IDS 0x0000_0005
	SECURITY_SERVICE 0x0000_0006
	SECURITY_ANONYMOUS_LOGON 0x0000_0007
	SECURITY_PROXY 0x0000_0008
	SECURITY_ENTERPRISE_CONTROLLERS 0x00000_009
	SECURITY_SERVER_LOGON Self::SECURITY_ENTERPRISE_CONTROLLERS.0
	SECURITY_PRINCIPAL_SELF 0x0000_000a
	SECURITY_AUTHENTICATED_USER 0x0000_000b
	SECURITY_RESTRICTED_CODE 0x0000_000c
	SECURITY_TERMINAL_SERVER 0x0000_000d
	SECURITY_REMOTE_LOGON 0x0000_000e
	SECURITY_THIS_ORGANIZATION 0x0000_000f
	SECURITY_IUSER 0x0000_0011
	SECURITY_LOCAL_SYSTEM 0x0000_0012
	SECURITY_LOCAL_SERVICE 0x0000_0013
	SECURITY_NETWORK_SERVICE 0x0000_0014
	SECURITY_NT_NON_UNIQUE 0x0000_0015
	SECURITY_ENTERPRISE_READONLY_CONTROLLERS 0x0000_0016
	SECURITY_BUILTIN_DOMAIN 0x0000_0020
	SECURITY_WRITE_RESTRICTED_CODE 0x0000_0021
	SECURITY_PACKAGE_BASE 0x0000_0040
	SECURITY_PACKAGE_NTLM 0x0000_000a
	SECURITY_PACKAGE_SCHANNEL 0x0000_000e
	SECURITY_PACKAGE_DIGEST 0x0000_0015
	SECURITY_CRED_TYPE_BASE 0x0000_0041
	SECURITY_CRED_TYPE_THIS_ORG_CERT 0x0000_0001
	SECURITY_MIN_BASE 0x0000_0050
	SECURITY_SERVICE_ID_BASE 0x0000_0050
	SECURITY_RESERVED_ID_BASE 0x0000_0051
	SECURITY_APPPOOL_ID_BASE 0x0000_0052
	SECURITY_VIRTUALSERVER_ID_BASE 0x0000_0053
	SECURITY_USERMODEDRIVERHOST_ID_BASE 0x0000_0054
	SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_BASE 0x0000_0055
	SECURITY_WMIHOST_ID_BASE 0x0000_0056
	SECURITY_TASK_ID_BASE 0x0000_0057
	SECURITY_NFS_ID_BASE 0x0000_0058
	SECURITY_COM_ID_BASE 0x0000_0059
	SECURITY_WINDOW_MANAGER_BASE 0x0000_005a
	SECURITY_RDV_GFX_BASE 0x0000_005b
	SECURITY_DASHOST_ID_BASE 0x0000_005c
	SECURITY_USERMANAGER_ID_BASE 0x0000_005d
	SECURITY_WINRM_ID_BASE 0x0000_005e
	SECURITY_CCG_ID_BASE 0x0000_005f
	SECURITY_UMFD_BASE 0x0000_0060
	SECURITY_MAX_BASE 0x0000_006f
	SECURITY_MAX_ALWAYS_FILTERED 0x0000_03e7
	SECURITY_MIN_NEVER_FILTERED 0x0000_03e8
	SECURITY_OTHER_ORGANIZATION 0x0000_03e8
	SECURITY_WINDOWSMOBILE_ID_BASE 0x0000_0070
	SECURITY_LOCAL_ACCOUNT 0x0000_0071
	SECURITY_LOCAL_ACCOUNT_AND_ADMIN 0x0000_0072

	DOMAIN_GROUP_AUTHORIZATION_DATA_IS_COMPOUNDED 0x0000_01f0
	DOMAIN_GROUP_AUTHORIZATION_DATA_CONTAINS_CLAIMS 0x0000_01f1
	DOMAIN_GROUP_ENTERPRISE_READONLY_DOMAIN_CONTROLLERS 0x0000_01f2
	DOMAIN_USER_ADMIn 0x0000_01f4
	DOMAIN_USER_GUEST 0x0000_01f5
	DOMAIN_USER_KRBTGT 0x0000_01f6
	DOMAIN_USER_DEFAULT_ACCOUNT 0x0000_01f7
	DOMAIN_USER_WDAG_ACCOUNT 0x0000_01f8
	DOMAIN_GROUP_ADMINS 0x0000_0200
	DOMAIN_GROUP_USERS 0x0000_0201
	DOMAIN_GROUP_GUESTS 0x0000_0202
	DOMAIN_GROUP_COMPUTERS 0x0000_0203
	DOMAIN_GROUP_CONTROLLERS 0x0000_0204
	DOMAIN_GROUP_CERT_ADMINS 0x0000_0205
	DOMAIN_GROUP_SCHEMA_ADMINS 0x0000_0206
	DOMAIN_GROUP_ENTERPRISE_ADMINS 0x0000_0207
	DOMAIN_GROUP_POLICY_ADMINS 0x0000_0208
	DOMAIN_GROUP_READONLY_CONTROLLERS 0x0000_0209
	DOMAIN_GROUP_CLONEABLE_CONTROLLERS 0x0000_020a
	DOMAIN_GROUP_CDC_RESERVED 0x0000_020c
	DOMAIN_GROUP_PROTECTED_USERS 0x0000_020d
	DOMAIN_GROUP_KEY_ADMINS 0x0000_020e
	DOMAIN_GROUP_ENTERPRISE_KEY_ADMINS 0x0000_020f
	DOMAIN_ALIAS_ADMINS 0x0000_0220
	DOMAIN_ALIAS_USERS 0x0000_0221
	DOMAIN_ALIAS_GUESTS 0x0000_0222
	DOMAIN_ALIAS_POWER_USERS 0x0000_0223
	DOMAIN_ALIAS_ACCOUNT_OPS 0x0000_0224
	DOMAIN_ALIAS_SYSTEM_OPS 0x0000_0225
	DOMAIN_ALIAS_PRINT_OPS 0x0000_0226
	DOMAIN_ALIAS_BACKUP_OPS 0x0000_0227
	DOMAIN_ALIAS_REPLICATOR 0x0000_0228
	DOMAIN_ALIAS_RAS_SERVERS 0x0000_0229
	DOMAIN_ALIAS_PREW2KCOMPACCESS 0x0000_022a
	DOMAIN_ALIAS_REMOTE_DESKTOP_USERS 0x0000_022b
	DOMAIN_ALIAS_NETWORK_CONFIGURATION_OPS 0x0000_022c
	DOMAIN_ALIAS_INCOMING_FOREST_TRUST_BUILDERS 0x0000_022d
	DOMAIN_ALIAS_MONITORING_USERS 0x0000_022e
	DOMAIN_ALIAS_LOGGING_USERS 0x0000_022f
	DOMAIN_ALIAS_AUTHORIZATIONACCESS 0x0000_0230
	DOMAIN_ALIAS_TS_LICENSE_SERVERS 0x0000_0231
	DOMAIN_ALIAS_DCOM_USERS 0x0000_0232
	DOMAIN_ALIAS_IUSERS 0x0000_0238
	DOMAIN_ALIAS_CRYPTO_OPERATORS 0x0000_0239
	DOMAIN_ALIAS_CACHEABLE_PRINCIPALS_GROUP 0x0000_023b
	DOMAIN_ALIAS_NON_CACHEABLE_PRINCIPALS_GROUP 0x0000_023c
	DOMAIN_ALIAS_EVENT_LOG_READERS_GROUP 0x0000_023d
	DOMAIN_ALIAS_CERTSVC_DCOM_ACCESS_GROUP 0x0000_023e
	DOMAIN_ALIAS_RDS_REMOTE_ACCESS_SERVERS 0x0000_023f
	DOMAIN_ALIAS_RDS_ENDPOINT_SERVERS 0x0000_0240
	DOMAIN_ALIAS_RDS_MANAGEMENT_SERVERS 0x0000_0241
	DOMAIN_ALIAS_HYPER_V_ADMINS 0x0000_0242
	DOMAIN_ALIAS_ACCESS_CONTROL_ASSISTANCE_OPS 0x0000_0243
	DOMAIN_ALIAS_REMOTE_MANAGEMENT_USERS 0x0000_0244
	DOMAIN_ALIAS_DEFAULT_ACCOUNT 0x0000_0245
	DOMAIN_ALIAS_STORAGE_REPLICA_ADMINS 0x0000_0246
	DOMAIN_ALIAS_DEVICE_OWNERS 0x0000_0247
	SECURITY_APP_PACKAGE_BASE 0x0000_0002
	SECURITY_CAPABILITY_BASE 0x0000_0003
	SECURITY_CAPABILITY_APP 0x00000_0400
	SECURITY_BUILTIN_PACKAGE_ANY_PACKAGE 0x0000_0001
	SECURITY_BUILTIN_PACKAGE_ANY_RESTRICTED_PACKAGE 0x0000_0002
	SECURITY_CAPABILITY_INTERNET_CLIENT 0x0000_0001
	SECURITY_CAPABILITY_INTERNET_CLIENT_SERVER 0x0000_0002
	SECURITY_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER 0x0000_0003
	SECURITY_CAPABILITY_PICTURES_LIBRARY 0x0000_0004
	SECURITY_CAPABILITY_VIDEOS_LIBRARY 0x0000_0005
	SECURITY_CAPABILITY_MUSIC_LIBRARY 0x0000_0006
	SECURITY_CAPABILITY_DOCUMENTS_LIBRARY 0x0000_0007
	SECURITY_CAPABILITY_ENTERPRISE_AUTHENTICATION 0x0000_0008
	SECURITY_CAPABILITY_SHARED_USER_CERTIFICATES 0x0000_0009
	SECURITY_CAPABILITY_REMOVABLE_STORAGE 0x0000_000a
	SECURITY_CAPABILITY_APPOINTMENTS 0x0000_000b
	SECURITY_CAPABILITY_CONTACTS 0x0000_000c
	SECURITY_CAPABILITY_INTERNET_EXPLORER 0x0000_1000
	SECURITY_AUTHENTICATION_AUTHORITY_ASSERTED 0x0000_0001
	SECURITY_AUTHENTICATION_SERVICE_ASSERTED 0x0000_0002
	SECURITY_AUTHENTICATION_FRESH_KEY_AUTH 0x0000_0003
	SECURITY_AUTHENTICATION_KEY_TRUST 0x0000_0004
	SECURITY_AUTHENTICATION_KEY_PROPERTY_MFA 0x0000_0005
	SECURITY_AUTHENTICATION_KEY_PROPERTY_ATTESTATION 0x0000_0006
	SECURITY_PROCESS_PROTECTION_TYPE_FULL 0x0000_0400
	SECURITY_PROCESS_PROTECTION_TYPE_LITE 0x0000_0200
	SECURITY_PROCESS_PROTECTION_TYPE_NONE 0x0000_0000
	SECURITY_PROCESS_PROTECTION_LEVEL_WINTCB 0x0000_2000
	SECURITY_PROCESS_PROTECTION_LEVEL_WINDOWS 0x0000_1000
	SECURITY_PROCESS_PROTECTION_LEVEL_APP 0x0000_0800
	SECURITY_PROCESS_PROTECTION_LEVEL_ANTIMALWARE 0x0000_0600
	SECURITY_PROCESS_PROTECTION_LEVEL_AUTHENTICODE 0x0000_0400
	SECURITY_PROCESS_PROTECTION_LEVEL_NONE 0x0000_0000
	SECURITY_TRUSTED_INSTALLER_1 9_5600_8885
	SECURITY_TRUSTED_INSTALLER_2 34_1852_2649
	SECURITY_TRUSTED_INSTALLER_3 18_3103_8044
	SECURITY_TRUSTED_INSTALLER_4 18_5329_2631
	SECURITY_TRUSTED_INSTALLER_5 22_7147_8464
}

const_bitflag! { RRF: u32;
	/// [`HKEY::GetValue`](crate::prelude::advapi_Hkey::RegGetValue) `dwFlags`
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

const_bitflag! { SC_MANAGER: u32;
	/// Service Control Manager access rights
	/// [`flags`](https://learn.microsoft.com/en-us/windows/win32/services/service-security-and-access-rights)
	/// (`u32`).
	=>
	=>
	ALL_ACCESS 0xf003f
	CREATE_SERVICE 0x0002
	CONNECT 0x0001
	ENUMERATE_SERVICE 0x0004
	LOCK 0x0008
	MODIFY_BOOT_CONFIG 0x0020
	QUERY_LOCK_STATUS 0x0010

	GENERIC_READ STANDARD_RIGHTS::READ.raw() | Self::ENUMERATE_SERVICE.0 | Self::QUERY_LOCK_STATUS.0
	GENERIC_WRITE STANDARD_RIGHTS::WRITE.raw() | Self::CREATE_SERVICE.0 | Self::MODIFY_BOOT_CONFIG.0
	GENERIC_EXECUTE STANDARD_RIGHTS::EXECUTE.raw() | Self::CONNECT.0 | Self::LOCK.0
	GENERIC_ALL Self::ALL_ACCESS.0
}

const_str! { SE_PRIV;
	/// [Privilege constants](https://learn.microsoft.com/en-us/windows/win32/secauthz/privilege-constants)
	/// (`&'static str`).
	///
	/// Originally has `SE` prefix.
	=>
	CREATE_TOKEN_NAME "SeCreateTokenPrivilege"
	ASSIGNPRIMARYTOKEN_NAME "SeAssignPrimaryTokenPrivilege"
	LOCK_MEMORY_NAME "SeLockMemoryPrivilege"
	INCREASE_QUOTA_NAME "SeIncreaseQuotaPrivilege"
	UNSOLICITED_INPUT_NAME "SeUnsolicitedInputPrivilege"
	MACHINE_ACCOUNT_NAME "SeMachineAccountPrivilege"
	TCB_NAME "SeTcbPrivilege"
	SECURITY_NAME "SeSecurityPrivilege"
	TAKE_OWNERSHIP_NAME "SeTakeOwnershipPrivilege"
	LOAD_DRIVER_NAME "SeLoadDriverPrivilege"
	SYSTEM_PROFILE_NAME "SeSystemProfilePrivilege"
	SYSTEMTIME_NAME "SeSystemtimePrivilege"
	PROF_SINGLE_PROCESS_NAME "SeProfileSingleProcessPrivilege"
	INC_BASE_PRIORITY_NAME "SeIncreaseBasePriorityPrivilege"
	CREATE_PAGEFILE_NAME "SeCreatePagefilePrivilege"
	CREATE_PERMANENT_NAME "SeCreatePermanentPrivilege"
	BACKUP_NAME "SeBackupPrivilege"
	RESTORE_NAME "SeRestorePrivilege"
	SHUTDOWN_NAME "SeShutdownPrivilege"
	DEBUG_NAME "SeDebugPrivilege"
	AUDIT_NAME "SeAuditPrivilege"
	SYSTEM_ENVIRONMENT_NAME "SeSystemEnvironmentPrivilege"
	CHANGE_NOTIFY_NAME "SeChangeNotifyPrivilege"
	REMOTE_SHUTDOWN_NAME "SeRemoteShutdownPrivilege"
	UNDOCK_NAME "SeUndockPrivilege"
	SYNC_AGENT_NAME "SeSyncAgentPrivilege"
	ENABLE_DELEGATION_NAME "SeEnableDelegationPrivilege"
	MANAGE_VOLUME_NAME "SeManageVolumePrivilege"
	IMPERSONATE_NAME "SeImpersonatePrivilege"
	CREATE_GLOBAL_NAME "SeCreateGlobalPrivilege"
	TRUSTED_CREDMAN_ACCESS_NAME "SeTrustedCredManAccessPrivilege"
	RELABEL_NAME "SeRelabelPrivilege"
	INC_WORKING_SET_NAME "SeIncreaseWorkingSetPrivilege"
	TIME_ZONE_NAME "SeTimeZonePrivilege"
	CREATE_SYMBOLIC_LINK_NAME "SeCreateSymbolicLinkPrivilege"
	DELEGATE_SESSION_USER_IMPERSONATE_NAME "SeDelegateSessionUserImpersonatePrivilege"
}

const_bitflag! { SE_PRIV_ATTR: u32;
	/// [Privilege attributes](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-adjusttokenprivileges)
	/// (`u32`).
	///
	/// Originally has `SE_PRIVILEGE` prefix.
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	ENABLED_BY_DEFAULT 0x0000_0001
	ENABLED 0x0000_0002
	REMOVED 0x0000_0004
	USED_FOR_ACCESS 0x8000_0000
	VALID_ATTRIBUTES Self::ENABLED_BY_DEFAULT.0 | Self::ENABLED.0 | Self::REMOVED.0 | Self::USED_FOR_ACCESS.0
}

const_bitflag! { SECURITY_IMPERSONATION: u32;
	/// [`SECURITY_IMPERSONATION_LEVEL`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-security_impersonation_level)
	/// enumeration (`u32`).
	=>
	=>
	Anonymous 0
	Identification 1
	Impersonation 2
	Delegation 3
}

const_bitflag! { SERVICE_ACCEPT: u32;
	/// [`SERVICE_STATUS`](crate::SERVICE_STATUS) `dwControlsAccepted` (`u32`).
	=>
	=>
	NETBINDCHANGE 0x0000_0010
	PARAMCHANGE 0x0000_0008
	PAUSE_CONTINUE 0x0000_0002
	PRESHUTDOWN 0x0000_0100
	SHUTDOWN 0x0000_0004
	STOP 0x0000_0001
	HARDWAREPROFILECHANGE 0x0000_0020
	POWEREVENT 0x0000_0040
	SESSIONCHANGE 0x0000_0080
	TIMECHANGE 0x0000_0200
	TRIGGEREVENT 0x0000_0400
	USERMODEREBOOT 0x0000_0800
}

const_bitflag! { SERVICE_CONTROL: u32;
	/// [`LPHANDLER_FUNCTION_EX`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nc-winsvc-lphandler_function_ex)
	/// `dwControl` (`u32`).
	///
	/// Used within [`SvcCtl`](crate::SvcCtl) parsing.
	=>
	=>
	CONTINUE 0x0000_0003
	INTERROGATE 0x0000_0004
	NETBINDADD 0x0000_0007
	NETBINDDISABLE 0x0000_000a
	NETBINDENABLE 0x0000_0009
	NETBINDREMOVE 0x0000_0008
	PARAMCHANGE 0x0000_0006
	PAUSE 0x0000_0002
	PRESHUTDOWN 0x0000_000f
	SHUTDOWN 0x0000_0005
	STOP 0x0000_0001

	DEVICEEVENT 0x0000_000b
	HARDWAREPROFILECHANGE 0x0000_000c
	POWEREVENT 0x0000_000d
	SESSIONCHANGE 0x0000_000e
	TIMECHANGE 0x0000_0010
	TRIGGEREVENT 0x0000_0020
	USERMODEREBOOT 0x0000_0040
}

const_ordinary! { SERVICE_ERROR: u32;
	/// [`HSC::CreateService`](crate::prelude::advapi_Hsc::CreateService)
	/// `error_control` (`u32`).
	=>
	=>
	CRITICAL 0x0000_0003
	IGNORE 0x0000_0000
	NORMAL 0x0000_0001
	SEVERE 0x0000_0002
}

const_ordinary! { SERVICE_START: u32;
	/// [`HSC::CreateService`](crate::prelude::advapi_Hsc::CreateService)
	/// `start_type` (`u32`).
	///
	/// Originally has `SERVICE` prefix.
	=>
	=>
	AUTO_START 0x0000_0002
	BOOT_START 0x0000_0000
	DEMAND_START 0x0000_0003
	DISABLED 0x0000_0004
	SYSTEM_START 0x0000_0001
}

const_ordinary! { SERVICE_STATE: u32;
	/// [`HSERVICESTATUS::SetServiceStatus`](crate::prelude::advapi_Hservicestatus::SetServiceStatus)
	/// `current_state` (u32)
	=>
	=>
	CONTINUE_PENDING 0x0000_0005
	PAUSE_PENDING 0x0000_0006
	PAUSED 0x0000_0007
	RUNNING 0x0000_0004
	START_PENDING 0x0000_0002
	STOP_PENDING 0x0000_0003
	STOPPED 0x0000_0001
}

const_ordinary! { SERVICE_TYPE: u32;
	/// [`HSC::CreateService`](crate::prelude::advapi_Hsc::CreateService)
	/// `service_type` (`u32`).
	///
	/// Originally has `SERVICE` prefix.
	=>
	=>
	ADAPTER 0x0000_0004
	FILE_SYSTEM_DRIVER 0x0000_0002
	KERNEL_DRIVER 0x0000_0001
	RECOGNIZER_DRIVER 0x0000_0008
	WIN32_OWN_PROCESS 0x0000_0010
	WIN32_SHARE_PROCESS 0x0000_0020

	WIN32_OWN_PROCESS_INTERACTIVE Self::WIN32_OWN_PROCESS.0 | 0x0000_0100
	WIN32_SHARE_PROCESS_INTERACTIVE Self::WIN32_SHARE_PROCESS.0 | 0x0000_0100
}

const_ordinary! { SID_NAME_USE: u32;
	/// [`SID_NAME_USE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-sid_name_use)
	/// enumeration (`u32`).
	///
	/// Originally has `Sid` prefix.
	=>
	=>
	User 1
	Group 2
	Domain 3
	Alias 4
	WellKnownGroup 5
	DeletedAccount 6
	Invalid 7
	Unknown 8
	Computer 9
	Label 10
	LogonSession 11
}

const_ordinary! { TOKEN_ELEVATION_TYPE: u32;
	/// [`TOKEN_ELEVATION_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_elevation_type)
	/// enumeration (`u32`).
	=>
	=>
	Default 1
	Full 2
	Limited 3
}

const_ordinary! { TOKEN_INFORMATION_CLASS: u32;
	/// [`TOKEN_INFORMATION_CLASS`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_information_class)
	/// enumeration (`u32`).
	=>
	=>
	User 1
	Groups 2
	Privileges 3
	Owner 4
	PrimaryGroup 5
	DefaultDacl 6
	Source 7
	Type 8
	ImpersonationLevel 9
	Statistics 10
	RestrictedSids 11
	SessionId 12
	GroupsAndPrivileges 13
	SessionReference 14
	SandBoxInert 15
	AuditPolicy 16
	Origin 17
	ElevationType 18
	LinkedToken 19
	Elevation 20
	HasRestrictions 21
	AccessInformation 22
	VirtualizationAllowed 23
	VirtualizationEnabled 24
	IntegrityLevel 25
	UIAccess 26
	MandatoryPolicy 27
	LogonSid 28
	IsAppContainer 29
	Capabilities 30
	AppContainerSid 31
	AppContainerNumber 32
	UserClaimAttributes 33
	DeviceClaimAttributes 34
	RestrictedUserClaimAttributes 35
	RestrictedDeviceClaimAttributes 36
	DeviceGroups 37
	RestrictedDeviceGroups 38
	SecurityAttributes 39
	IsRestricted 40
	ProcessTrustLevel 41
	PrivateNameSpace 42
	SingletonAttributes 43
	BnoIsolation 44
	ChildProcessFlags 45
	IsLessPrivilegedAppContainer 46
	IsSandboxed 47
	OriginatingProcessTrustLevel 48
}

const_ordinary! { TOKEN_TYPE: u32;
	/// [`TOKEN_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-token_type)
	/// enumeration (`u32`).
	=>
	=>
	Primary 1
	Impersonation 2
}

const_bitflag! { TRANSACTION: u32;
	/// [`Transaction access masks`](https://learn.microsoft.com/en-us/windows/win32/ktm/transaction-access-masks)
	/// (`u32`).
	=>
	=>
	QUERY_INFORMATION 0x0001
	SET_INFORMATION 0x0002
	ENLIST 0x0004
	COMMIT 0x0008
	ROLLBACK 0x0010
	PROPAGATE 0x0020
	RIGHT_RESERVED1 0x0040
	GENERIC_READ STANDARD_RIGHTS::READ.raw() | TRANSACTION::QUERY_INFORMATION.0 | ACCESS_RIGHTS::SYNCHRONIZE.raw()
	GENERIC_WRITE STANDARD_RIGHTS::WRITE.raw() | TRANSACTION::SET_INFORMATION.0 | TRANSACTION::COMMIT.0 | TRANSACTION::ENLIST.0 | TRANSACTION::ROLLBACK.0 | TRANSACTION::PROPAGATE.0 | ACCESS_RIGHTS::SYNCHRONIZE.raw()
	GENERIC_EXECUTE STANDARD_RIGHTS::EXECUTE.raw() | TRANSACTION::COMMIT.0 | TRANSACTION::ROLLBACK.0 | ACCESS_RIGHTS::SYNCHRONIZE.raw()
	ALL_ACCESS STANDARD_RIGHTS::REQUIRED.raw() | TRANSACTION::GENERIC_READ.0 | TRANSACTION::GENERIC_WRITE.0 | TRANSACTION::GENERIC_EXECUTE.0
	RESOURCE_MANAGER_RIGHTS TRANSACTION::GENERIC_READ.0 | STANDARD_RIGHTS::WRITE.raw() | TRANSACTION::SET_INFORMATION.0 | TRANSACTION::ENLIST.0 | TRANSACTION::ROLLBACK.0 | TRANSACTION::PROPAGATE.0 | ACCESS_RIGHTS::SYNCHRONIZE.raw()
}

const_bitflag! { TRANSACTION_OPT: u32;
	/// [`CrateTransaction`](crate::prelude::advapi_Htransaction::CreateTransaction)
	/// `options` (`u32`).
	///
	/// Originally has `TRANSACTION` prefix.
	=>
	=>
	/// None of the actual values (zero).
	NoValue 0
	DO_NOT_PROMOTE 0x0000_0001
}

const_ordinary! { WELL_KNOWN_SID_TYPE: u32;
	/// [`WELL_KNOWN_SID_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-well_known_sid_type)
	/// enumeration (`u32`).
	=>
	=>
	Null 0
	World 1
	Local 2
	CreatorOwner 3
	CreatorGroup 4
	CreatorOwnerServer 5
	CreatorGroupServer 6
	NtAuthority 7
	Dialup 8
	Network 9
	Batch 10
	Interactive 11
	Service 12
	Anonymous 13
	Proxy 14
	EnterpriseControllers 15
	SelfSid 16
	AuthenticatedUser 17
	RestrictedCode 18
	TerminalServer 19
	RemoteLogonId 20
	LogonIds 21
	LocalSystem 22
	LocalService 23
	NetworkService 24
	BuiltinDomain 25
	BuiltinAdministrators 26
	BuiltinUsers 27
	BuiltinGuests 28
	BuiltinPowerUsers 29
	BuiltinAccountOperators 30
	BuiltinSystemOperators 31
	BuiltinPrintOperators 32
	BuiltinBackupOperators 33
	BuiltinReplicator 34
	BuiltinPreWindows2000CompatibleAccess 35
	BuiltinRemoteDesktopUsers 36
	BuiltinNetworkConfigurationOperators 37
	AccountAdministrator 38
	AccountGuest 39
	AccountKrbtgt 40
	AccountDomainAdmins 41
	AccountDomainUsers 42
	AccountDomainGuests 43
	AccountComputers 44
	AccountControllers 45
	AccountCertAdmins 46
	AccountSchemaAdmins 47
	AccountEnterpriseAdmins 48
	AccountPolicyAdmins 49
	AccountRasAndIasServers 50
	NTLMAuthentication 51
	DigestAuthentication 52
	SChannelAuthentication 53
	ThisOrganization 54
	OtherOrganization 55
	BuiltinIncomingForestTrustBuilders 56
	BuiltinPerfMonitoringUsers 57
	BuiltinPerfLoggingUsers 58
	BuiltinAuthorizationAccess 59
	BuiltinTerminalServerLicenseServers 60
	BuiltinDCOMUsers 61
	BuiltinIUsers 62
	IUser 63
	BuiltinCryptoOperators 64
	UntrustedLabel 65
	LowLabel 66
	MediumLabel 67
	HighLabel 68
	SystemLabel 69
	WriteRestrictedCode 70
	CreatorOwnerRights 71
	CacheablePrincipalsGroup 72
	NonCacheablePrincipalsGroup 73
	EnterpriseReadonlyControllers 74
	AccountReadonlyControllers 75
	BuiltinEventLogReadersGroup 76
	NewEnterpriseReadonlyControllers 77
	BuiltinCertSvcDComAccessGroup 78
	MediumPlusLabel 79
	LocalLogon 80
	ConsoleLogon 81
	ThisOrganizationCertificate 82
	ApplicationPackageAuthority 83
	BuiltinAnyPackage 84
	CapabilityInternetClient 85
	CapabilityInternetClientServer 86
	CapabilityPrivateNetworkClientServer 87
	CapabilityPicturesLibrary 88
	CapabilityVideosLibrary 89
	CapabilityMusicLibrary 90
	CapabilityDocumentsLibrary 91
	CapabilitySharedUserCertificates 92
	CapabilityEnterpriseAuthentication 93
	CapabilityRemovableStorage 94
	BuiltinRDSRemoteAccessServers 95
	BuiltinRDSEndpointServers 96
	BuiltinRDSManagementServers 97
	UserModeDrivers 98
	BuiltinHyperVAdmins 99
	AccountCloneableControllers 100
	BuiltinAccessControlAssistanceOperators 101
	BuiltinRemoteManagementUsers 102
	AuthenticationAuthorityAsserted 103
	AuthenticationServiceAsserted 104
	LocalAccount 105
	LocalAccountAndAdministrator 106
	AccountProtectedUsers 107
	CapabilityAppointments 108
	CapabilityContacts 109
	AccountDefaultSystemManaged 110
	BuiltinDefaultSystemManagedGroup 111
	BuiltinStorageReplicaAdmins 112
	AccountKeyAdmins 113
	AccountEnterpriseKeyAdmins 114
	AuthenticationKeyTrust 115
	AuthenticationKeyPropertyMFA 116
	AuthenticationKeyPropertyAttestation 117
	AuthenticationFreshKeyAuth 118
	BuiltinDeviceOwners 119
}
