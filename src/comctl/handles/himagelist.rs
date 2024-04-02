#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;

use crate::co;
use crate::comctl::{ffi, privs::*};
use crate::decl::*;
use crate::guard::*;
use crate::kernel::privs::*;
use crate::prelude::*;

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
/// ```no_run
/// use winsafe::prelude::*;
/// ```
pub trait comctl_Himagelist: Handle {
	/// [`ImageList_Add`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_add)
	/// function.
	///
	/// A copy of the bitmap is made and stored in the image list, so you're
	/// free to release the original bitmap.
	fn Add(&self,
		hbmp_image: &HBITMAP,
		hbmp_mask: Option<&HBITMAP>,
	) -> SysResult<u32>
	{
		match unsafe {
			ffi::ImageList_Add(
				self.ptr(),
				hbmp_image.ptr(),
				hbmp_mask.map_or(std::ptr::null_mut(), |h| h.ptr()),
			)
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_AddIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addicon)
	/// macro.
	///
	/// A copy of the icon is made and stored in the image list, so you're free
	/// to release the original icon.
	fn AddIcon(&self, hicon: &HICON) -> SysResult<u32> {
		match unsafe { ffi::ImageList_ReplaceIcon(self.ptr(), -1, hicon.ptr()) } {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_AddMasked`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addmasked)
	/// function.
	///
	/// A copy of the bitmap is made and stored in the image list, so you're
	/// free to release the original bitmap.
	fn AddMasked(&self,
		hbmp_image: &HBITMAP,
		color_mask: COLORREF,
	) -> SysResult<u32>
	{
		match unsafe {
			ffi::ImageList_AddMasked(
				self.ptr(), hbmp_image.ptr(), color_mask.into(),
			)
		} {
			-1 => Err(GetLastError()),
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
	/// # w::SysResult::Ok(())
	/// ```
	fn BeginDrag(&self,
		itrack: u32,
		hotspot: POINT,
	) -> SysResult<ImageListEndDragGuard<'_>>
	{
		unsafe {
			bool_to_sysresult(
				ffi::ImageList_BeginDrag(
					self.ptr(),
					itrack as _,
					hotspot.x, hotspot.y,
				),
			).map(|_| ImageListEndDragGuard::new(PhantomData))
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
	/// # w::SysResult::Ok(())
	/// ```
	#[must_use]
	fn Create(
		image_sz: SIZE,
		flags: co::ILC,
		initial_size: i32,
		grow_size: i32,
	) -> SysResult<ImageListDestroyGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::ImageList_Create(
					image_sz.cx, image_sz.cy,
					flags.raw(),
					initial_size,
					grow_size,
				),
			).map(|h| ImageListDestroyGuard::new(h))
		}
	}

	/// [`ImageList_DragMove`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragmove)
	/// function.
	fn DragMove(&self, x: i32, y: i32) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::ImageList_DragMove(self.ptr(), x, y) })
	}

	/// [`ImageList_DragShowNolock`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragshownolock)
	/// function.
	fn DragShowNolock(show: bool) -> SysResult<()> {
		bool_to_sysresult(unsafe { ffi::ImageList_DragShowNolock(show as _) })
	}

	/// [`ImageList_Draw`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_draw)
	/// function.
	fn Draw(&self,
		index: u32,
		hdc_dest: &HDC,
		dest: POINT,
		style: co::ILD,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::ImageList_Draw(
					self.ptr(),
					index as _,
					hdc_dest.ptr(),
					dest.x, dest.y,
					style.raw(),
				)
			},
		)
	}

	/// [`ImageList_DrawEx`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_drawex)
	/// function.
	fn DrawEx(&self,
		index: u32,
		hdc_dest: &HDC,
		dest: POINT,
		img_portion: Option<SIZE>,
		background_color: ClrDefNone,
		foreground_color: ClrDefNone,
		style: co::ILD,
	) -> SysResult<()>
	{
		bool_to_sysresult(
			unsafe {
				ffi::ImageList_DrawEx(
					self.ptr(),
					index as _,
					hdc_dest.ptr(),
					dest.x, dest.y,
					img_portion.unwrap_or_default().cx,
					img_portion.unwrap_or_default().cy,
					background_color.as_u32(),
					foreground_color.as_u32(),
					style.raw(),
				)
			},
		)
	}

	/// [`DrawIndirect`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_drawindirect)
	/// function.
	fn DrawIndirect(&self, imldp: &IMAGELISTDRAWPARAMS) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::ImageList_DrawIndirect(self.ptr(), imldp as *const _ as _)
			},
		)
	}

	/// [`ImageList_Duplicate`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_duplicate)
	/// function.
	fn Duplicate(&self) -> SysResult<ImageListDestroyGuard> {
		unsafe {
			ptr_to_sysresult_handle(ffi::ImageList_Duplicate(self.ptr()))
				.map(|h| ImageListDestroyGuard::new(h))
		}
	}

	/// [`ImageList_ExtractIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_extracticon)
	/// macro.
	///
	/// A copy of the stored icon is returned.
	#[must_use]
	fn ExtractIcon(&self, index: u32) -> SysResult<DestroyIconGuard> {
		self.GetIcon(index, co::ILD::NORMAL)
	}

	/// [`ImageList_GetBkColor`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_getbkcolor)
	/// function.
	#[must_use]
	fn GetBkColor(&self) -> COLORREF {
		unsafe { COLORREF::from_raw(ffi::ImageList_GetBkColor(self.ptr())) }
	}

	/// [`ImageList_GetIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_geticon)
	/// function.
	///
	/// A copy of the stored icon is returned.
	#[must_use]
	fn GetIcon(&self,
		index: u32,
		flags: co::ILD,
	) -> SysResult<DestroyIconGuard>
	{
		unsafe {
			ptr_to_sysresult_handle(
				ffi::ImageList_GetIcon(
					self.ptr(),
					index as _,
					flags.raw(),
				)
			).map(|h| DestroyIconGuard::new(h))
		}
	}

	/// [`ImageList_GetIconSize`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_geticonsize)
	/// function.
	#[must_use]
	fn GetIconSize(&self) -> SysResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_sysresult(
			unsafe {
				ffi::ImageList_GetIconSize(self.ptr(), &mut sz.cx, &mut sz.cy)
			},
		).map(|_| sz)
	}

	/// [`ImageList_GetImageCount`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_getimagecount)
	/// function.
	#[must_use]
	fn GetImageCount(&self) -> u32 {
		unsafe { ffi::ImageList_GetImageCount(self.ptr()) as _ }
	}

	/// [`ImageList_Remove`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_remove)
	/// function.
	fn Remove(&self, index: Option<u32>) -> SysResult<()> {
		bool_to_sysresult(
			unsafe {
				ffi::ImageList_Remove(self.ptr(), index.map_or(-1, |i| i as _))
			},
		)
	}

	/// [`ImageList_ReplaceIcon`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_replaceicon)
	/// function.
	///
	/// A copy of the icon is made and stored in the image list, so you're free
	/// to release the original icon.
	fn ReplaceIcon(&self,
		index: u32,
		hicon_new: &HICON,
	) -> SysResult<u32>
	{
		match unsafe {
			ffi::ImageList_ReplaceIcon(self.ptr(), index as _, hicon_new.ptr())
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_SetBkColor`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_setbkcolor)
	/// function.
	fn SetBkColor(&self, bk_color: Option<COLORREF>) -> Option<COLORREF> {
		match unsafe {
			ffi::ImageList_SetBkColor(
				self.ptr(),
				bk_color.unwrap_or_default().raw(),
			)
		} {
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
	/// [`HIMAGELIST::ReplaceIcon`](crate::prelude::comctl_Himagelist::ReplaceIcon)
	/// to fill the new indexes, otherwise draw operations will be
	/// unpredictable.
	unsafe fn SetImageCount(&self, new_count: u32) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::ImageList_SetImageCount(self.ptr(), new_count) },
		)
	}

	/// [`ImageList_Write`](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_write)
	/// function.
	fn Write(&self, stream: &impl ole_IStream) -> SysResult<()> {
		bool_to_sysresult(
			unsafe { ffi::ImageList_Write(self.ptr(), stream.ptr()) },
		)
	}
}
