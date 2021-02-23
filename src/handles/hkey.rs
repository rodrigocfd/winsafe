#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::enums::RegistryValue;
use crate::ffi::advapi32;
use crate::WString;

handle_type! {
	/// Handle to a
	/// [registry key](https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hkey).
	/// Exposes methods.
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
		match co::ERROR(unsafe { advapi32::RegCloseKey(self.ptr) } as u32) {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
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
	///   "Control Panel\\Mouse",
	///   "Beep",
	/// ).unwrap();
	///
	/// match val {
	///   RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///   RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///   RegistryValue::Sz(str) => println!("String: {}", str),
	///   RegistryValue::Binary(bin) => {
	///     println!("Binary:");
	///     for b in bin.iter() {
	///       print!("{:02x} ", b);
	///     }
	///     println!("");
	///   },
	///   RegistryValue::None => println!("No value"),
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
			} as u32
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
							&mut dwordBuf as *mut _ as *mut _,
							&mut dataLen,
						)
					} as u32
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
							&mut qwordBuf as *mut _ as *mut _,
							&mut dataLen,
						)
					} as u32
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
							szBuf.as_mut_ptr() as *mut _,
							&mut dataLen,
						)
					} as u32
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
							byteBuf.as_mut_ptr() as *mut _,
							&mut dataLen,
						)
					} as u32
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
	/// use winsafe::co::{KEY, REG_OPTION};
	/// use winsafe::HKEY;
	///
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///   "Control Panel\\Mouse",
	///   REG_OPTION::default(),
	///   KEY::READ,
	/// ).unwrap();
	///
	/// hkey.RegCloseKey().unwrap();
	/// ```
	pub fn RegOpenKeyEx(self, lpSubKey: &str,
		ulOptions: co::REG_OPTION, samDesired: co::KEY) -> WinResult<HKEY>
	{
		let mut hKey = unsafe { Self::null_handle() };

		match co::ERROR(
			unsafe {
				advapi32::RegOpenKeyExW(
					self.ptr,
					WString::from_str(lpSubKey).as_ptr(),
					ulOptions.0,
					samDesired.0,
					&mut hKey.ptr,
				)
			} as u32
		) {
			co::ERROR::SUCCESS => Ok(hKey),
			err => Err(err),
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
	/// use winsafe::co::{KEY, REG_OPTION};
	/// use winsafe::{HKEY, RegistryValue};
	///
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///   "Control Panel\\Mouse",
	///   REG_OPTION::default(),
	///   KEY::READ,
	/// ).unwrap();
	///
	/// let val = hkey.RegQueryValueEx("Beep")
	///   .unwrap();
	///
	/// match val {
	///   RegistryValue::Dword(n) => println!("Number u32: {}", n),
	///   RegistryValue::Qword(n) => println!("Number u64: {}", n),
	///   RegistryValue::Sz(str) => println!("String: {}", str),
	///   RegistryValue::Binary(bin) => {
	///     println!("Binary:");
	///     for b in bin.iter() {
	///       print!("{:02x} ", b);
	///     }
	///     println!("");
	///   },
	///   RegistryValue::None => println!("No value"),
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
			} as u32
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
							&mut dwordBuf as *mut u32 as *mut u8,
							&mut dataLen,
						)
					} as u32
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
							&mut qwordBuf as *mut u64 as *mut u8,
							&mut dataLen,
						)
					} as u32
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
							szBuf.as_mut_ptr() as *mut u8,
							&mut dataLen,
						)
					} as u32
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
					} as u32
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
	///   "Software\\My Company",
	///   "Color",
	///   RegistryValue::Sz("blue".to_owned()),
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
					lpData.len() as u32,
				)
			} as u32
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
	/// use winsafe::co::{KEY, REG_OPTION};
	/// use winsafe::{HKEY, RegistryValue};
	///
	/// let hkey = HKEY::CURRENT_USER.RegOpenKeyEx(
	///   "Console\\Git Bash",
	///   REG_OPTION::default(),
	///   KEY::ALL_ACCESS,
	/// ).unwrap();
	///
	/// hkey.RegSetValueEx(
	///   "Color",
	///   RegistryValue::Sz("blue".to_owned()),
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
					lpData.as_ptr() as *const u8,
					lpData.len() as u32,
				)
			} as u32
		) {
			co::ERROR::SUCCESS => Ok(()),
			err => Err(err),
		}
	}
}
