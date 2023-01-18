use std::marker::PhantomData;

use crate::comctl;

/// RAII implementation for imagelist drag which automatically calls
/// [`ImageList_EndDrag`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct HimagelistDragGuard<'a> {
	pub(crate) _himagelist: PhantomData<&'a ()>,
}

impl<'a> Drop for HimagelistDragGuard<'a> {
	fn drop(&mut self) {
		unsafe { comctl::ffi::ImageList_EndDrag(); }
	}
}
