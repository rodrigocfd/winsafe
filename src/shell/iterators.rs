use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;
use crate::shell::ffi;

pub(in crate::shell) struct HdropIter<'a> {
	hdrop: &'a HDROP,
	double_idx: DoubleIterIndex,
	buffer: WString,
}

impl<'a> Iterator for HdropIter<'a> {
	type Item = SysResult<String>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for HdropIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> HdropIter<'a> {
	#[must_use]
	pub(in crate::shell) fn new(hdrop: &'a HDROP) -> SysResult<Self> {
		let count =
			unsafe { ffi::DragQueryFileW(hdrop.ptr(), 0xffff_ffff, std::ptr::null_mut(), 0) };
		if count == 0 {
			let err = GetLastError();
			if err != co::ERROR::SUCCESS {
				return Err(err);
			}
		}

		Ok(Self {
			hdrop,
			double_idx: DoubleIterIndex::new(count),
			buffer: WString::new_alloc_buf(MAX_PATH + 1), // so we alloc just once
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<SysResult<String>> {
		self.double_idx.grab(is_front, |cur_idx| {
			match unsafe {
				ffi::DragQueryFileW(
					self.hdrop.ptr(),
					cur_idx,
					self.buffer.as_mut_ptr(),
					self.buffer.buf_len() as _,
				)
			} {
				0 => DoubleIter::YieldLast(Err(GetLastError())),
				_ => DoubleIter::Yield(Ok(self.buffer.to_string())),
			}
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
	double_idx: DoubleIterIndex,
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
		Ok(Self {
			shi_arr,
			double_idx: DoubleIterIndex::new(shi_arr.GetCount()?),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<HrResult<IShellItem>> {
		self.double_idx
			.grab(is_front, |cur_idx| match self.shi_arr.GetItemAt(cur_idx) {
				Err(e) => DoubleIter::YieldLast(Err(e)),
				Ok(shell_item) => DoubleIter::Yield(Ok(shell_item)),
			})
	}
}
