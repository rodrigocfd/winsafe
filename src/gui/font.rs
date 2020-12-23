use crate::handles::HFONT;

/// Manages an [`HFONT`](crate::HFONT) resource.
pub struct Font {
	hfont: HFONT,
}