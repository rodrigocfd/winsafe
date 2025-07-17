#![allow(non_camel_case_types, non_snake_case)]

use crate::advapi::{ffi, iterators::*};
use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

handle! { HKEY;
	/// Handle to a
	/// [registry key](https://learn.microsoft.com/en-us/windows/win32/winprog/windows-data-types#hkey).
	///
	/// This handle also exposes several
	/// [predefined registry keys](https://learn.microsoft.com/en-us/windows/win32/sysinfo/predefined-keys),
	/// like `HKEY::CURRENT_USER`, which are always open and ready to be used.
	/// Usually, they are the starting point to open a registry key.
}

macro_rules! predef_key {
	($name:ident, $val:expr) => {
		/// Predefined registry key, always open.
		pub const $name: HKEY = HKEY($val as *mut _);
	};
}

impl HKEY {
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

	pub(in crate::advapi) fn is_predef_key(&self) -> bool {
		// Note that we are not constructing HKEY objects, so no drop() is called.
		(self.0 as usize) >= (Self::CLASSES_ROOT.0 as usize)
			&& (self.0 as usize) <= (Self::PERFORMANCE_NLSTEXT.0 as usize)
	}

	/// [`RegConnectRegistry`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regconnectregistryw)
	/// function.
	///
	/// # Panics
	///
	/// Panics if `predef_key` is different from:
	///
	/// - [`HKEY::LOCAL_MACHINE`](crate::HKEY::LOCAL_MACHINE);
	/// - [`HKEY::PERFORMANCE_DATA`](crate::HKEY::PERFORMANCE_DATA);
	/// - [`HKEY::USERS`](crate::HKEY::USERS).
	#[must_use]
	pub fn RegConnectRegistry(
		machine_name: Option<&str>,
		predef_hkey: &HKEY,
	) -> SysResult<RegCloseKeyGuard> {
		if *predef_hkey != HKEY::LOCAL_MACHINE
			&& *predef_hkey != HKEY::PERFORMANCE_DATA
			&& *predef_hkey != HKEY::USERS
		{
			panic!("Invalid predef_key.");
		}

		let mut hkey = HKEY::NULL;
		unsafe {
			ErrorRet(ffi::RegConnectRegistryW(
				WString::from_opt_str(machine_name).as_ptr(),
				predef_hkey.ptr(),
				hkey.as_mut(),
			))
			.to_sysresult()
			.map(|_| RegCloseKeyGuard::new(hkey))
		}
	}

	/// [`RegCopyTree`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcopytreew)
	/// function.
	pub fn RegCopyTree(&self, sub_key: Option<&str>, dest: &HKEY) -> SysResult<()> {
		ErrorRet(unsafe {
			ffi::RegCopyTreeW(self.ptr(), WString::from_opt_str(sub_key).as_ptr(), dest.ptr())
		})
		.to_sysresult()
	}

	/// [`RegCreateKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeyexw)
	/// function.
	#[must_use]
	pub fn RegCreateKeyEx(
		&self,
		sub_key: &str,
		class: Option<&str>,
		options: co::REG_OPTION,
		access_rights: co::KEY,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
	) -> SysResult<(RegCloseKeyGuard, co::REG_DISPOSITION)> {
		let mut hkey = HKEY::NULL;
		let mut disposition = co::REG_DISPOSITION::default();

		unsafe {
			ErrorRet(ffi::RegCreateKeyExW(
				self.ptr(),
				WString::from_str(sub_key).as_ptr(),
				0,
				WString::from_opt_str(class).as_ptr(),
				options.raw(),
				access_rights.raw(),
				pcvoid_or_null(security_attributes),
				hkey.as_mut(),
				disposition.as_mut(),
			))
			.to_sysresult()
			.map(|_| (RegCloseKeyGuard::new(hkey), disposition))
		}
	}

	/// [`RegCreateKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeytransactedw)
	/// function.
	#[must_use]
	pub fn RegCreateKeyTransacted(
		&self,
		sub_key: &str,
		class: Option<&str>,
		options: co::REG_OPTION,
		access_rights: co::KEY,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
		htransaction: &HTRANSACTION,
	) -> SysResult<(RegCloseKeyGuard, co::REG_DISPOSITION)> {
		let mut hkey = HKEY::NULL;
		let mut disposition = co::REG_DISPOSITION::default();

		unsafe {
			ErrorRet(ffi::RegCreateKeyTransactedW(
				self.ptr(),
				WString::from_str(sub_key).as_ptr(),
				0,
				WString::from_opt_str(class).as_ptr(),
				options.raw(),
				access_rights.raw(),
				pcvoid_or_null(security_attributes),
				hkey.as_mut(),
				disposition.as_mut(),
				htransaction.ptr(),
				std::ptr::null_mut(),
			))
			.to_sysresult()
			.map(|_| (RegCloseKeyGuard::new(hkey), disposition))
		}
	}

	/// [`RegDeleteKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeyw)
	/// function.
	pub fn RegDeleteKey(&self, sub_key: &str) -> SysResult<()> {
		ErrorRet(unsafe { ffi::RegDeleteKeyW(self.ptr(), WString::from_str(sub_key).as_ptr()) })
			.to_sysresult()
	}

	/// [`RegDeleteKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeyexw)
	/// function.
	///
	/// # Panics
	///
	/// Panics if `platform_view` is different from
	/// [`co::KEY::WOW64_32KEY`](crate::co::KEY::WOW64_32KEY) and
	/// [`co::KEY::WOW64_64KEY`](crate::co::KEY::WOW64_64KEY).
	pub fn RegDeleteKeyEx(&self, sub_key: &str, platform_view: co::KEY) -> SysResult<()> {
		if platform_view != co::KEY::WOW64_32KEY && platform_view != co::KEY::WOW64_64KEY {
			panic!("Platform view must be co::KEY::WOW64_32KEY or co::KEY::WOW64_64KEY");
		}

		ErrorRet(unsafe {
			ffi::RegDeleteKeyExW(
				self.ptr(),
				WString::from_str(sub_key).as_ptr(),
				platform_view.raw(),
				0,
			)
		})
		.to_sysresult()
	}

	/// [`RegDeleteKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletekeytransactedw)
	/// function.
	pub fn RegDeleteKeyTransacted(
		&self,
		sub_key: &str,
		access_rights: co::KEY,
		htransaction: &HTRANSACTION,
	) -> SysResult<()> {
		ErrorRet(unsafe {
			ffi::RegDeleteKeyTransactedW(
				self.ptr(),
				WString::from_str(sub_key).as_ptr(),
				access_rights.raw(),
				0,
				htransaction.ptr(),
				std::ptr::null_mut(),
			)
		})
		.to_sysresult()
	}

	/// [`RegDeleteTree`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletetreew)
	/// function.
	pub fn RegDeleteTree(&self, sub_key: Option<&str>) -> SysResult<()> {
		ErrorRet(unsafe {
			ffi::RegDeleteTreeW(self.ptr(), WString::from_opt_str(sub_key).as_ptr())
		})
		.to_sysresult()
	}

	/// [`RegDeleteValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdeletevaluew)
	/// function.
	pub fn RegDeleteValue(&self, value_name: Option<&str>) -> SysResult<()> {
		ErrorRet(unsafe {
			ffi::RegDeleteValueW(self.ptr(), WString::from_opt_str(value_name).as_ptr())
		})
		.to_sysresult()
	}

	/// [`RegDisableReflectionKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regdisablereflectionkey)
	/// function.
	pub fn RegDisableReflectionKey(&self) -> SysResult<()> {
		ErrorRet(unsafe { ffi::RegDisableReflectionKey(self.ptr()) }).to_sysresult()
	}

	/// [`RegEnableReflectionKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenablereflectionkey)
	/// function.
	pub fn RegEnableReflectionKey(&self) -> SysResult<()> {
		ErrorRet(unsafe { ffi::RegEnableReflectionKey(self.ptr()) }).to_sysresult()
	}

	/// Returns an iterator over the names of the keys, which calls
	/// [`RegEnumKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumkeyexw)
	/// repeatedly.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// for key_name in hkey.RegEnumKeyEx()? {
	///     let key_name = key_name?;
	///     println!("{}", key_name);
	/// }
	///
	/// // Collecting into a Vec
	/// let names: Vec<String> =
	///     hkey.RegEnumKeyEx()?
	///         .collect::<w::SysResult<Vec<_>>>()?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn RegEnumKeyEx(
		&self,
	) -> SysResult<impl DoubleEndedIterator<Item = SysResult<String>> + '_> {
		Ok(HkeyKeyIter::new(self)?)
	}

	/// Returns an iterator of the names and types of the values, which calls
	/// [`RegEnumValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumvaluew)
	/// repeatedly.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
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
	///         .collect::<w::SysResult<Vec<_>>>()?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn RegEnumValue(
		&self,
	) -> SysResult<impl DoubleEndedIterator<Item = SysResult<(String, co::REG)>> + '_> {
		Ok(HkeyValueIter::new(self)?)
	}

	/// [`RegFlushKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regflushkey)
	/// function.
	pub fn RegFlushKey(&self) -> SysResult<()> {
		ErrorRet(unsafe { ffi::RegFlushKey(self.ptr()) }).to_sysresult()
	}

	/// [`RegGetValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-reggetvaluew)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let val = w::HKEY::CURRENT_USER.RegGetValue(
	///     Some("Control Panel\\Mouse"),
	///     Some("Beep"),
	///     co::RRF::RT_ANY,
	/// )?;
	///
	/// println!("{val}");
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn RegGetValue(
		&self,
		sub_key: Option<&str>,
		value_name: Option<&str>,
		flags: co::RRF,
	) -> SysResult<RegistryValue> {
		let sub_key_w = WString::from_opt_str(sub_key);
		let value_name_w = WString::from_opt_str(value_name);
		let mut buf = Vec::<u8>::default();

		loop {
			let mut data_len = 0u32; // in bytes

			ErrorRet(unsafe {
				ffi::RegGetValueW(
					self.ptr(),
					sub_key_w.as_ptr(),
					value_name_w.as_ptr(),
					flags.raw(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					&mut data_len, // first call to retrieve the size only
				)
			})
			.to_sysresult()?;

			buf.resize(data_len as _, 0x00);
			let mut data_type = 0u32;

			match unsafe {
				co::ERROR::from_raw(ffi::RegGetValueW(
					self.ptr(),
					sub_key_w.as_ptr(),
					value_name_w.as_ptr(),
					flags.raw(),
					&mut data_type,
					buf.as_mut_ptr() as _, // second call to retrieve the data
					&mut data_len,
				) as _)
			} {
				co::ERROR::SUCCESS => {
					buf.resize(data_len as _, 0x00); // data length may have shrunk
					return unsafe { RegistryValue::from_raw(buf, co::REG::from_raw(data_type)) };
				},
				co::ERROR::MORE_DATA => continue, // value changed in a concurrent operation; retry
				e => return Err(e),
			}
		}
	}

	/// [`RegLoadKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regloadkeyw)
	/// function.
	pub fn RegLoadKey(&self, sub_key: Option<&str>, file_path: &str) -> SysResult<()> {
		ErrorRet(unsafe {
			ffi::RegLoadKeyW(
				self.ptr(),
				WString::from_opt_str(sub_key).as_ptr(),
				WString::from_str(file_path).as_ptr(),
			)
		})
		.to_sysresult()
	}

	/// [`RegOpenCurrentUser`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopencurrentuser)
	/// function.
	#[must_use]
	pub fn RegOpenCurrentUser(access_rights: co::KEY) -> SysResult<RegCloseKeyGuard> {
		let mut hkey = HKEY::NULL;
		unsafe {
			ErrorRet(ffi::RegOpenCurrentUser(access_rights.raw(), hkey.as_mut()))
				.to_sysresult()
				.map(|_| RegCloseKeyGuard::new(hkey))
		}
	}

	/// [`RegOpenKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel\\Mouse"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn RegOpenKeyEx(
		&self,
		sub_key: Option<&str>,
		options: co::REG_OPTION,
		access_rights: co::KEY,
	) -> SysResult<RegCloseKeyGuard> {
		let mut hkey = HKEY::NULL;
		unsafe {
			ErrorRet(ffi::RegOpenKeyExW(
				self.ptr(),
				WString::from_opt_str(sub_key).as_ptr(),
				options.raw(),
				access_rights.raw(),
				hkey.as_mut(),
			))
			.to_sysresult()
			.map(|_| RegCloseKeyGuard::new(hkey))
		}
	}

	/// [`RegOpenKeyTransacted`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeytransactedw)
	/// function.
	#[must_use]
	pub fn RegOpenKeyTransacted(
		&self,
		sub_key: &str,
		options: co::REG_OPTION,
		access_rights: co::KEY,
		htransaction: &HTRANSACTION,
	) -> SysResult<RegCloseKeyGuard> {
		let mut hkey = HKEY::NULL;
		unsafe {
			ErrorRet(ffi::RegOpenKeyTransactedW(
				self.ptr(),
				WString::from_str(sub_key).as_ptr(),
				options.raw(),
				access_rights.raw(),
				hkey.as_mut(),
				htransaction.ptr(),
				std::ptr::null_mut(),
			))
			.to_sysresult()
			.map(|_| RegCloseKeyGuard::new(hkey))
		}
	}

	/// [`RegQueryInfoKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryinfokeyw)
	/// function.
	pub fn RegQueryInfoKey(
		&self,
		mut class: Option<&mut WString>,
		num_sub_keys: Option<&mut u32>,
		max_sub_key_name_len: Option<&mut u32>,
		max_class_len: Option<&mut u32>,
		num_values: Option<&mut u32>,
		max_value_name_len: Option<&mut u32>,
		max_value_len: Option<&mut u32>,
		security_descr_len: Option<&mut u32>,
		last_write_time: Option<&mut FILETIME>,
	) -> SysResult<()> {
		let (mut class_ptr, mut class_len) = match &mut class {
			Some(class) => {
				if class.buf_len() < WString::SSO_LEN {
					// start with no string heap allocation
					**class = WString::new_alloc_buf(WString::SSO_LEN); // make buffer at least this length
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
		let last_write_time = last_write_time.map_or(std::ptr::null_mut(), |re| pvoid(re));

		// Loop until class is large enough.
		loop {
			match unsafe {
				co::ERROR::from_raw(ffi::RegQueryInfoKeyW(
					self.ptr(),
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
				) as _)
			} {
				co::ERROR::MORE_DATA => match &mut class {
					Some(class) => {
						**class = WString::new_alloc_buf(class.buf_len() * 2); // double the buffer size to try again
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
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel\\Desktop"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// for val in hkey.RegQueryMultipleValues(&["DpiScalingVer", "WallPaper"])? {
	///     println!("{val}");
	/// }
	///
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn RegQueryMultipleValues(
		&self,
		value_names: &[impl AsRef<str>],
	) -> SysResult<Vec<RegistryValue>> {
		let mut valents = vec![VALENT::default(); value_names.len()];
		let value_names_w = value_names
			.iter()
			.map(|value_name| WString::from_str(value_name.as_ref()))
			.collect::<Vec<_>>();
		valents
			.iter_mut()
			.zip(value_names_w.iter())
			.for_each(|(valent, value_name_w)| valent.ve_valuename = value_name_w.as_ptr() as _);
		let mut buf = Vec::<u8>::default();

		loop {
			let mut data_len = 0u32;

			match unsafe {
				co::ERROR::from_raw(ffi::RegQueryMultipleValuesW(
					self.ptr(),
					valents.as_mut_ptr() as _,
					value_names.len() as _,
					std::ptr::null_mut(),
					&mut data_len, // first call to retrieve size only
				) as _)
			} {
				co::ERROR::MORE_DATA => {},
				e => return Err(e),
			}

			buf.resize(data_len as _, 0x00);

			match unsafe {
				co::ERROR::from_raw(ffi::RegQueryMultipleValuesW(
					self.ptr(),
					valents.as_mut_ptr() as _,
					value_names.len() as _,
					buf.as_mut_ptr() as _,
					&mut data_len,
				) as _)
			} {
				co::ERROR::SUCCESS => {
					buf.resize(data_len as _, 0x00); // data length may have shrunk
					return valents
						.iter()
						.map(|valent| unsafe {
							RegistryValue::from_raw(
								valent.buf_projection(&buf).to_vec(),
								valent.ve_type,
							)
						})
						.collect::<SysResult<Vec<_>>>();
				},
				co::ERROR::MORE_DATA => continue, // value changed in a concurrent operation; retry
				e => return Err(e),
			}
		}
	}

	/// [`RegQueryReflectionKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryreflectionkey)
	/// function.
	#[must_use]
	pub fn RegQueryReflectionKey(&self) -> SysResult<bool> {
		let mut is_disabled = 0;
		ErrorRet(unsafe { ffi::RegQueryReflectionKey(self.ptr(), &mut is_disabled) })
			.to_sysresult()
			.map(|_| is_disabled != 0)
	}

	/// [`RegQueryValueEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regqueryvalueexw)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Control Panel\\Mouse"),
	///     co::REG_OPTION::default(),
	///     co::KEY::READ,
	/// )?;
	///
	/// let val = hkey.RegQueryValueEx(Some("Beep"))?;
	/// println!("{val}");
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	pub fn RegQueryValueEx(&self, value_name: Option<&str>) -> SysResult<RegistryValue> {
		let value_name_w = WString::from_opt_str(value_name);
		let mut buf = Vec::<u8>::default();

		loop {
			let mut data_len = 0u32; // in bytes

			ErrorRet(unsafe {
				ffi::RegQueryValueExW(
					self.ptr(),
					value_name_w.as_ptr(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					&mut data_len, // first call to retrieve size only
				)
			})
			.to_sysresult()?;

			buf.resize(data_len as _, 0x00);
			let mut data_type = 0u32;

			match unsafe {
				co::ERROR::from_raw(ffi::RegQueryValueExW(
					self.ptr(),
					value_name_w.as_ptr(),
					std::ptr::null_mut(),
					&mut data_type,
					buf.as_mut_ptr() as _,
					&mut data_len,
				) as _)
			} {
				co::ERROR::SUCCESS => {
					buf.resize(data_len as _, 0x00); // data length may have shrunk
					return unsafe { RegistryValue::from_raw(buf, co::REG::from_raw(data_type)) };
				},
				co::ERROR::MORE_DATA => continue, // value changed in a concurrent operation; retry
				e => return Err(e),
			}
		}
	}

	/// [`RegRenameKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regrenamekey)
	/// function.
	pub fn RegRenameKey(&self, sub_key_name: &str, new_key_name: &str) -> SysResult<()> {
		ErrorRet(unsafe {
			ffi::RegRenameKey(
				self.ptr(),
				WString::from_str(sub_key_name).as_ptr(),
				WString::from_str(new_key_name).as_ptr(),
			)
		})
		.to_sysresult()
	}

	/// [`RegReplaceKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regreplacekeyw)
	/// function.
	pub fn RegReplaceKey(
		&self,
		sub_key: Option<&str>,
		new_src_file: &str,
		old_file_backup: &str,
	) -> SysResult<()> {
		ErrorRet(unsafe {
			ffi::RegReplaceKeyW(
				self.ptr(),
				WString::from_opt_str(sub_key).as_ptr(),
				WString::from_str(new_src_file).as_ptr(),
				WString::from_str(old_file_backup).as_ptr(),
			)
		})
		.to_sysresult()
	}

	/// [`RegRestoreKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regrestorekeyw)
	/// function.
	pub fn RegRestoreKey(&self, file_path: &str, flags: co::REG_RESTORE) -> SysResult<()> {
		ErrorRet(unsafe {
			ffi::RegRestoreKeyW(self.ptr(), WString::from_str(file_path).as_ptr(), flags.raw())
		})
		.to_sysresult()
	}

	/// [`RegSaveKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsavekeyw)
	/// function.
	pub fn RegSaveKey(
		&self,
		dest_file_path: &str,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
	) -> SysResult<()> {
		ErrorRet(unsafe {
			ffi::RegSaveKeyW(
				self.ptr(),
				WString::from_str(dest_file_path).as_ptr(),
				pcvoid_or_null(security_attributes),
			)
		})
		.to_sysresult()
	}

	/// [`RegSaveKeyEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsavekeyexw)
	/// function.
	pub fn RegSaveKeyEx(
		&self,
		dest_file_path: &str,
		security_attributes: Option<&SECURITY_ATTRIBUTES>,
		flags: co::REG_SAVE,
	) -> SysResult<()> {
		ErrorRet(unsafe {
			ffi::RegSaveKeyExW(
				self.ptr(),
				WString::from_str(dest_file_path).as_ptr(),
				pcvoid_or_null(security_attributes),
				flags.raw(),
			)
		})
		.to_sysresult()
	}

	/// [`RegSetKeyValue`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetkeyvaluew)
	/// function.
	///
	/// If the value doesn't exist, if will be created. If new type is different
	/// from current type, new type will take over.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// w::HKEY::CURRENT_USER.RegSetKeyValue(
	///     Some("Software\\My Company"),
	///     Some("Color"),
	///     w::RegistryValue::Sz("blue".to_owned()),
	/// )?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn RegSetKeyValue(
		&self,
		sub_key: Option<&str>,
		value_name: Option<&str>,
		data: RegistryValue,
	) -> SysResult<()> {
		let mut str_buf = WString::new();
		let (data_ptr, data_len) = data.as_ptr_with_len(&mut str_buf);

		ErrorRet(unsafe {
			ffi::RegSetKeyValueW(
				self.ptr(),
				WString::from_opt_str(sub_key).as_ptr(),
				WString::from_opt_str(value_name).as_ptr(),
				data.reg_type().raw(),
				data_ptr,
				data_len,
			)
		})
		.to_sysresult()
	}

	/// [`RegSetValueEx`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regsetvalueexw)
	/// function.
	///
	/// If the value doesn't exist, if will be created. If new type is different
	/// from current type, new type will prevail.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let hkey = w::HKEY::CURRENT_USER.RegOpenKeyEx(
	///     Some("Console\\Git Bash"),
	///     co::REG_OPTION::default(),
	///     co::KEY::ALL_ACCESS,
	/// )?;
	///
	/// hkey.RegSetValueEx(
	///     Some("Color"),
	///     w::RegistryValue::Sz("blue".to_owned()),
	/// )?;
	/// # w::SysResult::Ok(())
	/// ```
	pub fn RegSetValueEx(&self, value_name: Option<&str>, data: RegistryValue) -> SysResult<()> {
		let mut str_buf = WString::new();
		let (data_ptr, data_len) = data.as_ptr_with_len(&mut str_buf);

		ErrorRet(unsafe {
			ffi::RegSetValueExW(
				self.ptr(),
				WString::from_opt_str(value_name).as_ptr(),
				0,
				data.reg_type().raw(),
				data_ptr as _,
				data_len,
			)
		})
		.to_sysresult()
	}

	/// [`RegUnLoadKey`](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regunloadkeyw)
	/// function.
	pub fn RegUnLoadKey(&self, sub_key: Option<&str>) -> SysResult<()> {
		ErrorRet(unsafe { ffi::RegUnLoadKeyW(self.ptr(), WString::from_opt_str(sub_key).as_ptr()) })
			.to_sysresult()
	}
}
