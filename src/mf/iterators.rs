use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;

pub(in crate::mf) struct ImfcollectionIter<'a, I>
where
	I: mf_IMFCollection,
{
	collection: &'a I,
	double_idx: DoubleIterIndex,
}

impl<'a, I> Iterator for ImfcollectionIter<'a, I>
where
	I: mf_IMFCollection,
{
	type Item = HrResult<IUnknown>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a, I> DoubleEndedIterator for ImfcollectionIter<'a, I>
where
	I: mf_IMFCollection,
{
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a, I> ImfcollectionIter<'a, I>
where
	I: mf_IMFCollection,
{
	#[must_use]
	pub(in crate::mf) fn new(collection: &'a I) -> HrResult<Self> {
		Ok(Self {
			collection,
			double_idx: DoubleIterIndex::new(collection.GetElementCount()?),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<HrResult<IUnknown>> {
		self.double_idx.grab(is_front, |cur_idx| {
			match self.collection.GetElement(cur_idx) {
				Ok(elem) => match elem {
					Some(elem) => DoubleIter::Yield(Ok(elem)),
					None => DoubleIter::Halt, // if a null pointer is returned, interpret as the end
				},
				Err(e) => DoubleIter::YieldLast(Err(e)),
			}
		})
	}
}
