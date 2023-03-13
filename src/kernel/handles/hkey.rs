#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, kernel};
use crate::kernel::decl::{
	FILETIME, HTRANSACTION, RegistryValue, SECURITY_ATTRIBUTES, SysResult,
	VALENT, WString,
};
use crate::kernel::ffi_types::BOOL;
use crate::kernel::guard::RegCloseKeyGuard;
use crate::kernel::privs::error_to_sysresult;
use crate::prelude::Handle;

impl_handle! { HKEY;
	/// Handle to a
	/// [registry key](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hkey).
	///
	/// This handle also exposes several
	/// [predefined registry keys](https://learn.microsoft.com/en-us/windows/win32/sysinfo/predefined-keys),
	/// like `HKEY::CURRENT_USER`, which are always open and ready to be used.
	/// Usually, they are the starting point to open a registry key.
}

impl kernel_Hkey for HKEY {}

macro_rules! predef_key {
	($name:ident, $val:expr) => {
		/// Predefined registry key, always open.
		const $name: HKEY = HKEY($val as *mut _);
	};
}

/// This trait is enabled with the `kernel` feature, and provides methods for
/// [`HKEY`](crate::HKEY).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait kernel_Hkey: Handle {
	predef_key!(CLASSES_ROOT, 0x8000_0000);
	predef_key!(CURRENT_USER, 0x8000_0001);
	predef_key!(LOCAL_MACHINE, 0x8000_0002);
	predef_key!(USERS, 0x8000_0003);
	predef_key!(PERFORMANCE_DATA, 0x8000_0004);
	predef_key!(CURRENT_CONFIG, 0x8000_0005);
	predef_key!(DYN_DATA, 0x8000_0006);
	predef_key!(CURRENT_USER_LOCAL_SETTINGS, 0x8000_0007);
	predef_key!(PERFORMANCE_TEXT, 0x8000_0050);
	predef_key!(PERFORMANCE_NLSTEXT, 0x8000_0060);

	/// [`RegConnectRegistry`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regconnectregistryw)
	/// static method.
	///
	/// # Panics
	///
	/// Panics if `predef_key` is different from:
	///
	/// - [`HKEY::LOCAL_MACHINE`](crate::prelude::kernel_Hkey::LOCAL_MACHINE);
	/// - [`HKEY::PERFORMANCE_DATA`](crate::prelude::kernel_Hkey::PERFORMANCE_DATA);
	/// - [`HKEY::USERS`](crate::prelude::kernel_Hkey::USERS).
	#[must_use]
	fn RegConnectRegistry(
		machine_name: Option<&str>,
		predef_hkey: &HKEY,
	) -> SysResult<RegCloseKeyGuard>
	{
		if *predef_hkey != HKEY::LOCAL_MACHINE
			&& *predef_hkey != HKEY::PERFORMANCE_DATA
			&& *predef_hkey != HKEY::USERS
		{
			panic!("Invalid predef_key.");
		}

		let mut hkey = HKEY::NULL;
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegConnectRegistryW(
					WString::from_opt_str(machine_name).as_ptr(),
					predef_hkey.as_ptr(),
					&mut hkey.0,
				)
			},
		).map(|_| RegCloseKeyGuard::new(hkey))
	}

	/// [`RegCopyTree`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcopytreew)
	/// method.
	fn RegCopyTree(&self, sub_key: Option<&str>, dest: &HKEY) -> SysResult<()> {
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegCopyTreeW(
					self.as_ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
					dest.as_ptr(),
				)
			},
		)
	}

	/// [`RegCreateKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeyexw)
	/// method.
	#[must_use]
	fn RegCreateKeyEx(&self,
		sub_key: &str,
		class: Option<&str>,
		options: co::REG_OPTION,
		access_rights: co::KEY,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
	) -> SysResult<(RegCloseKeyGuard, co::REG_DISPOSITION)>
	{
		let mut hkey = HKEY::NULL;
		let mut disposition = co::REG_DISPOSITION::NoValue;

		error_to_sysresult(
			unsafe {
				kernel::ffi::RegCreateKeyExW(
					self.as_ptr(),
					WString::from_str(sub_key).as_ptr(),
					0,
					WString::from_opt_str(class).as_ptr(),
					options.0,
					access_rights.0,
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
					&mut hkey.0,
					&mut disposition.0,
				)
			},
		).map(|_| (RegCloseKeyGuard::new(hkey), disposition))
	}

	/// [`RegCreateKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeytransactedw)
	/// method.
	#[must_use]
	fn RegCreateKeyTransacted(&self,
		sub_key: &str,
		class: Option<&str>,
		options: co::REG_OPTION,
		access_rights: co::KEY,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
		htransaction: &HTRANSACTION,
	) -> SysResult<(RegCloseKeyGuard, co::REG_DISPOSITION)>
	{
		let mut hkey = HKEY::NULL;
		let mut disposition = co::REG_DISPOSITION::NoValue;

		error_to_sysresult(
			unsafe {
				kernel::ffi::RegCreateKeyTransactedW(
					self.as_ptr(),
					WString::from_str(sub_key).as_ptr(),
					0,
					WString::from_opt_str(class).as_ptr(),
					options.0,
					access_rights.0,
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
					&mut hkey.0,
					&mut disposition.0,
					htransaction.as_ptr(),
					std::ptr::null_mut(),
				)
			},
		).map(|_| (RegCloseKeyGuard::new(hkey), disposition))
	}

	/// [`RegDeleteKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeyw)
	/// method.
	fn RegDeleteKey(&self, sub_key: &str) -> SysResult<()> {
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegDeleteKeyW(
					self.as_ptr(),
					WString::from_str(sub_key).as_ptr(),
				)
			},
		)
	}

	/// [`RegDeleteKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeyexw)
	/// method.
	///
	/// # Panics
	///
	/// Panics if `platform_view` is different from
	/// [`co::KEY::WOW64_32KEY`](crate::co::KEY::WOW64_32KEY) and
	/// [`co::KEY::WOW64_64KEY`](crate::co::KEY::WOW64_64KEY).
	fn RegDeleteKeyEx(&self,
		sub_key: &str, platform_view: co::KEY) -> SysResult<()>
	{
		if platform_view != co::KEY::WOW64_32KEY
			&& platform_view != co::KEY::WOW64_64KEY
		{
			panic!("Platform view must be co::KEY::WOW64_32KEY or co::KEY::WOW64_64KEY");
		}

		error_to_sysresult(
			unsafe {
				kernel::ffi::RegDeleteKeyExW(
					self.as_ptr(),
					WString::from_str(sub_key).as_ptr(),
					platform_view.0,
					0,
				)
			},
		)
	}

	/// [`RegDeleteKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeytransactedw)
	/// method.
	fn RegDeleteKeyTransacted(&self,
		sub_key: &str,
		access_rights: co::KEY,
		htransaction: &HTRANSACTION,
	) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegDeleteKeyTransactedW(
					self.as_ptr(),
					WString::from_str(sub_key).as_ptr(),
					access_rights.0,
					0,
					htransaction.as_ptr(),
					std::ptr::null_mut(),
				)
			},
		)
	}

	/// [`RegDeleteTree`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletetreew)
	/// method.
	fn RegDeleteTree(&self, sub_key: Option<&str>) -> SysResult<()> {
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegDeleteTreeW(
					self.as_ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
				)
			},
		)
	}

	/// [`RegDeleteValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletevaluew)
	/// method.
	fn RegDeleteValue(&self, value_name: Option<&str>) -> SysResult<()> {
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegDeleteValueW(
					self.as_ptr(),
					WString::from_opt_str(value_name).as_ptr(),
				)
			},
		)
	}

	/// [`RegDisablePredefinedCache`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdisablepredefinedcache)
	/// static method.
	fn RegDisablePredefinedCache() -> SysResult<()> {
		error_to_sysresult(unsafe { kernel::ffi::RegDisablePredefinedCache() })
	}

	/// [`RegDisablePredefinedCacheEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdisablepredefinedcacheex)
	/// static method.
	fn RegDisablePredefinedCacheEx() -> SysResult<()> {
		error_to_sysresult(unsafe { kernel::ffi::RegDisablePredefinedCacheEx() })
	}

	/// [`RegDisableReflectionKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdisablereflectionkey)
	/// method.
	fn RegDisableReflectionKey(&self) -> SysResult<()> {
		error_to_sysresult(
			unsafe { kernel::ffi::RegDisableReflectionKey(self.as_ptr()) },
		)
	}

	/// [`RegEnableReflectionKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenablereflectionkey)
	/// method.
	fn RegEnableReflectionKey(&self) -> SysResult<()> {
		error_to_sysresult(
			unsafe { kernel::ffi::RegEnableReflectionKey(self.as_ptr()) },
		)
	}

	/// Returns an iterator over the names of the keys, which calls
	/// [`RegEnumKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumkeyexw)
	/// repeatedly.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HKEY};
	///
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// for key_name in hkey.RegEnumKeyEx()? {
	///     let key_name = key_name?;
	///     println!("{}", key_name);
	/// }
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn RegEnumKeyEx(&self,
	) -> SysResult<Box<dyn Iterator<Item = SysResult<String>> + '_>>
	{
		Ok(Box::new(EnumKeyIter::new(self)?))
	}

	/// Returns an iterator of the names and types of the values, which calls
	/// [`RegEnumValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumvaluew)
	/// repeatedly.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HKEY, SysResult};
	///
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel\\Appearance"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// for value_and_type in hkey.RegEnumValue()? {
	///     let (value, reg_type) = value_and_type?;
	///     println!("{}, {}", value, reg_type);
	/// }
	///
	/// // Collecting into a Vec
	/// let values_and_types: Vec<(String, co::REG)> =
	///     hkey.RegEnumValue()?
	///         .collect::<SysResult<Vec<_>>>()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn RegEnumValue(&self,
	) -> SysResult<Box<dyn Iterator<Item = SysResult<(String, co::REG)>> + '_>>
	{
		Ok(Box::new(EnumValueIter::new(self)?))
	}

	/// [`RegFlushKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regflushkey)
	/// method.
	fn RegFlushKey(&self) -> SysResult<()> {
		error_to_sysresult(unsafe { kernel::ffi::RegFlushKey(self.as_ptr()) })
	}

	/// [`RegGetValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-reggetvaluew)
	/// method.
	///
	/// Note that this method validates some race conditions, returning
	/// [`co::ERROR::TRANSACTION_REQUEST_NOT_VALID`](crate::co::ERROR::TRANSACTION_REQUEST_NOT_VALID)
	/// and [`co::ERROR::INVALID_DATA`](crate::co::ERROR::INVALID_DATA).
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{ExpandEnvironmentStrings, HKEY, RegistryValue};
	///
	/// let val = HKEY::CURRENT_USER.RegGetValue(
	///     Some("Control Panel\\Mouse"),
	///     Some("Beep"),
	/// )?;
	///
	/// match val {
	///     RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///     RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///     RegistryValue::Sz(s) => println!("String: {}", s),
	///     RegistryValue::ExpandSz(s) => {
	///         println!("Env string: {}", ExpandEnvironmentStrings(&s)?);
	///     },
	///     RegistryValue::MultiSz(strs) => {
	///        println!("Multi string:");
	///        for s in strs.iter() {
	///            print!("[{}] ", s);
	///        }
	///        println!("");
	///     },
	///     RegistryValue::Binary(bin) => {
	///         println!("Binary:");
	///         for b in bin.iter() {
	///             print!("{:02x} ", b);
	///         }
	///         println!("");
	///     },
	///     RegistryValue::None => println!("No value"),
	/// }
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	fn RegGetValue(&self,
		sub_key: Option<&str>,
		value_name: Option<&str>,
	) -> SysResult<RegistryValue>
	{
		let sub_key_w = WString::from_opt_str(sub_key);
		let value_name_w = WString::from_opt_str(value_name);
		let mut raw_data_type1 = u32::default();
		let mut data_len1 = u32::default();

		// Query data type and length.
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegGetValueW(
					self.as_ptr(),
					sub_key_w.as_ptr(),
					value_name_w.as_ptr(),
					(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
					&mut raw_data_type1,
					std::ptr::null_mut(),
					&mut data_len1,
				)
			},
		)?;

		// Alloc the receiving block.
		let mut buf: Vec<u8> = vec![0x00; data_len1 as _];

		let mut raw_data_type2 = u32::default();
		let mut data_len2 = data_len1;

		// Retrieve the value content.
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegGetValueW(
					self.as_ptr(),
					sub_key_w.as_ptr(),
					value_name_w.as_ptr(),
					(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
					&mut raw_data_type2,
					buf.as_mut_ptr() as _,
					&mut data_len2,
				)
			},
		)?;

		validate_retrieved_reg_val(
			co::REG(raw_data_type1), data_len1,
			co::REG(raw_data_type2), data_len2, buf)
	}

	/// [`RegLoadKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regloadkeyw)
	/// method.
	fn RegLoadKey(&self,
		sub_key: Option<&str>, file_path: &str) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegLoadKeyW(
					self.as_ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
					WString::from_str(file_path).as_ptr(),
				)
			},
		)
	}

	/// [`RegOpenCurrentUser`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopencurrentuser)
	/// static method.
	#[must_use]
	fn RegOpenCurrentUser(
		access_rights: co::KEY) -> SysResult<RegCloseKeyGuard>
	{
		let mut hkey = HKEY::NULL;
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegOpenCurrentUser(
					access_rights.0,
					&mut hkey.0,
				)
			},
		).map(|_| RegCloseKeyGuard::new(hkey))
	}

	/// [`RegOpenKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw)
	/// method.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HKEY};
	///
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel\\Mouse"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn RegOpenKeyEx(&self,
		sub_key: Option<&str>,
		options: co::REG_OPTION,
		access_rights: co::KEY,
	) -> SysResult<RegCloseKeyGuard>
	{
		let mut hkey = HKEY::NULL;
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegOpenKeyExW(
					self.as_ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
					options.0,
					access_rights.0,
					&mut hkey.0,
				)
			},
		).map(|_| RegCloseKeyGuard::new(hkey))
	}

	/// [`RegOpenKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeytransactedw)
	/// method.
	#[must_use]
	fn RegOpenKeyTransacted(&self,
		sub_key: &str,
		options: co::REG_OPTION,
		access_rights: co::KEY,
		htransaction: &HTRANSACTION,
	) -> SysResult<RegCloseKeyGuard>
	{
		let mut hkey = HKEY::NULL;
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegOpenKeyTransactedW(
					self.as_ptr(),
					WString::from_str(sub_key).as_ptr(),
					options.0,
					access_rights.0,
					&mut hkey.0,
					htransaction.as_ptr(),
					std::ptr::null_mut(),
				)
			},
		).map(|_| RegCloseKeyGuard::new(hkey))
	}

	/// [`RegQueryInfoKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryinfokeyw)
	/// method.
	fn RegQueryInfoKey(&self,
		mut class: Option<&mut WString>,
		num_sub_keys: Option<&mut u32>,
		max_sub_key_name_len: Option<&mut u32>,
		max_class_len: Option<&mut u32>,
		num_values: Option<&mut u32>,
		max_value_name_len: Option<&mut u32>,
		max_value_len: Option<&mut u32>,
		security_descr_len: Option<&mut u32>,
		last_write_time: Option<&mut FILETIME>,
	) -> SysResult<()>
	{
		const BLOCK: usize = 32; // arbitrary

		let (mut class_ptr, mut class_len) = match &mut class {
			Some(class) => {
				if class.buf_len() < BLOCK {
					unsafe { class.buf_realloc(BLOCK); } // make it at least BLOCK_SZ
				}
				(unsafe { class.as_mut_ptr() }, class.buf_len() as u32)
			},
			None => (std::ptr::null_mut(), 0),
		};

		let num_sub_keys = num_sub_keys.map_or(std::ptr::null_mut(), |re| re as _);
		let max_sub_key_name_len = max_sub_key_name_len.map_or(std::ptr::null_mut(), |re| re as _);
		let max_class_len = max_class_len.map_or(std::ptr::null_mut(), |re| re as _);
		let num_values = num_values.map_or(std::ptr::null_mut(), |re| re as _);
		let max_value_name_len = max_value_name_len.map_or(std::ptr::null_mut(), |re| re as _);
		let max_value_len = max_value_len.map_or(std::ptr::null_mut(), |re| re as _);
		let security_descr_len = security_descr_len.map_or(std::ptr::null_mut(), |re| re as _);
		let last_write_time = last_write_time.map_or(std::ptr::null_mut(), |re| re as *mut _ as _);

		loop { // until class is large enough
			match co::ERROR(
				unsafe {
					kernel::ffi::RegQueryInfoKeyW(
						self.as_ptr(),
						class_ptr,
						&mut class_len,
						std::ptr::null_mut(),
						num_sub_keys,
						max_sub_key_name_len,
						max_class_len,
						num_values,
						max_value_name_len,
						max_value_len,
						security_descr_len,
						last_write_time,
					)
				} as _,
			) {
				co::ERROR::MORE_DATA => match &mut class {
					Some(class) => {
						unsafe { class.buf_realloc(class.buf_len() + BLOCK); }
						class_ptr = unsafe { class.as_mut_ptr() };
						class_len = class.buf_len() as _;
					},
					None => return Err(co::ERROR::MORE_DATA),
				},
				co::ERROR::SUCCESS => return Ok(()),
				err => return Err(err),
			}
		}
	}

	/// [`RegQueryMultipleValues`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regquerymultiplevaluesw)
	/// method.
	///
	/// This method is a multi-value version of
	/// [`HKEY::RegQueryValueEx`](crate::prelude::kernel_Hkey::RegQueryValueEx).
	///
	/// Note that this method validates some race conditions, returning
	/// [`co::ERROR::TRANSACTION_REQUEST_NOT_VALID`](crate::co::ERROR::TRANSACTION_REQUEST_NOT_VALID).
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, ExpandEnvironmentStrings, HKEY, RegistryValue};
	///
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel\\Desktop"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// for val in hkey.RegQueryMultipleValues(&["DpiScalingVer", "WallPaper"])? {
	///     match val {
	///         RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///         RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///         RegistryValue::Sz(s) => println!("String: {}", s),
	///         RegistryValue::ExpandSz(s) => {
	///             println!("Env string: {}", ExpandEnvironmentStrings(&s)?);
	///         },
	///         RegistryValue::MultiSz(strs) => {
	///            println!("Multi string:");
	///            for s in strs.iter() {
	///                print!("[{}] ", s);
	///            }
	///            println!("");
	///         },
	///         RegistryValue::Binary(bin) => {
	///             println!("Binary:");
	///             for b in bin.iter() {
	///                 print!("{:02x} ", b);
	///             }
	///             println!("");
	///         },
	///         RegistryValue::None => println!("No value"),
	///     }
	/// }
	///
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn RegQueryMultipleValues(&self,
		value_names: &[impl AsRef<str>]) -> SysResult<Vec<RegistryValue>>
	{
		let mut valents1 = vec![VALENT::default(); value_names.len()];
		let value_names_w = value_names.iter()
			.zip(valents1.iter_mut())
			.map(|(value_name, valent)| {
				let value_name_w = WString::from_str(value_name.as_ref());
				valent.ve_valuename = unsafe { value_name_w.as_ptr() as _ };
				value_name_w
			})
			.collect::<Vec<_>>();
		let mut data_len1 = u32::default();

		// Query data types and lenghts.
		match co::ERROR(
			unsafe {
				kernel::ffi::RegQueryMultipleValuesW(
					self.as_ptr(),
					valents1.as_mut_ptr() as _,
					value_names.len() as _,
					std::ptr::null_mut(),
					&mut data_len1,
				)
			} as _,
		) {
			co::ERROR::MORE_DATA => {},
			err => return Err(err),
		}

		// Alloc the receiving block.
		let mut buf: Vec<u8> = vec![0x00; data_len1 as _];

		let mut valents2 = value_names_w.iter()
			.map(|value_name_w| {
				let mut valent = VALENT::default();
				valent.ve_valuename = unsafe { value_name_w.as_ptr() as _ };
				valent
			})
			.collect::<Vec<_>>();
		let mut data_len2 = data_len1;

		// Retrieve the values content.
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegQueryMultipleValuesW(
					self.as_ptr(),
					valents2.as_mut_ptr() as _,
					value_names.len() as _,
					buf.as_mut_ptr() as _,
					&mut data_len2,
				)
			},
		)?;

		if data_len1 != data_len2 {
			// Race condition: someone modified the data content in between our calls.
			return Err(co::ERROR::TRANSACTION_REQUEST_NOT_VALID);
		}

		Ok(
			valents2.iter() // first VALENT array is not filled with len/type values
				.map(|v2| unsafe {
					RegistryValue::from_raw(
						v2.buf_projection(&buf).to_vec(),
						v2.ve_type,
					)
				})
				.collect::<Vec<_>>()
		)
	}

	/// [`RegQueryReflectionKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryreflectionkey)
	/// method.
	#[must_use]
	fn RegQueryReflectionKey(&self) -> SysResult<bool> {
		let mut is_disabled: BOOL = 0;
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegQueryReflectionKey(self.as_ptr(), &mut is_disabled)
			},
		).map(|_| is_disabled != 0)
	}

	/// [`RegQueryValueEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryvalueexw)
	/// method.
	///
	/// This method is a single-value version of
	/// [`HKEY::RegQueryMultipleValues`](crate::prelude::kernel_Hkey::RegQueryMultipleValues).
	///
	/// Note that this method validates some race conditions, returning
	/// [`co::ERROR::TRANSACTION_REQUEST_NOT_VALID`](crate::co::ERROR::TRANSACTION_REQUEST_NOT_VALID)
	/// and [`co::ERROR::INVALID_DATA`](crate::co::ERROR::INVALID_DATA).
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, ExpandEnvironmentStrings, HKEY, RegistryValue};
	///
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel\\Mouse"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// let val = hkey.RegQueryValueEx(Some("Beep"))?;
	///
	/// match val {
	///     RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///     RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///     RegistryValue::Sz(s) => println!("String: {}", s),
	///     RegistryValue::ExpandSz(s) => {
	///         println!("Env string: {}", ExpandEnvironmentStrings(&s)?);
	///     },
	///     RegistryValue::MultiSz(strs) => {
	///        println!("Multi string:");
	///        for s in strs.iter() {
	///            print!("[{}] ", s);
	///        }
	///        println!("");
	///     },
	///     RegistryValue::Binary(bin) => {
	///         println!("Binary:");
	///         for b in bin.iter() {
	///             print!("{:02x} ", b);
	///         }
	///         println!("");
	///     },
	///     RegistryValue::None => println!("No value"),
	/// }
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn RegQueryValueEx(&self,
		value_name: Option<&str>) -> SysResult<RegistryValue>
	{
		let value_name_w = WString::from_opt_str(value_name);
		let mut raw_data_type1 = u32::default();
		let mut data_len1 = u32::default();

		// Query data type and length.
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegQueryValueExW(
					self.as_ptr(),
					value_name_w.as_ptr(),
					std::ptr::null_mut(),
					&mut raw_data_type1,
					std::ptr::null_mut(),
					&mut data_len1,
				)
			},
		)?;

		// Alloc the receiving block.
		let mut buf: Vec<u8> = vec![0x00; data_len1 as _];

		let mut raw_data_type2 = u32::default();
		let mut data_len2 = data_len1;

		// Retrieve the value content.
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegQueryValueExW(
					self.as_ptr(),
					value_name_w.as_ptr(),
					std::ptr::null_mut(),
					&mut raw_data_type2,
					buf.as_mut_ptr() as _,
					&mut data_len2,
				)
			},
		)?;

		validate_retrieved_reg_val(
			co::REG(raw_data_type1), data_len1,
			co::REG(raw_data_type2), data_len2, buf)
	}

	/// [`RegRenameKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regrenamekey)
	/// method.
	fn RegRenameKey(&self,
		sub_key_name: &str, new_key_name: &str) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegRenameKey(
					self.as_ptr(),
					WString::from_str(sub_key_name).as_ptr(),
					WString::from_str(new_key_name).as_ptr(),
				)
			},
		)
	}

	/// [`RegReplaceKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regreplacekeyw)
	/// method.
	fn RegReplaceKey(&self,
		sub_key: Option<&str>,
		new_src_file: &str,
		old_file_backup: &str,
	) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegReplaceKeyW(
					self.as_ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
					WString::from_str(new_src_file).as_ptr(),
					WString::from_str(old_file_backup).as_ptr(),
				)
			},
		)
	}

	/// [`RegRestoreKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regrestorekeyw)
	/// method.
	fn RegRestoreKey(&self,
		file_path: &str, flags: co::REG_RESTORE) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegRestoreKeyW(
					self.as_ptr(),
					WString::from_str(file_path).as_ptr(),
					flags.0,
				)
			},
		)
	}

	/// [`RegSaveKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsavekeyw)
	/// method.
	fn RegSaveKey(&self,
		dest_file_path: &str,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
	) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegSaveKeyW(
					self.as_ptr(),
					WString::from_str(dest_file_path).as_ptr(),
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
				)
			},
		)
	}

	/// [`RegSaveKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsavekeyexw)
	/// method.
	fn RegSaveKeyEx(&self,
		dest_file_path: &str,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
		flags: co::REG_SAVE,
	) -> SysResult<()>
	{
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegSaveKeyExW(
					self.as_ptr(),
					WString::from_str(dest_file_path).as_ptr(),
					security_attributes.map_or(std::ptr::null_mut(), |sa| sa as *const _ as _),
					flags.0,
				)
			},
		)
	}

	/// [`RegSetKeyValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetkeyvaluew)
	/// method.
	///
	/// If the value doesn't exist, if will be created. If new type is different
	/// from current type, new type will take over.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HKEY, RegistryValue, WString};
	///
	/// HKEY::CURRENT_USER.RegSetKeyValue(
	///     Some("Software\\My Company"),
	///     Some("Color"),
	///     RegistryValue::Sz("blue".to_owned()),
	/// )?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn RegSetKeyValue(&self,
		sub_key: Option<&str>,
		value_name: Option<&str>,
		data: RegistryValue,
	) -> SysResult<()>
	{
		let mut str_buf = WString::default();
		let (data_ptr, data_len) = data.as_ptr_with_len(&mut str_buf);

		error_to_sysresult(
			unsafe {
				kernel::ffi::RegSetKeyValueW(
					self.as_ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
					WString::from_opt_str(value_name).as_ptr(),
					data.reg_type().0,
					data_ptr,
					data_len,
				)
			},
		)
	}

	/// [`RegSetValueEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetvalueexw)
	/// method.
	///
	/// If the value doesn't exist, if will be created. If new type is different
	/// from current type, new type will prevail.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HKEY, RegistryValue, WString};
	///
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Console\\Git Bash"),
	///     co::REG_OPTION::default(),
	///     co::KEY::ALL_ACCESS,
	/// )?;
	///
	/// hkey.RegSetValueEx(
	///     Some("Color"),
	///     RegistryValue::Sz("blue".to_owned()),
	/// )?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn RegSetValueEx(&self,
		value_name: Option<&str>, data: RegistryValue) -> SysResult<()>
	{
		let mut str_buf = WString::default();
		let (data_ptr, data_len) = data.as_ptr_with_len(&mut str_buf);

		error_to_sysresult(
			unsafe {
				kernel::ffi::RegSetValueExW(
					self.as_ptr(),
					WString::from_opt_str(value_name).as_ptr(),
					0,
					data.reg_type().0,
					data_ptr as _,
					data_len,
				)
			},
		)
	}

	/// [`RegUnLoadKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regunloadkeyw)
	/// method.
	fn RegUnLoadKey(&self, sub_key: Option<&str>) -> SysResult<()> {
		error_to_sysresult(
			unsafe {
				kernel::ffi::RegUnLoadKeyW(
					self.as_ptr(),
					WString::from_opt_str(sub_key).as_ptr(),
				)
			},
		)
	}
}

//------------------------------------------------------------------------------

fn validate_retrieved_reg_val(
	data_type1: co::REG,
	data_len1: u32,
	data_type2: co::REG,
	mut data_len2: u32,
	buf: Vec<u8>,
) -> SysResult<RegistryValue>
{
	if data_type1 != data_type2 {
		// Race condition: someone modified the data type in between our calls.
		return Err(co::ERROR::TRANSACTION_REQUEST_NOT_VALID);
	}

	if data_type1 == co::REG::SZ || data_type1 == co::REG::MULTI_SZ {
		data_len2 += 2; // also count wchar terminating null
	}

	if data_len1 != data_len2 {
		// Race condition: someone modified the data content in between our calls.
		return Err(co::ERROR::TRANSACTION_REQUEST_NOT_VALID);
	}

	if data_type1 == co::REG::DWORD && data_len1 != 4
		|| data_type1 == co::REG::QWORD && data_len1 != 8
	{
		// Data length makes no sense, possibly corrupted.
		return Err(co::ERROR::INVALID_DATA);
	}

	if data_type1 == co::REG::SZ {
		if data_len1 == 0 // empty data
			|| data_len1 % 2 != 0 // odd number of bytes
			|| buf[data_len1 as usize - 2] != 0 // terminating null
			|| buf[data_len1 as usize - 1] != 0
		{
			// Bad string.
			return Err(co::ERROR::INVALID_DATA);
		}
	}

	Ok(unsafe { RegistryValue::from_raw(buf, data_type1) })
}

//------------------------------------------------------------------------------

struct EnumKeyIter<'a, H>
	where H: kernel_Hkey,
{
	hkey: &'a H,
	count: u32,
	current: u32,
	name_buffer: WString,
}

impl<'a, H> Iterator for EnumKeyIter<'a, H>
	where H: kernel_Hkey,
{
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let mut len_buffer = self.name_buffer.buf_len() as u32;
		match co::ERROR(
			unsafe {
				kernel::ffi::RegEnumKeyExW(
					self.hkey.as_ptr(),
					self.current,
					self.name_buffer.as_mut_ptr(),
					&mut len_buffer,
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
				)
			} as _,
		) {
			co::ERROR::SUCCESS => {
				self.current += 1;
				Some(Ok(self.name_buffer.to_string()))
			},
			e => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
		}
	}
}

impl<'a, H> EnumKeyIter<'a, H>
	where H: kernel_Hkey,
{
	fn new(hkey: &'a H) -> SysResult<Self> {
		let mut num_keys = u32::default();
		let mut max_key_name_len = u32::default();
		hkey.RegQueryInfoKey(
			None, Some(&mut num_keys), Some(&mut max_key_name_len),
			None, None, None, None, None, None)?;

		Ok(Self {
			hkey,
			count: num_keys,
			current: 0,
			name_buffer: WString::new_alloc_buf(max_key_name_len as usize + 1),
		})
	}
}

//------------------------------------------------------------------------------

struct EnumValueIter<'a, H>
	where H: kernel_Hkey,
{
	hkey: &'a H,
	count: u32,
	current: u32,
	name_buffer: WString,
}

impl<'a, H> Iterator for EnumValueIter<'a, H>
	where H: kernel_Hkey,
{
	type Item = SysResult<(String, co::REG)>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let mut raw_data_type = u32::default();
		let mut len_buffer = self.name_buffer.buf_len() as u32;
		match co::ERROR(
			unsafe {
				kernel::ffi::RegEnumValueW(
					self.hkey.as_ptr(),
					self.current,
					self.name_buffer.as_mut_ptr(),
					&mut len_buffer,
					std::ptr::null_mut(),
					&mut raw_data_type,
					std::ptr::null_mut(),
					std::ptr::null_mut(),
				)
			} as _,
		) {
			co::ERROR::SUCCESS => {
				self.current += 1;
				Some(Ok((self.name_buffer.to_string(), co::REG(raw_data_type))))
			},
			e => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
		}
	}
}

impl<'a, H> EnumValueIter<'a, H>
	where H: kernel_Hkey,
{
	fn new(hkey: &'a H) -> SysResult<Self> {
		let mut num_vals = u32::default();
		let mut max_val_name_len = u32::default();
		hkey.RegQueryInfoKey(
			None, None, None, None, Some(&mut num_vals), Some(&mut max_val_name_len),
			None, None, None)?;

		Ok(Self {
			hkey,
			count: num_vals,
			current: 0,
			name_buffer: WString::new_alloc_buf(max_val_name_len as usize + 1),
		})
	}
}
