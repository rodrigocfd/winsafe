use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::shell::ffi;

pub(in crate::shell) struct HdropIter<'a> {
	hdrop: &'a HDROP,
	buffer: WString,
	count: u32,
	current: u32,
}

impl<'a> Iterator for HdropIter<'a> {
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match unsafe {
			ffi::DragQueryFileW(
				self.hdrop.ptr(),
				self.current,
				self.buffer.as_mut_ptr(),
				self.buffer.buf_len() as _,
			)
		} {
			0 => {
				self.current = self.count; // no further iterations will be made
				Some(Err(GetLastError()))
			},
			_ => {
				self.current += 1;
				Some(Ok(self.buffer.to_string()))
			},
		}
	}
}

impl<'a> HdropIter<'a> {
	#[must_use]
	pub(in crate::shell) fn new(hdrop: &'a HDROP) -> SysResult<Self> {
		let count = unsafe {
			ffi::DragQueryFileW(hdrop.ptr(), 0xffff_ffff, std::ptr::null_mut(), 0) // preliminar call to retrieve the file count
		};

		Ok(Self {
			hdrop,
			buffer: WString::new_alloc_buf(MAX_PATH + 1), // so we alloc just once
			count,
			current: 0,
		})
	}
}

pub(in crate::shell) struct IenumshellitemsIter<'a, I>
where
	I: shell_IEnumShellItems,
{
	enum_shi: &'a I,
}

impl<'a, I> Iterator for IenumshellitemsIter<'a, I>
where
	I: shell_IEnumShellItems,
{
	type Item = HrResult<IShellItem>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.enum_shi.Next() {
			Err(err) => Some(Err(err)),
			Ok(maybe_item) => maybe_item.map(|item| Ok(item)),
		}
	}
}

impl<'a, I> IenumshellitemsIter<'a, I>
where
	I: shell_IEnumShellItems,
{
	#[must_use]
	pub(in crate::shell) fn new(enum_shi: &'a I) -> Self {
		Self { enum_shi }
	}
}

pub(in crate::shell) struct IshellitemarrayIter<'a, I>
where
	I: shell_IShellItemArray,
{
	shi_arr: &'a I,
	front_idx: u32,
	past_back_idx: u32,
}

impl<'a, I> Iterator for IshellitemarrayIter<'a, I>
where
	I: shell_IShellItemArray,
{
	type Item = HrResult<IShellItem>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a, I> DoubleEndedIterator for IshellitemarrayIter<'a, I>
where
	I: shell_IShellItemArray,
{
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a, I> IshellitemarrayIter<'a, I>
where
	I: shell_IShellItemArray,
{
	#[must_use]
	pub(in crate::shell) fn new(shi_arr: &'a I) -> HrResult<Self> {
		let count = shi_arr.GetCount()?;
		Ok(Self {
			shi_arr,
			front_idx: 0,
			past_back_idx: count,
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<HrResult<IShellItem>> {
		if self.front_idx == self.past_back_idx {
			return None;
		}
		let our_idx = if is_front { self.front_idx } else { self.past_back_idx - 1 };

		match self.shi_arr.GetItemAt(our_idx) {
			Err(e) => {
				(self.front_idx, self.past_back_idx) = (0, 0); // halt
				Some(Err(e))
			},
			Ok(shell_item) => {
				if is_front {
					self.front_idx += 1;
				} else {
					self.past_back_idx -= 1;
				}
				Some(Ok(shell_item))
			},
		}
	}
}
