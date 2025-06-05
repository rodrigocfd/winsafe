use crate::decl::*;
use crate::guard::*;

pub(in crate::comctl) struct HimagelistIter<'a> {
	himagelist: &'a HIMAGELIST,
	num_items: u32,
	current: u32,
}

impl<'a> Iterator for HimagelistIter<'a> {
	type Item = HrResult<DestroyIconGuard>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.num_items {
			None
		} else {
			Some(self.himagelist.ExtractIcon(self.current))
		}
	}
}

impl<'a> HimagelistIter<'a> {
	#[must_use]
	pub(in crate::comctl) fn new(himagelist: &'a HIMAGELIST) -> Self {
		Self {
			himagelist,
			num_items: himagelist.GetImageCount(),
			current: 0,
		}
	}
}
