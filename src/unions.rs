use crate::co;
use crate::handles::HBITMAP;
use crate::structs::COLORREF;

/// Variant field for:
///
/// * [`LOGBRUSH`](crate::LOGBRUSH) `lbColor`.
#[repr(C)]
pub union ColorrefDibU {
	pub colorref: COLORREF,
	pub dib: co::DIB,
}

/// Variant field for:
///
/// * [`LOGBRUSH`](crate::LOGBRUSH) `lbHatch`.
#[repr(C)]
pub union ColorrefHbitmapU {
	pub colorref: COLORREF,
	pub hbitmap: HBITMAP,
}
