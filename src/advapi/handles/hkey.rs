#![allow(non_camel_case_types, non_snake_case)]

use std::ops::Deref;

use crate::{advapi, co};
use crate::advapi::decl::RegistryValue;
use crate::kernel::decl::{FILETIME, SysResult, WString};
use crate::prelude::Handle;

impl_handle! { HKEY: "advapi";
	/// Handle to a
	/// [registry key](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hkey).
	///
	/// This handle also exposes several
	/// [predefined registry keys](https://learn.microsoft.com/en-us/windows/win32/sysinfo/predefined-keys),
	/// like `HKEY::CURRENT_USER`, which are always open and ready to be used.
	/// Usually, they are the starting point to open a registry key.
}

impl advapi_Hkey for HKEY {}

macro_rules! predef_key {
	($name:ident, $val:expr) => {
		/// Predefined registry key, always open.
		const $name: HKEY = HKEY($val as *mut _);
	};
}

/// This trait is enabled with the `advapi` feature, and provides methods for
/// [`HKEY`](crate::HKEY).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "advapi")))]
pub trait advapi_Hkey: Handle {
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
	///     "Control Panel",
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
	fn RegEnumKeyEx(&self) -> SysResult<Box<dyn Iterator<Item = SysResult<String>> + '_>> {
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
	///     "Control Panel\\Appearance",
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
	fn RegEnumValue(&self) -> SysResult<Box<dyn Iterator<Item = SysResult<(String, co::REG)>> + '_>> {
		Ok(Box::new(EnumValueIter::new(self)?))
	}

	/// [`RegFlushKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regflushkey)
	/// method.
	fn RegFlushKey(&self) -> SysResult<()> {
		match co::ERROR(unsafe { advapi::ffi::RegFlushKey(self.as_ptr()) } as _) {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
	}

	/// [`RegGetValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-reggetvaluew)
	/// method.
	///
	/// The data type will be automatically queried with a first call to
	/// `RegGetValue`.
	///
	/// Note that this method validates some race conditions, returning
	/// [`co::ERROR::TRANSACTION_REQUEST_NOT_VALID`](crate::co::ERROR::TRANSACTION_REQUEST_NOT_VALID)
	/// and [`co::ERROR::INVALID_DATA`](crate::co::ERROR::INVALID_DATA).
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HKEY, RegistryValue};
	///
	/// let val = HKEY::CURRENT_USER.RegGetValue(
	///     Some("Control Panel\\Mouse"),
	///     Some("Beep"),
	/// )?;
	///
	/// match val {
	///     RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///     RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///     RegistryValue::Sz(str) => println!("String: {}", str),
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
		sub_key: Option<&str>, value: Option<&str>) -> SysResult<RegistryValue>
	{
		let sub_key_w = WString::from_opt_str(sub_key);
		let value_w = WString::from_opt_str(value);
		let mut raw_data_type1 = u32::default();
		let mut data_len1 = u32::default();

		// Query data type and length.
		match co::ERROR(
			unsafe {
				advapi::ffi::RegGetValueW(
					self.as_ptr(),
					sub_key_w.as_ptr(),
					value_w.as_ptr(),
					(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
					&mut raw_data_type1,
					std::ptr::null_mut(),
					&mut data_len1,
				)
			} as _,
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		// Alloc the receiving block.
		let mut buf: Vec<u8> = vec![0x00; data_len1 as _];

		let mut raw_data_type2 = u32::default();
		let mut data_len2 = data_len1;

		// Retrieve the value content.
		match co::ERROR(
			unsafe {
				advapi::ffi::RegGetValueW(
					self.as_ptr(),
					sub_key_w.as_ptr(),
					value_w.as_ptr(),
					(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
					&mut raw_data_type2,
					buf.as_mut_ptr() as _,
					&mut data_len2,
				)
			} as _,
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		validate_retrieved_reg_val(
			co::REG(raw_data_type1), data_len1,
			co::REG(raw_data_type2), data_len2, buf)
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
	///     "Control Panel\\Mouse",
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn RegOpenKeyEx(&self, sub_key: &str,
		options: co::REG_OPTION, access_rights: co::KEY) -> SysResult<HkeyGuard>
	{
		let mut hkey = HKEY::NULL;
		match co::ERROR(
			unsafe {
				advapi::ffi::RegOpenKeyExW(
					self.as_ptr(),
					WString::from_str(sub_key).as_ptr(),
					options.0,
					access_rights.0,
					&mut hkey.0,
				)
			} as _,
		) {
			co::ERROR::SUCCESS => Ok(HkeyGuard { hkey }),
			err => Err(err),
		}
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
		last_write_time: Option<&mut FILETIME>) -> SysResult<()>
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
					advapi::ffi::RegQueryInfoKeyW(
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

	/// [`RegQueryValueEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryvalueexw)
	/// method.
	///
	/// The data type will be automatically queried with a first call to
	/// `RegQueryValueEx`.
	///
	/// Note that this method validates some race conditions, returning
	/// [`co::ERROR::TRANSACTION_REQUEST_NOT_VALID`](crate::co::ERROR::TRANSACTION_REQUEST_NOT_VALID)
	/// and [`co::ERROR::INVALID_DATA`](crate::co::ERROR::INVALID_DATA).
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HKEY, RegistryValue};
	///
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     "Control Panel\\Mouse",
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// let val = hkey.RegQueryValueEx("Beep")?;
	///
	/// match val {
	///     RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///     RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///     RegistryValue::Sz(str) => println!("String: {}", str),
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
	fn RegQueryValueEx(&self, value: &str) -> SysResult<RegistryValue> {
		let value_w = WString::from_str(value);
		let mut raw_data_type1 = u32::default();
		let mut data_len1 = u32::default();

		// Query data type and length.
		match co::ERROR(
			unsafe {
				advapi::ffi::RegQueryValueExW(
					self.as_ptr(),
					value_w.as_ptr(),
					std::ptr::null_mut(),
					&mut raw_data_type1,
					std::ptr::null_mut(),
					&mut data_len1,
				)
			} as _,
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		// Alloc the receiving block.
		let mut buf: Vec<u8> = vec![0x00; data_len1 as _];

		let mut raw_data_type2 = u32::default();
		let mut data_len2 = data_len1;

		// Retrieve the value content.
		match co::ERROR(
			unsafe {
				advapi::ffi::RegQueryValueExW(
					self.as_ptr(),
					value_w.as_ptr(),
					std::ptr::null_mut(),
					&mut raw_data_type2,
					buf.as_mut_ptr() as _,
					&mut data_len2,
				)
			} as _
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		validate_retrieved_reg_val(
			co::REG(raw_data_type1), data_len1,
			co::REG(raw_data_type2), data_len2, buf)
	}

	/// [`RegSetKeyValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetvalueexw)
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
	///     "Software\\My Company",
	///     "Color",
	///     RegistryValue::Sz("blue".to_owned()),
	/// )?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn RegSetKeyValue(&self,
		sub_key: &str, value: &str, data: RegistryValue) -> SysResult<()>
	{
		let mut str_buf = WString::default();
		let (data_ptr, data_len) = data.as_ptr_with_len(&mut str_buf);

		match co::ERROR(
			unsafe {
				advapi::ffi::RegSetKeyValueW(
					self.as_ptr(),
					WString::from_str(sub_key).as_ptr(),
					WString::from_str(value).as_ptr(),
					data.reg_type().0,
					data_ptr,
					data_len,
				)
			} as _,
		) {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
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
	///     "Console\\Git Bash",
	///     co::REG_OPTION::default(),
	///     co::KEY::ALL_ACCESS,
	/// )?;
	///
	/// hkey.RegSetValueEx(
	///     "Color",
	///     RegistryValue::Sz("blue".to_owned()),
	/// )?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn RegSetValueEx(&self, value: &str, data: RegistryValue) -> SysResult<()> {
		let mut str_buf = WString::default();
		let (data_ptr, data_len) = data.as_ptr_with_len(&mut str_buf);

		match co::ERROR(
			unsafe {
				advapi::ffi::RegSetValueExW(
					self.as_ptr(),
					WString::from_str(value).as_ptr(),
					0,
					data.reg_type().0,
					data_ptr as _,
					data_len,
				)
			} as _,
		) {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
	}
}

//------------------------------------------------------------------------------

fn validate_retrieved_reg_val(
	data_type1: co::REG,
	data_len1: u32,
	data_type2: co::REG,
	mut data_len2: u32,
	buf: Vec<u8>) -> SysResult<RegistryValue>
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

/// RAII implementation for [`HKEY`](crate::HKEY) which automatically calls
/// [`RegCloseKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "advapi")))]
pub struct HkeyGuard {
	pub(crate) hkey: HKEY,
}

impl Drop for HkeyGuard {
	fn drop(&mut self) {
		if let Some(h) = self.hkey.as_opt() {
			if h.0 < HKEY::CLASSES_ROOT.0 || h.0 > HKEY::PERFORMANCE_NLSTEXT.0 { // guard predefined keys
				unsafe { advapi::ffi::RegCloseKey(h.as_ptr()); } // ignore errors
			}
		}
	}
}

impl Deref for HkeyGuard {
	type Target = HKEY;

	fn deref(&self) -> &Self::Target {
		&self.hkey
	}
}

//------------------------------------------------------------------------------

struct EnumKeyIter<'a, H>
	where H: advapi_Hkey,
{
	hkey: &'a H,
	count: u32,
	current: u32,
	name_buffer: WString,
}

impl<'a, H> Iterator for EnumKeyIter<'a, H>
	where H: advapi_Hkey,
{
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let mut len_buffer = self.name_buffer.buf_len() as u32;
		match co::ERROR(
			unsafe {
				advapi::ffi::RegEnumKeyExW(
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
	where H: advapi_Hkey,
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
	where H: advapi_Hkey,
{
	hkey: &'a H,
	count: u32,
	current: u32,
	name_buffer: WString,
}

impl<'a, H> Iterator for EnumValueIter<'a, H>
	where H: advapi_Hkey,
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
				advapi::ffi::RegEnumValueW(
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
	where H: advapi_Hkey,
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
