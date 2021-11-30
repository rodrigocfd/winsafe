#![allow(non_snake_case)]

use crate::aliases::WinResult;
use crate::co;
use crate::ffi::comctl32;
use crate::funcs::{GetLastError, SHGetFileInfo};
use crate::handles::{HBITMAP, HICON};
use crate::privs::bool_to_winresult;
use crate::structs::{COLORREF, POINT, SIZE, SHFILEINFO};

/// Handle to an
/// [image list](https://docs.microsoft.com/en-us/windows/win32/controls/image-lists).
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HIMAGELIST(pub(crate) *mut std::ffi::c_void);

impl_handle!(HIMAGELIST);

impl HIMAGELIST {
	/// [`ImageList_Add`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_add)
	/// method.
	///
	/// **Note:** A copy of the bitmap is made, and this copy is then stored.
	/// You're still responsible for freeing the original bitmap.
	pub fn Add(self,
		hbmp_image: HBITMAP, hbmp_mask: Option<HBITMAP>) -> WinResult<u32>
	{
		match unsafe {
			comctl32::ImageList_Add(
				self.0,
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
	/// use winsafe::{co, HIMAGELIST, SIZE};
	///
	/// let himgl = HIMAGELIST::Create(
	///     SIZE::new(16, 16), co::ILC::COLOR32, 1, 1)?;
	///
	/// himgl.AddIconFromShell(&["mp3", "wav"])?;
	///
	/// himgl.Destroy()?;
	/// ```
	pub fn AddIconFromShell(self, file_extensions: &[&str]) -> WinResult<()> {
		let sz = self.GetIconSize()?;
		if !sz.is(16, 16) && !sz.is(32, 32) {
			return Err(co::ERROR::NOT_SUPPORTED); // only 16x16 or 32x32 icons can be loaded
		}

		let mut shfi = SHFILEINFO::default();
		for fileExtension in file_extensions.iter() {
			SHGetFileInfo(&format!("*.{}", fileExtension), co::FILE_ATTRIBUTE::NORMAL,
				&mut shfi, co::SHGFI::USEFILEATTRIBUTES | co::SHGFI::ICON |
				if sz.is(16, 16) { co::SHGFI::SMALLICON } else { co::SHGFI::LARGEICON })?;
			self.AddIcon(shfi.hIcon)?;
			shfi.hIcon.DestroyIcon()?;
		}
		Ok(())
	}

	/// [`ImageList_AddMasked`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_addmasked)
	/// method.
	pub fn AddMasked(self,
		hbmp_image: HBITMAP, color_mask: COLORREF) -> WinResult<u32>
	{
		match unsafe {
			comctl32::ImageList_AddMasked(self.0, hbmp_image.0, color_mask.0)
		} {
			-1 => Err(GetLastError()),
			idx => Ok(idx as _),
		}
	}

	/// [`ImageList_BeginDrag`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_begindrag)
	/// method.
	///
	/// **Note:** Must be paired with an
	/// [`HIMAGELIST::EndDrag`](crate::HIMAGELIST::EndDrag) call.
	pub fn BeginDrag(self, track: u32, hotspot: POINT) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				comctl32::ImageList_BeginDrag(
					self.0,
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
	/// [`HIMAGELIST::Destroy`](crate::HIMAGELIST::Destroy) call.
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use winsafe::{co, HIMAGELIST, SIZE};
	///
	/// let himgl = HIMAGELIST::Create(
	///     SIZE::new(16, 16), co::ILC::COLOR32, 1, 1)?;
	///
	/// himgl.Destroy()?;
	/// ```
	pub fn Create(
		image_sz: SIZE, flags: co::ILC,
		initial_size: i32, grow_size: i32) -> WinResult<HIMAGELIST>
	{
		unsafe {
			comctl32::ImageList_Create(
				image_sz.cx, image_sz.cy,
				flags.0,
				initial_size,
				grow_size,
			).as_mut()
		}.map(|ptr| Self(ptr))
			.ok_or_else(|| GetLastError())
	}

	/// [`ImageList_Destroy`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_destroy)
	/// method.
	pub fn Destroy(self) -> WinResult<()> {
		bool_to_winresult(unsafe { comctl32::ImageList_Destroy(self.0) })
	}

	/// [`ImageList_DragMove`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragmove)
	/// method.
	pub fn DragMove(self, x: i32, y: i32) -> WinResult<()> {
		bool_to_winresult(unsafe { comctl32::ImageList_DragMove(self.0, x, y) })
	}

	/// [`ImageList_DragShowNolock`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_dragshownolock)
	/// static method.
	pub fn DragShowNolock(show: bool) -> WinResult<()> {
		bool_to_winresult(
			unsafe { comctl32::ImageList_DragShowNolock(show as _) },
		)
	}

	/// [`ImageList_EndDrag`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_enddrag)
	/// static method.
	pub fn EndDrag() {
		unsafe { comctl32::ImageList_EndDrag(); }
	}

	/// [`ImageList_GetIconSize`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_geticonsize)
	/// method.
	pub fn GetIconSize(self) -> WinResult<SIZE> {
		let mut sz = SIZE::default();
		bool_to_winresult(
			unsafe {
				comctl32::ImageList_GetIconSize(self.0, &mut sz.cx, &mut sz.cy)
			}
		).map(|_| sz)
	}

	/// [`ImageList_GetImageCount`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_getimagecount)
	/// method.
	pub fn GetImageCount(self) -> u32 {
		unsafe { comctl32::ImageList_GetImageCount(self.0) as _ }
	}

	/// [`ImageList_Remove`](https://docs.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-imagelist_remove)
	/// method.
	pub fn Remove(self, index: Option<u32>) -> WinResult<()> {
		bool_to_winresult(
			unsafe {
				comctl32::ImageList_Remove(
					self.0,
					index.map_or(-1, |i| i as _),
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
	pub fn ReplaceIcon(self,
		index: Option<u32>, hicon_new: HICON) -> WinResult<u32>
	{
		match unsafe {
			comctl32::ImageList_ReplaceIcon(
				self.0,
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
	pub fn SetImageCount(self, new_count: u32) -> WinResult<()> {
		bool_to_winresult(
			unsafe { comctl32::ImageList_SetImageCount(self.0, new_count) },
		)
	}
}
