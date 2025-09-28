use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;

pub(in crate::comctl) struct HimagelistIter<'a> {
	himagelist: &'a HIMAGELIST,
	double_idx: DoubleIterIndex,
}

impl<'a> Iterator for HimagelistIter<'a> {
	type Item = HrResult<DestroyIconGuard>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a> DoubleEndedIterator for HimagelistIter<'a> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a> HimagelistIter<'a> {
	#[must_use]
	pub(in crate::comctl) fn new(himagelist: &'a HIMAGELIST) -> Self {
		Self {
			himagelist,
			double_idx: DoubleIterIndex::new(himagelist.GetImageCount()),
		}
	}

	fn grab(&mut self, is_front: bool) -> Option<HrResult<DestroyIconGuard>> {
		self.double_idx
			.grab(is_front, |cur_idx| match self.himagelist.ExtractIcon(cur_idx) {
				Ok(ico) => DoubleIter::Yield(Ok(ico)),
				Err(e) => DoubleIter::YieldLast(Err(e)),
			})
	}
}
