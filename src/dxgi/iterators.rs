use crate::co;
use crate::dxgi::decl::{IDXGIAdapter, IDXGIOutput};
use crate::ole::decl::HrResult;
use crate::prelude::{dxgi_IDXGIAdapter, dxgi_IDXGIFactory};

pub(in crate::dxgi) struct IdxgiadapterOutputsIter<'a, I>
	where I: dxgi_IDXGIAdapter,
{
	adapter: &'a I,
	cur_index: u32,
}

impl<'a, I> Iterator for IdxgiadapterOutputsIter<'a, I>
	where I: dxgi_IDXGIAdapter,
{
	type Item = HrResult<IDXGIOutput>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur_index == 0xffff_ffff {
			None
		} else {
			match self.adapter.EnumOutputs(self.cur_index) {
				Err(err) => {
					self.cur_index = 0xffff_ffff; // no further iterations will be made
					match err {
						co::HRESULT::DXGI_ERROR_NOT_FOUND => None, // no more entries
						_ => Some(Err(err)), // actual error
					}
				},
				Ok(output) => {
					self.cur_index += 1;
					Some(Ok(output))
				},
			}
		}
	}
}

impl<'a, I> IdxgiadapterOutputsIter<'a, I>
	where I: dxgi_IDXGIAdapter,
{
	pub(in crate::dxgi) fn new(adapter: &'a I) -> Self {
		Self { adapter, cur_index: 0 }
	}
}

//------------------------------------------------------------------------------

pub(in crate::dxgi) struct IdxgifactoryAdaptersIter<'a, I>
	where I: dxgi_IDXGIFactory,
{
	fact: &'a I,
	cur_index: u32,
}

impl<'a, I> Iterator for IdxgifactoryAdaptersIter<'a, I>
	where I: dxgi_IDXGIFactory,
{
	type Item = HrResult<IDXGIAdapter>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur_index == 0xffff_ffff {
			None
		} else {
			match self.fact.EnumAdapters(self.cur_index) {
				Err(err) => {
					self.cur_index = 0xffff_ffff; // no further iterations will be made
					match err {
						co::HRESULT::DXGI_ERROR_NOT_FOUND => None, // no more entries
						_ => Some(Err(err)), // actual error
					}
				},
				Ok(adapter) => {
					self.cur_index += 1;
					Some(Ok(adapter))
				},
			}
		}
	}
}

impl<'a, I> IdxgifactoryAdaptersIter<'a, I>
	where I: dxgi_IDXGIFactory,
{
	pub(in crate::dxgi) fn new(fact: &'a I) -> Self {
		Self { fact, cur_index: 0 }
	}
}
