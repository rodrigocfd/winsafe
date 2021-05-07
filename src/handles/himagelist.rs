#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::comctl32;
use crate::funcs::GetLastError;
use crate::handles::{HBITMAP, HICON};
use crate::privs::bool_to_winresult;
use crate::structs::COLORREF;

handle_type! {
	/// Handle to an
	/// [image list](https://docs.microsoft.com/en-us/windows/win32/controls/image-lists).
	HIMAGELIST
}

impl HIMAGELIST {
	/// [`ImageList_Add`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_add)
	/// method.
	///
	/// **Note:** A copy of the bitmap is made, and this copy is then stored.
	/// You're still responsible for freeing the original bitmap.
	pub fn ImageList_Add(self,
		hbmImage: HBITMAP, hbmMask: Option<HBITMAP>) -> WinResult<u32>
	{
		match unsafe {
			comctl32::ImageList_Add(
				self.ptr,
				hbmImage.ptr,
				hbmMask.map_or(std::ptr::null_mut(), |h| h.ptr),
			)
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_AddIcon`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addicon)
	/// method.
	pub fn ImageList_AddIcon(self, hicon: HICON) -> WinResult<u32> {
		self.ImageList_ReplaceIcon(None, hicon)
	}

	/// [`ImageList_AddMasked`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addmasked)
	/// method.
	pub fn ImageList_AddMasked(self,
		hbmImage: HBITMAP, crMask: COLORREF) -> WinResult<u32>
	{
		match unsafe {
			comctl32::ImageList_AddMasked(self.ptr, hbmImage.ptr, crMask.0)
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_BeginDrag`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_begindrag)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`ImageList_EndDrag`](crate::HIMAGELIST::ImageList_EndDrag) call.
	pub fn ImageList_BeginDrag(self,
		iTrack: u32, dxHotspot: i32, dyHotspot: i32) -> WinResult<()>
	{
		bool_to_winresult(
			unsafe {
				comctl32::ImageList_BeginDrag(
					self.ptr,
					iTrack as _,
					dxHotspot,
					dyHotspot,
				)
			},
		)
	}

	/// [`ImageList_Create`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_create)
	/// static method.
	///
	/// **Note:** Must be paired with an
	/// [`ImageList_Destroy`](crate::HIMAGELIST::ImageList_Destroy) call.
	pub fn ImageList_Create(cx: i32, cy: i32,
		flags: co::ILS, cInitial: i32, cGrow: i32) -> WinResult<HIMAGELIST>
	{
		unsafe {
			comctl32::ImageList_Create(cx, cy, flags.0, cInitial, cGrow).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`ImageList_Destroy`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_destroy)
	/// method.
	pub fn ImageList_Destroy(self) -> WinResult<()> {
		bool_to_winresult(unsafe { comctl32::ImageList_Destroy(self.ptr) })
	}

	/// [`ImageList_DragMove`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragmove)
	/// method.
	pub fn ImageList_DragMove(self, x: i32, y: i32) -> WinResult<()> {
		bool_to_winresult(unsafe { comctl32::ImageList_DragMove(self.ptr, x, y) })
	}

	/// [`ImageList_DragShowNolock`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragshownolock)
	/// static method.
	pub fn ImageList_DragShowNolock(fShow: bool) -> WinResult<()> {
		bool_to_winresult(
			unsafe { comctl32::ImageList_DragShowNolock(fShow as _) },
		)
	}

	// [`ImageList_EndDrag`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag)
	/// static method.
	pub fn ImageList_EndDrag() {
		unsafe { comctl32::ImageList_EndDrag(); }
	}

	/// [`ImageList_GetImageCount`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_getimagecount)
	/// method.
	pub fn ImageList_GetImageCount(self) -> u32 {
		unsafe { comctl32::ImageList_GetImageCount(self.ptr) as _ }
	}

	/// [`ImageList_Remove`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_remove)
	/// method.
	pub fn ImageList_Remove(self, i: Option<u32>) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				comctl32::ImageList_Remove(
					self.ptr,
					i.map_or(-1, |i| i as _),
				)
			},
		)
	}

	/// [`ImageList_RemoveAll`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_removeall)
	/// method.
	pub fn ImageList_RemoveAll(self) -> WinResult<()> {
		self.ImageList_Remove(None)
	}

	/// [`ImageList_ReplaceIcon`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_replaceicon)
	/// method.
	pub fn ImageList_ReplaceIcon(self,
		i: Option<u32>, hicon: HICON) -> WinResult<u32>
	{
		match unsafe {
			comctl32::ImageList_ReplaceIcon(
				self.ptr,
				i.map_or(-1, |i| i as _),
				hicon.ptr,
			)
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_SetImageCount`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_setimagecount)
	/// methods.
	pub fn ImageList_SetImageCount(self, uNewCount: u32) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				comctl32::ImageList_SetImageCount(self.ptr, uNewCount)
			},
		)
	}
}
