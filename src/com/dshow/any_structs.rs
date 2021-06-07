#![allow(non_snake_case)]

use std::marker::PhantomData;

use crate::ffi::BOOL;
use crate::structs::GUID;

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
	pbFormat: usize,
	m_pbFormat: PhantomData<&'a usize>,
}

impl_default_zero!(AM_MEDIA_TYPE, 'a);

impl<'a> AM_MEDIA_TYPE<'a> {
	/// Returns the `bFixedSizeSamples` field.
	pub fn bFixedSizeSamples(&self) -> bool {
		self.bFixedSizeSamples != 0
	}

	/// Sets the `bFixedSizeSamples` field.
	pub fn set_bFixedSizeSamples(&mut self, val: bool) {
		self.bFixedSizeSamples = val as _;
	}

	/// Returns the `bTemporalCompression` field.
	pub fn bTemporalCompression(&self) -> bool {
		self.bTemporalCompression != 0
	}

	/// Sets the `bTemporalCompression` field.
	pub fn set_bTemporalCompression(&mut self, val: bool) {
		self.bTemporalCompression = val as _;
	}

	/// Returns the `pbFormat` field.
	///
	/// Varies according to the `formattype`. If you set it wrong, you're likely
	/// to cause a buffer overrun.
	pub unsafe fn pbFormat<T>(&self) -> &T {
		&*(self.pbFormat as *const T)
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
