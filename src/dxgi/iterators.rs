use crate::co;
use crate::decl::*;
use crate::dxgi::vts::*;
use crate::ole::privs::*;
use crate::prelude::*;

pub(in crate::dxgi) struct IdxgiadapterEnumoutputsIter<'a, I>
where
	I: dxgi_IDXGIAdapter,
{
	adapter: &'a I,
	cur_index: u32,
}

impl<'a, I> Iterator for IdxgiadapterEnumoutputsIter<'a, I>
where
	I: dxgi_IDXGIAdapter,
{
	type Item = HrResult<IDXGIOutput>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur_index == 0xffff_ffff {
			None
		} else {
			let mut queried = unsafe { IDXGIOutput::null() };
			match HrRet(unsafe {
				(vt::<IDXGIAdapterVT>(self.adapter).EnumOutputs)(
					self.adapter.ptr(),
					self.cur_index,
					queried.as_mut(),
				)
			})
			.to_hrresult()
			{
				Err(err) => {
					self.cur_index = 0xffff_ffff; // no further iterations will be made
					match err {
						co::HRESULT::DXGI_ERROR_NOT_FOUND => None, // no more entries
						_ => Some(Err(err)),                       // actual error
					}
				},
				Ok(_) => {
					self.cur_index += 1;
					Some(Ok(queried))
				},
			}
		}
	}
}

impl<'a, I> IdxgiadapterEnumoutputsIter<'a, I>
where
	I: dxgi_IDXGIAdapter,
{
	#[must_use]
	pub(in crate::dxgi) const fn new(adapter: &'a I) -> Self {
		Self { adapter, cur_index: 0 }
	}
}

pub(in crate::dxgi) struct IdxgifactoryEnumadaptersIter<'a, I>
where
	I: dxgi_IDXGIFactory,
{
	fact: &'a I,
	cur_index: u32,
}

impl<'a, I> Iterator for IdxgifactoryEnumadaptersIter<'a, I>
where
	I: dxgi_IDXGIFactory,
{
	type Item = HrResult<IDXGIAdapter>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur_index == 0xffff_ffff {
			None
		} else {
			let mut queried = unsafe { IDXGIAdapter::null() };
			match HrRet(unsafe {
				(vt::<IDXGIFactoryVT>(self.fact).EnumAdapters)(
					self.fact.ptr(),
					self.cur_index,
					queried.as_mut(),
				)
			})
			.to_hrresult()
			{
				Err(err) => {
					self.cur_index = 0xffff_ffff; // no further iterations will be made
					match err {
						co::HRESULT::DXGI_ERROR_NOT_FOUND => None, // no more entries
						_ => Some(Err(err)),                       // actual error
					}
				},
				Ok(_) => {
					self.cur_index += 1;
					Some(Ok(queried))
				},
			}
		}
	}
}

impl<'a, I> IdxgifactoryEnumadaptersIter<'a, I>
where
	I: dxgi_IDXGIFactory,
{
	#[must_use]
	pub(in crate::dxgi) const fn new(fact: &'a I) -> Self {
		Self { fact, cur_index: 0 }
	}
}

pub(in crate::dxgi) struct IdxgifactoryEnumadapters1Iter<'a, I>
where
	I: dxgi_IDXGIFactory1,
{
	fact: &'a I,
	cur_index: u32,
}

impl<'a, I> Iterator for IdxgifactoryEnumadapters1Iter<'a, I>
where
	I: dxgi_IDXGIFactory1,
{
	type Item = HrResult<IDXGIAdapter1>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur_index == 0xffff_ffff {
			None
		} else {
			let mut queried = unsafe { IDXGIAdapter1::null() };
			match HrRet(unsafe {
				(vt::<IDXGIFactory1VT>(self.fact).EnumAdapters1)(
					self.fact.ptr(),
					self.cur_index,
					queried.as_mut(),
				)
			})
			.to_hrresult()
			{
				Err(err) => {
					self.cur_index = 0xffff_ffff; // no further iterations will be made
					match err {
						co::HRESULT::DXGI_ERROR_NOT_FOUND => None, // no more entries
						_ => Some(Err(err)),                       // actual error
					}
				},
				Ok(_) => {
					self.cur_index += 1;
					Some(Ok(queried))
				},
			}
		}
	}
}

impl<'a, I> IdxgifactoryEnumadapters1Iter<'a, I>
where
	I: dxgi_IDXGIFactory1,
{
	#[must_use]
	pub(in crate::dxgi) const fn new(fact: &'a I) -> Self {
		Self { fact, cur_index: 0 }
	}
}
