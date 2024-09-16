use crate::decl::*;
use crate::guard::*;
use crate::prelude::*;

pub(in crate::comctl) struct HimagelistIter<'a, H>
	where H: comctl_Himagelist,
{
	himagelist: &'a H,
	num_items: u32,
	current: u32,
}

impl<'a, H> Iterator for HimagelistIter<'a, H>
	where H: comctl_Himagelist,
{
	type Item = HrResult<DestroyIconGuard>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.num_items {
			None
		} else {
			Some(self.himagelist.ExtractIcon(self.current))
		}
	}
}

impl<'a, H> HimagelistIter<'a, H>
	where H: comctl_Himagelist,
{
	#[must_use]
	pub(in crate::comctl) fn new(himagelist: &'a H) -> Self {
		Self {
			himagelist,
			num_items: himagelist.GetImageCount(),
			current: 0,
		}
	}
}
