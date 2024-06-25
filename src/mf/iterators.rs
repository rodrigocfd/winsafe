use crate::decl::*;
use crate::prelude::*;

pub(in crate::mf) struct ImfcollectionIter<'a, I>
	where I: mf_IMFCollection,
{
	collection: &'a I,
	count: u32,
	current: u32,
}

impl<'a, I> Iterator for ImfcollectionIter<'a, I>
	where I: mf_IMFCollection,
{
	type Item = HrResult<IUnknown>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match self.collection.GetElement(self.current) {
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
			Ok(element) => match element {
				Some(element) => {
					self.current += 1;
					Some(Ok(element))
				},
				None => { // if a null pointer is returned, interpret as the end
					self.current = self.count; // no further iterations will be made
					None
				},
			},
		}
	}
}

impl<'a, I> ImfcollectionIter<'a, I>
	where I: mf_IMFCollection,
{
	#[must_use]
	pub(in crate::mf) fn new(collection: &'a I) -> HrResult<Self> {
		let count = collection.GetElementCount()?;
		Ok(Self { collection, count, current: 0 })
	}
}
