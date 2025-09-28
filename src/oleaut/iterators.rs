use crate::co;
use crate::decl::*;
use crate::kernel::privs::*;
use crate::prelude::*;

pub(in crate::oleaut) struct IpropertystoreIter<'a, I>
where
	I: oleaut_IPropertyStore,
{
	prop_st: &'a I,
	double_idx: DoubleIterIndex,
}

impl<'a, I> Iterator for IpropertystoreIter<'a, I>
where
	I: oleaut_IPropertyStore,
{
	type Item = HrResult<co::PKEY>;

	fn next(&mut self) -> Option<Self::Item> {
		self.grab(true)
	}
}
impl<'a, I> DoubleEndedIterator for IpropertystoreIter<'a, I>
where
	I: oleaut_IPropertyStore,
{
	fn next_back(&mut self) -> Option<Self::Item> {
		self.grab(false)
	}
}

impl<'a, I> IpropertystoreIter<'a, I>
where
	I: oleaut_IPropertyStore,
{
	#[must_use]
	pub(in crate::oleaut) fn new(prop_st: &'a I) -> HrResult<Self> {
		Ok(Self {
			prop_st,
			double_idx: DoubleIterIndex::new(prop_st.GetCount()?),
		})
	}

	fn grab(&mut self, is_front: bool) -> Option<HrResult<co::PKEY>> {
		self.double_idx
			.grab(is_front, |cur_idx| match self.prop_st.GetAt(cur_idx) {
				Ok(pkey) => DoubleIter::Yield(Ok(pkey)),
				Err(e) => DoubleIter::YieldLast(Err(e)),
			})
	}
}
