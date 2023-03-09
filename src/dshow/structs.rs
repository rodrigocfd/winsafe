#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::dshow::decl::IBaseFilter;
use crate::kernel::decl::GUID;
use crate::kernel::ffi_types::BOOL;
use crate::ole::decl::ComPtr;
use crate::prelude::dshow_IFilterGraph;

/// [`AM_MEDIA_TYPE`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/ns-strmif-am_media_type)
/// struct.
#[repr(C)]
pub struct AM_MEDIA_TYPE<'a> {
	pub majortype: co::MEDIATYPE,
	pub subtype: GUID,
	bFixedSizeSamples: BOOL,
	bTemporalCompression: BOOL,
	pub lSampleSize: u32,
	pub formattype: co::MEDIA_FORMAT,
	pUnk: usize,
	cbFormat: u32,
	pbFormat: *mut std::ffi::c_void,

	_pbFormat: PhantomData<&'a mut usize>,
}

impl_default!(AM_MEDIA_TYPE, 'a);

impl<'a> AM_MEDIA_TYPE<'a> {
	pub_fn_bool_get_set!(bFixedSizeSamples, set_bFixedSizeSamples);
	pub_fn_bool_get_set!(bTemporalCompression, set_bTemporalCompression);

	/// Returns the `pbFormat` field.
	///
	/// # Safety
	///
	/// Varies according to the `formattype`. If you set it wrong, you're likely
	/// to cause a buffer overrun.
	#[must_use]
	pub unsafe fn pbFormat<T>(&self) -> Option<&mut T> {
		(self.pbFormat as *mut T).as_mut()
	}

	/// Sets the `pbFormat` field.
	///
	/// # Safety
	///
	/// Varies according to the `formattype`. If you set it wrong, you're likely
	/// to cause a buffer overrun.
	pub unsafe fn set_pbFormat<T>(&mut self, val: &'a mut T) {
		self.pbFormat = val as *mut _ as _;
		self.cbFormat = std::mem::size_of::<T>() as _;
	}
}

/// [`DVINFO`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/ns-strmif-dvinfo)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct DVINFO {
	pub dwDVAAuxSrc: u32,
	pub dwDVAAuxCtl: u32,
	pub dwDVAAuxSrc1: u32,
	pub dwDVAAuxCtl1: u32,
	pub dwDVVAuxSrc: u32,
	pub dwDVVAuxCtl: u32,
	dwDVReserved: [u32; 2],
}

/// [`FILTER_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/ns-strmif-filter_info)
/// struct.
#[repr(C)]
pub struct FILTER_INFO {
	achName: [u16; 128],
	pGraph: ComPtr,
}

impl_default!(FILTER_INFO);
impl_drop_comptr!(pGraph, FILTER_INFO);

impl FILTER_INFO {
	pub_fn_string_arr_get_set!(achName, set_achName);
	pub_fn_comptr_get_set!(pGraph, set_pGraph, dshow_IFilterGraph);
}

/// [`MFVideoNormalizedRect`](https://learn.microsoft.com/en-us/windows/win32/api/evr/ns-evr-mfvideonormalizedrect)
/// struct.
#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq)]
pub struct MFVideoNormalizedRect {
	pub left: f32,
	pub top: f32,
	pub right: f32,
	pub bottom: f32,
}

impl std::fmt::Display for MFVideoNormalizedRect {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "left {:.2}, top {:.2}, right {:.2}, bottom {:.2}",
			self.left, self.top, self.right, self.bottom)
	}
}

impl MFVideoNormalizedRect {
	/// Creates a new `MFVideoNormalizedRect`.
	#[must_use]
	pub const fn new(
		left: f32, top: f32, right: f32, bottom: f32) -> MFVideoNormalizedRect
	{
		Self { left, top, right, bottom }
	}
}

/// [`PIN_INFO`](https://learn.microsoft.com/en-us/windows/win32/api/strmif/ns-strmif-pin_info)
/// struct.
#[repr(C)]
pub struct PIN_INFO {
	pub pFilter: IBaseFilter,
	pub dir: co::PIN_DIRECTION,
	achName: [u16; 128],
}

impl_default!(PIN_INFO);

impl PIN_INFO {
	pub_fn_string_arr_get_set!(achName, set_achName);
}
