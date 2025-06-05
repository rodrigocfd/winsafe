#![allow(non_camel_case_types, non_snake_case)]

use crate::co;
use crate::comctl::{ffi, iterators::*, privs::*};
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

handle! { HIMAGELIST;
	/// Handle to an
	/// [image list](https://learn.microsoft.com/en-us/windows/win32/controls/image-lists).
}

impl HIMAGELIST {
	/// Returns an iterator over all icons in the image list, by calling
	/// [`HIMAGELIST::ExtractIcon`](crate::HIMAGELIST::ExtractIcon) for each
	/// one.
	///
	/// # Examples
	///
	/// Collecting the icons into a [`Vec`](std::vec::Vec):
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let himgl: w::HIMAGELIST; // initialized somewhere
	/// # let himgl = w::HIMAGELIST::NULL;
	///
	/// let icons = himgl.iter()
	///     .collect::<w::HrResult<Vec<_>>>()?;
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	pub fn iter(&self) -> impl Iterator<Item = HrResult<DestroyIconGuard>> + '_ {
		HimagelistIter::new(self)
	}

	/// [`ImageList_Add`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_add)
	/// function.
	///
	/// A copy of the bitmap is made and stored in the image list, so you're
	/// free to release the original bitmap.
	pub fn Add(&self, hbmp_image: &HBITMAP, hbmp_mask: Option<&HBITMAP>) -> HrResult<u32> {
		match unsafe {
			ffi::ImageList_Add(
				self.ptr(),
				hbmp_image.ptr(),
				hbmp_mask.map_or(std::ptr::null_mut(), |h| h.ptr()),
			)
		} {
			-1 => Err(co::HRESULT::E_FAIL),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_AddIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addicon)
	/// macro.
	///
	/// A copy of the icon is made and stored in the image list, so you're free
	/// to release the original icon.
	pub fn AddIcon(&self, hicon: &HICON) -> HrResult<u32> {
		match unsafe { ffi::ImageList_ReplaceIcon(self.ptr(), -1, hicon.ptr()) } {
			-1 => Err(co::HRESULT::E_FAIL),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_AddMasked`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addmasked)
	/// function.
	///
	/// A copy of the bitmap is made and stored in the image list, so you're
	/// free to release the original bitmap.
	pub fn AddMasked(&self, hbmp_image: &HBITMAP, color_mask: COLORREF) -> HrResult<u32> {
		match unsafe { ffi::ImageList_AddMasked(self.ptr(), hbmp_image.ptr(), color_mask.into()) } {
			-1 => Err(co::HRESULT::E_FAIL),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_BeginDrag`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_begindrag)
	/// function.
	///
	/// In the original C implementation, you must call
	/// [`ImageList_EndDrag`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag)
	/// as a cleanup operation.
	///
	/// Here, the cleanup is performed automatically, because `BeginDrag`
	/// returns an
	/// [`ImageListEndDragGuard`](crate::guard::ImageListEndDragGuard), which
	/// automatically calls `ImageList_EndDrag` when the guard goes out of
	/// scope. You must, however, keep the guard alive, otherwise the cleanup
	/// will be performed right away.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*};
	///
	/// let himgl: w::HIMAGELIST; // initialized somewhere
	/// # let himgl = w::HIMAGELIST::NULL;
	///
	/// let _drag = himgl.BeginDrag(0, w::POINT::new(0, 0))?; // keep guard alive
	/// # w::HrResult::Ok(())
	/// ```
	pub fn BeginDrag(&self, itrack: u32, hotspot: POINT) -> HrResult<ImageListEndDragGuard<'_>> {
		unsafe {
			match ffi::ImageList_BeginDrag(self.ptr(), itrack as _, hotspot.x, hotspot.y) {
				0 => Err(co::HRESULT::E_FAIL),
				_ => Ok(ImageListEndDragGuard::new()),
			}
		}
	}

	/// [`ImageList_Create`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_create)
	/// function.
	///
	/// # Examples
	///
	/// ```no_run
	/// use winsafe::{self as w, prelude::*, co};
	///
	/// let himgl = w::HIMAGELIST::Create(
	///     w::SIZE::new(16, 16),
	///     co::ILC::COLOR32,
	///     1,
	///     1,
	/// )?;
	///
	/// // ImageList_Destroy() automatically called
	/// # w::HrResult::Ok(())
	/// ```
	#[must_use]
	pub fn Create(
		image_sz: SIZE,
		flags: co::ILC,
		initial_size: i32,
		grow_size: i32,
	) -> HrResult<ImageListDestroyGuard> {
		unsafe {
			match ptr_to_option_handle(ffi::ImageList_Create(
				image_sz.cx,
				image_sz.cy,
				flags.raw(),
				initial_size,
				grow_size,
			)) {
				None => Err(co::HRESULT::E_FAIL),
				Some(h) => Ok(ImageListDestroyGuard::new(h)),
			}
		}
	}

	/// [`ImageList_DragMove`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragmove)
	/// function.
	pub fn DragMove(&self, x: i32, y: i32) -> HrResult<()> {
		match unsafe { ffi::ImageList_DragMove(self.ptr(), x, y) } {
			0 => Err(co::HRESULT::E_FAIL),
			_ => Ok(()),
		}
	}

	/// [`ImageList_DragShowNolock`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragshownolock)
	/// function.
	pub fn DragShowNolock(show: bool) -> HrResult<()> {
		match unsafe { ffi::ImageList_DragShowNolock(show as _) } {
			0 => Err(co::HRESULT::E_FAIL),
			_ => Ok(()),
		}
	}

	/// [`ImageList_Draw`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_draw)
	/// function.
	pub fn Draw(&self, index: u32, hdc_dest: &HDC, dest: POINT, style: co::ILD) -> HrResult<()> {
		match unsafe {
			ffi::ImageList_Draw(self.ptr(), index as _, hdc_dest.ptr(), dest.x, dest.y, style.raw())
		} {
			0 => Err(co::HRESULT::E_FAIL),
			_ => Ok(()),
		}
	}

	/// [`ImageList_DrawEx`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_drawex)
	/// function.
	pub fn DrawEx(
		&self,
		index: u32,
		hdc_dest: &HDC,
		dest: POINT,
		img_portion: Option<SIZE>,
		background_color: ClrDefNone,
		foreground_color: ClrDefNone,
		style: co::ILD,
	) -> HrResult<()> {
		match unsafe {
			ffi::ImageList_DrawEx(
				self.ptr(),
				index as _,
				hdc_dest.ptr(),
				dest.x,
				dest.y,
				img_portion.unwrap_or_default().cx,
				img_portion.unwrap_or_default().cy,
				background_color.as_u32(),
				foreground_color.as_u32(),
				style.raw(),
			)
		} {
			0 => Err(co::HRESULT::E_FAIL),
			_ => Ok(()),
		}
	}

	/// [`ImageList_Duplicate`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_duplicate)
	/// function.
	pub fn Duplicate(&self) -> HrResult<ImageListDestroyGuard> {
		unsafe {
			match ptr_to_option_handle(ffi::ImageList_Duplicate(self.ptr())) {
				None => Err(co::HRESULT::E_FAIL),
				Some(h) => Ok(ImageListDestroyGuard::new(h)),
			}
		}
	}

	/// [`ImageList_ExtractIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_extracticon)
	/// macro.
	///
	/// A copy of the stored icon is returned.
	#[must_use]
	pub fn ExtractIcon(&self, index: u32) -> HrResult<DestroyIconGuard> {
		self.GetIcon(index, co::ILD::NORMAL)
	}

	/// [`ImageList_GetBkColor`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_getbkcolor)
	/// function.
	#[must_use]
	pub fn GetBkColor(&self) -> COLORREF {
		unsafe { COLORREF::from_raw(ffi::ImageList_GetBkColor(self.ptr())) }
	}

	/// [`ImageList_GetIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_geticon)
	/// function.
	///
	/// A copy of the stored icon is returned.
	#[must_use]
	pub fn GetIcon(&self, index: u32, flags: co::ILD) -> HrResult<DestroyIconGuard> {
		unsafe {
			match ptr_to_option_handle(ffi::ImageList_GetIcon(self.ptr(), index as _, flags.raw()))
			{
				None => Err(co::HRESULT::E_FAIL),
				Some(h) => Ok(DestroyIconGuard::new(h)),
			}
		}
	}

	/// [`ImageList_GetIconSize`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_geticonsize)
	/// function.
	#[must_use]
	pub fn GetIconSize(&self) -> HrResult<SIZE> {
		let mut sz = SIZE::default();
		match unsafe { ffi::ImageList_GetIconSize(self.ptr(), &mut sz.cx, &mut sz.cy) } {
			0 => Err(co::HRESULT::E_FAIL),
			_ => Ok(sz),
		}
	}

	/// [`ImageList_GetImageCount`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_getimagecount)
	/// function.
	#[must_use]
	pub fn GetImageCount(&self) -> u32 {
		unsafe { ffi::ImageList_GetImageCount(self.ptr()) as _ }
	}

	/// [`ImageList_Remove`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_remove)
	/// function.
	pub fn Remove(&self, index: Option<u32>) -> HrResult<()> {
		match unsafe { ffi::ImageList_Remove(self.ptr(), index.map_or(-1, |i| i as _)) } {
			0 => Err(co::HRESULT::E_FAIL),
			_ => Ok(()),
		}
	}

	/// [`ImageList_ReplaceIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_replaceicon)
	/// function.
	///
	/// A copy of the icon is made and stored in the image list, so you're free
	/// to release the original icon.
	pub fn ReplaceIcon(&self, index: u32, hicon_new: &HICON) -> HrResult<u32> {
		match unsafe { ffi::ImageList_ReplaceIcon(self.ptr(), index as _, hicon_new.ptr()) } {
			-1 => Err(co::HRESULT::E_FAIL),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_SetBkColor`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_setbkcolor)
	/// function.
	pub fn SetBkColor(&self, bk_color: Option<COLORREF>) -> Option<COLORREF> {
		match unsafe { ffi::ImageList_SetBkColor(self.ptr(), bk_color.unwrap_or_default().raw()) } {
			CLR_NONE => None,
			c => Some(unsafe { COLORREF::from_raw(c) }),
		}
	}

	/// [`ImageList_SetImageCount`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_setimagecount)
	/// function.
	///
	/// # Safety
	///
	/// If the size is increased, you must call
	/// [`HIMAGELIST::ReplaceIcon`](crate::HIMAGELIST::ReplaceIcon) to fill the
	/// new indexes, otherwise draw operations will be
	/// unpredictable.
	pub unsafe fn SetImageCount(&self, new_count: u32) -> HrResult<()> {
		match unsafe { ffi::ImageList_SetImageCount(self.ptr(), new_count) } {
			0 => Err(co::HRESULT::E_FAIL),
			_ => Ok(()),
		}
	}

	/// [`ImageList_Write`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_write)
	/// function.
	pub fn Write(&self, stream: &impl ole_IStream) -> HrResult<()> {
		match unsafe { ffi::ImageList_Write(self.ptr(), stream.ptr()) } {
			0 => Err(co::HRESULT::E_FAIL),
			_ => Ok(()),
		}
	}
}
