#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::enums::RegistryValue;
use crate::ffi::advapi32;
use crate::privs::bool_to_winresult;
use crate::structs::FILETIME;
use crate::WString;

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
	predef_key!(CLASSES_ROOT, 0x80000000);
	predef_key!(CURRENT_USER, 0x80000001);
	predef_key!(LOCAL_MACHINE, 0x80000002);
	predef_key!(USERS, 0x80000003);
	predef_key!(PERFORMANCE_DATA, 0x80000004);
	predef_key!(PERFORMANCE_TEXT, 0x80000050);
	predef_key!(PERFORMANCE_NLSTEXT, 0x80000060);
	predef_key!(CURRENT_CONFIG, 0x80000005);
	predef_key!(DYN_DATA, 0x80000006);
	predef_key!(CURRENT_USER_LOCAL_SETTINGS, 0x80000007);

	/// [`RegCloseKey`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey)
	/// method.
	pub fn RegCloseKey(self) -> WinResult<()> {
		bool_to_winresult(unsafe { advapi32::RegCloseKey(self.ptr) })
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
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     "Control Panel",
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// ).unwrap();
	///
	/// for key_name in hkey.RegEnumKeyEx().unwrap() {
	///     println!("{}", key_name);
	/// }
	///
	/// hkey.RegCloseKey().unwrap();
	/// ```
	pub fn RegEnumKeyEx(self) -> WinResult<Vec<String>> {
		let mut nKeys: u32 = 0;
		let mut maxKeyLen: u32 = 0; // length of longest subkey name
		self.RegQueryInfoKey(
			None, Some(&mut nKeys), Some(&mut maxKeyLen),
			None, None, None, None, None, None)?;

		let mut namesVec = Vec::with_capacity(nKeys as _);
		let mut nameBuf = WString::new_alloc_buffer(maxKeyLen as usize + 1);
		let mut lenBuf;

		for index in 0..nKeys {
			lenBuf = nameBuf.buffer_size() as u32;

			let err = co::ERROR(
				unsafe {
					advapi32::RegEnumKeyExW(
						self.ptr,
						index,
						nameBuf.as_mut_ptr(),
						&mut lenBuf,
						std::ptr::null_mut(),
						std::ptr::null_mut(),
						std::ptr::null_mut(),
						std::ptr::null_mut(),
					)
				} as _
			);

			if err != co::ERROR::SUCCESS {
				return Err(err);
			}
			namesVec.push(nameBuf.to_string());
		}

		namesVec.sort_by(|a, b| a.to_uppercase().cmp(&b.to_uppercase()));
		Ok(namesVec)
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
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     "Control Panel\\Appearance",
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// ).unwrap();
	///
	/// for (value, reg_type) in hkey.RegEnumValue().unwrap() {
	///     println!("{}, {}", value, reg_type);
	/// }
	///
	/// hkey.RegCloseKey().unwrap();
	/// ```
	pub fn RegEnumValue(self) -> WinResult<Vec<(String, co::REG)>> {
		let mut nVals: u32 = 0;
		let mut maxValLen: u32 = 0; // length of longest value name
		self.RegQueryInfoKey(
			None, None, None, None, Some(&mut nVals), Some(&mut maxValLen),
			None, None, None)?;

		let mut namesTypesVec = Vec::with_capacity(nVals as _);
		let mut nameBuf = WString::new_alloc_buffer(maxValLen as usize + 1);
		let mut lenBuf;
		let mut rawDataType: u32 = 0;

		for index in 0..nVals {
			lenBuf = nameBuf.buffer_size() as u32;

			let err = co::ERROR(
				unsafe {
					advapi32::RegEnumValueW(
						self.ptr,
						index,
						nameBuf.as_mut_ptr(),
						&mut lenBuf,
						std::ptr::null_mut(),
						&mut rawDataType,
						std::ptr::null_mut(),
						std::ptr::null_mut(),
					)
				} as _
			);

			if err != co::ERROR::SUCCESS {
				return Err(err);
			}
			namesTypesVec.push((nameBuf.to_string(), co::REG(rawDataType)));
		}

		namesTypesVec.sort_by(|a, b| a.0.to_uppercase().cmp(&b.0.to_uppercase()));
		Ok(namesTypesVec)
	}

	/// [`RegGetValue`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-reggetvaluew)
	/// method.
	///
	/// The data type will be automatically queried with a first call to `RegGetValue`.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{HKEY, RegistryValue};
	///
	/// let val = HKEY::CURRENT_USER.RegGetValue(
	///     "Control Panel\\Mouse",
	///     "Beep",
	/// ).unwrap();
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
	pub fn RegGetValue(self,
		lpSubKey: &str, lpValue: &str) -> WinResult<RegistryValue>
	{
		let wSubKey = WString::from_str(lpSubKey);
		let wValueName = WString::from_str(lpValue);
		let mut rawDataType: u32 = 0;
		let mut dataLen: u32 = 0;

		// Query data type and length.
		match co::ERROR(
			unsafe {
				advapi32::RegGetValueW(
					self.ptr,
					wSubKey.as_ptr(),
					wValueName.as_ptr(),
					(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
					&mut rawDataType,
					std::ptr::null_mut(),
					&mut dataLen,
				)
			} as _
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		// Retrieve value according to informed data type.
		match co::REG(rawDataType) {
			co::REG::NONE => Ok(RegistryValue::None), // no value to query
			co::REG::DWORD => {
				let mut dwordBuf: u32 = 0;

				match co::ERROR(
					unsafe {
						advapi32::RegGetValueW( // query DWORD value
							self.ptr,
							wSubKey.as_ptr(),
							wValueName.as_ptr(),
							(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
							std::ptr::null_mut(),
							&mut dwordBuf as *mut _ as _,
							&mut dataLen,
						)
					} as _
				) {
					co::ERROR::SUCCESS => Ok(RegistryValue::Dword(dwordBuf)),
					err => Err(err),
				}
			},
			co::REG::QWORD => {
				let mut qwordBuf: u64 = 0;

				match co::ERROR(
					unsafe {
						advapi32::RegGetValueW( // query QWORD value
							self.ptr,
							wSubKey.as_ptr(),
							wValueName.as_ptr(),
							(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
							std::ptr::null_mut(),
							&mut qwordBuf as *mut _ as _,
							&mut dataLen,
						)
					} as _
				) {
					co::ERROR::SUCCESS => Ok(RegistryValue::Qword(qwordBuf)),
					err => Err(err),
				}
			},
			co::REG::SZ => {
				let mut szBuf: Vec<u16> = vec![0; dataLen as usize]; // alloc wchar buffer

				match co::ERROR(
					unsafe {
						advapi32::RegGetValueW( // query string value
							self.ptr,
							wSubKey.as_ptr(),
							wValueName.as_ptr(),
							(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
							std::ptr::null_mut(),
							szBuf.as_mut_ptr() as _,
							&mut dataLen,
						)
					} as _
				) {
					co::ERROR::SUCCESS => Ok(
						RegistryValue::Sz(WString::from_wchars_slice(&szBuf)),
					),
					err => Err(err),
				}
			},
			co::REG::BINARY => {
				let mut byteBuf: Vec<u8> = vec![0; dataLen as usize]; // alloc byte buffer

				match co::ERROR(
					unsafe {
						advapi32::RegGetValueW( // query binary value
							self.ptr,
							wSubKey.as_ptr(),
							wValueName.as_ptr(),
							(co::RRF::RT_ANY | co::RRF::NOEXPAND).0,
							std::ptr::null_mut(),
							byteBuf.as_mut_ptr() as _,
							&mut dataLen,
						)
					} as _
				) {
					co::ERROR::SUCCESS => Ok(RegistryValue::Binary(byteBuf)),
					err => Err(err),
				}
			},
			_ => Ok(RegistryValue::None), // other types not implemented yet
		}
	}

	/// [`RegOpenKeyEx`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw)
	/// method.
	///
	/// **Note:** Must be paired with a [`RegCloseKey`](crate::HKEY::RegCloseKey)
	/// call.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, HKEY};
	///
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     "Control Panel\\Mouse",
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// ).unwrap();
	///
	/// hkey.RegCloseKey().unwrap();
	/// ```
	pub fn RegOpenKeyEx(self, lpSubKey: &str,
		ulOptions: co::REG_OPTION, samDesired: co::KEY) -> WinResult<HKEY>
	{
		let mut hKey = Self::NULL;

		match co::ERROR(
			unsafe {
				advapi32::RegOpenKeyExW(
					self.ptr,
					WString::from_str(lpSubKey).as_ptr(),
					ulOptions.0,
					samDesired.0,
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
	pub fn RegQueryInfoKey(self,
		mut lpClass: Option<&mut WString>,
		lpcSubKeys: Option<&mut u32>,
		lpcbMaxSubKeyLen: Option<&mut u32>,
		lpcbMaxClassLen: Option<&mut u32>,
		lpcValues: Option<&mut u32>,
		lpcbMaxValueNameLen: Option<&mut u32>,
		lpcbMaxValueLen: Option<&mut u32>,
		lpcbSecurityDescriptor: Option<&mut u32>,
		lpftLastWriteTime: Option<&mut FILETIME>) -> WinResult<()>
	{
		let (mut lpClass2, mut lpcchClass) = match &mut lpClass {
			Some(lpClass) => {
				if lpClass.buffer_size() < 32 {
					lpClass.realloc_buffer(32); // arbitrary
				}
				(unsafe { lpClass.as_mut_ptr() }, lpClass.buffer_size() as u32)
			},
			None => (std::ptr::null_mut(), 0),
		};

		let lpcSubKeys2 = lpcSubKeys.map_or(std::ptr::null_mut(), |re| re as _);
		let lpcbMaxSubKeyLen2 = lpcbMaxSubKeyLen.map_or(std::ptr::null_mut(), |re| re as _);
		let lpcbMaxClassLen2 = lpcbMaxClassLen.map_or(std::ptr::null_mut(), |re| re as _);
		let lpcValues2 = lpcValues.map_or(std::ptr::null_mut(), |re| re as _);
		let lpcbMaxValueNameLen2 = lpcbMaxValueNameLen.map_or(std::ptr::null_mut(), |re| re as _);
		let lpcbMaxValueLen2 = lpcbMaxValueLen.map_or(std::ptr::null_mut(), |re| re as _);
		let lpcbSecurityDescriptor2 = lpcbSecurityDescriptor.map_or(std::ptr::null_mut(), |re| re as _);
		let lpftLastWriteTime2 = lpftLastWriteTime.map_or(std::ptr::null_mut(), |re| re as *mut _ as _);

		loop { // until lpClass is large enough
			match co::ERROR(
				unsafe {
					advapi32::RegQueryInfoKeyW(
						self.ptr,
						lpClass2,
						&mut lpcchClass,
						std::ptr::null_mut(),
						lpcSubKeys2,
						lpcbMaxSubKeyLen2,
						lpcbMaxClassLen2,
						lpcValues2,
						lpcbMaxValueNameLen2,
						lpcbMaxValueLen2,
						lpcbSecurityDescriptor2,
						lpftLastWriteTime2,
					)
				} as _
			) {
				co::ERROR::MORE_DATA => match &mut lpClass {
					Some(lpClass) => {
						lpClass.realloc_buffer(lpClass.buffer_size() + 32); // arbitrary
						lpClass2 = unsafe { lpClass.as_mut_ptr() };
						lpcchClass = lpClass.buffer_size() as _;
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
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     "Control Panel\\Mouse",
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// ).unwrap();
	///
	/// let val = hkey.RegQueryValueEx("Beep")
	///   .unwrap();
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
	/// hkey.RegCloseKey().unwrap();
	/// ```
	pub fn RegQueryValueEx(self, lpValueName: &str) -> WinResult<RegistryValue> {
		let wValueName = WString::from_str(lpValueName);
		let mut rawDataType: u32 = 0;
		let mut dataLen: u32 = 0;

		// Query data type and length.
		match co::ERROR(
			unsafe {
				advapi32::RegQueryValueExW(
					self.ptr,
					wValueName.as_ptr(),
					std::ptr::null_mut(),
					&mut rawDataType,
					std::ptr::null_mut(),
					&mut dataLen,
				)
			} as _
		) {
			co::ERROR::SUCCESS => {},
			err => return Err(err),
		}

		// Retrieve value according to informed data type.
		match co::REG(rawDataType) {
			co::REG::NONE => Ok(RegistryValue::None), // no value to query
			co::REG::DWORD => {
				let mut dwordBuf: u32 = 0;

				match co::ERROR(
					unsafe {
						advapi32::RegQueryValueExW( // query DWORD value
							self.ptr,
							wValueName.as_ptr(),
							std::ptr::null_mut(),
							std::ptr::null_mut(),
							&mut dwordBuf as *mut _ as _,
							&mut dataLen,
						)
					} as _
				) {
					co::ERROR::SUCCESS => Ok(RegistryValue::Dword(dwordBuf)),
					err => Err(err),
				}
			},
			co::REG::QWORD => {
				let mut qwordBuf: u64 = 0;

				match co::ERROR(
					unsafe {
						advapi32::RegQueryValueExW( // query QWORD value
							self.ptr,
							wValueName.as_ptr(),
							std::ptr::null_mut(),
							std::ptr::null_mut(),
							&mut qwordBuf as *mut _ as _,
							&mut dataLen,
						)
					} as _
				) {
					co::ERROR::SUCCESS => Ok(RegistryValue::Qword(qwordBuf)),
					err => Err(err),
				}
			},
			co::REG::SZ => {
				let mut szBuf: Vec<u16> = vec![0; dataLen as usize]; // alloc wchar buffer

				match co::ERROR(
					unsafe {
						advapi32::RegQueryValueExW( // query string value
							self.ptr,
							wValueName.as_ptr(),
							std::ptr::null_mut(),
							std::ptr::null_mut(),
							szBuf.as_mut_ptr() as _,
							&mut dataLen,
						)
					} as _
				) {
					co::ERROR::SUCCESS => Ok(
						RegistryValue::Sz(WString::from_wchars_slice(&szBuf)),
					),
					err => Err(err),
				}
			},
			co::REG::BINARY => {
				let mut byteBuf: Vec<u8> = vec![0; dataLen as usize]; // alloc byte buffer

				match co::ERROR(
					unsafe {
						advapi32::RegQueryValueExW( // query binary value
							self.ptr,
							wValueName.as_ptr(),
							std::ptr::null_mut(),
							std::ptr::null_mut(),
							byteBuf.as_mut_ptr(),
							&mut dataLen,
						)
					} as _
				) {
					co::ERROR::SUCCESS => Ok(RegistryValue::Binary(byteBuf)),
					err => Err(err),
				}
			},
			_ => Ok(RegistryValue::None), // other types not implemented yet
		}
	}

	/// [`RegSetKeyValue`](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetvalueexw)
	/// method.
	///
	/// If the value doesn't exist, if will be created. If new type is different
	/// from current type, new type will prevail.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{HKEY, RegistryValue};
	///
	/// HKEY::CURRENT_USER.RegSetKeyValue(
	///     "Software\\My Company",
	///     "Color",
	///     RegistryValue::Sz("blue".to_owned()),
	/// ).unwrap();
	/// ```
	pub fn RegSetKeyValue(self, lpSubKey: &str,
		lpValueName: &str, lpData: RegistryValue) -> WinResult<()>
	{
		match co::ERROR(
			unsafe {
				advapi32::RegSetKeyValueW(
					self.ptr,
					WString::from_str(lpSubKey).as_ptr(),
					WString::from_str(lpValueName).as_ptr(),
					lpData.reg_type().0,
					lpData.as_ptr(),
					lpData.len() as _,
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
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///     "Console\\Git Bash",
	///     co::REG_OPTION::default(),
	///     co::KEY::ALL_ACCESS,
	/// ).unwrap();
	///
	/// hkey.RegSetValueEx(
	///     "Color",
	///     RegistryValue::Sz("blue".to_owned()),
	/// ).unwrap();
	///
	/// hkey.RegCloseKey().unwrap();
	/// ```
	pub fn RegSetValueEx(self,
		lpValueName: &str, lpData: RegistryValue) -> WinResult<()>
	{
		match co::ERROR(
			unsafe {
				advapi32::RegSetValueExW(
					self.ptr,
					WString::from_str(lpValueName).as_ptr(),
					0,
					lpData.reg_type().0,
					lpData.as_ptr() as _,
					lpData.len() as _,
				)
			} as _
		) {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
	}
}
