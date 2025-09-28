use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::user::ffi;

pub(in crate::user) struct EnumdisplaydevicesIter<'a> {
	device_name: Option<&'a str>,
	display_device: DISPLAY_DEVICE,
	flags: Option<co::EDD>,
	idev_num: u32,
}

impl<'a> Iterator for EnumdisplaydevicesIter<'a> {
	type Item = SysResult<&'a DISPLAY_DEVICE>;

	fn next(&mut self) -> Option<Self::Item> {
		match unsafe {
			ffi::EnumDisplayDevicesW(
				WString::from_opt_str(self.device_name).as_ptr(),
				self.idev_num,
				pvoid(&mut self.display_device),
				self.flags.unwrap_or_default().raw(),
			)
		} {
			// Empirical tests have shown that two different error codes can be
			// returned to signal the end of the loop, so we consider both.
			// https://github.com/rodrigocfd/winsafe/issues/36
			0 => match GetLastError() {
				co::ERROR::SUCCESS | co::ERROR::PROC_NOT_FOUND | co::ERROR::ENVVAR_NOT_FOUND => {
					None
				},
				err => Some(Err(err)), // actual error
			},
			_ => {
				self.idev_num += 1;
				// Returning a reference cannot be done until GATs
				// stabilization, so we simply cheat the borrow checker.
				let ptr = &self.display_device as *const DISPLAY_DEVICE;
				Some(Ok(unsafe { &*ptr }))
			},
		}
	}
}

impl<'a> EnumdisplaydevicesIter<'a> {
	#[must_use]
	pub(in crate::user) fn new(device_name: Option<&'a str>, flags: Option<co::EDD>) -> Self {
		Self {
			device_name,
			display_device: DISPLAY_DEVICE::default(),
			idev_num: 0,
			flags,
		}
	}
}

pub(in crate::user) struct HmenuIteritems<'a> {
	hmenu: &'a HMENU,
	double_idx: DoubleIterIndex,
}

impl<'a> Iterator for HmenuIteritems<'a> {
	type Item = SysResult<MenuItemInfo>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for HmenuIteritems<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> HmenuIteritems<'a> {
	#[must_use]
	pub(in crate::user) fn new(hmenu: &'a HMENU) -> SysResult<Self> {
		Ok(Self {
			hmenu,
			double_idx: DoubleIterIndex::new(hmenu.GetMenuItemCount()?),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<SysResult<MenuItemInfo>> {
		self.double_idx.grab(is_front, |cur_idx| {
			let nfo = self.hmenu.item_info(IdPos::Pos(cur_idx));
			DoubleIter::Yield(nfo)
		})
	}
}
