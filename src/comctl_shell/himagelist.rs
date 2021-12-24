use crate::co;
use crate::comctl::decl::HIMAGELIST;
use crate::kernel::decl::WinResult;
use crate::prelude::{ComctlHimagelist, UserHicon};
use crate::shell::decl::{SHFILEINFO, SHGetFileInfo};

impl ComctlShellHimagelist for HIMAGELIST {}

/// [`HIMAGELIST`](crate::HIMAGELIST) methods from `comctl`+`shell` features.
#[cfg_attr(docsrs, doc(cfg(all(feature = "comctl", feature = "shell"))))]
pub trait ComctlShellHimagelist: ComctlHimagelist {
	/// Calls [`SHGetFileInfo`](crate::SHGetFileInfo) to retrieve one or more
	/// shell file icons, then passes them to
	/// [`AddIcon`](crate::prelude::ComctlHimagelist::AddIcon).
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
	fn add_icon_from_shell(self, file_extensions: &[&str]) -> WinResult<()> {
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
