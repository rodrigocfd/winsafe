#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, comctl};
use crate::kernel::decl::{GetLastError, SysResult};
use crate::kernel::privs::bool_to_sysresult;
use crate::prelude::Handle;
use crate::user::decl::{COLORREF, HBITMAP, HICON, POINT, SIZE};

impl_handle! { HIMAGELIST: "comctl";
	/// Handle to an
	/// [image list](https://docs.microsoft.com/en-us/windows/win32/controls/image-lists).
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
#[cfg_attr(docsrs, doc(cfg(feature = "comctl")))]
pub trait comctl_Himagelist: Handle {
	/// [`ImageList_Add`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_add)
	/// method.
	///
	/// **Note:** A copy of the bitmap is made, and this copy is then stored.
	/// You're still responsible for freeing the original bitmap.
	fn Add(self,
		hbmp_image: HBITMAP, hbmp_mask: Option<HBITMAP>) -> SysResult<u32>
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

	/// [`ImageList_AddIcon`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addicon)
	/// method.
	///
	/// The icon can be destroyed right after this call, its handle its
	/// duplicated inside the image list.
	fn AddIcon(self, hicon: HICON) -> SysResult<u32> {
		self.ReplaceIcon(None, hicon)
	}

	/// [`ImageList_AddMasked`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addmasked)
	/// method.
	fn AddMasked(self,
		hbmp_image: HBITMAP, color_mask: COLORREF) -> SysResult<u32>
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

	/// [`ImageList_BeginDrag`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_begindrag)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HIMAGELIST::EndDrag`](crate::prelude::comctl_Himagelist::EndDrag)
	/// call.
	fn BeginDrag(self, track: u32, hotspot: POINT) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				comctl::ffi::ImageList_BeginDrag(
					self.as_ptr(),
					track as _,
					hotspot.x, hotspot.y,
				)
			},
		)
	}

	/// [`ImageList_Create`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_create)
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

	/// [`ImageList_Destroy`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_destroy)
	/// method.
	fn Destroy(self) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { comctl::ffi::ImageList_Destroy(self.as_ptr()) },
		)
	}

	/// [`ImageList_DragMove`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragmove)
	/// method.
	fn DragMove(self, x: i32, y: i32) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { comctl::ffi::ImageList_DragMove(self.as_ptr(), x, y) },
		)
	}

	/// [`ImageList_DragShowNolock`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragshownolock)
	/// static method.
	fn DragShowNolock(show: bool) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { comctl::ffi::ImageList_DragShowNolock(show as _) },
		)
	}

	/// [`ImageList_EndDrag`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag)
	/// static method.
	fn EndDrag() {
		unsafe { comctl::ffi::ImageList_EndDrag(); }
	}

	/// [`ImageList_GetIconSize`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_geticonsize)
	/// method.
	#[must_use]
	fn GetIconSize(self) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				comctl::ffi::ImageList_GetIconSize(
					self.as_ptr(), &mut sz.cx, &mut sz.cy,
				)
			}
		).map(|_| sz)
	}

	/// [`ImageList_GetImageCount`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_getimagecount)
	/// method.
	#[must_use]
	fn GetImageCount(self) -> u32 {
		unsafe { comctl::ffi::ImageList_GetImageCount(self.as_ptr()) as _ }
	}

	/// [`ImageList_Remove`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_remove)
	/// method.
	fn Remove(self, index: Option<u32>) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				comctl::ffi::ImageList_Remove(
					self.as_ptr(), index.map_or(-1, |i| i as _),
				)
			},
		)
	}

	/// [`ImageList_ReplaceIcon`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_replaceicon)
	/// method.
	///
	/// The icon can be destroyed right after this call, its handle its
	/// duplicated inside the image list.
	fn ReplaceIcon(self,
		index: Option<u32>, hicon_new: HICON) -> SysResult<u32>
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

	/// [`ImageList_SetImageCount`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_setimagecount)
	/// methods.
	fn SetImageCount(self, new_count: u32) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				comctl::ffi::ImageList_SetImageCount(self.as_ptr(), new_count)
			},
		)
	}
}
