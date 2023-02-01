use std::marker::PhantomData;

use crate::comctl;

/// RAII implementation for imagelist drag which automatically calls
/// [`ImageList_EndDrag`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct ImageListEndDragGuard<'a> {
	_himagelist: PhantomData<&'a ()>,
}

impl<'a> Drop for ImageListEndDragGuard<'a> {
	fn drop(&mut self) {
		unsafe { comctl::ffi::ImageList_EndDrag(); }
	}
}

impl<'a> ImageListEndDragGuard<'a> {
	/// Constructs the guard by taking ownership of the object.
	#[must_use]
	pub const fn new(himagelist: PhantomData<&'a ()>) -> Self {
		Self { _himagelist: himagelist }
	}
}
