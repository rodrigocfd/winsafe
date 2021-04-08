#![allow(non_upper_case_globals)]

const_type! { MFVideoARMode, u32,
	/// [`MFVideoAspectRatioMode`](https://docs.microsoft.com/en-us/windows/win32/api/evr/ne-evr-mfvideoaspectratiomode)
	/// enumeration.
	->
	None, 0
	PreservePicture, 0x1
	PreservePixel, 0x2
	NonLinearStretch, 0x4
}
