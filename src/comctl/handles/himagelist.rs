#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::{co, comctl};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::{bool_to_sysresult, replace_handle_value};
use crate::prelude::Handle;
use crate::user::decl::{COLORREF, HBITMAP, HICON, POINT, SIZE};

impl_handle! { HIMAGELIST;
	/// Handle to an
	/// [image list](https://learn.microsoft.com/en-us/windows/win32/controls/image-lists).
}

impl comctl_Himagelist for HIMAGELIST {}

/// This trait is enabled with the `comctl` feature, and provides methods for
/// [`HIMAGELIST`](crate::HIMAGELIST).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait comctl_Himagelist: Handle {
	/// [`ImageList_Add`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_add)
	/// method.
	///
	/// **Note:** A copy of the bitmap is made, and this copy is then stored.
	/// You're still responsible for freeing the original bitmap.
	fn Add(&self,
		hbmp_image: &HBITMAP, hbmp_mask: Option<&HBITMAP>) -> SysResult<u32>
	{
		match unsafe {
			comctl::ffi::ImageList_Add(
				self.as_ptr(),
				hbmp_image.0,
				hbmp_mask.map_or(std::ptr::null_mut(), |h| h.0),
			)
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_AddIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addicon)
	/// method.
	///
	/// **Note:** A copy of the bitmap is made, and this copy is then stored.
	/// You're still responsible for freeing the original bitmap.
	fn AddIcon(&self, hicon: &HICON) -> SysResult<u32> {
		self.ReplaceIcon(None, hicon)
	}

	/// [`ImageList_AddMasked`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addmasked)
	/// method.
	///
	/// **Note:** A copy of the bitmap is made, and this copy is then stored.
	/// You're still responsible for freeing the original bitmap.
	fn AddMasked(&self,
		hbmp_image: &HBITMAP, color_mask: COLORREF) -> SysResult<u32>
	{
		match unsafe {
			comctl::ffi::ImageList_AddMasked(
				self.as_ptr(), hbmp_image.0, color_mask.0,
			)
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_BeginDrag`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_begindrag)
	/// method.
	///
	/// Note that the guard returned by this method must be kept alive while you
	/// work on the drag. This is necessary because, when the guard goes out of
	/// scope, it will automatically call
	/// [`ImageList_EndDrag`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag).
	///
	/// In the example below, the returned guard is kept alive:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{HIMAGELIST, POINT};
	///
	/// let himgl: HIMAGELIST; // initialized somewhere
	/// # let himgl = HIMAGELIST::NULL;
	///
	/// let _drag = himgl.BeginDrag(0, POINT::new(0, 0))?; // keep the returned guard alive
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	fn BeginDrag(&self,
		itrack: u32, hotspot: POINT) -> SysResult<HimagelistDragGuard<'_>>
	{
		bool_to_sysresult(
			unsafe {
				comctl::ffi::ImageList_BeginDrag(
					self.as_ptr(),
					itrack as _,
					hotspot.x, hotspot.y,
				)
			},
		).map(|_| HimagelistDragGuard { _himagelist: PhantomData })
	}

	/// [`ImageList_Create`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_create)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`HIMAGELIST::Destroy`](crate::prelude::comctl_Himagelist::Destroy)
	/// call.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::{co, HIMAGELIST, SIZE};
	///
	/// let himgl = HIMAGELIST::Create(
	///     SIZE::new(16, 16), co::ILC::COLOR32, 1, 1)?;
	///
	/// himgl.Destroy()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	#[must_use]
	fn Create(
		image_sz: SIZE, flags: co::ILC,
		initial_size: i32, grow_size: i32) -> SysResult<HIMAGELIST>
	{
		unsafe {
			comctl::ffi::ImageList_Create(
				image_sz.cx, image_sz.cy,
				flags.0,
				initial_size,
				grow_size,
			).as_mut()
		}.map(|ptr| HIMAGELIST(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`ImageList_Destroy`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_destroy)
	/// method.
	///
	/// After calling this method, the handle will be invalidated and further
	/// operations will fail with
	/// [`ERROR::INVALID_HANDLE`](crate::co::ERROR::INVALID_HANDLE) error code.
	fn Destroy(&self) -> SysResult<()> {
		let ret = bool_to_sysresult(
			unsafe { comctl::ffi::ImageList_Destroy(self.as_ptr()) },
		);
		replace_handle_value(self, Self::INVALID);
		ret
	}

	/// [`ImageList_DragMove`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragmove)
	/// method.
	fn DragMove(&self, x: i32, y: i32) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { comctl::ffi::ImageList_DragMove(self.as_ptr(), x, y) },
		)
	}

	/// [`ImageList_DragShowNolock`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragshownolock)
	/// static method.
	fn DragShowNolock(show: bool) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { comctl::ffi::ImageList_DragShowNolock(show as _) },
		)
	}

	/// [`ImageList_GetIconSize`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_geticonsize)
	/// method.
	#[must_use]
	fn GetIconSize(&self) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				comctl::ffi::ImageList_GetIconSize(
					self.as_ptr(), &mut sz.cx, &mut sz.cy,
				)
			}
		).map(|_| sz)
	}

	/// [`ImageList_GetImageCount`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_getimagecount)
	/// method.
	#[must_use]
	fn GetImageCount(&self) -> u32 {
		unsafe { comctl::ffi::ImageList_GetImageCount(self.as_ptr()) as _ }
	}

	/// [`ImageList_Remove`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_remove)
	/// method.
	fn Remove(&self, index: Option<u32>) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				comctl::ffi::ImageList_Remove(
					self.as_ptr(), index.map_or(-1, |i| i as _),
				)
			},
		)
	}

	/// [`ImageList_ReplaceIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_replaceicon)
	/// method.
	///
	/// **Note:** A copy of the bitmap is made, and this copy is then stored.
	/// You're still responsible for freeing the original bitmap.
	fn ReplaceIcon(&self,
		index: Option<u32>, hicon_new: &HICON) -> SysResult<u32>
	{
		match unsafe {
			comctl::ffi::ImageList_ReplaceIcon(
				self.as_ptr(),
				index.map_or(-1, |i| i as _),
				hicon_new.0,
			)
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_SetImageCount`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_setimagecount)
	/// methods.
	fn SetImageCount(&self, new_count: u32) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				comctl::ffi::ImageList_SetImageCount(self.as_ptr(), new_count)
			},
		)
	}
}

//------------------------------------------------------------------------------

/// RAII implementation for imagelist drag which automatically calls
/// [`ImageList_EndDrag`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag)
/// when the object goes out of scope.
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub struct HimagelistDragGuard<'a> {
	_himagelist: PhantomData<&'a ()>,
}

impl<'a> Drop for HimagelistDragGuard<'a> {
	fn drop(&mut self) {
		unsafe { comctl::ffi::ImageList_EndDrag(); }
	}
}
