#![allow(non_snake_case)]

use std::convert::TryInto;
use std::marker::PhantomData;

use crate::aliases::WinResult;
use crate::co;
use crate::enums::RegistryValue;
use crate::ffi::advapi32;
use crate::handles::prelude::Handle;
use crate::structs::FILETIME;
use crate::various::WString;

/// Handle to a
/// [registry key](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hkey).
///
/// This handle also exposes several
/// [predefined registry keys](https://docs.microsoft.com/en-us/windows/win32/sysinfo/predefined-keys),
/// like `HKEY::CURRENT_USER`, which are always open and ready to be used.
/// Usually, they are the starting point to open a registry key.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HKEY(pub(crate) *mut std::ffi::c_void);

impl_handle!(HKEY);

macro_rules! predef_key {
	($name:ident, $val:expr) => {
		/// Predefined registry key, always open.
		pub const $name: Self = Self($val as *mut _);
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
		match co::ERROR(unsafe { advapi32::RegCloseKey(self.0) as _ }) {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
	}

	/// Returns an iterator over the names of the keys, which calls
	/// [`RegEnumKeyEx`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumkeyexw)
	/// repeatedly.
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
	/// for key_name in hkey.EnumKeyEx()? {
	///     let key_name = key_name?;
	///     println!("{}", key_name);
	/// }
	///
	/// hkey.CloseKey()?;
	/// ```
	pub fn EnumKeyEx<'a>(self) -> WinResult<impl Iterator<Item = WinResult<String>> + 'a> {
		EnumKeyIter::new(self)
	}

	/// Returns an iterator of the names and types of the values, which calls
	/// [`RegEnumValue`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumvaluew)
	/// repeatedly.
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
	/// for value_and_type in hkey.EnumValue()? {
	///     let (value, reg_type) = value_and_type?;
	///     println!("{}, {}", value, reg_type);
	/// }
	///
	/// hkey.CloseKey()?;
	/// ```
	pub fn EnumValue<'a>(self) -> WinResult<impl Iterator<Item = WinResult<(String, co::REG)>> + 'a> {
		EnumValueIter::new(self)
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
					self.0,
					sub_key_w.as_ptr(),
					value_w.as_ptr(),
					(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
					&mut raw_data_type,
					std::ptr::null_mut(),
					&mut data_len,
				)
			} as _,
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		// Retrieve value.
		let mut buf: Vec<u8> = vec![0x00; data_len as _];

		match co::ERROR(
			unsafe {
				advapi32::RegGetValueW(
					self.0,
					sub_key_w.as_ptr(),
					value_w.as_ptr(),
					(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
					std::ptr::null_mut(),
					buf.as_mut_ptr() as _,
					&mut data_len,
				)
			} as _,
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
					self.0,
					WString::from_str(sub_key).as_ptr(),
					options.0,
					access_rights.0,
					&mut hKey.0,
				)
			} as _,
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
						self.0,
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
					self.0,
					value_w.as_ptr(),
					std::ptr::null_mut(),
					&mut raw_data_type,
					std::ptr::null_mut(),
					&mut data_len,
				)
			} as _,
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		// Retrieve value.
		let mut buf: Vec<u8> = vec![0x00; data_len as _];

		match co::ERROR(
			unsafe {
				advapi32::RegQueryValueExW(
					self.0,
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
					self.0,
					WString::from_str(sub_key).as_ptr(),
					WString::from_str(value).as_ptr(),
					data.reg_type().0,
					data.as_ptr(),
					data.len() as _,
				)
			} as _,
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
					self.0,
					WString::from_str(value).as_ptr(),
					0,
					data.reg_type().0,
					data.as_ptr() as _,
					data.len() as _,
				)
			} as _,
		) {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
	}
}

//------------------------------------------------------------------------------

struct EnumKeyIter<'a> {
	hkey: HKEY,
	count: u32,
	current: u32,
	name_buffer: WString,
	owner_: PhantomData<&'a ()>,
}

impl<'a> Iterator for EnumKeyIter<'a> {
	type Item = WinResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let mut len_buffer = self.name_buffer.buffer_size() as u32;
		match co::ERROR(
			unsafe {
				advapi32::RegEnumKeyExW(
					self.hkey.0,
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

impl<'a> EnumKeyIter<'a> {
	fn new(hkey: HKEY) -> WinResult<Self> {
		let mut num_keys = u32::default();
		let mut max_key_name_len = u32::default();
		hkey.QueryInfoKey(
			None, Some(&mut num_keys), Some(&mut max_key_name_len),
			None, None, None, None, None, None)?;

		Ok(Self {
			hkey,
			count: num_keys,
			current: 0,
			name_buffer: WString::new_alloc_buffer(max_key_name_len as usize + 1),
			owner_: PhantomData,
		})
	}
}

struct EnumValueIter<'a> {
	hkey: HKEY,
	count: u32,
	current: u32,
	name_buffer: WString,
	owner_: PhantomData<&'a ()>,
}

impl<'a> Iterator for EnumValueIter<'a> {
	type Item = WinResult<(String, co::REG)>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let mut raw_data_type = u32::default();
		let mut len_buffer = self.name_buffer.buffer_size() as u32;
		match co::ERROR(
			unsafe {
				advapi32::RegEnumValueW(
					self.hkey.0,
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

impl<'a> EnumValueIter<'a> {
	fn new(hkey: HKEY) -> WinResult<Self> {
		let mut num_vals = u32::default();
		let mut max_val_name_len = u32::default();
		hkey.QueryInfoKey(
			None, None, None, None, Some(&mut num_vals), Some(&mut max_val_name_len),
			None, None, None)?;

		Ok(Self {
			hkey,
			count: num_vals,
			current: 0,
			name_buffer: WString::new_alloc_buffer(max_val_name_len as usize + 1),
			owner_: PhantomData,
		})
	}
}
