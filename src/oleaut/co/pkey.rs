#![allow(non_upper_case_globals)]

use crate::decl::*;

/// [`PROPERTYKEY`](https://learn.microsoft.com/en-us/windows/win32/api/wtypes/ns-wtypes-propertykey)
/// struct.
#[repr(C, packed(2))]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct PKEY {
	pub fmtid: GUID,
	pub pid: u32,
}

impl PKEY {
	/// Creates a new `PKEY` constant.
	///
	/// # Safety
	///
	/// Be sure the given value is meaningful for the actual type.
	#[must_use]
	pub const unsafe fn from_raw(fmtid: GUID, pid: u32) -> Self {
		Self { fmtid, pid }
	}
}

macro_rules! const_pkey_values {
	(
		$(
			$( #[$valdoc:meta] )*
			$name:ident $guid:expr, $pid:expr
		)*
	) => {
		impl PKEY {
			$(
				$( #[$valdoc] )*
				pub const $name: PKEY = unsafe { PKEY::from_raw(GUID::from_str($guid), $pid) };
			)*
		}
	};
}

// Address properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Address_Country "c07b4199-e1df-4493-b1e1-de5946fb58f8", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Address_CountryCode "c07b4199-e1df-4493-b1e1-de5946fb58f8", 101
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Address_Region "c07b4199-e1df-4493-b1e1-de5946fb58f8", 102
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Address_RegionCode "c07b4199-e1df-4493-b1e1-de5946fb58f8", 103
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Address_Town "c07b4199-e1df-4493-b1e1-de5946fb58f8", 104
}

// Audio properties
const_pkey_values! {
	/// UInt32 -- VT_UI4
	Audio_ChannelCount "64440490-4c8b-11d1-8b70-080036b11a03", 7
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Audio_Compression "64440490-4c8b-11d1-8b70-080036b11a03", 10
	/// UInt32 -- VT_UI4
	Audio_EncodingBitrate "64440490-4c8b-11d1-8b70-080036b11a03", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)  Legacy code may treat this as VT_BSTR.
	Audio_Format "64440490-4c8b-11d1-8b70-080036b11a03", 2
	/// Boolean -- VT_BOOL
	Audio_IsVariableBitRate "e6822fee-8c17-4d62-823c-8e9cfcbd1d5c", 100
	/// UInt32 -- VT_UI4
	Audio_PeakValue "2579e5d0-1116-4084-bd9a-9b4f7cb4df5e", 100
	/// UInt32 -- VT_UI4
	Audio_SampleRate "64440490-4c8b-11d1-8b70-080036b11a03", 5
	/// UInt32 -- VT_UI4
	Audio_SampleSize "64440490-4c8b-11d1-8b70-080036b11a03", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Audio_StreamName "64440490-4c8b-11d1-8b70-080036b11a03", 9
	/// UInt16 -- VT_UI2
	Audio_StreamNumber "64440490-4c8b-11d1-8b70-080036b11a03", 8
}

// Calendar properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Calendar_Duration "293ca35a-09aa-4dd2-b180-1fe245728a52", 100
	/// Boolean -- VT_BOOL
	Calendar_IsOnline "bfee9149-e3e2-49a7-a862-c05988145cec", 100
	/// Boolean -- VT_BOOL
	Calendar_IsRecurring "315b9c8d-80a9-4ef9-ae16-8e746da51d70", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Calendar_Location "f6272d18-cecc-40b1-b26a-3911717aa7bd", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Calendar_OptionalAttendeeAddresses "d55bae5a-3892-417a-a649-c6ac5aaaeab3", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Calendar_OptionalAttendeeNames "09429607-582d-437f-84c3-de93a2b24c3c", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Calendar_OrganizerAddress "744c8242-4df5-456c-ab9e-014efb9021e3", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Calendar_OrganizerName "aaa660f9-9865-458e-b484-01bc7fe3973e", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Calendar_ReminderTime "72fc5ba4-24f9-4011-9f3f-add27afad818", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Calendar_RequiredAttendeeAddresses "0ba7d6c3-568d-4159-ab91-781a91fb71e5", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Calendar_RequiredAttendeeNames "b33af30b-f552-4584-936c-cb93e5cda29f", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Calendar_Resources "00f58a38-c54b-4c40-8696-97235980eae1", 100
	/// UInt16 -- VT_UI2
	Calendar_ResponseStatus "188c1f91-3c40-4132-9ec5-d8b03b72a8a2", 100
	/// UInt16 -- VT_UI2
	Calendar_ShowTimeAs "5bf396d4-5eb2-466f-bde9-2fb3f2361d6e", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Calendar_ShowTimeAsText "53da57cf-62c0-45c4-81de-7610bcefd7f5", 100
}

// Communication properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Communication_AccountName "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 9
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Communication_DateItemExpires "428040ac-a177-4c8a-9760-f6f761227f9a", 100
	/// UInt16 -- VT_UI2
	Communication_Direction "8e531030-b960-4346-ae0d-66bc9a86fb94", 100
	/// Int32 -- VT_I4
	Communication_FollowupIconIndex "83a6347e-6fe4-4f40-ba9c-c4865240d1f4", 100
	/// Boolean -- VT_BOOL
	Communication_HeaderItem "c9c34f84-2241-4401-b607-bd20ed75ae7f", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Communication_PolicyTag "ec0b4191-ab0b-4c66-90b6-c6637cdebbab", 100
	/// Int32 -- VT_I4
	Communication_SecurityFlags "8619a4b6-9f4d-4429-8c0f-b996ca59e335", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Communication_Suffix "807b653a-9e91-43ef-8f97-11ce04ee20c5", 100
	/// UInt16 -- VT_UI2
	Communication_TaskStatus "be1a72c6-9a1d-46b7-afe7-afaf8cef4999", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Communication_TaskStatusText "a6744477-c237-475b-a075-54f34498292a", 100
}

// Computer properties
const_pkey_values! {
	/// Multivalue UInt64 -- VT_VECTOR | VT_UI8  (For variants: VT_ARRAY | VT_UI8)
	Computer_DecoratedFreeSpace "9b174b35-40ff-11d2-a27e-00c04fc30871", 7
}

// Contact properties
const_pkey_values! {
	/// Stream -- VT_STREAM
	Contact_AccountPictureDynamicVideo "0b8bb018-2725-4b44-92ba-7933aeb2dde7", 2
	/// Stream -- VT_STREAM
	Contact_AccountPictureLarge "0b8bb018-2725-4b44-92ba-7933aeb2dde7", 3
	/// Stream -- VT_STREAM
	Contact_AccountPictureSmall "0b8bb018-2725-4b44-92ba-7933aeb2dde7", 4
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Contact_Anniversary "9ad5badb-cea7-4470-a03d-b84e51b9949e", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_AssistantName "cd102c9c-5540-4a88-a6f6-64e4981c8cd1", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_AssistantTelephone "9a93244d-a7ad-4ff8-9b99-45ee4cc09af6", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Contact_Birthday "176dc63c-2688-4e89-8143-a347800f25e9", 47
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress "730fb6dd-cf7c-426b-a03f-bd166cc9ee24", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress1Country "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 119
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress1Locality "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 117
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress1PostalCode "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 120
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress1Region "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 118
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress1Street "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 116
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress2Country "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 124
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress2Locality "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 122
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress2PostalCode "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 125
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress2Region "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 123
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress2Street "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 121
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress3Country "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 129
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress3Locality "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 127
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress3PostalCode "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 130
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress3Region "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 128
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddress3Street "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 126
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddressCity "402b5934-ec5a-48c3-93e6-85e86a2d934e", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddressCountry "b0b87314-fcf6-4feb-8dff-a50da6af561c", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddressPostalCode "e1d4a09e-d758-4cd1-b6ec-34a8b5a73f80", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddressPostOfficeBox "bc4e71ce-17f9-48d5-bee9-021df0ea5409", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddressState "446f787f-10c4-41cb-a6c4-4d0343551597", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessAddressStreet "ddd1460f-c0bf-4553-8ce4-10433c908fb0", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_BusinessEmailAddresses "f271c659-7e5e-471f-ba25-7f77b286f836", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessFaxNumber "91eff6f3-2e27-42ca-933e-7c999fbe310b", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessHomePage "56310920-2491-4919-99ce-eadb06fafdb2", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_BusinessTelephone "6a15e5a0-0a1e-4cd7-bb8c-d2f1b0c929bc", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_CallbackTelephone "bf53d1c3-49e0-4f7f-8567-5a821d8ac542", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_CarTelephone "8fdc6dea-b929-412b-ba90-397a257465fe", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_Children "d4729704-8ef1-43ef-9024-2bd381187fd5", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_CompanyMainTelephone "8589e481-6040-473d-b171-7fa89c2708ed", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_ConnectedServiceDisplayName "39b77f4f-a104-4863-b395-2db2ad8f7bc1", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_ConnectedServiceIdentities "80f41eb8-afc4-4208-aa5f-cce21a627281", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_ConnectedServiceName "b5c84c9e-5927-46b5-a3cc-933c21b78469", 100
	/// UInt32 -- VT_UI4
	Contact_ConnectedServiceSupportedActions "a19fb7a9-024b-4371-a8bf-4d29c3e4e9c9", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_DataSuppliers "9660c283-fc3a-4a08-a096-eed3aac46da2", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_Department "fc9f7306-ff8f-4d49-9fb6-3ffe5c0951ec", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_DisplayBusinessPhoneNumbers "364028da-d895-41fe-a584-302b1bb70a76", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_DisplayHomePhoneNumbers "5068bcdf-d697-4d85-8c53-1f1cdab01763", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_DisplayMobilePhoneNumbers "9cb0c358-9d7a-46b1-b466-dcc6f1a3d93d", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_DisplayOtherPhoneNumbers "03089873-8ee8-4191-bd60-d31f72b7900b", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_EmailAddress "f8fa7fa3-d12b-4785-8a4e-691a94f7a3e7", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_EmailAddress2 "38965063-edc8-4268-8491-b7723172cf29", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_EmailAddress3 "644d37b4-e1b3-4bad-b099-7e7c04966aca", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_EmailAddresses "84d8f337-981d-44b3-9615-c7596dba17e3", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_EmailName "cc6f4f24-6083-4bd4-8754-674d0de87ab8", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_FileAsName "f1a24aa7-9ca7-40f6-89ec-97def9ffe8db", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_FirstName "14977844-6b49-4aad-a714-a4513bf60460", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_FullName "635e9051-50a5-4ba2-b9db-4ed056c77296", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_Gender "3c8cee58-d4f0-4cf9-b756-4e5d24447bcd", 100
	/// UInt16 -- VT_UI2
	Contact_GenderValue "3c8cee58-d4f0-4cf9-b756-4e5d24447bcd", 101
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_Hobbies "5dc2253f-5e11-4adf-9cfe-910dd01e3e70", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress "98f98354-617a-46b8-8560-5b1b64bf1f89", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress1Country "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 104
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress1Locality "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 102
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress1PostalCode "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 105
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress1Region "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 103
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress1Street "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 101
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress2Country "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 109
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress2Locality "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 107
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress2PostalCode "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 110
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress2Region "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 108
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress2Street "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 106
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress3Country "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 114
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress3Locality "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 112
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress3PostalCode "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 115
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress3Region "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 113
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddress3Street "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 111
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddressCity "176dc63c-2688-4e89-8143-a347800f25e9", 65
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddressCountry "08a65aa1-f4c9-43dd-9ddf-a33d8e7ead85", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddressPostalCode "8afcc170-8a46-4b53-9eee-90bae7151e62", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddressPostOfficeBox "7b9f6399-0a3f-4b12-89bd-4adc51c918af", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddressState "c89a23d0-7d6d-4eb8-87d4-776a82d493e5", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeAddressStreet "0adef160-db3f-4308-9a21-06237b16fa2a", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_HomeEmailAddresses "56c90e9d-9d46-4963-886f-2e1cd9a694ef", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeFaxNumber "660e04d6-81ab-4977-a09f-82313113ab26", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_HomeTelephone "176dc63c-2688-4e89-8143-a347800f25e9", 20
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_IMAddress "d68dbd8a-3374-4b81-9972-3ec30682db3d", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_Initials "f3d8f40d-50cb-44a2-9718-40cb9119495d", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JA_CompanyNamePhonetic "897b3694-fe9e-43e6-8066-260f590c0100", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JA_FirstNamePhonetic "897b3694-fe9e-43e6-8066-260f590c0100", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JA_LastNamePhonetic "897b3694-fe9e-43e6-8066-260f590c0100", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo1CompanyAddress "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 120
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo1CompanyName "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 102
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo1Department "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 106
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo1Manager "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 105
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo1OfficeLocation "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 104
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo1Title "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 103
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo1YomiCompanyName "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 101
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo2CompanyAddress "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 121
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo2CompanyName "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 108
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo2Department "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 113
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo2Manager "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 112
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo2OfficeLocation "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 110
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo2Title "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 109
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo2YomiCompanyName "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 107
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo3CompanyAddress "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 123
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo3CompanyName "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 115
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo3Department "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 119
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo3Manager "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 118
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo3OfficeLocation "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 117
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo3Title "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 116
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobInfo3YomiCompanyName "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 114
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_JobTitle "176dc63c-2688-4e89-8143-a347800f25e9", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_Label "97b0ad89-df49-49cc-834e-660974fd755b", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_LastName "8f367200-c270-457c-b1d4-e07c5bcd90c7", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_MailingAddress "c0ac206a-827e-4650-95ae-77e2bb74fcc9", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_MiddleName "176dc63c-2688-4e89-8143-a347800f25e9", 71
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_MobileTelephone "176dc63c-2688-4e89-8143-a347800f25e9", 35
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_NickName "176dc63c-2688-4e89-8143-a347800f25e9", 74
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OfficeLocation "176dc63c-2688-4e89-8143-a347800f25e9", 7
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress "508161fa-313b-43d5-83a1-c1accf68622c", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress1Country "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 134
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress1Locality "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 132
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress1PostalCode "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 135
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress1Region "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 133
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress1Street "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 131
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress2Country "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 139
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress2Locality "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 137
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress2PostalCode "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 140
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress2Region "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 138
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress2Street "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 136
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress3Country "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 144
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress3Locality "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 142
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress3PostalCode "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 145
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress3Region "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 143
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddress3Street "a7b6f596-d678-4bc1-b05f-0203d27e8aa1", 141
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddressCity "6e682923-7f7b-4f0c-a337-cfca296687bf", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddressCountry "8f167568-0aae-4322-8ed9-6055b7b0e398", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddressPostalCode "95c656c1-2abf-4148-9ed3-9ec602e3b7cd", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddressPostOfficeBox "8b26ea41-058f-43f6-aecc-4035681ce977", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddressState "71b377d6-e570-425f-a170-809fae73e54e", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_OtherAddressStreet "ff962609-b7d6-4999-862d-95180d529aea", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_OtherEmailAddresses "11d6336b-38c4-4ec9-84d6-eb38d0b150af", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_PagerTelephone "d6304e01-f8f5-4f45-8b15-d024a6296789", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_PersonalTitle "176dc63c-2688-4e89-8143-a347800f25e9", 69
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Contact_PhoneNumbersCanonical "d042d2a1-927e-40b5-a503-6edbd42a517e", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_Prefix "176dc63c-2688-4e89-8143-a347800f25e9", 75
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_PrimaryAddressCity "c8ea94f0-a9e3-4969-a94b-9c62a95324e0", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_PrimaryAddressCountry "e53d799d-0f3f-466e-b2ff-74634a3cb7a4", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_PrimaryAddressPostalCode "18bbd425-ecfd-46ef-b612-7b4a6034eda0", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_PrimaryAddressPostOfficeBox "de5ef3c7-46e1-484e-9999-62c5308394c1", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_PrimaryAddressState "f1176dfe-7138-4640-8b4c-ae375dc70a6d", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_PrimaryAddressStreet "63c25b20-96be-488f-8788-c09c407ad812", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_PrimaryEmailAddress "176dc63c-2688-4e89-8143-a347800f25e9", 48
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_PrimaryTelephone "176dc63c-2688-4e89-8143-a347800f25e9", 25
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_Profession "7268af55-1ce4-4f6e-a41f-b6e4ef10e4a9", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_SpouseName "9d2408b6-3167-422b-82b0-f583b7a7cfe3", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_Suffix "176dc63c-2688-4e89-8143-a347800f25e9", 73
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_TelexNumber "c554493c-c1f7-40c1-a76c-ef8c0614003e", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_TTYTDDTelephone "aaf16bac-2b55-45e6-9f6d-415eb94910df", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_WebPage "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 18
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_Webpage2 "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 124
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Contact_Webpage3 "00f63dd8-22bd-4a5d-ba34-5cb0b9bdcb03", 125
}

// Core properties
const_pkey_values! {
	/// Int32 -- VT_I4
	AcquisitionID "65a98875-3c80-40ab-abbc-efdaf77dbee2", 100
	/// Any -- VT_NULL  Legacy code may treat this as VT_UNKNOWN.
	ApplicationDefinedProperties "cdbfc167-337e-41d8-af7c-8c09205429c7", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)  Legacy code may treat this as VT_LPSTR.
	ApplicationName "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 18
	/// UInt32 -- VT_UI4
	AppZoneIdentifier "502cfeab-47eb-459c-b960-e6d8728f7701", 102
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)  Legacy code may treat this as VT_LPSTR.
	Author "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	CachedFileUpdaterContentIdForConflictResolution "fceff153-e839-4cf3-a9e7-ea22832094b8", 114
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	CachedFileUpdaterContentIdForStream "fceff153-e839-4cf3-a9e7-ea22832094b8", 113
	/// UInt64 -- VT_UI8
	Capacity "9b174b35-40ff-11d2-a27e-00c04fc30871", 3
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Category "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)  Legacy code may treat this as VT_LPSTR.
	Comment "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Company "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 15
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ComputerName "28636aa6-953d-11d2-b5d6-00c04fd918d0", 5
	/// Multivalue Guid -- VT_VECTOR | VT_CLSID  (For variants: VT_ARRAY | VT_CLSID)
	ContainedItems "28636aa6-953d-11d2-b5d6-00c04fd918d0", 29
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ContentId "fceff153-e839-4cf3-a9e7-ea22832094b8", 132
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ContentStatus "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 27
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ContentType "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 26
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ContentUri "fceff153-e839-4cf3-a9e7-ea22832094b8", 131
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Copyright "64440492-4c8b-11d1-8b70-080036b11a03", 11
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	CreatorAppId "c2ea046e-033c-4e91-bd5b-d4942f6bbe49", 2
	/// UInt32 -- VT_UI4
	CreatorOpenWithUIOptions "c2ea046e-033c-4e91-bd5b-d4942f6bbe49", 3
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	DataObjectFormat "1e81a3f8-a30f-4247-b9ee-1d0368a9425c", 2
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	DateAccessed "b725f130-47ef-101a-a5f1-02608c9eebac", 16
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	DateAcquired "2cbaa8f5-d81f-47ca-b17a-f8d822300131", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	DateArchived "43f8d7b7-a444-4f87-9383-52271c9b915c", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	DateCompleted "72fab781-acda-43e5-b155-b2434f85e678", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	DateCreated "b725f130-47ef-101a-a5f1-02608c9eebac", 15
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	DateImported "14b81da1-0135-4d31-96d9-6cbfc9671a99", 18258
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	DateModified "b725f130-47ef-101a-a5f1-02608c9eebac", 14
	/// UInt32 -- VT_UI4
	DefaultSaveLocationDisplay "5d76b67f-9b3d-44bb-b6ae-25da4f638a67", 10
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	DueDate "3f8472b5-e0af-4db2-8071-c53fe76ae7ce", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	EndDate "c75faa05-96fd-49e7-9cb4-9f601082d553", 100
	/// Any -- VT_NULL  Legacy code may treat this as VT_UNKNOWN.
	ExpandoProperties "6fa20de6-d11c-4d9d-a154-64317628c12d", 100
	/// UInt64 -- VT_UI8
	FileAllocationSize "b725f130-47ef-101a-a5f1-02608c9eebac", 18
	/// UInt32 -- VT_UI4
	FileAttributes "b725f130-47ef-101a-a5f1-02608c9eebac", 13
	/// UInt64 -- VT_UI8
	FileCount "28636aa6-953d-11d2-b5d6-00c04fd918d0", 12
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	FileDescription "0cef7d53-fa64-11d1-a203-0000f81fedee", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	FileExtension "e4f10a3c-49e6-405d-8288-a23bd4eeaa6c", 100
	/// UInt64 -- VT_UI8
	FileFRN "b725f130-47ef-101a-a5f1-02608c9eebac", 21
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	FileName "41cf5ae0-f75a-4806-bd87-59c7d9248eb9", 100
	/// UInt32 -- VT_UI4
	FileOfflineAvailabilityStatus "fceff153-e839-4cf3-a9e7-ea22832094b8", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	FileOwner "9b174b34-40ff-11d2-a27e-00c04fc30871", 4
	/// UInt32 -- VT_UI4
	FilePlaceholderStatus "b2f9b9d6-fec4-4dd5-94d7-8957488c807b", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	FileVersion "0cef7d53-fa64-11d1-a203-0000f81fedee", 4
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	FindData "28636aa6-953d-11d2-b5d6-00c04fd918d0", 0
	/// UInt16 -- VT_UI2
	FlagColor "67df94de-0ca7-4d6f-b792-053a3e4f03cf", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	FlagColorText "45eae747-8e2a-40ae-8cbf-ca52aba6152a", 100
	/// Int32 -- VT_I4
	FlagStatus "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 12
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	FlagStatusText "dc54fd2e-189d-4871-aa01-08c2f57a4abc", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	FolderKind "fceff153-e839-4cf3-a9e7-ea22832094b8", 101
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	FolderNameDisplay "b725f130-47ef-101a-a5f1-02608c9eebac", 25
	/// UInt64 -- VT_UI8
	FreeSpace "9b174b35-40ff-11d2-a27e-00c04fc30871", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	FullText "1e3ee840-bc2b-476c-8237-2acd1a839b22", 6
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	HighKeywords "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 24
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity "a26f4afc-7346-4299-be47-eb1ae613139f", 100
	/// Blob -- VT_BLOB
	Identity_Blob "8c3b93a4-baed-1a83-9a32-102ee313f6eb", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity_DisplayName "7d683fc9-d155-45a8-bb1f-89d19bcb792f", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity_InternetSid "6d6d5d49-265d-4688-9f4e-1fdd33e7cc83", 100
	/// Boolean -- VT_BOOL
	Identity_IsMeIdentity "a4108708-09df-4377-9dfc-6d99986d5a67", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity_KeyProviderContext "a26f4afc-7346-4299-be47-eb1ae613139f", 17
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity_KeyProviderName "a26f4afc-7346-4299-be47-eb1ae613139f", 16
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity_LogonStatusString "f18dedf3-337f-42c0-9e03-cee08708a8c3", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity_PrimaryEmailAddress "fcc16823-baed-4f24-9b32-a0982117f7fa", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity_PrimarySid "2b1b801e-c0c1-4987-9ec5-72fa89814787", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity_ProviderData "a8a74b92-361b-4e9a-b722-7c4a7330a312", 100
	/// Guid -- VT_CLSID
	Identity_ProviderID "74a7de49-fa11-4d3d-a006-db7e08675916", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity_QualifiedUserName "da520e51-f4e9-4739-ac82-02e0a95c9030", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity_UniqueID "e55fc3b0-2b60-4220-918e-b21e8bf16016", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Identity_UserName "c4322503-78ca-49c6-9acc-a68e2afd7b6b", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	IdentityProvider_Name "b96eff7b-35ca-4a35-8607-29e3a54c46ea", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	IdentityProvider_Picture "2425166f-5642-4864-992f-98fd98f294c3", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	ImageParsingName "d7750ee0-c6a4-48ec-b53e-b87b52e6d073", 100
	/// Int32 -- VT_I4
	Importance "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 11
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ImportanceText "a3b29791-7713-4e1d-bb40-17db85f01831", 100
	/// Boolean -- VT_BOOL
	IsAttachment "f23f425c-71a1-4fa8-922f-678ea4a60408", 100
	/// Boolean -- VT_BOOL
	IsDefaultNonOwnerSaveLocation "5d76b67f-9b3d-44bb-b6ae-25da4f638a67", 5
	/// Boolean -- VT_BOOL
	IsDefaultSaveLocation "5d76b67f-9b3d-44bb-b6ae-25da4f638a67", 3
	/// Boolean -- VT_BOOL
	IsDeleted "5cda5fc8-33ee-4ff3-9094-ae7bd8868c4d", 100
	/// Boolean -- VT_BOOL
	IsEncrypted "90e5e14e-648b-4826-b2aa-acaf790e3513", 10
	/// Boolean -- VT_BOOL
	IsFlagged "5da84765-e3ff-4278-86b0-a27967fbdd03", 100
	/// Boolean -- VT_BOOL
	IsFlaggedComplete "a6f360d2-55f9-48de-b909-620e090a647c", 100
	/// Boolean -- VT_BOOL
	IsIncomplete "346c8bd1-2e6a-4c45-89a4-61b78e8e700f", 100
	/// Boolean -- VT_BOOL
	IsLocationSupported "5d76b67f-9b3d-44bb-b6ae-25da4f638a67", 8
	/// Boolean -- VT_BOOL
	IsPinnedToNameSpaceTree "5d76b67f-9b3d-44bb-b6ae-25da4f638a67", 2
	/// Boolean -- VT_BOOL
	IsRead "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 10
	/// Boolean -- VT_BOOL
	IsSearchOnlyItem "5d76b67f-9b3d-44bb-b6ae-25da4f638a67", 4
	/// Boolean -- VT_BOOL
	IsSendToTarget "28636aa6-953d-11d2-b5d6-00c04fd918d0", 33
	/// Boolean -- VT_BOOL
	IsShared "ef884c5b-2bfe-41bb-aae5-76eedf4f9902", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	ItemAuthors "d0a04f0a-462a-48a4-bb2f-3706e88dbd7d", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemClassType "048658ad-2db8-41a4-bbb6-ac1ef1207eb1", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	ItemDate "f7db74b4-4287-4103-afba-f1b13dcd75cf", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemFolderNameDisplay "b725f130-47ef-101a-a5f1-02608c9eebac", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemFolderPathDisplay "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemFolderPathDisplayNarrow "dabd30ed-0043-4789-a7f8-d013a4736622", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemName "6b8da074-3b5c-43bc-886f-0a2cdce00b6f", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemNameDisplay "b725f130-47ef-101a-a5f1-02608c9eebac", 10
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemNameDisplayWithoutExtension "b725f130-47ef-101a-a5f1-02608c9eebac", 24
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemNamePrefix "d7313ff1-a77a-401c-8c99-3dbdd68add36", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemNameSortOverride "b725f130-47ef-101a-a5f1-02608c9eebac", 23
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	ItemParticipants "d4d0aa16-9948-41a4-aa85-d97ff9646993", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemPathDisplay "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 7
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemPathDisplayNarrow "28636aa6-953d-11d2-b5d6-00c04fd918d0", 8
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemSubType "28636aa6-953d-11d2-b5d6-00c04fd918d0", 37
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemType "28636aa6-953d-11d2-b5d6-00c04fd918d0", 11
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemTypeText "b725f130-47ef-101a-a5f1-02608c9eebac", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ItemUrl "49691c90-7e17-101a-a91c-08002b2ecda9", 9
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)  Legacy code may treat this as VT_LPSTR.
	Keywords "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 5
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Kind "1e3ee840-bc2b-476c-8237-2acd1a839b22", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	KindText "f04bef95-c585-4197-a2b7-df46fdc9ee6d", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Language "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 28
	/// UInt32 -- VT_UI4
	LastSyncError "fceff153-e839-4cf3-a9e7-ea22832094b8", 107
	/// UInt32 -- VT_UI4
	LastSyncWarning "fceff153-e839-4cf3-a9e7-ea22832094b8", 128
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	LastWriterPackageFamilyName "502cfeab-47eb-459c-b960-e6d8728f7701", 101
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	LowKeywords "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 25
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	MediumKeywords "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 26
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	MileageInformation "fdf84370-031a-4add-9e91-0d775f1c6605", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	MIMEType "0b63e350-9ccc-11d0-bcdb-00805fccce04", 5
	/// Null -- VT_NULL
	Null "00000000-0000-0000-0000-000000000000", 0
	/// UInt32 -- VT_UI4
	OfflineAvailability "a94688b6-7d9f-4570-a648-e3dfc0ab2b3f", 100
	/// UInt32 -- VT_UI4
	OfflineStatus "6d24888f-4718-4bda-afed-ea0fb4386cd8", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	OriginalFileName "0cef7d53-fa64-11d1-a203-0000f81fedee", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	OwnerSID "5d76b67f-9b3d-44bb-b6ae-25da4f638a67", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ParentalRating "64440492-4c8b-11d1-8b70-080036b11a03", 21
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ParentalRatingReason "10984e0a-f9f2-4321-b7ef-baf195af4319", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ParentalRatingsOrganization "a7fe0840-1344-46f0-8d37-52ed712a4bf9", 100
	/// Any -- VT_NULL  Legacy code may treat this as VT_UNKNOWN.
	ParsingBindContext "dfb9a04d-362f-4ca3-b30b-0254b17b5b84", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ParsingName "28636aa6-953d-11d2-b5d6-00c04fd918d0", 24
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ParsingPath "28636aa6-953d-11d2-b5d6-00c04fd918d0", 30
	/// Int32 -- VT_I4
	PerceivedType "28636aa6-953d-11d2-b5d6-00c04fd918d0", 9
	/// UInt32 -- VT_UI4
	PercentFull "9b174b35-40ff-11d2-a27e-00c04fc30871", 5
	/// UInt16 -- VT_UI2
	Priority "9c1fcf74-2d97-41ba-b4ae-cb2e3661a6e4", 5
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PriorityText "d98be98b-b86b-4095-bf52-9d23b2e0a752", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Project "39a7f922-477c-48de-8bc8-b28441e342e3", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	ProviderItemID "f21d9941-81f0-471a-adee-4e74b49217ed", 100
	/// UInt32 -- VT_UI4
	Rating "64440492-4c8b-11d1-8b70-080036b11a03", 9
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	RatingText "90197ca7-fd8f-4e8c-9da3-b57e1e609295", 100
	/// Object -- VT_UNKNOWN
	RemoteConflictingFile "fceff153-e839-4cf3-a9e7-ea22832094b8", 115
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Security_AllowedEnterpriseDataProtectionIdentities "38d43380-d418-4830-84d5-46935a81c5c6", 32
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Security_EncryptionOwners "5f5aff6a-37e5-4780-97ea-80c7565cf535", 34
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Security_EncryptionOwnersDisplay "de621b8f-e125-43a3-a32d-5665446d632a", 25
	/// UInt16 -- VT_UI2
	Sensitivity "f8d3f6ac-4874-42cb-be59-ab454b30716a", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	SensitivityText "d0c7f054-3f72-4725-8527-129a577cb269", 100
	/// UInt32 -- VT_UI4
	SFGAOFlags "28636aa6-953d-11d2-b5d6-00c04fd918d0", 25
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	SharedWith "ef884c5b-2bfe-41bb-aae5-76eedf4f9902", 200
	/// UInt32 -- VT_UI4
	ShareUserRating "64440492-4c8b-11d1-8b70-080036b11a03", 12
	/// UInt32 -- VT_UI4
	SharingStatus "ef884c5b-2bfe-41bb-aae5-76eedf4f9902", 300
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Shell_OmitFromView "de35258c-c695-4cbc-b982-38b0ad24ced0", 2
	/// UInt32 -- VT_UI4
	SimpleRating "a09f084e-ad41-489f-8076-aa5be3082bca", 100
	/// UInt64 -- VT_UI8
	Size "b725f130-47ef-101a-a5f1-02608c9eebac", 12
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	SoftwareUsed "14b81da1-0135-4d31-96d9-6cbfc9671a99", 305
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	SourceItem "668cdfa5-7a1b-4323-ae4b-e527393a1d81", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	SourcePackageFamilyName "ffae9db7-1c8d-43ff-818c-84403aa3732d", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	StartDate "48fd6ec8-8a12-4cdf-a03e-4ec5a511edde", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Status "000214a1-0000-0000-c000-000000000046", 9
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	StorageProviderCallerVersionInformation "b2f9b9d6-fec4-4dd5-94d7-8957488c807b", 7
	/// UInt32 -- VT_UI4
	StorageProviderError "fceff153-e839-4cf3-a9e7-ea22832094b8", 109
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	StorageProviderFileChecksum "b2f9b9d6-fec4-4dd5-94d7-8957488c807b", 5
	/// UInt32 -- VT_UI4
	StorageProviderFileFlags "b2f9b9d6-fec4-4dd5-94d7-8957488c807b", 8
	/// Boolean -- VT_BOOL
	StorageProviderFileHasConflict "b2f9b9d6-fec4-4dd5-94d7-8957488c807b", 9
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	StorageProviderFileIdentifier "b2f9b9d6-fec4-4dd5-94d7-8957488c807b", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	StorageProviderFileRemoteUri "fceff153-e839-4cf3-a9e7-ea22832094b8", 112
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	StorageProviderFileVersion "b2f9b9d6-fec4-4dd5-94d7-8957488c807b", 4
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	StorageProviderFileVersionWaterline "b2f9b9d6-fec4-4dd5-94d7-8957488c807b", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	StorageProviderId "fceff153-e839-4cf3-a9e7-ea22832094b8", 108
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	StorageProviderShareStatuses "fceff153-e839-4cf3-a9e7-ea22832094b8", 111
	/// UInt32 -- VT_UI4
	StorageProviderSharingStatus "fceff153-e839-4cf3-a9e7-ea22832094b8", 117
	/// UInt64 -- VT_UI8
	StorageProviderStatus "fceff153-e839-4cf3-a9e7-ea22832094b8", 110
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Subject "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 3
	/// UInt32 -- VT_UI4
	SyncTransferStatus "fceff153-e839-4cf3-a9e7-ea22832094b8", 103
	/// Clipboard -- VT_CF
	Thumbnail "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 17
	/// UInt64 -- VT_UI8
	ThumbnailCacheId "446d16b1-8dad-4870-a748-402ea43d788c", 100
	/// Stream -- VT_STREAM
	ThumbnailStream "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 27
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)  Legacy code may treat this as VT_LPSTR.
	Title "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)  Legacy code may treat this as VT_LPSTR.
	TitleSortOverride "f0f7984d-222e-4ad2-82ab-1dd8ea40e57e", 300
	/// UInt64 -- VT_UI8
	TotalFileSize "28636aa6-953d-11d2-b5d6-00c04fd918d0", 14
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Trademarks "0cef7d53-fa64-11d1-a203-0000f81fedee", 9
	/// UInt64 -- VT_UI8
	TransferOrder "fceff153-e839-4cf3-a9e7-ea22832094b8", 106
	/// UInt64 -- VT_UI8
	TransferPosition "fceff153-e839-4cf3-a9e7-ea22832094b8", 104
	/// UInt64 -- VT_UI8
	TransferSize "fceff153-e839-4cf3-a9e7-ea22832094b8", 105
	/// Guid -- VT_CLSID
	VolumeId "446d16b1-8dad-4870-a748-402ea43d788c", 104
	/// UInt32 -- VT_UI4
	ZoneIdentifier "502cfeab-47eb-459c-b960-e6d8728f7701", 100
}

// Devices properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Device_PrinterURL "0b48f35a-be6e-4f17-b108-3c4073d1669a", 15
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	DeviceInterface_Bluetooth_DeviceAddress "2bd67d8b-8beb-48d5-87e0-6cda3428040a", 1
	/// UInt32 -- VT_UI4
	DeviceInterface_Bluetooth_Flags "2bd67d8b-8beb-48d5-87e0-6cda3428040a", 3
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	DeviceInterface_Bluetooth_LastConnectedTime "2bd67d8b-8beb-48d5-87e0-6cda3428040a", 11
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	DeviceInterface_Bluetooth_Manufacturer "2bd67d8b-8beb-48d5-87e0-6cda3428040a", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	DeviceInterface_Bluetooth_ModelNumber "2bd67d8b-8beb-48d5-87e0-6cda3428040a", 5
	/// UInt16 -- VT_UI2
	DeviceInterface_Bluetooth_ProductId "2bd67d8b-8beb-48d5-87e0-6cda3428040a", 8
	/// UInt16 -- VT_UI2
	DeviceInterface_Bluetooth_ProductVersion "2bd67d8b-8beb-48d5-87e0-6cda3428040a", 9
	/// Guid -- VT_CLSID
	DeviceInterface_Bluetooth_ServiceGuid "2bd67d8b-8beb-48d5-87e0-6cda3428040a", 2
	/// UInt16 -- VT_UI2
	DeviceInterface_Bluetooth_VendorId "2bd67d8b-8beb-48d5-87e0-6cda3428040a", 7
	/// Byte -- VT_UI1
	DeviceInterface_Bluetooth_VendorIdSource "2bd67d8b-8beb-48d5-87e0-6cda3428040a", 6
	/// Boolean -- VT_BOOL
	DeviceInterface_Hid_IsReadOnly "cbf38310-4a17-4310-a1eb-247f0b67593b", 4
	/// UInt16 -- VT_UI2
	DeviceInterface_Hid_ProductId "cbf38310-4a17-4310-a1eb-247f0b67593b", 6
	/// UInt16 -- VT_UI2
	DeviceInterface_Hid_UsageId "cbf38310-4a17-4310-a1eb-247f0b67593b", 3
	/// UInt16 -- VT_UI2
	DeviceInterface_Hid_UsagePage "cbf38310-4a17-4310-a1eb-247f0b67593b", 2
	/// UInt16 -- VT_UI2
	DeviceInterface_Hid_VendorId "cbf38310-4a17-4310-a1eb-247f0b67593b", 5
	/// UInt16 -- VT_UI2
	DeviceInterface_Hid_VersionNumber "cbf38310-4a17-4310-a1eb-247f0b67593b", 7
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	DeviceInterface_PrinterDriverDirectory "847c66de-b8d6-4af9-abc3-6f4f926bc039", 14
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	DeviceInterface_PrinterDriverName "afc47170-14f5-498c-8f30-b0d19be449c6", 11
	/// UInt32 -- VT_UI4
	DeviceInterface_PrinterEnumerationFlag "a00742a1-cd8c-4b37-95ab-70755587767a", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	DeviceInterface_PrinterName "0a7b84ef-0c27-463f-84ef-06c5070001be", 10
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	DeviceInterface_PrinterPortName "eec7b761-6f94-41b1-949f-c729720dd13c", 12
	/// Boolean -- VT_BOOL
	DeviceInterface_Proximity_SupportsNfc "fb3842cd-9e2a-4f83-8fcc-4b0761139ae9", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	DeviceInterface_Serial_PortName "4c6bf15c-4c03-4aac-91f5-64c0f852bcf4", 4
	/// UInt16 -- VT_UI2
	DeviceInterface_Serial_UsbProductId "4c6bf15c-4c03-4aac-91f5-64c0f852bcf4", 3
	/// UInt16 -- VT_UI2
	DeviceInterface_Serial_UsbVendorId "4c6bf15c-4c03-4aac-91f5-64c0f852bcf4", 2
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	DeviceInterface_WinUsb_DeviceInterfaceClasses "95e127b5-79cc-4e83-9c9e-8422187b3e0e", 7
	/// Byte -- VT_UI1
	DeviceInterface_WinUsb_UsbClass "95e127b5-79cc-4e83-9c9e-8422187b3e0e", 4
	/// UInt16 -- VT_UI2
	DeviceInterface_WinUsb_UsbProductId "95e127b5-79cc-4e83-9c9e-8422187b3e0e", 3
	/// Byte -- VT_UI1
	DeviceInterface_WinUsb_UsbProtocol "95e127b5-79cc-4e83-9c9e-8422187b3e0e", 6
	/// Byte -- VT_UI1
	DeviceInterface_WinUsb_UsbSubClass "95e127b5-79cc-4e83-9c9e-8422187b3e0e", 5
	/// UInt16 -- VT_UI2
	DeviceInterface_WinUsb_UsbVendorId "95e127b5-79cc-4e83-9c9e-8422187b3e0e", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Aep_AepId "3b2ce006-5e61-4fde-bab8-9b8aac9b26df", 8
	/// UInt16 -- VT_UI2
	Devices_Aep_Bluetooth_Cod_Major "5fbd34cd-561a-412e-ba98-478a6b0fef1d", 2
	/// UInt16 -- VT_UI2
	Devices_Aep_Bluetooth_Cod_Minor "5fbd34cd-561a-412e-ba98-478a6b0fef1d", 3
	/// Boolean -- VT_BOOL
	Devices_Aep_Bluetooth_Cod_Services_Audio "5fbd34cd-561a-412e-ba98-478a6b0fef1d", 10
	/// Boolean -- VT_BOOL
	Devices_Aep_Bluetooth_Cod_Services_Capturing "5fbd34cd-561a-412e-ba98-478a6b0fef1d", 8
	/// Boolean -- VT_BOOL
	Devices_Aep_Bluetooth_Cod_Services_Information "5fbd34cd-561a-412e-ba98-478a6b0fef1d", 12
	/// Boolean -- VT_BOOL
	Devices_Aep_Bluetooth_Cod_Services_LimitedDiscovery "5fbd34cd-561a-412e-ba98-478a6b0fef1d", 4
	/// Boolean -- VT_BOOL
	Devices_Aep_Bluetooth_Cod_Services_Networking "5fbd34cd-561a-412e-ba98-478a6b0fef1d", 6
	/// Boolean -- VT_BOOL
	Devices_Aep_Bluetooth_Cod_Services_ObjectXfer "5fbd34cd-561a-412e-ba98-478a6b0fef1d", 9
	/// Boolean -- VT_BOOL
	Devices_Aep_Bluetooth_Cod_Services_Positioning "5fbd34cd-561a-412e-ba98-478a6b0fef1d", 5
	/// Boolean -- VT_BOOL
	Devices_Aep_Bluetooth_Cod_Services_Rendering "5fbd34cd-561a-412e-ba98-478a6b0fef1d", 7
	/// Boolean -- VT_BOOL
	Devices_Aep_Bluetooth_Cod_Services_Telephony "5fbd34cd-561a-412e-ba98-478a6b0fef1d", 11
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Devices_Aep_Bluetooth_LastSeenTime "2bd67d8b-8beb-48d5-87e0-6cda3428040a", 12
	/// Byte -- VT_UI1
	Devices_Aep_Bluetooth_Le_AddressType "995ef0b0-7eb3-4a8b-b9ce-068bb3f4af69", 4
	/// UInt16 -- VT_UI2
	Devices_Aep_Bluetooth_Le_Appearance "995ef0b0-7eb3-4a8b-b9ce-068bb3f4af69", 1
	/// UInt16 -- VT_UI2
	Devices_Aep_Bluetooth_Le_Appearance_Category "995ef0b0-7eb3-4a8b-b9ce-068bb3f4af69", 5
	/// UInt16 -- VT_UI2
	Devices_Aep_Bluetooth_Le_Appearance_Subcategory "995ef0b0-7eb3-4a8b-b9ce-068bb3f4af69", 6
	/// Boolean -- VT_BOOL
	Devices_Aep_Bluetooth_Le_IsConnectable "995ef0b0-7eb3-4a8b-b9ce-068bb3f4af69", 8
	/// Boolean -- VT_BOOL
	Devices_Aep_CanPair "e7c3fb29-caa7-4f47-8c8b-be59b330d4c5", 3
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_Aep_Category "a35996ab-11cf-4935-8b61-a6761081ecdf", 17
	/// Guid -- VT_CLSID
	Devices_Aep_ContainerId "e7c3fb29-caa7-4f47-8c8b-be59b330d4c5", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Aep_DeviceAddress "a35996ab-11cf-4935-8b61-a6761081ecdf", 12
	/// Boolean -- VT_BOOL
	Devices_Aep_IsConnected "a35996ab-11cf-4935-8b61-a6761081ecdf", 7
	/// Boolean -- VT_BOOL
	Devices_Aep_IsPaired "a35996ab-11cf-4935-8b61-a6761081ecdf", 16
	/// Boolean -- VT_BOOL
	Devices_Aep_IsPresent "a35996ab-11cf-4935-8b61-a6761081ecdf", 9
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Aep_Manufacturer "a35996ab-11cf-4935-8b61-a6761081ecdf", 5
	/// Guid -- VT_CLSID
	Devices_Aep_ModelId "a35996ab-11cf-4935-8b61-a6761081ecdf", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Aep_ModelName "a35996ab-11cf-4935-8b61-a6761081ecdf", 3
	/// Int32 -- VT_I4
	Devices_Aep_PointOfService_ConnectionTypes "d4bf61b3-442e-4ada-882d-fa7b70c832d9", 6
	/// Guid -- VT_CLSID
	Devices_Aep_ProtocolId "3b2ce006-5e61-4fde-bab8-9b8aac9b26df", 5
	/// Int32 -- VT_I4
	Devices_Aep_SignalStrength "a35996ab-11cf-4935-8b61-a6761081ecdf", 6
	/// Boolean -- VT_BOOL
	Devices_AepContainer_CanPair "0bba1ede-7566-4f47-90ec-25fc567ced2a", 3
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_AepContainer_Categories "0bba1ede-7566-4f47-90ec-25fc567ced2a", 9
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_AepContainer_Children "0bba1ede-7566-4f47-90ec-25fc567ced2a", 2
	/// Guid -- VT_CLSID
	Devices_AepContainer_ContainerId "0bba1ede-7566-4f47-90ec-25fc567ced2a", 12
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_AepContainer_DialProtocol_InstalledApplications "6af55d45-38db-4495-acb0-d4728a3b8314", 6
	/// Boolean -- VT_BOOL
	Devices_AepContainer_IsPaired "0bba1ede-7566-4f47-90ec-25fc567ced2a", 4
	/// Boolean -- VT_BOOL
	Devices_AepContainer_IsPresent "0bba1ede-7566-4f47-90ec-25fc567ced2a", 11
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_AepContainer_Manufacturer "0bba1ede-7566-4f47-90ec-25fc567ced2a", 6
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_AepContainer_ModelIds "0bba1ede-7566-4f47-90ec-25fc567ced2a", 8
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_AepContainer_ModelName "0bba1ede-7566-4f47-90ec-25fc567ced2a", 7
	/// Multivalue Guid -- VT_VECTOR | VT_CLSID  (For variants: VT_ARRAY | VT_CLSID)
	Devices_AepContainer_ProtocolIds "0bba1ede-7566-4f47-90ec-25fc567ced2a", 13
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_AepContainer_SupportedUriSchemes "6af55d45-38db-4495-acb0-d4728a3b8314", 5
	/// Boolean -- VT_BOOL
	Devices_AepContainer_SupportsAudio "6af55d45-38db-4495-acb0-d4728a3b8314", 2
	/// Boolean -- VT_BOOL
	Devices_AepContainer_SupportsCapturing "6af55d45-38db-4495-acb0-d4728a3b8314", 11
	/// Boolean -- VT_BOOL
	Devices_AepContainer_SupportsImages "6af55d45-38db-4495-acb0-d4728a3b8314", 4
	/// Boolean -- VT_BOOL
	Devices_AepContainer_SupportsInformation "6af55d45-38db-4495-acb0-d4728a3b8314", 14
	/// Boolean -- VT_BOOL
	Devices_AepContainer_SupportsLimitedDiscovery "6af55d45-38db-4495-acb0-d4728a3b8314", 7
	/// Boolean -- VT_BOOL
	Devices_AepContainer_SupportsNetworking "6af55d45-38db-4495-acb0-d4728a3b8314", 9
	/// Boolean -- VT_BOOL
	Devices_AepContainer_SupportsObjectTransfer "6af55d45-38db-4495-acb0-d4728a3b8314", 12
	/// Boolean -- VT_BOOL
	Devices_AepContainer_SupportsPositioning "6af55d45-38db-4495-acb0-d4728a3b8314", 8
	/// Boolean -- VT_BOOL
	Devices_AepContainer_SupportsRendering "6af55d45-38db-4495-acb0-d4728a3b8314", 10
	/// Boolean -- VT_BOOL
	Devices_AepContainer_SupportsTelephony "6af55d45-38db-4495-acb0-d4728a3b8314", 13
	/// Boolean -- VT_BOOL
	Devices_AepContainer_SupportsVideo "6af55d45-38db-4495-acb0-d4728a3b8314", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_AepService_AepId "c9c141a9-1b4c-4f17-a9d1-f298538cadb8", 6
	/// Byte -- VT_UI1
	Devices_AepService_Bluetooth_CacheMode "9744311e-7951-4b2e-b6f0-ecb293cac119", 5
	/// Guid -- VT_CLSID
	Devices_AepService_Bluetooth_ServiceGuid "a399aac7-c265-474e-b073-ffce57721716", 2
	/// UInt64 -- VT_UI8
	Devices_AepService_Bluetooth_TargetDevice "9744311e-7951-4b2e-b6f0-ecb293cac119", 6
	/// Guid -- VT_CLSID
	Devices_AepService_ContainerId "71724756-3e74-4432-9b59-e7b2f668a593", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_AepService_FriendlyName "71724756-3e74-4432-9b59-e7b2f668a593", 2
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_AepService_IoT_ServiceInterfaces "79d94e82-4d79-45aa-821a-74858b4e4ca6", 2
	/// Boolean -- VT_BOOL
	Devices_AepService_ParentAepIsPaired "c9c141a9-1b4c-4f17-a9d1-f298538cadb8", 7
	/// Guid -- VT_CLSID
	Devices_AepService_ProtocolId "c9c141a9-1b4c-4f17-a9d1-f298538cadb8", 5
	/// Guid -- VT_CLSID
	Devices_AepService_ServiceClassId "71724756-3e74-4432-9b59-e7b2f668a593", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_AepService_ServiceId "c9c141a9-1b4c-4f17-a9d1-f298538cadb8", 2
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_AppPackageFamilyName "51236583-0c4a-4fe8-b81f-166aec13f510", 100
	/// Boolean -- VT_BOOL
	Devices_AudioDevice_Microphone_IsFarField "8943b373-388c-4395-b557-bc6dbaffafdb", 6
	/// Double -- VT_R8
	Devices_AudioDevice_Microphone_SensitivityInDbfs "8943b373-388c-4395-b557-bc6dbaffafdb", 3
	/// Double -- VT_R8
	Devices_AudioDevice_Microphone_SensitivityInDbfs2 "8943b373-388c-4395-b557-bc6dbaffafdb", 5
	/// Double -- VT_R8
	Devices_AudioDevice_Microphone_SignalToNoiseRatioInDb "8943b373-388c-4395-b557-bc6dbaffafdb", 4
	/// Boolean -- VT_BOOL
	Devices_AudioDevice_RawProcessingSupported "8943b373-388c-4395-b557-bc6dbaffafdb", 2
	/// Boolean -- VT_BOOL
	Devices_AudioDevice_SpeechProcessingSupported "fb1de864-e06d-47f4-82a6-8a0aef44493c", 2
	/// Byte -- VT_UI1
	Devices_BatteryLife "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 10
	/// Byte -- VT_UI1
	Devices_BatteryPlusCharging "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 22
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_BatteryPlusChargingText "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 23
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_Category "78c34fc8-104a-4aca-9ea4-524d52996e57", 91
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_CategoryGroup "78c34fc8-104a-4aca-9ea4-524d52996e57", 94
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_CategoryIds "78c34fc8-104a-4aca-9ea4-524d52996e57", 90
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_CategoryPlural "78c34fc8-104a-4aca-9ea4-524d52996e57", 92
	/// Boolean -- VT_BOOL
	Devices_ChallengeAep "0774315e-b714-48ec-8de8-8125c077ac11", 2
	/// Byte -- VT_UI1
	Devices_ChargingState "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 11
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_Children "4340a6c5-93fa-4706-972c-7b648008a5a7", 9
	/// Guid -- VT_CLSID
	Devices_ClassGuid "a45c254e-df1c-4efd-8020-67d146a850e0", 10
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_CompatibleIds "a45c254e-df1c-4efd-8020-67d146a850e0", 4
	/// Boolean -- VT_BOOL
	Devices_Connected "78c34fc8-104a-4aca-9ea4-524d52996e57", 55
	/// Guid -- VT_CLSID
	Devices_ContainerId "8c7ed206-3f8a-4827-b3ab-ae9e1faefc6c", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_DefaultTooltip "880f70a2-6082-47ac-8aab-a739d1a300c3", 153
	/// UInt32 -- VT_UI4
	Devices_DeviceCapabilities "a45c254e-df1c-4efd-8020-67d146a850e0", 17
	/// UInt32 -- VT_UI4
	Devices_DeviceCharacteristics "a45c254e-df1c-4efd-8020-67d146a850e0", 29
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_DeviceDescription1 "78c34fc8-104a-4aca-9ea4-524d52996e57", 81
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_DeviceDescription2 "78c34fc8-104a-4aca-9ea4-524d52996e57", 82
	/// Boolean -- VT_BOOL
	Devices_DeviceHasProblem "540b947e-8b40-45bc-a8a2-6a0b894cbda2", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_DeviceInstanceId "78c34fc8-104a-4aca-9ea4-524d52996e57", 256
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_DeviceManufacturer "a45c254e-df1c-4efd-8020-67d146a850e0", 13
	/// UInt32 -- VT_UI4
	Devices_DevObjectType "13673f42-a3d6-49f6-b4da-ae46e0c5237c", 2
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_DialProtocol_InstalledApplications "6845cc72-1b71-48c3-af86-b09171a19b14", 3
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_DiscoveryMethod "78c34fc8-104a-4aca-9ea4-524d52996e57", 52
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Dnssd_Domain "bf79c0ab-bb74-4cee-b070-470b5ae202ea", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Dnssd_FullName "bf79c0ab-bb74-4cee-b070-470b5ae202ea", 5
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Dnssd_HostName "bf79c0ab-bb74-4cee-b070-470b5ae202ea", 7
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Dnssd_InstanceName "bf79c0ab-bb74-4cee-b070-470b5ae202ea", 4
	/// Guid -- VT_CLSID
	Devices_Dnssd_NetworkAdapterId "bf79c0ab-bb74-4cee-b070-470b5ae202ea", 11
	/// UInt16 -- VT_UI2
	Devices_Dnssd_PortNumber "bf79c0ab-bb74-4cee-b070-470b5ae202ea", 12
	/// UInt16 -- VT_UI2
	Devices_Dnssd_Priority "bf79c0ab-bb74-4cee-b070-470b5ae202ea", 9
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Dnssd_ServiceName "bf79c0ab-bb74-4cee-b070-470b5ae202ea", 2
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_Dnssd_TextAttributes "bf79c0ab-bb74-4cee-b070-470b5ae202ea", 6
	/// UInt32 -- VT_UI4
	Devices_Dnssd_Ttl "bf79c0ab-bb74-4cee-b070-470b5ae202ea", 10
	/// UInt16 -- VT_UI2
	Devices_Dnssd_Weight "bf79c0ab-bb74-4cee-b070-470b5ae202ea", 8
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_FriendlyName "656a3bb3-ecc0-43fd-8477-4ae0404a96cd", 12288
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_FunctionPaths "d08dd4c0-3a9e-462e-8290-7b636b2576b9", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_GlyphIcon "51236583-0c4a-4fe8-b81f-166aec13f510", 123
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_HardwareIds "a45c254e-df1c-4efd-8020-67d146a850e0", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Icon "78c34fc8-104a-4aca-9ea4-524d52996e57", 57
	/// Boolean -- VT_BOOL
	Devices_InLocalMachineContainer "8c7ed206-3f8a-4827-b3ab-ae9e1faefc6c", 4
	/// Guid -- VT_CLSID
	Devices_InterfaceClassGuid "026e516e-b814-414b-83cd-856d6fef4822", 4
	/// Boolean -- VT_BOOL
	Devices_InterfaceEnabled "026e516e-b814-414b-83cd-856d6fef4822", 3
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_InterfacePaths "d08dd4c0-3a9e-462e-8290-7b636b2576b9", 2
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_IpAddress "656a3bb3-ecc0-43fd-8477-4ae0404a96cd", 12297
	/// Boolean -- VT_BOOL
	Devices_IsDefault "78c34fc8-104a-4aca-9ea4-524d52996e57", 86
	/// Boolean -- VT_BOOL
	Devices_IsNetworkConnected "78c34fc8-104a-4aca-9ea4-524d52996e57", 85
	/// Boolean -- VT_BOOL
	Devices_IsShared "78c34fc8-104a-4aca-9ea4-524d52996e57", 84
	/// Boolean -- VT_BOOL
	Devices_IsSoftwareInstalling "83da6326-97a6-4088-9453-a1923f573b29", 9
	/// Boolean -- VT_BOOL
	Devices_LaunchDeviceStageFromExplorer "78c34fc8-104a-4aca-9ea4-524d52996e57", 77
	/// Boolean -- VT_BOOL
	Devices_LocalMachine "78c34fc8-104a-4aca-9ea4-524d52996e57", 70
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_LocationPaths "a45c254e-df1c-4efd-8020-67d146a850e0", 37
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Manufacturer "656a3bb3-ecc0-43fd-8477-4ae0404a96cd", 8192
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_MetadataPath "78c34fc8-104a-4aca-9ea4-524d52996e57", 71
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	Devices_MicrophoneArray_Geometry "a1829ea2-27eb-459e-935d-b2fad7b07762", 2
	/// Byte -- VT_UI1
	Devices_MissedCalls "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 5
	/// Guid -- VT_CLSID
	Devices_ModelId "80d81ea6-7473-4b0c-8216-efc11a2c4c8b", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_ModelName "656a3bb3-ecc0-43fd-8477-4ae0404a96cd", 8194
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_ModelNumber "656a3bb3-ecc0-43fd-8477-4ae0404a96cd", 8195
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_NetworkedTooltip "880f70a2-6082-47ac-8aab-a739d1a300c3", 152
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_NetworkName "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 7
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_NetworkType "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 8
	/// UInt16 -- VT_UI2
	Devices_NewPictures "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Notification "06704b0c-e830-4c81-9178-91e4e95a80a0", 3
	/// Byte -- VT_UI1
	Devices_Notifications_LowBattery "c4c07f2b-8524-4e66-ae3a-a6235f103beb", 2
	/// Byte -- VT_UI1
	Devices_Notifications_MissedCall "6614ef48-4efe-4424-9eda-c79f404edf3e", 2
	/// Byte -- VT_UI1
	Devices_Notifications_NewMessage "2be9260a-2012-4742-a555-f41b638b7dcb", 2
	/// Byte -- VT_UI1
	Devices_Notifications_NewVoicemail "59569556-0a08-4212-95b9-fae2ad6413db", 2
	/// UInt64 -- VT_UI8
	Devices_Notifications_StorageFull "a0e00ee1-f0c7-4d41-b8e7-26a7bd8d38b0", 2
	/// UInt64 -- VT_UI8
	Devices_Notifications_StorageFullLinkText "a0e00ee1-f0c7-4d41-b8e7-26a7bd8d38b0", 3
	/// Object -- VT_UNKNOWN
	Devices_NotificationStore "06704b0c-e830-4c81-9178-91e4e95a80a0", 2
	/// Boolean -- VT_BOOL
	Devices_NotWorkingProperly "78c34fc8-104a-4aca-9ea4-524d52996e57", 83
	/// Boolean -- VT_BOOL
	Devices_Paired "78c34fc8-104a-4aca-9ea4-524d52996e57", 56
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Panel_PanelGroup "8dbc9c86-97a9-4bff-9bc6-bfe95d3e6dad", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Panel_PanelId "8dbc9c86-97a9-4bff-9bc6-bfe95d3e6dad", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Parent "4340a6c5-93fa-4706-972c-7b648008a5a7", 8
	/// Boolean -- VT_BOOL
	Devices_PhoneLineTransportDevice_Connected "aecf2fe8-1d00-4fee-8a6d-a70d719b772b", 2
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	Devices_PhysicalDeviceLocation "540b947e-8b40-45bc-a8a2-6a0b894cbda2", 9
	/// UInt32 -- VT_UI4
	Devices_PlaybackPositionPercent "3633de59-6825-4381-a49b-9f6ba13a1471", 5
	/// Byte -- VT_UI1
	Devices_PlaybackState "3633de59-6825-4381-a49b-9f6ba13a1471", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_PlaybackTitle "3633de59-6825-4381-a49b-9f6ba13a1471", 3
	/// Boolean -- VT_BOOL
	Devices_Present "540b947e-8b40-45bc-a8a2-6a0b894cbda2", 5
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_PresentationUrl "656a3bb3-ecc0-43fd-8477-4ae0404a96cd", 8198
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_PrimaryCategory "d08dd4c0-3a9e-462e-8290-7b636b2576b9", 10
	/// UInt64 -- VT_UI8
	Devices_RemainingDuration "3633de59-6825-4381-a49b-9f6ba13a1471", 4
	/// Boolean -- VT_BOOL
	Devices_RestrictedInterface "026e516e-b814-414b-83cd-856d6fef4822", 6
	/// Byte -- VT_UI1
	Devices_Roaming "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 9
	/// Boolean -- VT_BOOL
	Devices_SafeRemovalRequired "afd97640-86a3-4210-b67c-289c41aabe55", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_SchematicName "026e516e-b814-414b-83cd-856d6fef4822", 9
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_ServiceAddress "656a3bb3-ecc0-43fd-8477-4ae0404a96cd", 16384
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_ServiceId "656a3bb3-ecc0-43fd-8477-4ae0404a96cd", 16385
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_SharedTooltip "880f70a2-6082-47ac-8aab-a739d1a300c3", 151
	/// Byte -- VT_UI1
	Devices_SignalStrength "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 2
	/// Byte -- VT_UI1
	Devices_SmartCards_ReaderKind "d6b5b883-18bd-4b4d-b2ec-9e38affeda82", 2
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_Status "d08dd4c0-3a9e-462e-8290-7b636b2576b9", 259
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Status1 "d08dd4c0-3a9e-462e-8290-7b636b2576b9", 257
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_Status2 "d08dd4c0-3a9e-462e-8290-7b636b2576b9", 258
	/// UInt64 -- VT_UI8
	Devices_StorageCapacity "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 12
	/// UInt64 -- VT_UI8
	Devices_StorageFreeSpace "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 13
	/// UInt32 -- VT_UI4
	Devices_StorageFreeSpacePercent "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 14
	/// Byte -- VT_UI1
	Devices_TextMessages "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 3
	/// Byte -- VT_UI1
	Devices_Voicemail "49cd1f76-5626-4b17-a4e8-18b4aa1a2213", 6
	/// UInt32 -- VT_UI4
	Devices_WiaDeviceType "6bdd1fc6-810f-11d0-bec7-08002be2092f", 2
	/// Guid -- VT_CLSID
	Devices_WiFi_InterfaceGuid "ef1167eb-cbfc-4341-a568-a7c91a68982c", 2
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	Devices_WiFiDirect_DeviceAddress "1506935d-e3e7-450f-8637-82233ebe5f6e", 13
	/// Guid -- VT_CLSID
	Devices_WiFiDirect_GroupId "1506935d-e3e7-450f-8637-82233ebe5f6e", 4
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	Devices_WiFiDirect_InformationElements "1506935d-e3e7-450f-8637-82233ebe5f6e", 12
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	Devices_WiFiDirect_InterfaceAddress "1506935d-e3e7-450f-8637-82233ebe5f6e", 2
	/// Guid -- VT_CLSID
	Devices_WiFiDirect_InterfaceGuid "1506935d-e3e7-450f-8637-82233ebe5f6e", 3
	/// Boolean -- VT_BOOL
	Devices_WiFiDirect_IsConnected "1506935d-e3e7-450f-8637-82233ebe5f6e", 5
	/// Boolean -- VT_BOOL
	Devices_WiFiDirect_IsLegacyDevice "1506935d-e3e7-450f-8637-82233ebe5f6e", 7
	/// Boolean -- VT_BOOL
	Devices_WiFiDirect_IsMiracastLcpSupported "1506935d-e3e7-450f-8637-82233ebe5f6e", 9
	/// Boolean -- VT_BOOL
	Devices_WiFiDirect_IsVisible "1506935d-e3e7-450f-8637-82233ebe5f6e", 6
	/// UInt32 -- VT_UI4
	Devices_WiFiDirect_MiracastVersion "1506935d-e3e7-450f-8637-82233ebe5f6e", 8
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Devices_WiFiDirect_Services "1506935d-e3e7-450f-8637-82233ebe5f6e", 10
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	Devices_WiFiDirect_SupportedChannelList "1506935d-e3e7-450f-8637-82233ebe5f6e", 11
	/// UInt32 -- VT_UI4
	Devices_WiFiDirectServices_AdvertisementId "31b37743-7c5e-4005-93e6-e953f92b82e9", 5
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_WiFiDirectServices_RequestServiceInformation "31b37743-7c5e-4005-93e6-e953f92b82e9", 7
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	Devices_WiFiDirectServices_ServiceAddress "31b37743-7c5e-4005-93e6-e953f92b82e9", 2
	/// UInt32 -- VT_UI4
	Devices_WiFiDirectServices_ServiceConfigMethods "31b37743-7c5e-4005-93e6-e953f92b82e9", 6
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	Devices_WiFiDirectServices_ServiceInformation "31b37743-7c5e-4005-93e6-e953f92b82e9", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Devices_WiFiDirectServices_ServiceName "31b37743-7c5e-4005-93e6-e953f92b82e9", 3
	/// UInt32 -- VT_UI4
	Devices_WinPhone8CameraFlags "b7b4d61c-5a64-4187-a52e-b1539f359099", 2
	/// Guid -- VT_CLSID
	Devices_Wwan_InterfaceGuid "ff1167eb-cbfc-4341-a568-a7c91a68982c", 2
	/// Boolean -- VT_BOOL
	Storage_Portable "4d1ebee8-0803-4774-9842-b77db50265e9", 2
	/// Boolean -- VT_BOOL
	Storage_RemovableMedia "4d1ebee8-0803-4774-9842-b77db50265e9", 3
	/// Boolean -- VT_BOOL
	Storage_SystemCritical "4d1ebee8-0803-4774-9842-b77db50265e9", 4
}

// Document properties
const_pkey_values! {
	/// Int32 -- VT_I4
	Document_ByteCount "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 4
	/// Int32 -- VT_I4
	Document_CharacterCount "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 16
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Document_ClientID "276d7bb0-5b34-4fb0-aa4b-158ed12a1809", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Document_Contributor "f334115e-da1b-4509-9b3d-119504dc7abb", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Document_DateCreated "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 12
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Document_DatePrinted "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 11
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Document_DateSaved "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 13
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Document_Division "1e005ee6-bf27-428b-b01c-79676acd2870", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Document_DocumentID "e08805c8-e395-40df-80d2-54f0d6c43154", 100
	/// Int32 -- VT_I4
	Document_HiddenSlideCount "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 9
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Document_LastAuthor "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 8
	/// Int32 -- VT_I4
	Document_LineCount "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 5
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Document_Manager "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 14
	/// Int32 -- VT_I4
	Document_MultimediaClipCount "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 10
	/// Int32 -- VT_I4
	Document_NoteCount "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 8
	/// Int32 -- VT_I4
	Document_PageCount "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 14
	/// Int32 -- VT_I4
	Document_ParagraphCount "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Document_PresentationFormat "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Document_RevisionNumber "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 9
	/// Int32 -- VT_I4
	Document_Security "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 19
	/// Int32 -- VT_I4
	Document_SlideCount "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 7
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Document_Template "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 7
	/// UInt64 -- VT_UI8
	Document_TotalEditingTime "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 10
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Document_Version "d5cdd502-2e9c-101b-9397-08002b2cf9ae", 29
	/// Int32 -- VT_I4
	Document_WordCount "f29f85e0-4ff9-1068-ab91-08002b27b3d9", 15
}

// DRM properties
const_pkey_values! {
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	DRM_DatePlayExpires "aeac19e4-89ae-4508-b9b7-bb867abee2ed", 6
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	DRM_DatePlayStarts "aeac19e4-89ae-4508-b9b7-bb867abee2ed", 5
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	DRM_Description "aeac19e4-89ae-4508-b9b7-bb867abee2ed", 3
	/// Boolean -- VT_BOOL
	DRM_IsDisabled "aeac19e4-89ae-4508-b9b7-bb867abee2ed", 7
	/// Boolean -- VT_BOOL
	DRM_IsProtected "aeac19e4-89ae-4508-b9b7-bb867abee2ed", 2
	/// UInt32 -- VT_UI4
	DRM_PlayCount "aeac19e4-89ae-4508-b9b7-bb867abee2ed", 4
}

// GPS properties
const_pkey_values! {
	/// Double -- VT_R8
	GPS_Altitude "827edb4f-5b73-44a7-891d-fdffabea35ca", 100
	/// UInt32 -- VT_UI4
	GPS_AltitudeDenominator "78342dcb-e358-4145-ae9a-6bfe4e0f9f51", 100
	/// UInt32 -- VT_UI4
	GPS_AltitudeNumerator "2dad1eb7-816d-40d3-9ec3-c9773be2aade", 100
	/// Byte -- VT_UI1
	GPS_AltitudeRef "46ac629d-75ea-4515-867f-6dc4321c5844", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_AreaInformation "972e333e-ac7e-49f1-8adf-a70d07a9bcab", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	GPS_Date "3602c812-0f3b-45f0-85ad-603468d69423", 100
	/// Double -- VT_R8
	GPS_DestBearing "c66d4b3c-e888-47cc-b99f-9dca3ee34dea", 100
	/// UInt32 -- VT_UI4
	GPS_DestBearingDenominator "7abcf4f8-7c3f-4988-ac91-8d2c2e97eca5", 100
	/// UInt32 -- VT_UI4
	GPS_DestBearingNumerator "ba3b1da9-86ee-4b5d-a2a4-a271a429f0cf", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_DestBearingRef "9ab84393-2a0f-4b75-bb22-7279786977cb", 100
	/// Double -- VT_R8
	GPS_DestDistance "a93eae04-6804-4f24-ac81-09b266452118", 100
	/// UInt32 -- VT_UI4
	GPS_DestDistanceDenominator "9bc2c99b-ac71-4127-9d1c-2596d0d7dcb7", 100
	/// UInt32 -- VT_UI4
	GPS_DestDistanceNumerator "2bda47da-08c6-4fe1-80bc-a72fc517c5d0", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_DestDistanceRef "ed4df2d3-8695-450b-856f-f5c1c53acb66", 100
	/// Multivalue Double -- VT_VECTOR | VT_R8  (For variants: VT_ARRAY | VT_R8)
	GPS_DestLatitude "9d1d7cc5-5c39-451c-86b3-928e2d18cc47", 100
	/// Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
	GPS_DestLatitudeDenominator "3a372292-7fca-49a7-99d5-e47bb2d4e7ab", 100
	/// Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
	GPS_DestLatitudeNumerator "ecf4b6f6-d5a6-433c-bb92-4076650fc890", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_DestLatitudeRef "cea820b9-ce61-4885-a128-005d9087c192", 100
	/// Multivalue Double -- VT_VECTOR | VT_R8  (For variants: VT_ARRAY | VT_R8)
	GPS_DestLongitude "47a96261-cb4c-4807-8ad3-40b9d9dbc6bc", 100
	/// Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
	GPS_DestLongitudeDenominator "425d69e5-48ad-4900-8d80-6eb6b8d0ac86", 100
	/// Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
	GPS_DestLongitudeNumerator "a3250282-fb6d-48d5-9a89-dbcace75cccf", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_DestLongitudeRef "182c1ea6-7c1c-4083-ab4b-ac6c9f4ed128", 100
	/// UInt16 -- VT_UI2
	GPS_Differential "aaf4ee25-bd3b-4dd7-bfc4-47f77bb00f6d", 100
	/// Double -- VT_R8
	GPS_DOP "0cf8fb02-1837-42f1-a697-a7017aa289b9", 100
	/// UInt32 -- VT_UI4
	GPS_DOPDenominator "a0be94c5-50ba-487b-bd35-0654be8881ed", 100
	/// UInt32 -- VT_UI4
	GPS_DOPNumerator "47166b16-364f-4aa0-9f31-e2ab3df449c3", 100
	/// Double -- VT_R8
	GPS_ImgDirection "16473c91-d017-4ed9-ba4d-b6baa55dbcf8", 100
	/// UInt32 -- VT_UI4
	GPS_ImgDirectionDenominator "10b24595-41a2-4e20-93c2-5761c1395f32", 100
	/// UInt32 -- VT_UI4
	GPS_ImgDirectionNumerator "dc5877c7-225f-45f7-bac7-e81334b6130a", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_ImgDirectionRef "a4aaa5b7-1ad0-445f-811a-0f8f6e67f6b5", 100
	/// Multivalue Double -- VT_VECTOR | VT_R8  (For variants: VT_ARRAY | VT_R8)
	GPS_Latitude "8727cfff-4868-4ec6-ad5b-81b98521d1ab", 100
	/// Double -- VT_R8
	GPS_LatitudeDecimal "0f55cde2-4f49-450d-92c1-dcd16301b1b7", 100
	/// Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
	GPS_LatitudeDenominator "16e634ee-2bff-497b-bd8a-4341ad39eeb9", 100
	/// Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
	GPS_LatitudeNumerator "7ddaaad1-ccc8-41ae-b750-b2cb8031aea2", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_LatitudeRef "029c0252-5b86-46c7-aca0-2769ffc8e3d4", 100
	/// Multivalue Double -- VT_VECTOR | VT_R8  (For variants: VT_ARRAY | VT_R8)
	GPS_Longitude "c4c4dbb2-b593-466b-bbda-d03d27d5e43a", 100
	/// Double -- VT_R8
	GPS_LongitudeDecimal "4679c1b5-844d-4590-baf5-f322231f1b81", 100
	/// Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
	GPS_LongitudeDenominator "be6e176c-4534-4d2c-ace5-31dedac1606b", 100
	/// Multivalue UInt32 -- VT_VECTOR | VT_UI4  (For variants: VT_ARRAY | VT_UI4)
	GPS_LongitudeNumerator "02b0f689-a914-4e45-821d-1dda452ed2c4", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_LongitudeRef "33dcf22b-28d5-464c-8035-1ee9efd25278", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_MapDatum "2ca2dae6-eddc-407d-bef1-773942abfa95", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_MeasureMode "a015ed5d-aaea-4d58-8a86-3c586920ea0b", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_ProcessingMethod "59d49e61-840f-4aa9-a939-e2099b7f6399", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_Satellites "467ee575-1f25-4557-ad4e-b8b58b0d9c15", 100
	/// Double -- VT_R8
	GPS_Speed "da5d0862-6e76-4e1b-babd-70021bd25494", 100
	/// UInt32 -- VT_UI4
	GPS_SpeedDenominator "7d122d5a-ae5e-4335-8841-d71e7ce72f53", 100
	/// UInt32 -- VT_UI4
	GPS_SpeedNumerator "acc9ce3d-c213-4942-8b48-6d0820f21c6d", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_SpeedRef "ecf7f4c9-544f-4d6d-9d98-8ad79adaf453", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_Status "125491f4-818f-46b2-91b5-d537753617b2", 100
	/// Double -- VT_R8
	GPS_Track "76c09943-7c33-49e3-9e7e-cdba872cfada", 100
	/// UInt32 -- VT_UI4
	GPS_TrackDenominator "c8d1920c-01f6-40c0-ac86-2f3a4ad00770", 100
	/// UInt32 -- VT_UI4
	GPS_TrackNumerator "702926f4-44a6-43e1-ae71-45627116893b", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	GPS_TrackRef "35dbe6fe-44c3-4400-aaae-d2c799c407e8", 100
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	GPS_VersionID "22704da4-c6b2-4a99-8e56-f16df8c92599", 100
}

// History properties
const_pkey_values! {
	/// Int32 -- VT_I4
	History_VisitCount "5cbf2787-48cf-4208-b90e-ee5e5d420294", 7
}

// Image properties
const_pkey_values! {
	/// UInt32 -- VT_UI4
	Image_BitDepth "6444048f-4c8b-11d1-8b70-080036b11a03", 7
	/// UInt16 -- VT_UI2
	Image_ColorSpace "14b81da1-0135-4d31-96d9-6cbfc9671a99", 40961
	/// Double -- VT_R8
	Image_CompressedBitsPerPixel "364b6fa9-37ab-482a-be2b-ae02f60d4318", 100
	/// UInt32 -- VT_UI4
	Image_CompressedBitsPerPixelDenominator "1f8844e1-24ad-4508-9dfd-5326a415ce02", 100
	/// UInt32 -- VT_UI4
	Image_CompressedBitsPerPixelNumerator "d21a7148-d32c-4624-8900-277210f79c0f", 100
	/// UInt16 -- VT_UI2
	Image_Compression "14b81da1-0135-4d31-96d9-6cbfc9671a99", 259
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Image_CompressionText "3f08e66f-2f44-4bb9-a682-ac35d2562322", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Image_Dimensions "6444048f-4c8b-11d1-8b70-080036b11a03", 13
	/// Double -- VT_R8
	Image_HorizontalResolution "6444048f-4c8b-11d1-8b70-080036b11a03", 5
	/// UInt32 -- VT_UI4
	Image_HorizontalSize "6444048f-4c8b-11d1-8b70-080036b11a03", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Image_ImageID "10dabe05-32aa-4c29-bf1a-63e2d220587f", 100
	/// Int16 -- VT_I2
	Image_ResolutionUnit "19b51fa6-1f92-4a5c-ab48-7df0abd67444", 100
	/// Double -- VT_R8
	Image_VerticalResolution "6444048f-4c8b-11d1-8b70-080036b11a03", 6
	/// UInt32 -- VT_UI4
	Image_VerticalSize "6444048f-4c8b-11d1-8b70-080036b11a03", 4
}

// Journal properties
const_pkey_values! {
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Journal_Contacts "dea7c82c-1d89-4a66-9427-a4e3debabcb1", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Journal_EntryType "95beb1fc-326d-4644-b396-cd3ed90e6ddf", 100
}

// LayoutPattern properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	LayoutPattern_ContentViewModeForBrowse "c9944a21-a406-48fe-8225-aec7e24c211b", 500
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	LayoutPattern_ContentViewModeForSearch "c9944a21-a406-48fe-8225-aec7e24c211b", 501
}

// Link properties
const_pkey_values! {
	/// UInt32 -- VT_UI4
	History_SelectionCount "1ce0d6bc-536c-4600-b0dd-7e0c66b350d5", 8
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	History_TargetUrlHostName "1ce0d6bc-536c-4600-b0dd-7e0c66b350d5", 9
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Link_Arguments "436f2667-14e2-4feb-b30a-146c53b5b674", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Link_Comment "b9b4b3fc-2b51-4a42-b5d8-324146afcf25", 5
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Link_DateVisited "5cbf2787-48cf-4208-b90e-ee5e5d420294", 23
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Link_Description "5cbf2787-48cf-4208-b90e-ee5e5d420294", 21
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Link_FeedItemLocalId "8a2f99f9-3c37-465d-a8d7-69777a246d0c", 2
	/// Int32 -- VT_I4
	Link_Status "b9b4b3fc-2b51-4a42-b5d8-324146afcf25", 3
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Link_TargetExtension "7a7d76f4-b630-4bd7-95ff-37cc51a975c9", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Link_TargetParsingPath "b9b4b3fc-2b51-4a42-b5d8-324146afcf25", 2
	/// UInt32 -- VT_UI4
	Link_TargetSFGAOFlags "b9b4b3fc-2b51-4a42-b5d8-324146afcf25", 8
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Link_TargetUrlHostName "8a2f99f9-3c37-465d-a8d7-69777a246d0c", 5
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Link_TargetUrlPath "8a2f99f9-3c37-465d-a8d7-69777a246d0c", 6
}

// Media properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_AuthorUrl "64440492-4c8b-11d1-8b70-080036b11a03", 32
	/// UInt32 -- VT_UI4
	Media_AverageLevel "09edd5b6-b301-43c5-9990-d00302effd46", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_ClassPrimaryID "64440492-4c8b-11d1-8b70-080036b11a03", 13
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_ClassSecondaryID "64440492-4c8b-11d1-8b70-080036b11a03", 14
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_CollectionGroupID "64440492-4c8b-11d1-8b70-080036b11a03", 24
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_CollectionID "64440492-4c8b-11d1-8b70-080036b11a03", 25
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_ContentDistributor "64440492-4c8b-11d1-8b70-080036b11a03", 18
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_ContentID "64440492-4c8b-11d1-8b70-080036b11a03", 26
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_CreatorApplication "64440492-4c8b-11d1-8b70-080036b11a03", 27
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_CreatorApplicationVersion "64440492-4c8b-11d1-8b70-080036b11a03", 28
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Media_DateEncoded "2e4b640d-5019-46d8-8881-55414cc5caa0", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_DateReleased "de41cc29-6971-4290-b472-f59f2e2f31e2", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Media_DlnaProfileID "cfa31b45-525d-4998-bb44-3f7d81542fa4", 100
	/// UInt64 -- VT_UI8
	Media_Duration "64440490-4c8b-11d1-8b70-080036b11a03", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_DVDID "64440492-4c8b-11d1-8b70-080036b11a03", 15
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_EncodedBy "64440492-4c8b-11d1-8b70-080036b11a03", 36
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_EncodingSettings "64440492-4c8b-11d1-8b70-080036b11a03", 37
	/// UInt32 -- VT_UI4
	Media_EpisodeNumber "64440492-4c8b-11d1-8b70-080036b11a03", 100
	/// UInt32 -- VT_UI4
	Media_FrameCount "6444048f-4c8b-11d1-8b70-080036b11a03", 12
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_MCDI "64440492-4c8b-11d1-8b70-080036b11a03", 16
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_MetadataContentProvider "64440492-4c8b-11d1-8b70-080036b11a03", 17
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Media_Producer "64440492-4c8b-11d1-8b70-080036b11a03", 22
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_PromotionUrl "64440492-4c8b-11d1-8b70-080036b11a03", 33
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_ProtectionType "64440492-4c8b-11d1-8b70-080036b11a03", 38
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_ProviderRating "64440492-4c8b-11d1-8b70-080036b11a03", 39
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_ProviderStyle "64440492-4c8b-11d1-8b70-080036b11a03", 40
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_Publisher "64440492-4c8b-11d1-8b70-080036b11a03", 30
	/// UInt32 -- VT_UI4
	Media_SeasonNumber "64440492-4c8b-11d1-8b70-080036b11a03", 101
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_SeriesName "64440492-4c8b-11d1-8b70-080036b11a03", 42
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_SubscriptionContentId "9aebae7a-9644-487d-a92c-657585ed751a", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_SubTitle "56a3372e-ce9c-11d2-9f0e-006097c686f6", 38
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_ThumbnailLargePath "64440492-4c8b-11d1-8b70-080036b11a03", 47
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_ThumbnailLargeUri "64440492-4c8b-11d1-8b70-080036b11a03", 48
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_ThumbnailSmallPath "64440492-4c8b-11d1-8b70-080036b11a03", 49
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_ThumbnailSmallUri "64440492-4c8b-11d1-8b70-080036b11a03", 50
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_UniqueFileIdentifier "64440492-4c8b-11d1-8b70-080036b11a03", 35
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_UserNoAutoInfo "64440492-4c8b-11d1-8b70-080036b11a03", 41
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Media_UserWebUrl "64440492-4c8b-11d1-8b70-080036b11a03", 34
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Media_Writer "64440492-4c8b-11d1-8b70-080036b11a03", 23
	/// UInt32 -- VT_UI4
	Media_Year "56a3372e-ce9c-11d2-9f0e-006097c686f6", 5
}

// Message properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Message_AttachmentContents "3143bf7c-80a8-4854-8880-e2e40189bdd0", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Message_AttachmentNames "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 21
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Message_BccAddress "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 2
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Message_BccName "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 3
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Message_CcAddress "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 4
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Message_CcName "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 5
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Message_ConversationID "dc8f80bd-af1e-4289-85b6-3dfc1b493992", 100
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	Message_ConversationIndex "dc8f80bd-af1e-4289-85b6-3dfc1b493992", 101
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Message_DateReceived "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 20
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Message_DateSent "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 19
	/// Int32 -- VT_I4
	Message_Flags "a82d9ee7-ca67-4312-965e-226bcea85023", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Message_FromAddress "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 13
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Message_FromName "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 14
	/// Boolean -- VT_BOOL
	Message_HasAttachments "9c1fcf74-2d97-41ba-b4ae-cb2e3661a6e4", 8
	/// Int32 -- VT_I4
	Message_IsFwdOrReply "9a9bc088-4f6d-469e-9919-e705412040f9", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Message_MessageClass "cd9ed458-08ce-418f-a70e-f912c7bb9c5c", 103
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Message_Participants "1a9ba605-8e7c-4d11-ad7d-a50ada18ba1b", 2
	/// Boolean -- VT_BOOL
	Message_ProofInProgress "9098f33c-9a7d-48a8-8de5-2e1227a64e91", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Message_SenderAddress "0be1c8e7-1981-4676-ae14-fdd78f05a6e7", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Message_SenderName "0da41cfa-d224-4a18-ae2f-596158db4b3a", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Message_Store "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 15
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Message_ToAddress "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 16
	/// Int32 -- VT_I4
	Message_ToDoFlags "1f856a9f-6900-4aba-9505-2d5f1b4d66cb", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Message_ToDoTitle "bccc8a3c-8cef-42e5-9b1c-c69079398bc7", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Message_ToName "e3e0584c-b788-4a5a-bb20-7f5a44c9acdd", 17
}

// Music properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_AlbumArtist "56a3372e-ce9c-11d2-9f0e-006097c686f6", 13
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_AlbumArtistSortOverride "f1fdb4af-f78c-466c-bb05-56e92db0b8ec", 103
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_AlbumID "56a3372e-ce9c-11d2-9f0e-006097c686f6", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_AlbumTitle "56a3372e-ce9c-11d2-9f0e-006097c686f6", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_AlbumTitleSortOverride "13eb7ffc-ec89-4346-b19d-ccc6f1784223", 101
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Music_Artist "56a3372e-ce9c-11d2-9f0e-006097c686f6", 2
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Music_ArtistSortOverride "deeb2db5-0696-4ce0-94fe-a01f77a45fb5", 102
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_BeatsPerMinute "56a3372e-ce9c-11d2-9f0e-006097c686f6", 35
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Music_Composer "64440492-4c8b-11d1-8b70-080036b11a03", 19
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Music_ComposerSortOverride "00bc20a3-bd48-4085-872c-a88d77f5097e", 105
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Music_Conductor "56a3372e-ce9c-11d2-9f0e-006097c686f6", 36
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_ContentGroupDescription "56a3372e-ce9c-11d2-9f0e-006097c686f6", 33
	/// UInt32 -- VT_UI4
	Music_DiscNumber "6afe7437-9bcd-49c7-80fe-4a5c65fa5874", 104
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_DisplayArtist "fd122953-fa93-4ef7-92c3-04c946b2f7c8", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Music_Genre "56a3372e-ce9c-11d2-9f0e-006097c686f6", 11
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_InitialKey "56a3372e-ce9c-11d2-9f0e-006097c686f6", 34
	/// Boolean -- VT_BOOL
	Music_IsCompilation "c449d5cb-9ea4-4809-82e8-af9d59ded6d1", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_Lyrics "56a3372e-ce9c-11d2-9f0e-006097c686f6", 12
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_Mood "56a3372e-ce9c-11d2-9f0e-006097c686f6", 39
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_PartOfSet "56a3372e-ce9c-11d2-9f0e-006097c686f6", 37
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Music_Period "64440492-4c8b-11d1-8b70-080036b11a03", 31
	/// Blob -- VT_BLOB
	Music_SynchronizedLyrics "6b223b6a-162e-4aa9-b39f-05d678fc6d77", 100
	/// UInt32 -- VT_UI4
	Music_TrackNumber "56a3372e-ce9c-11d2-9f0e-006097c686f6", 7
}

// Note properties
const_pkey_values! {
	/// UInt16 -- VT_UI2
	Note_Color "4776cafa-bce4-4cb1-a23e-265e76d8eb11", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Note_ColorText "46b4e8de-cdb2-440d-885c-1658eb65b914", 100
}

// Photo properties
const_pkey_values! {
	/// Double -- VT_R8
	Photo_Aperture "14b81da1-0135-4d31-96d9-6cbfc9671a99", 37378
	/// UInt32 -- VT_UI4
	Photo_ApertureDenominator "e1a9a38b-6685-46bd-875e-570dc7ad7320", 100
	/// UInt32 -- VT_UI4
	Photo_ApertureNumerator "0337ecec-39fb-4581-a0bd-4c4cc51e9914", 100
	/// Double -- VT_R8
	Photo_Brightness "1a701bf6-478c-4361-83ab-3701bb053c58", 100
	/// UInt32 -- VT_UI4
	Photo_BrightnessDenominator "6ebe6946-2321-440a-90f0-c043efd32476", 100
	/// UInt32 -- VT_UI4
	Photo_BrightnessNumerator "9e7d118f-b314-45a0-8cfb-d654b917c9e9", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_CameraManufacturer "14b81da1-0135-4d31-96d9-6cbfc9671a99", 271
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_CameraModel "14b81da1-0135-4d31-96d9-6cbfc9671a99", 272
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_CameraSerialNumber "14b81da1-0135-4d31-96d9-6cbfc9671a99", 273
	/// UInt32 -- VT_UI4
	Photo_Contrast "2a785ba9-8d23-4ded-82e6-60a350c86a10", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_ContrastText "59dde9f2-5253-40ea-9a8b-479e96c6249a", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Photo_DateTaken "14b81da1-0135-4d31-96d9-6cbfc9671a99", 36867
	/// Double -- VT_R8
	Photo_DigitalZoom "f85bf840-a925-4bc2-b0c4-8e36b598679e", 100
	/// UInt32 -- VT_UI4
	Photo_DigitalZoomDenominator "745baf0e-e5c1-4cfb-8a1b-d031a0a52393", 100
	/// UInt32 -- VT_UI4
	Photo_DigitalZoomNumerator "16cbb924-6500-473b-a5be-f1599bcbe413", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Photo_Event "14b81da1-0135-4d31-96d9-6cbfc9671a99", 18248
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_EXIFVersion "d35f743a-eb2e-47f2-a286-844132cb1427", 100
	/// Double -- VT_R8
	Photo_ExposureBias "14b81da1-0135-4d31-96d9-6cbfc9671a99", 37380
	/// Int32 -- VT_I4
	Photo_ExposureBiasDenominator "ab205e50-04b7-461c-a18c-2f233836e627", 100
	/// Int32 -- VT_I4
	Photo_ExposureBiasNumerator "738bf284-1d87-420b-92cf-5834bf6ef9ed", 100
	/// Double -- VT_R8
	Photo_ExposureIndex "967b5af8-995a-46ed-9e11-35b3c5b9782d", 100
	/// UInt32 -- VT_UI4
	Photo_ExposureIndexDenominator "93112f89-c28b-492f-8a9d-4be2062cee8a", 100
	/// UInt32 -- VT_UI4
	Photo_ExposureIndexNumerator "cdedcf30-8919-44df-8f4c-4eb2ffdb8d89", 100
	/// UInt32 -- VT_UI4
	Photo_ExposureProgram "14b81da1-0135-4d31-96d9-6cbfc9671a99", 34850
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_ExposureProgramText "fec690b7-5f30-4646-ae47-4caafba884a3", 100
	/// Double -- VT_R8
	Photo_ExposureTime "14b81da1-0135-4d31-96d9-6cbfc9671a99", 33434
	/// UInt32 -- VT_UI4
	Photo_ExposureTimeDenominator "55e98597-ad16-42e0-b624-21599a199838", 100
	/// UInt32 -- VT_UI4
	Photo_ExposureTimeNumerator "257e44e2-9031-4323-ac38-85c552871b2e", 100
	/// Byte -- VT_UI1
	Photo_Flash "14b81da1-0135-4d31-96d9-6cbfc9671a99", 37385
	/// Double -- VT_R8
	Photo_FlashEnergy "14b81da1-0135-4d31-96d9-6cbfc9671a99", 41483
	/// UInt32 -- VT_UI4
	Photo_FlashEnergyDenominator "d7b61c70-6323-49cd-a5fc-c84277162c97", 100
	/// UInt32 -- VT_UI4
	Photo_FlashEnergyNumerator "fcad3d3d-0858-400f-aaa3-2f66cce2a6bc", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_FlashManufacturer "aabaf6c9-e0c5-4719-8585-57b103e584fe", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_FlashModel "fe83bb35-4d1a-42e2-916b-06f3e1af719e", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_FlashText "6b8b68f6-200b-47ea-8d25-d8050f57339f", 100
	/// Double -- VT_R8
	Photo_FNumber "14b81da1-0135-4d31-96d9-6cbfc9671a99", 33437
	/// UInt32 -- VT_UI4
	Photo_FNumberDenominator "e92a2496-223b-4463-a4e3-30eabba79d80", 100
	/// UInt32 -- VT_UI4
	Photo_FNumberNumerator "1b97738a-fdfc-462f-9d93-1957e08be90c", 100
	/// Double -- VT_R8
	Photo_FocalLength "14b81da1-0135-4d31-96d9-6cbfc9671a99", 37386
	/// UInt32 -- VT_UI4
	Photo_FocalLengthDenominator "305bc615-dca1-44a5-9fd4-10c0ba79412e", 100
	/// UInt16 -- VT_UI2
	Photo_FocalLengthInFilm "a0e74609-b84d-4f49-b860-462bd9971f98", 100
	/// UInt32 -- VT_UI4
	Photo_FocalLengthNumerator "776b6b3b-1e3d-4b0c-9a0e-8fbaf2a8492a", 100
	/// Double -- VT_R8
	Photo_FocalPlaneXResolution "cfc08d97-c6f7-4484-89dd-ebef4356fe76", 100
	/// UInt32 -- VT_UI4
	Photo_FocalPlaneXResolutionDenominator "0933f3f5-4786-4f46-a8e8-d64dd37fa521", 100
	/// UInt32 -- VT_UI4
	Photo_FocalPlaneXResolutionNumerator "dccb10af-b4e2-4b88-95f9-031b4d5ab490", 100
	/// Double -- VT_R8
	Photo_FocalPlaneYResolution "4fffe4d0-914f-4ac4-8d6f-c9c61de169b1", 100
	/// UInt32 -- VT_UI4
	Photo_FocalPlaneYResolutionDenominator "1d6179a6-a876-4031-b013-3347b2b64dc8", 100
	/// UInt32 -- VT_UI4
	Photo_FocalPlaneYResolutionNumerator "a2e541c5-4440-4ba8-867e-75cfc06828cd", 100
	/// Double -- VT_R8
	Photo_GainControl "fa304789-00c7-4d80-904a-1e4dcc7265aa", 100
	/// UInt32 -- VT_UI4
	Photo_GainControlDenominator "42864dfd-9da4-4f77-bded-4aad7b256735", 100
	/// UInt32 -- VT_UI4
	Photo_GainControlNumerator "8e8ecf7c-b7b8-4eb8-a63f-0ee715c96f9e", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_GainControlText "c06238b2-0bf9-4279-a723-25856715cb9d", 100
	/// UInt16 -- VT_UI2
	Photo_ISOSpeed "14b81da1-0135-4d31-96d9-6cbfc9671a99", 34855
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_LensManufacturer "e6ddcaf7-29c5-4f0a-9a68-d19412ec7090", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_LensModel "e1277516-2b5f-4869-89b1-2e585bd38b7a", 100
	/// UInt32 -- VT_UI4
	Photo_LightSource "14b81da1-0135-4d31-96d9-6cbfc9671a99", 37384
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	Photo_MakerNote "fa303353-b659-4052-85e9-bcac79549b84", 100
	/// UInt64 -- VT_UI8
	Photo_MakerNoteOffset "813f4124-34e6-4d17-ab3e-6b1f3c2247a1", 100
	/// Double -- VT_R8
	Photo_MaxAperture "08f6d7c2-e3f2-44fc-af1e-5aa5c81a2d3e", 100
	/// UInt32 -- VT_UI4
	Photo_MaxApertureDenominator "c77724d4-601f-46c5-9b89-c53f93bceb77", 100
	/// UInt32 -- VT_UI4
	Photo_MaxApertureNumerator "c107e191-a459-44c5-9ae6-b952ad4b906d", 100
	/// UInt16 -- VT_UI2
	Photo_MeteringMode "14b81da1-0135-4d31-96d9-6cbfc9671a99", 37383
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_MeteringModeText "f628fd8c-7ba8-465a-a65b-c5aa79263a9e", 100
	/// UInt16 -- VT_UI2
	Photo_Orientation "14b81da1-0135-4d31-96d9-6cbfc9671a99", 274
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_OrientationText "a9ea193c-c511-498a-a06b-58e2776dcc28", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)  Legacy code may treat this as VT_LPSTR.
	Photo_PeopleNames "e8309b6e-084c-49b4-b1fc-90a80331b638", 100
	/// UInt16 -- VT_UI2
	Photo_PhotometricInterpretation "341796f1-1df9-4b1c-a564-91bdefa43877", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_PhotometricInterpretationText "821437d6-9eab-4765-a589-3b1cbbd22a61", 100
	/// UInt32 -- VT_UI4
	Photo_ProgramMode "6d217f6d-3f6a-4825-b470-5f03ca2fbe9b", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_ProgramModeText "7fe3aa27-2648-42f3-89b0-454e5cb150c3", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_RelatedSoundFile "318a6b45-087f-4dc2-b8cc-05359551fc9e", 100
	/// UInt32 -- VT_UI4
	Photo_Saturation "49237325-a95a-4f67-b211-816b2d45d2e0", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_SaturationText "61478c08-b600-4a84-bbe4-e99c45f0a072", 100
	/// UInt32 -- VT_UI4
	Photo_Sharpness "fc6976db-8349-4970-ae97-b3c5316a08f0", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_SharpnessText "51ec3f47-dd50-421d-8769-334f50424b1e", 100
	/// Double -- VT_R8
	Photo_ShutterSpeed "14b81da1-0135-4d31-96d9-6cbfc9671a99", 37377
	/// Int32 -- VT_I4
	Photo_ShutterSpeedDenominator "e13d8975-81c7-4948-ae3f-37cae11e8ff7", 100
	/// Int32 -- VT_I4
	Photo_ShutterSpeedNumerator "16ea4042-d6f4-4bca-8349-7c78d30fb333", 100
	/// Double -- VT_R8
	Photo_SubjectDistance "14b81da1-0135-4d31-96d9-6cbfc9671a99", 37382
	/// UInt32 -- VT_UI4
	Photo_SubjectDistanceDenominator "0c840a88-b043-466d-9766-d4b26da3fa77", 100
	/// UInt32 -- VT_UI4
	Photo_SubjectDistanceNumerator "8af4961c-f526-43e5-aa81-db768219178d", 100
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)  Legacy code may treat this as VT_LPSTR.
	Photo_TagViewAggregate "b812f15d-c2d8-4bbf-bacd-79744346113f", 100
	/// Boolean -- VT_BOOL
	Photo_TranscodedForSync "9a8ebb75-6458-4e82-bacb-35c0095b03bb", 100
	/// UInt32 -- VT_UI4
	Photo_WhiteBalance "ee3d3d8a-5381-4cfa-b13b-aaf66b5f4ec9", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Photo_WhiteBalanceText "6336b95e-c7a7-426d-86fd-7ae3d39c84b4", 100
}

// PropGroup properties
const_pkey_values! {
	/// Null -- VT_NULL
	PropGroup_Advanced "900a403b-097b-4b95-8ae2-071fdaeeb118", 100
	/// Null -- VT_NULL
	PropGroup_Audio "2804d469-788f-48aa-8570-71b9c187e138", 100
	/// Null -- VT_NULL
	PropGroup_Calendar "9973d2b5-bfd8-438a-ba94-5349b293181a", 100
	/// Null -- VT_NULL
	PropGroup_Camera "de00de32-547e-4981-ad4b-542f2e9007d8", 100
	/// Null -- VT_NULL
	PropGroup_Contact "df975fd3-250a-4004-858f-34e29a3e37aa", 100
	/// Null -- VT_NULL
	PropGroup_Content "d0dab0ba-368a-4050-a882-6c010fd19a4f", 100
	/// Null -- VT_NULL
	PropGroup_Description "8969b275-9475-4e00-a887-ff93b8b41e44", 100
	/// Null -- VT_NULL
	PropGroup_FileSystem "e3a7d2c1-80fc-4b40-8f34-30ea111bdc2e", 100
	/// Null -- VT_NULL
	PropGroup_General "cc301630-b192-4c22-b372-9f4c6d338e07", 100
	/// Null -- VT_NULL
	PropGroup_GPS "f3713ada-90e3-4e11-aae5-fdc17685b9be", 100
	/// Null -- VT_NULL
	PropGroup_Image "e3690a87-0fa8-4a2a-9a9f-fce8827055ac", 100
	/// Null -- VT_NULL
	PropGroup_Media "61872cf7-6b5e-4b4b-ac2d-59da84459248", 100
	/// Null -- VT_NULL
	PropGroup_MediaAdvanced "8859a284-de7e-4642-99ba-d431d044b1ec", 100
	/// Null -- VT_NULL
	PropGroup_Message "7fd7259d-16b4-4135-9f97-7c96ecd2fa9e", 100
	/// Null -- VT_NULL
	PropGroup_Music "68dd6094-7216-40f1-a029-43fe7127043f", 100
	/// Null -- VT_NULL
	PropGroup_Origin "2598d2fb-5569-4367-95df-5cd3a177e1a5", 100
	/// Null -- VT_NULL
	PropGroup_PhotoAdvanced "0cb2bf5a-9ee7-4a86-8222-f01e07fdadaf", 100
	/// Null -- VT_NULL
	PropGroup_RecordedTV "e7b33238-6584-4170-a5c0-ac25efd9da56", 100
	/// Null -- VT_NULL
	PropGroup_Video "bebe0920-7671-4c54-a3eb-49fddfc191ee", 100
}

// PropList properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	InfoTipText "c9944a21-a406-48fe-8225-aec7e24c211b", 17
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_ConflictPrompt "c9944a21-a406-48fe-8225-aec7e24c211b", 11
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_ContentViewModeForBrowse "c9944a21-a406-48fe-8225-aec7e24c211b", 13
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_ContentViewModeForSearch "c9944a21-a406-48fe-8225-aec7e24c211b", 14
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_ExtendedTileInfo "c9944a21-a406-48fe-8225-aec7e24c211b", 9
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_FileOperationPrompt "c9944a21-a406-48fe-8225-aec7e24c211b", 10
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_FullDetails "c9944a21-a406-48fe-8225-aec7e24c211b", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_InfoTip "c9944a21-a406-48fe-8225-aec7e24c211b", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_NonPersonal "49d1091f-082e-493f-b23f-d2308aa9668c", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_PreviewDetails "c9944a21-a406-48fe-8225-aec7e24c211b", 8
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_PreviewTitle "c9944a21-a406-48fe-8225-aec7e24c211b", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_QuickTip "c9944a21-a406-48fe-8225-aec7e24c211b", 5
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_TileInfo "c9944a21-a406-48fe-8225-aec7e24c211b", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	PropList_XPDetailsPanel "f2275480-f782-4291-bd94-f13693513aec", 0
}

// RecordedTV properties
const_pkey_values! {
	/// UInt32 -- VT_UI4
	RecordedTV_ChannelNumber "6d748de2-8d38-4cc3-ac60-f009b057c557", 7
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	RecordedTV_Credits "6d748de2-8d38-4cc3-ac60-f009b057c557", 4
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	RecordedTV_DateContentExpires "6d748de2-8d38-4cc3-ac60-f009b057c557", 15
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	RecordedTV_EpisodeName "6d748de2-8d38-4cc3-ac60-f009b057c557", 2
	/// Boolean -- VT_BOOL
	RecordedTV_IsATSCContent "6d748de2-8d38-4cc3-ac60-f009b057c557", 16
	/// Boolean -- VT_BOOL
	RecordedTV_IsClosedCaptioningAvailable "6d748de2-8d38-4cc3-ac60-f009b057c557", 12
	/// Boolean -- VT_BOOL
	RecordedTV_IsDTVContent "6d748de2-8d38-4cc3-ac60-f009b057c557", 17
	/// Boolean -- VT_BOOL
	RecordedTV_IsHDContent "6d748de2-8d38-4cc3-ac60-f009b057c557", 18
	/// Boolean -- VT_BOOL
	RecordedTV_IsRepeatBroadcast "6d748de2-8d38-4cc3-ac60-f009b057c557", 13
	/// Boolean -- VT_BOOL
	RecordedTV_IsSAP "6d748de2-8d38-4cc3-ac60-f009b057c557", 14
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	RecordedTV_NetworkAffiliation "2c53c813-fb63-4e22-a1ab-0b331ca1e273", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	RecordedTV_OriginalBroadcastDate "4684fe97-8765-4842-9c13-f006447b178c", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	RecordedTV_ProgramDescription "6d748de2-8d38-4cc3-ac60-f009b057c557", 3
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	RecordedTV_RecordingTime "a5477f61-7a82-4eca-9dde-98b69b2479b3", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	RecordedTV_StationCallSign "6d748de2-8d38-4cc3-ac60-f009b057c557", 5
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	RecordedTV_StationName "1b5439e7-eba1-4af8-bdd7-7af1d4549493", 100
}

// Search properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Search_AutoSummary "560c36c0-503a-11cf-baa1-00004c752a9a", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Search_ContainerHash "bceee283-35df-4d53-826a-f36a3eefc6be", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Search_Contents "b725f130-47ef-101a-a5f1-02608c9eebac", 19
	/// Int32 -- VT_I4
	Search_EntryID "49691c90-7e17-101a-a91c-08002b2ecda9", 5
	/// Blob -- VT_BLOB
	Search_ExtendedProperties "7b03b546-fa4f-4a52-a2fe-03d5311e5865", 100
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Search_GatherTime "0b63e350-9ccc-11d0-bcdb-00805fccce04", 8
	/// Int32 -- VT_I4
	Search_HitCount "49691c90-7e17-101a-a91c-08002b2ecda9", 4
	/// Boolean -- VT_BOOL
	Search_IsClosedDirectory "0b63e343-9ccc-11d0-bcdb-00805fccce04", 23
	/// Boolean -- VT_BOOL
	Search_IsFullyContained "0b63e343-9ccc-11d0-bcdb-00805fccce04", 24
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Search_QueryFocusedSummary "560c36c0-503a-11cf-baa1-00004c752a9a", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Search_QueryFocusedSummaryWithFallback "560c36c0-503a-11cf-baa1-00004c752a9a", 4
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Search_QueryPropertyHits "49691c90-7e17-101a-a91c-08002b2ecda9", 21
	/// Int32 -- VT_I4
	Search_Rank "49691c90-7e17-101a-a91c-08002b2ecda9", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Search_Store "a06992b3-8caf-4ed7-a547-b259e32ac9fc", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Search_UrlToIndex "0b63e343-9ccc-11d0-bcdb-00805fccce04", 2
	/// Multivalue Any -- VT_VECTOR | VT_NULL  (For variants: VT_ARRAY | VT_NULL)
	Search_UrlToIndexWithModificationTime "0b63e343-9ccc-11d0-bcdb-00805fccce04", 12
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Supplemental_Album "0c73b141-39d6-4653-a683-cab291eaf95b", 6
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Supplemental_AlbumID "0c73b141-39d6-4653-a683-cab291eaf95b", 2
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Supplemental_Location "0c73b141-39d6-4653-a683-cab291eaf95b", 5
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Supplemental_Person "0c73b141-39d6-4653-a683-cab291eaf95b", 7
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Supplemental_ResourceId "0c73b141-39d6-4653-a683-cab291eaf95b", 3
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Supplemental_Tag "0c73b141-39d6-4653-a683-cab291eaf95b", 4
}

// Shell properties
const_pkey_values! {
	/// Buffer -- VT_VECTOR | VT_UI1  (For variants: VT_ARRAY | VT_UI1)
	DescriptionID "28636aa6-953d-11d2-b5d6-00c04fd918d0", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	InternalName "0cef7d53-fa64-11d1-a203-0000f81fedee", 5
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	LibraryLocationsCount "908696c7-8f87-44f2-80ed-a8c1c6894575", 2
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Link_TargetSFGAOFlagsStrings "d6942081-d53b-443d-ad47-5e059d9cd27a", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Link_TargetUrl "5cbf2787-48cf-4208-b90e-ee5e5d420294", 2
	/// Guid -- VT_CLSID
	NamespaceCLSID "28636aa6-953d-11d2-b5d6-00c04fd918d0", 6
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Shell_SFGAOFlagsStrings "d6942081-d53b-443d-ad47-5e059d9cd27a", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	StatusBarSelectedItemCount "26dc287c-6e3d-4bd3-b2b0-6a26ba2e346d", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	StatusBarViewItemCount "26dc287c-6e3d-4bd3-b2b0-6a26ba2e346d", 2
}

// Software properties
const_pkey_values! {
	/// Boolean -- VT_BOOL
	AppUserModel_ExcludeFromShowInNewInstall "9f4c2855-9f79-4b39-a8d0-e1d42de1d5f3", 8
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	AppUserModel_ID "9f4c2855-9f79-4b39-a8d0-e1d42de1d5f3", 5
	/// Boolean -- VT_BOOL
	AppUserModel_IsDestListSeparator "9f4c2855-9f79-4b39-a8d0-e1d42de1d5f3", 6
	/// Boolean -- VT_BOOL
	AppUserModel_IsDualMode "9f4c2855-9f79-4b39-a8d0-e1d42de1d5f3", 11
	/// Boolean -- VT_BOOL
	AppUserModel_PreventPinning "9f4c2855-9f79-4b39-a8d0-e1d42de1d5f3", 9
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	AppUserModel_RelaunchCommand "9f4c2855-9f79-4b39-a8d0-e1d42de1d5f3", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	AppUserModel_RelaunchDisplayNameResource "9f4c2855-9f79-4b39-a8d0-e1d42de1d5f3", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	AppUserModel_RelaunchIconResource "9f4c2855-9f79-4b39-a8d0-e1d42de1d5f3", 3
	/// UInt32 -- VT_UI4
	AppUserModel_StartPinOption "9f4c2855-9f79-4b39-a8d0-e1d42de1d5f3", 12
	/// Guid -- VT_CLSID
	AppUserModel_ToastActivatorCLSID "9f4c2855-9f79-4b39-a8d0-e1d42de1d5f3", 26
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	AppUserModel_VisualElementsManifestHintPath "9f4c2855-9f79-4b39-a8d0-e1d42de1d5f3", 31
	/// Boolean -- VT_BOOL
	EdgeGesture_DisableTouchWhenFullscreen "32ce38b2-2c9a-41b1-9bc5-b3784394aa44", 2
	/// DateTime -- VT_FILETIME  (For variants: VT_DATE)
	Software_DateLastUsed "841e4f90-ff59-4d16-8947-e81bbffab36d", 16
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Software_ProductName "0cef7d53-fa64-11d1-a203-0000f81fedee", 7
}

// Sync properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Sync_Comments "7bd5533e-af15-44db-b8c8-bd6624e1d032", 13
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Sync_ConflictDescription "ce50c159-2fb8-41fd-be68-d3e042e274bc", 4
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Sync_ConflictFirstLocation "ce50c159-2fb8-41fd-be68-d3e042e274bc", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Sync_ConflictSecondLocation "ce50c159-2fb8-41fd-be68-d3e042e274bc", 7
	/// Guid -- VT_CLSID
	Sync_HandlerCollectionID "7bd5533e-af15-44db-b8c8-bd6624e1d032", 2
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Sync_HandlerID "7bd5533e-af15-44db-b8c8-bd6624e1d032", 3
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Sync_HandlerName "ce50c159-2fb8-41fd-be68-d3e042e274bc", 2
	/// UInt32 -- VT_UI4
	Sync_HandlerType "7bd5533e-af15-44db-b8c8-bd6624e1d032", 8
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Sync_HandlerTypeLabel "7bd5533e-af15-44db-b8c8-bd6624e1d032", 9
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Sync_ItemID "7bd5533e-af15-44db-b8c8-bd6624e1d032", 6
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Sync_ItemName "ce50c159-2fb8-41fd-be68-d3e042e274bc", 3
	/// UInt32 -- VT_UI4
	Sync_ProgressPercentage "7bd5533e-af15-44db-b8c8-bd6624e1d032", 23
	/// UInt32 -- VT_UI4
	Sync_State "7bd5533e-af15-44db-b8c8-bd6624e1d032", 24
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Sync_Status "7bd5533e-af15-44db-b8c8-bd6624e1d032", 10
}

// Task properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Task_BillingInformation "d37d52c6-261c-4303-82b3-08b926ac6f12", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Task_CompletionStatus "084d8a0a-e6d5-40de-bf1f-c8820e7c877c", 100
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Task_Owner "08c7cc5f-60f2-4494-ad75-55e3e0b5add0", 100
}

// Video properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Video_Compression "64440491-4c8b-11d1-8b70-080036b11a03", 10
	/// Multivalue String -- VT_VECTOR | VT_LPWSTR  (For variants: VT_ARRAY | VT_BSTR)
	Video_Director "64440492-4c8b-11d1-8b70-080036b11a03", 20
	/// UInt32 -- VT_UI4
	Video_EncodingBitrate "64440491-4c8b-11d1-8b70-080036b11a03", 8
	/// UInt32 -- VT_UI4
	Video_FourCC "64440491-4c8b-11d1-8b70-080036b11a03", 44
	/// UInt32 -- VT_UI4
	Video_FrameHeight "64440491-4c8b-11d1-8b70-080036b11a03", 4
	/// UInt32 -- VT_UI4
	Video_FrameRate "64440491-4c8b-11d1-8b70-080036b11a03", 6
	/// UInt32 -- VT_UI4
	Video_FrameWidth "64440491-4c8b-11d1-8b70-080036b11a03", 3
	/// UInt32 -- VT_UI4
	Video_HorizontalAspectRatio "64440491-4c8b-11d1-8b70-080036b11a03", 42
	/// Boolean -- VT_BOOL
	Video_IsSpherical "64440491-4c8b-11d1-8b70-080036b11a03", 100
	/// Boolean -- VT_BOOL
	Video_IsStereo "64440491-4c8b-11d1-8b70-080036b11a03", 98
	/// UInt32 -- VT_UI4
	Video_Orientation "64440491-4c8b-11d1-8b70-080036b11a03", 99
	/// UInt32 -- VT_UI4
	Video_SampleSize "64440491-4c8b-11d1-8b70-080036b11a03", 9
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Video_StreamName "64440491-4c8b-11d1-8b70-080036b11a03", 2
	/// UInt16 -- VT_UI2
	Video_StreamNumber "64440491-4c8b-11d1-8b70-080036b11a03", 11
	/// UInt32 -- VT_UI4
	Video_TotalBitrate "64440491-4c8b-11d1-8b70-080036b11a03", 43
	/// Boolean -- VT_BOOL
	Video_TranscodedForSync "64440491-4c8b-11d1-8b70-080036b11a03", 46
	/// UInt32 -- VT_UI4
	Video_VerticalAspectRatio "64440491-4c8b-11d1-8b70-080036b11a03", 45
}

// Volume properties
const_pkey_values! {
	/// String -- VT_LPWSTR  (For variants: VT_BSTR)
	Volume_FileSystem "9b174b35-40ff-11d2-a27e-00c04fc30871", 4
	/// Boolean -- VT_BOOL
	Volume_IsMappedDrive "149c0b69-2c2d-48fc-808f-d318d78c4636", 2
	/// Boolean -- VT_BOOL
	Volume_IsRoot "9b174b35-40ff-11d2-a27e-00c04fc30871", 10
}
