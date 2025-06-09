use crate::co;
use crate::decl::*;
use crate::prelude::*;

pub(in crate::oleaut) struct IpropertystoreIter<'a, I>
where
	I: oleaut_IPropertyStore,
{
	prop_st: &'a I,
	count: u32,
	current: u32,
}

impl<'a, I> Iterator for IpropertystoreIter<'a, I>
where
	I: oleaut_IPropertyStore,
{
	type Item = HrResult<co::PKEY>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match self.prop_st.GetAt(self.current) {
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
			Ok(pkey) => {
				self.current += 1;
				Some(Ok(pkey))
			},
		}
	}
}

impl<'a, I> IpropertystoreIter<'a, I>
where
	I: oleaut_IPropertyStore,
{
	#[must_use]
	pub(in crate::oleaut) fn new(prop_st: &'a I) -> HrResult<Self> {
		let count = prop_st.GetCount()?;
		Ok(Self { prop_st, count, current: 0 })
	}
}
