use crate::advapi::ffi;
use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;

pub(in crate::advapi) struct HkeyKeyIter<'a> {
	hkey: &'a HKEY,
	double_idx: DoubleIterIndex,
	name_buffer: WString,
}

impl<'a> Iterator for HkeyKeyIter<'a> {
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for HkeyKeyIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> HkeyKeyIter<'a> {
	#[must_use]
	pub(in crate::advapi) fn new(hkey: &'a HKEY) -> SysResult<Self> {
		let mut num_keys = 0u32;
		let mut max_key_name_len = 0u32;
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
			double_idx: DoubleIterIndex::new(num_keys),
			name_buffer: WString::new_alloc_buf(max_key_name_len as usize + 1),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<SysResult<String>> {
		self.double_idx.grab(is_front, |cur_idx| {
			let mut len_buffer = self.name_buffer.buf_len() as u32;
			match unsafe {
				co::ERROR::from_raw(ffi::RegEnumKeyExW(
					self.hkey.ptr(),
					cur_idx,
					self.name_buffer.as_mut_ptr(),
					&mut len_buffer,
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
					std::ptr::null_mut(),
				) as _)
			} {
				co::ERROR::SUCCESS => DoubleIter::Yield(Ok(self.name_buffer.to_string())),
				e => DoubleIter::YieldLast(Err(e)),
			}
		})
	}
}

pub(in crate::advapi) struct HkeyValueIter<'a> {
	hkey: &'a HKEY,
	double_idx: DoubleIterIndex,
	name_buffer: WString,
}

impl<'a> Iterator for HkeyValueIter<'a> {
	type Item = SysResult<(String, co::REG)>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for HkeyValueIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> HkeyValueIter<'a> {
	#[must_use]
	pub(in crate::advapi) fn new(hkey: &'a HKEY) -> SysResult<Self> {
		let mut num_vals = 0u32;
		let mut max_val_name_len = 0u32;
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
			double_idx: DoubleIterIndex::new(num_vals),
			name_buffer: WString::new_alloc_buf(max_val_name_len as usize + 1),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<SysResult<(String, co::REG)>> {
		self.double_idx.grab(is_front, |cur_idx| {
			let mut raw_data_type = 0u32;
			let mut len_buffer = self.name_buffer.buf_len() as u32;
			match unsafe {
				co::ERROR::from_raw(ffi::RegEnumValueW(
					self.hkey.ptr(),
					cur_idx,
					self.name_buffer.as_mut_ptr(),
					&mut len_buffer,
					std::ptr::null_mut(),
					&mut raw_data_type,
					std::ptr::null_mut(),
					std::ptr::null_mut(),
				) as _)
			} {
				co::ERROR::SUCCESS => {
					DoubleIter::Yield(Ok((self.name_buffer.to_string(), unsafe {
						co::REG::from_raw(raw_data_type)
					})))
				},
				e => DoubleIter::YieldLast(Err(e)),
			}
		})
	}
}
