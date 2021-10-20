#![allow(non_snake_case)]

use std::convert::TryInto;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::RegistryValue;
use crate::ffi::advapi32;
use crate::structs::FILETIME;
use crate::various::WString;

pub_struct_handle! {
	/// Handle to a
	/// [registry key](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hkey).
	///
	/// This handle also exposes several
	/// [predefined registry keys](https://docs.microsoft.com/en-us/windows/win32/sysinfo/predefined-keys),
	/// like `HKEY::CURRENT_USER`, which are always open and ready to be used.
	/// Usually, they are the starting point to open a registry key.
	HKEY
}

macro_rules! predef_key {
	($name:ident, $val:expr) => {
		/// Predefined registry key, always open.
		pub const $name: Self = Self { ptr: $val as *mut _ };
	};
}

impl HKEY {
	predef_key!(CLASSES_ROOT, 0x8000_0000);
	predef_key!(CURRENT_USER, 0x8000_0001);
	predef_key!(LOCAL_MACHINE, 0x8000_0002);
	predef_key!(USERS, 0x8000_0003);
	predef_key!(PERFORMANCE_DATA, 0x8000_0004);
	predef_key!(PERFORMANCE_TEXT, 0x8000_0050);
	predef_key!(PERFORMANCE_NLSTEXT, 0x8000_0060);
	predef_key!(CURRENT_CONFIG, 0x8000_0005);
	predef_key!(DYN_DATA, 0x8000_0006);
	predef_key!(CURRENT_USER_LOCAL_SETTINGS, 0x8000_0007);

	/// [`RegCloseKey`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey)
	/// method.
	pub fn CloseKey(self) -> WinResult<()> {
		match co::ERROR(unsafe { advapi32::RegCloseKey(self.ptr) as _ }) {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
	}

	/// This method calls
	/// [`RegEnumKeyEx`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumkeyexw)
	/// method repeatedly to retrieve all key names.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, HKEY};
	///
	/// let hkey = HKEY::CURRENT_USER.OpenKeyEx(
	///     "Control Panel",
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// let key_names = hkey.EnumKeyEx()?;
	/// for key_name in key_names.iter() {
	///     println!("{}", key_name);
	/// }
	///
	/// hkey.CloseKey()?;
	/// ```
	pub fn EnumKeyEx(self) -> WinResult<Vec<String>> {
		let mut num_keys = u32::default();
		let mut max_key_name_len = u32::default();
		self.QueryInfoKey(
			None, Some(&mut num_keys), Some(&mut max_key_name_len),
			None, None, None, None, None, None)?;

		let mut names_vec = Vec::with_capacity(num_keys as _);
		let mut name_buf = WString::new_alloc_buffer(max_key_name_len as usize + 1);
		let mut len_buf;

		for index in 0..num_keys {
			len_buf = name_buf.buffer_size() as _;

			let err = co::ERROR(
				unsafe {
					advapi32::RegEnumKeyExW(
						self.ptr,
						index,
						name_buf.as_mut_ptr(),
						&mut len_buf,
						std::ptr::null_mut(),
						std::ptr::null_mut(),
						std::ptr::null_mut(),
						std::ptr::null_mut(),
					)
				} as _,
			);

			if err != co::ERROR::SUCCESS {
				return Err(err);
			}
			names_vec.push(name_buf.to_string());
		}

		names_vec.sort_by(|a, b| a.to_uppercase().cmp(&b.to_uppercase()));
		Ok(names_vec)
	}

	/// This method calls
	/// [`RegEnumValue`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumvaluew)
	/// method repeatedly to retrieve all value names and types.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, HKEY};
	///
	/// let hkey = HKEY::CURRENT_USER.OpenKeyEx(
	///     "Control Panel\\Appearance",
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// let values_and_types = hkey.EnumValue()?;
	/// for (value, reg_type) in values_and_types.iter() {
	///     println!("{}, {}", value, reg_type);
	/// }
	///
	/// hkey.CloseKey()?;
	/// ```
	pub fn EnumValue(self) -> WinResult<Vec<(String, co::REG)>> {
		let mut num_vals = u32::default();
		let mut max_val_name_len = u32::default();
		self.QueryInfoKey(
			None, None, None, None, Some(&mut num_vals), Some(&mut max_val_name_len),
			None, None, None)?;

		let mut names_types_vec = Vec::with_capacity(num_vals as _);
		let mut name_buf = WString::new_alloc_buffer(max_val_name_len as usize + 1);
		let mut raw_data_type = u32::default();

		for index in 0..num_vals {
			let mut len_buf = name_buf.buffer_size() as _;

			let err = co::ERROR(
				unsafe {
					advapi32::RegEnumValueW(
						self.ptr,
						index,
						name_buf.as_mut_ptr(),
						&mut len_buf,
						std::ptr::null_mut(),
						&mut raw_data_type,
						std::ptr::null_mut(),
						std::ptr::null_mut(),
					)
				} as _
			);

			if err != co::ERROR::SUCCESS {
				return Err(err);
			}
			names_types_vec.push((name_buf.to_string(), co::REG(raw_data_type)));
		}

		names_types_vec.sort_by(|a, b| a.0.to_uppercase().cmp(&b.0.to_uppercase()));
		Ok(names_types_vec)
	}

	/// [`RegGetValue`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-reggetvaluew)
	/// method.
	///
	/// The data type will be automatically queried with a first call to
	/// `RegGetValue`.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{HKEY, RegistryValue};
	///
	/// let val = HKEY::CURRENT_USER.GetValue(
	///     "Control Panel\\Mouse",
	///     "Beep",
	/// )?;
	///
	/// match val {
	///     RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///     RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///     RegistryValue::Sz(str) => println!("String: {}", str),
	///     RegistryValue::Binary(bin) => {
	///         println!("Binary:");
	///         for b in bin.iter() {
	///             print!("{:02x} ", b);
	///         }
	///         println!("");
	///     },
	///     RegistryValue::None => println!("No value"),
	/// }
	/// ```
	pub fn GetValue(self,
		sub_key: &str, value: &str) -> WinResult<RegistryValue>
	{
		let sub_key_w = WString::from_str(sub_key);
		let value_w = WString::from_str(value);
		let mut raw_data_type = u32::default();
		let mut data_len = u32::default();

		// Query data type and length.
		match co::ERROR(
			unsafe {
				advapi32::RegGetValueW(
					self.ptr,
					sub_key_w.as_ptr(),
					value_w.as_ptr(),
					(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
					&mut raw_data_type,
					std::ptr::null_mut(),
					&mut data_len,
				)
			} as _
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		// Retrieve value.
		let mut buf: Vec<u8> = vec![0x00; data_len as _];

		match co::ERROR(
			unsafe {
				advapi32::RegGetValueW(
					self.ptr,
					sub_key_w.as_ptr(),
					value_w.as_ptr(),
					(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
					std::ptr::null_mut(),
					buf.as_mut_ptr() as _,
					&mut data_len,
				)
			} as _
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		// Return the value according to type.
		match co::REG(raw_data_type) {
			co::REG::NONE => Ok(RegistryValue::None),
			co::REG::DWORD => Ok(RegistryValue::Dword(
				u32::from_ne_bytes(buf.try_into().unwrap())),
			),
			co::REG::QWORD => Ok(RegistryValue::Qword(
				u64::from_ne_bytes(buf.try_into().unwrap())),
			),
			co::REG::SZ => {
				let (_, vec16, _) = unsafe { buf.align_to::<u16>() };
				Ok(RegistryValue::Sz(WString::from_wchars_slice(&vec16)))
			},
			co::REG::BINARY => Ok(RegistryValue::Binary(buf)),
			_ => Ok(RegistryValue::None), // other types not implemented yet
		}
	}

	/// [`RegOpenKeyEx`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HKEY::CloseKey`](crate::HKEY::CloseKey) call.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, HKEY};
	///
	/// let hkey = HKEY::CURRENT_USER.OpenKeyEx(
	///     "Control Panel\\Mouse",
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// hkey.CloseKey()?;
	/// ```
	pub fn OpenKeyEx(self, sub_key: &str,
		options: co::REG_OPTION, access_rights: co::KEY) -> WinResult<HKEY>
	{
		let mut hKey = Self::NULL;

		match co::ERROR(
			unsafe {
				advapi32::RegOpenKeyExW(
					self.ptr,
					WString::from_str(sub_key).as_ptr(),
					options.0,
					access_rights.0,
					&mut hKey.ptr,
				)
			} as _
		) {
			co::ERROR::SUCCESS => Ok(hKey),
			err => Err(err),
		}
	}

	/// [`RegQueryInfoKey`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryinfokeyw)
	/// method.
	pub fn QueryInfoKey(self,
		mut class: Option<&mut WString>,
		num_sub_keys: Option<&mut u32>,
		max_sub_key_name_len: Option<&mut u32>,
		max_class_len: Option<&mut u32>,
		num_values: Option<&mut u32>,
		max_value_name_len: Option<&mut u32>,
		max_value_len: Option<&mut u32>,
		security_descr_len: Option<&mut u32>,
		last_write_time: Option<&mut FILETIME>) -> WinResult<()>
	{
		const BLOCK: usize = 32; // arbitrary

		let (mut class_ptr, mut class_len) = match &mut class {
			Some(class) => {
				if class.buffer_size() < BLOCK {
					class.realloc_buffer(BLOCK); // make it at least BLOCK_SZ
				}
				(unsafe { class.as_mut_ptr() }, class.buffer_size() as u32)
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
					advapi32::RegQueryInfoKeyW(
						self.ptr,
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
				} as _
			) {
				co::ERROR::MORE_DATA => match &mut class {
					Some(class) => {
						class.realloc_buffer(class.buffer_size() + BLOCK);
						class_ptr = unsafe { class.as_mut_ptr() };
						class_len = class.buffer_size() as _;
					},
					None => return Err(co::ERROR::MORE_DATA),
				},
				co::ERROR::SUCCESS => return Ok(()),
				err => return Err(err),
			}
		}
	}

	/// [`RegQueryValueEx`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryvalueexw)
	/// method.
	///
	/// The data type will be automatically queried with a first call to
	/// `RegQueryValueEx`.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, HKEY, RegistryValue};
	///
	/// let hkey = HKEY::CURRENT_USER.OpenKeyEx(
	///     "Control Panel\\Mouse",
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// let val = hkey.QueryValueEx("Beep")?;
	///
	/// match val {
	///     RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///     RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///     RegistryValue::Sz(str) => println!("String: {}", str),
	///     RegistryValue::Binary(bin) => {
	///         println!("Binary:");
	///         for b in bin.iter() {
	///             print!("{:02x} ", b);
	///         }
	///         println!("");
	///     },
	///     RegistryValue::None => println!("No value"),
	/// }
	///
	/// hkey.CloseKey()?;
	/// ```
	pub fn QueryValueEx(self, value: &str) -> WinResult<RegistryValue> {
		let value_w = WString::from_str(value);
		let mut raw_data_type = u32::default();
		let mut data_len = u32::default();

		// Query data type and length.
		match co::ERROR(
			unsafe {
				advapi32::RegQueryValueExW(
					self.ptr,
					value_w.as_ptr(),
					std::ptr::null_mut(),
					&mut raw_data_type,
					std::ptr::null_mut(),
					&mut data_len,
				)
			} as _
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		// Retrieve value.
		let mut buf: Vec<u8> = vec![0x00; data_len as _];

		match co::ERROR(
			unsafe {
				advapi32::RegQueryValueExW(
					self.ptr,
					value_w.as_ptr(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					buf.as_mut_ptr() as _,
					&mut data_len,
				)
			} as _
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		// Return the value according to type.
		match co::REG(raw_data_type) {
			co::REG::NONE => Ok(RegistryValue::None),
			co::REG::DWORD => Ok(RegistryValue::Dword(
				u32::from_ne_bytes(buf.try_into().unwrap())),
			),
			co::REG::QWORD => Ok(RegistryValue::Qword(
				u64::from_ne_bytes(buf.try_into().unwrap())),
			),
			co::REG::SZ => {
				let (_, vec16, _) = unsafe { buf.align_to::<u16>() };
				Ok(RegistryValue::Sz(WString::from_wchars_slice(&vec16)))
			},
			co::REG::BINARY => Ok(RegistryValue::Binary(buf)),
			_ => Ok(RegistryValue::None), // other types not implemented yet
		}
	}

	/// [`RegSetKeyValue`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetvalueexw)
	/// method.
	///
	/// If the value doesn't exist, if will be created. If new type is different
	/// from current type, new type will take over.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{HKEY, RegistryValue};
	///
	/// HKEY::CURRENT_USER.SetKeyValue(
	///     "Software\\My Company",
	///     "Color",
	///     RegistryValue::Sz("blue".to_owned()),
	/// )?;
	/// ```
	pub fn SetKeyValue(self,
		sub_key: &str, value: &str, data: RegistryValue) -> WinResult<()>
	{
		match co::ERROR(
			unsafe {
				advapi32::RegSetKeyValueW(
					self.ptr,
					WString::from_str(sub_key).as_ptr(),
					WString::from_str(value).as_ptr(),
					data.reg_type().0,
					data.as_ptr(),
					data.len() as _,
				)
			} as _
		) {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
	}

	/// [`RegSetValueEx`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetvalueexw)
	/// method.
	///
	/// If the value doesn't exist, if will be created. If new type is different
	/// from current type, new type will prevail.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, HKEY, RegistryValue};
	///
	/// let hkey = HKEY::CURRENT_USER.OpenKeyEx(
	///     "Console\\Git Bash",
	///     co::REG_OPTION::default(),
	///     co::KEY::ALL_ACCESS,
	/// )?;
	///
	/// hkey.SetValueEx(
	///     "Color",
	///     RegistryValue::Sz("blue".to_owned()),
	/// )?;
	///
	/// hkey.CloseKey()?;
	/// ```
	pub fn SetValueEx(self,
		value: &str, data: RegistryValue) -> WinResult<()>
	{
		match co::ERROR(
			unsafe {
				advapi32::RegSetValueExW(
					self.ptr,
					WString::from_str(value).as_ptr(),
					0,
					data.reg_type().0,
					data.as_ptr() as _,
					data.len() as _,
				)
			} as _
		) {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
	}
}
