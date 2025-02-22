use crate::advapi::ffi;
use crate::co;
use crate::decl::*;
use crate::prelude::*;

pub(in crate::advapi) struct HkeyKeyIter<'a, H>
	where H: advapi_Hkey,
{
	hkey: &'a H,
	count: u32,
	current: u32,
	name_buffer: WString,
}

impl<'a, H> Iterator for HkeyKeyIter<'a, H>
	where H: advapi_Hkey,
{
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let mut len_buffer = self.name_buffer.buf_len() as u32;
		match unsafe {
			co::ERROR::from_raw(
				ffi::RegEnumKeyExW(
					self.hkey.ptr(),
					self.current,
					self.name_buffer.as_mut_ptr(),
					&mut len_buffer,
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
				) as _,
			)
		} {
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

impl<'a, H> HkeyKeyIter<'a, H>
	where H: advapi_Hkey,
{
	#[must_use]
	pub(in crate::advapi) fn new(hkey: &'a H) -> SysResult<Self> {
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

pub(in crate::advapi) struct HkeyValueIter<'a, H>
	where H: advapi_Hkey,
{
	hkey: &'a H,
	count: u32,
	current: u32,
	name_buffer: WString,
}

impl<'a, H> Iterator for HkeyValueIter<'a, H>
	where H: advapi_Hkey,
{
	type Item = SysResult<(String, co::REG)>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		let mut raw_data_type = u32::default();
		let mut len_buffer = self.name_buffer.buf_len() as u32;
		match unsafe {
			co::ERROR::from_raw(
				ffi::RegEnumValueW(
					self.hkey.ptr(),
					self.current,
					self.name_buffer.as_mut_ptr(),
					&mut len_buffer,
					std::ptr::null_mut(),
					&mut raw_data_type,
					std::ptr::null_mut(),
					std::ptr::null_mut(),
				) as _,
			)
		} {
			co::ERROR::SUCCESS => {
				self.current += 1;
				Some(Ok((self.name_buffer.to_string(), unsafe { co::REG::from_raw(raw_data_type) })))
			},
			e => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
		}
	}
}

impl<'a, H> HkeyValueIter<'a, H>
	where H: advapi_Hkey,
{
	#[must_use]
	pub(in crate::advapi) fn new(hkey: &'a H) -> SysResult<Self> {
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
