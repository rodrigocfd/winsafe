#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::co::{ACCESS_RIGHTS, STANDARD_RIGHTS};

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
	READ (STANDARD_RIGHTS::READ.0 | Self::QUERY_VALUE.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
	WRITE (STANDARD_RIGHTS::WRITE.0 | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
	EXECUTE Self::READ.0 & !ACCESS_RIGHTS::SYNCHRONIZE.0
	ALL_ACCESS (STANDARD_RIGHTS::ALL.0 | Self::QUERY_VALUE.0 | Self::SET_VALUE.0 | Self::CREATE_SUB_KEY.0 | Self::ENUMERATE_SUB_KEYS.0 | Self::NOTIFY.0 | Self::CREATE_LINK.0) & !ACCESS_RIGHTS::SYNCHRONIZE.0
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

const_ordinary! { SID_NAME_USE: u32;
	/// [`SID_NAME_USE`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-sid_name_use)
	/// enumeration (`u32`). Originally has `Sid` prefix.
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
