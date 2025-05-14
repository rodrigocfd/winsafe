use crate::advapi::ffi;
use crate::co;
use crate::decl::*;
use crate::prelude::*;

pub(in crate::advapi) struct HkeyKeyIter<'a, H>
where
	H: advapi_Hkey,
{
	hkey: &'a H,
	front_idx: u32,
	past_back_idx: u32,
	name_buffer: WString,
}

impl<'a, H> Iterator for HkeyKeyIter<'a, H>
where
	H: advapi_Hkey,
{
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a, H> DoubleEndedIterator for HkeyKeyIter<'a, H>
where
	H: advapi_Hkey,
{
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a, H> HkeyKeyIter<'a, H>
where
	H: advapi_Hkey,
{
	#[must_use]
	pub(in crate::advapi) fn new(hkey: &'a H) -> SysResult<Self> {
		let mut num_keys = u32::default();
		let mut max_key_name_len = u32::default();
		hkey.RegQueryInfoKey(
			None,
			Some(&mut num_keys),
			Some(&mut max_key_name_len),
			None,
			None,
			None,
			None,
			None,
			None,
		)?;

		Ok(Self {
			hkey,
			front_idx: 0,
			past_back_idx: num_keys,
			name_buffer: WString::new_alloc_buf(max_key_name_len as usize + 1),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<SysResult<String>> {
		if self.front_idx == self.past_back_idx {
			return None;
		}
		let our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };

		let mut len_buffer = self.name_buffer.buf_len() as u32;
		match unsafe {
			co::ERROR::from_raw(ffi::RegEnumKeyExW(
				self.hkey.ptr(),
				our_idx,
				self.name_buffer.as_mut_ptr(),
				&mut len_buffer,
				std::ptr::null_mut(),
				std::ptr::null_mut(),
				std::ptr::null_mut(),
				std::ptr::null_mut(),
			) as _)
		} {
			co::ERROR::SUCCESS => {
				if is_front {
					self.front_idx += 1;
				} else {
					self.past_back_idx -= 1;
				}
				Some(Ok(self.name_buffer.to_string()))
			},
			e => {
				(self.front_idx, self.past_back_idx) = (0, 0); // halt
				Some(Err(e))
			},
		}
	}
}

pub(in crate::advapi) struct HkeyValueIter<'a, H>
where
	H: advapi_Hkey,
{
	hkey: &'a H,
	front_idx: u32,
	past_back_idx: u32,
	name_buffer: WString,
}

impl<'a, H> Iterator for HkeyValueIter<'a, H>
where
	H: advapi_Hkey,
{
	type Item = SysResult<(String, co::REG)>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a, H> DoubleEndedIterator for HkeyValueIter<'a, H>
where
	H: advapi_Hkey,
{
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a, H> HkeyValueIter<'a, H>
where
	H: advapi_Hkey,
{
	#[must_use]
	pub(in crate::advapi) fn new(hkey: &'a H) -> SysResult<Self> {
		let mut num_vals = u32::default();
		let mut max_val_name_len = u32::default();
		hkey.RegQueryInfoKey(
			None,
			None,
			None,
			None,
			Some(&mut num_vals),
			Some(&mut max_val_name_len),
			None,
			None,
			None,
		)?;

		Ok(Self {
			hkey,
			front_idx: 0,
			past_back_idx: num_vals,
			name_buffer: WString::new_alloc_buf(max_val_name_len as usize + 1),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<SysResult<(String, co::REG)>> {
		if self.front_idx == self.past_back_idx {
			return None;
		}
		let our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };

		let mut raw_data_type = u32::default();
		let mut len_buffer = self.name_buffer.buf_len() as u32;
		match unsafe {
			co::ERROR::from_raw(ffi::RegEnumValueW(
				self.hkey.ptr(),
				our_idx,
				self.name_buffer.as_mut_ptr(),
				&mut len_buffer,
				std::ptr::null_mut(),
				&mut raw_data_type,
				std::ptr::null_mut(),
				std::ptr::null_mut(),
			) as _)
		} {
			co::ERROR::SUCCESS => {
				if is_front {
					self.front_idx += 1;
				} else {
					self.past_back_idx -= 1;
				}
				Some(Ok((self.name_buffer.to_string(), unsafe {
					co::REG::from_raw(raw_data_type)
				})))
			},
			e => {
				(self.front_idx, self.past_back_idx) = (0, 0); // halt
				Some(Err(e))
			},
		}
	}
}
