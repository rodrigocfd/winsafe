#![allow(non_camel_case_types)]

use crate::co;
use crate::comctl::decl::HIMAGELIST;
use crate::kernel::decl::SysResult;
use crate::prelude::{comctl_Himagelist, user_Hicon};
use crate::shell::decl::{SHFILEINFO, SHGetFileInfo};

impl comctl_shell_Himagelist for HIMAGELIST {}

/// This trait is enabled with `comctl` and `shell` features, and provides
/// methods for [`HIMAGELIST`](crate::HIMAGELIST).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "shell"))))]
pub trait comctl_shell_Himagelist: comctl_Himagelist {
	/// Calls [`SHGetFileInfo`](crate::SHGetFileInfo) to retrieve one or more
	/// shell file icons, then passes them to
	/// [`AddIcon`](crate::prelude::comctl_Himagelist::AddIcon).
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
	/// himgl.add_icon_from_shell(&["mp3", "wav"])?;
	///
	/// himgl.Destroy()?;
	/// # Ok::<_, co::ERROR>(())
	/// ```
	fn add_icon_from_shell(self, file_extensions: &[&str]) -> SysResult<()> {
		let sz = self.GetIconSize()?;
		if !sz.is(16, 16) && !sz.is(32, 32) {
			return Err(co::ERROR::NOT_SUPPORTED); // only 16x16 or 32x32 icons can be loaded
		}

		let mut shfi = SHFILEINFO::default();
		for file_extension in file_extensions.iter() {
			SHGetFileInfo(&format!("*.{}", file_extension), co::FILE_ATTRIBUTE::NORMAL,
				&mut shfi, co::SHGFI::USEFILEATTRIBUTES | co::SHGFI::ICON |
				if sz.is(16, 16) { co::SHGFI::SMALLICON } else { co::SHGFI::LARGEICON })?;
			self.AddIcon(shfi.hIcon)?;
			shfi.hIcon.DestroyIcon()?;
		}
		Ok(())
	}
}
