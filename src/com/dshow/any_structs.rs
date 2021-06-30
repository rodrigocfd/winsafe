#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::ffi::BOOL;
use crate::structs::{BITMAPINFOHEADER, GUID, RECT};

/// [`AM_MEDIA_TYPE`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/ns-strmif-am_media_type)
/// struct.
#[repr(C)]
pub struct AM_MEDIA_TYPE<'a> {
	pub majortype: GUID,
	pub subtype: GUID,
	bFixedSizeSamples: BOOL,
	bTemporalCompression: BOOL,
	pub lSampleSize: u32,
	pub formattype: GUID,
	pUnk: usize,
	cbFormat: u32,
	pbFormat: *mut std::ffi::c_void,
	pbFormat_: PhantomData<&'a mut usize>,
}

impl_default!(AM_MEDIA_TYPE, 'a);

impl<'a> AM_MEDIA_TYPE<'a> {
	pub_fn_bool_get_set!(bFixedSizeSamples, set_bFixedSizeSamples);
	pub_fn_bool_get_set!(bTemporalCompression, set_bTemporalCompression);

	/// Returns the `pbFormat` field.
	///
	/// Varies according to the `formattype`. If you set it wrong, you're likely
	/// to cause a buffer overrun.
	pub unsafe fn pbFormat<T>(&self) -> Option<&mut T> {
		(self.pbFormat as *mut T).as_mut()
	}

	/// Sets the `pbFormat` field.
	///
	/// Varies according to the `formattype`. If you set it wrong, you're likely
	/// to cause a buffer overrun.
	pub unsafe fn set_pbFormat<T>(&mut self, val: &'a mut T) {
		self.pbFormat = val as *mut _ as _;
		self.cbFormat = std::mem::size_of::<T>() as _;
	}
}

/// [`DVINFO`](https://docs.microsoft.com/en-us/windows/win32/api/strmif/ns-strmif-dvinfo)
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

/// [`MFVideoNormalizedRect`](https://docs.microsoft.com/en-us/windows/win32/api/evr/ns-evr-mfvideonormalizedrect)
/// struct.
#[repr(C)]
#[derive(Default, Copy, Clone, PartialEq)]
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
	pub fn new(
		left: f32, top: f32, right: f32, bottom: f32) -> MFVideoNormalizedRect
	{
		Self { left, top, right, bottom }
	}
}

/// [`VIDEOINFOHEADER`](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/api/amvideo/ns-amvideo-videoinfoheader)
/// struct.
#[repr(C)]
#[derive(Default)]
pub struct VIDEOINFOHEADER {
	pub rcSource: RECT,
	pub rcTarget: RECT,
	pub dwBitRate: u32,
	pub dwBitErrorRate: u32,
	pub AvgTimePerFrame: i64,
	pub bmiHeader: BITMAPINFOHEADER,
}
