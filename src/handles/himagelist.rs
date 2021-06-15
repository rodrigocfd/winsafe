#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::comctl32;
use crate::funcs::{GetLastError, SHGetFileInfo};
use crate::handles::{HBITMAP, HICON};
use crate::privs::bool_to_winresult;
use crate::structs::{COLORREF, SHFILEINFO};

pub_struct_handle! {
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
	pub fn Add(self,
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
	pub fn AddIcon(self, hicon: HICON) -> WinResult<u32> {
		self.ReplaceIcon(None, hicon)
	}

	/// Calls [`SHGetFileInfo`](crate::SHGetFileInfo) to retrieve one or more
	/// shell file icons, then passes them to
	/// [`AddIcon`](crate::HIMAGELIST::AddIcon).
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, HIMAGELIST};
	///
	/// let himgl = HIMAGELIST::Create(
	///     16, 16, co::ILC::COLOR32, 1, 1).unwrap();
	///
	/// himgl.AddIconFromShell(&["mp3", "wav"]).unwrap();
	///
	/// himgl.Destroy().unwrap();
	/// ```
	pub fn AddIconFromShell(self, fileExtensions: &[&str]) -> WinResult<()> {
		let (cx, cy) = self.GetIconSize()?;
		let isIco16 = cx == 16 && cy == 16;
		let isIco32 = cx == 32 && cy == 32;
		if !isIco16 && !isIco32 {
			return Err(co::ERROR::NOT_SUPPORTED); // only 16x16 or 32x32 icons can be loaded
		}

		let mut shfi = SHFILEINFO::default();
		for fileExtension in fileExtensions.iter() {
			SHGetFileInfo(&format!("*.{}", fileExtension), co::FILE_ATTRIBUTE::NORMAL,
				&mut shfi, co::SHGFI::USEFILEATTRIBUTES | co::SHGFI::ICON |
				if isIco16 { co::SHGFI::SMALLICON } else { co::SHGFI::LARGEICON })?;
			self.AddIcon(shfi.hIcon)?;
			shfi.hIcon.DestroyIcon()?;
		}
		Ok(())
	}

	/// [`ImageList_AddMasked`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addmasked)
	/// method.
	pub fn AddMasked(self,
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
	/// **Note:** Must be paired with an [`EndDrag`](crate::HIMAGELIST::EndDrag)
	/// call.
	pub fn BeginDrag(self,
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
	/// **Note:** Must be paired with a [`Destroy`](crate::HIMAGELIST::Destroy)
	/// call.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, HIMAGELIST};
	///
	/// let himgl = HIMAGELIST::Create(
	///     16, 16, co::ILC::COLOR32, 1, 1).unwrap();
	///
	/// himgl.Destroy().unwrap();
	/// ```
	pub fn Create(cx: i32, cy: i32,
		flags: co::ILC, cInitial: i32, cGrow: i32) -> WinResult<HIMAGELIST>
	{
		unsafe {
			comctl32::ImageList_Create(cx, cy, flags.0, cInitial, cGrow).as_mut()
		}.map(|ptr| Self { ptr })
			.ok_or_else(|| GetLastError())
	}

	/// [`ImageList_Destroy`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_destroy)
	/// method.
	pub fn Destroy(self) -> WinResult<()> {
		bool_to_winresult(unsafe { comctl32::ImageList_Destroy(self.ptr) })
	}

	/// [`ImageList_DragMove`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragmove)
	/// method.
	pub fn DragMove(self, x: i32, y: i32) -> WinResult<()> {
		bool_to_winresult(unsafe { comctl32::ImageList_DragMove(self.ptr, x, y) })
	}

	/// [`ImageList_DragShowNolock`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragshownolock)
	/// static method.
	pub fn DragShowNolock(fShow: bool) -> WinResult<()> {
		bool_to_winresult(
			unsafe { comctl32::ImageList_DragShowNolock(fShow as _) },
		)
	}

	// [`ImageList_EndDrag`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag)
	/// static method.
	pub fn EndDrag() {
		unsafe { comctl32::ImageList_EndDrag(); }
	}

	/// [`ImageList_GetIconSize`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_geticonsize)
	/// method.
	pub fn GetIconSize(self) -> WinResult<(i32, i32)> {
		let (mut cx, mut cy) = (0, 0);
		bool_to_winresult(
			unsafe { comctl32::ImageList_GetIconSize(self.ptr, &mut cx, &mut cy) }
		).map(|_| (cx, cy))
	}

	/// [`ImageList_GetImageCount`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_getimagecount)
	/// method.
	pub fn GetImageCount(self) -> u32 {
		unsafe { comctl32::ImageList_GetImageCount(self.ptr) as _ }
	}

	/// [`ImageList_Remove`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_remove)
	/// method.
	pub fn Remove(self, i: Option<u32>) -> WinResult<()> {
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
	pub fn RemoveAll(self) -> WinResult<()> {
		self.Remove(None)
	}

	/// [`ImageList_ReplaceIcon`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_replaceicon)
	/// method.
	pub fn ReplaceIcon(self, i: Option<u32>, hicon: HICON) -> WinResult<u32> {
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
	pub fn SetImageCount(self, uNewCount: u32) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				comctl32::ImageList_SetImageCount(self.ptr, uNewCount)
			},
		)
	}
}
