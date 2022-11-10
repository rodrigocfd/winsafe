use crate::co;
use crate::kernel::decl::{File, FileAccess, HFILEMAP, HFILEMAPVIEW, SysResult};
use crate::prelude::{
	Handle, HandleClose, kernel_Hfile, kernel_Hfilemap, kernel_Hfilemapview,
};

/// Manages an [`HFILEMAP`](crate::HFILEMAP) handle, which provides
/// memory-mapped file operations, including read/write through slices. It is
/// closed automatically when the object goes out of scope.
///
/// # Examples
///
/// [Parsing](crate::WString::parse) a file as string by memory-mapping the file
/// (usually the fastest method):
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{FileAccess, FileMapped, WString};
///
/// let file_in = FileMapped::open("C:\\Temp\\foo.txt", FileAccess::ExistingReadOnly)?;
/// let str_contents = WString::parse(file_in.as_slice())?.to_string();
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub struct FileMapped {
	access: FileAccess,
	file: File,
	hmap: HFILEMAP,
	hview: HFILEMAPVIEW,
	size: usize,
}

impl Drop for FileMapped {
	fn drop(&mut self) {
		if self.hview != HFILEMAPVIEW::NULL { self.hview.UnmapViewOfFile().ok(); } // ignore errors
		if self.hmap != HFILEMAP::NULL { self.hmap.CloseHandle().ok(); }
	}
}

impl FileMapped {
	/// Opens a file with the desired access, then map its contents in memory.
	#[must_use]
	pub fn open(
		file_path: &str, access: FileAccess) -> SysResult<FileMapped>
	{
		let mut new_self = Self {
			access,
			file: File::open(file_path, access)?,
			hmap: HFILEMAP::NULL,
			hview: HFILEMAPVIEW::NULL,
			size: 0,
		};

		new_self.map_in_memory()?;
		Ok(new_self)
	}

	fn map_in_memory(&mut self) -> SysResult<()> {
		self.hmap = self.file.hfile().CreateFileMapping(
			None,
			match self.access {
				FileAccess::ExistingReadOnly => co::PAGE::READONLY,
				FileAccess::ExistingRW
					| FileAccess::OpenOrCreateRW => co::PAGE::READWRITE,
			},
			None,
			None,
		)?;

		self.hview = self.hmap.MapViewOfFile(
			match self.access {
				FileAccess::ExistingReadOnly => co::FILE_MAP::READ,
				FileAccess::ExistingRW
					| FileAccess::OpenOrCreateRW => co::FILE_MAP::READ | co::FILE_MAP::WRITE,
			},
			0,
			None,
		)?;

		self.size = self.file.hfile().GetFileSizeEx()?; // cache
		Ok(())
	}

	/// Returns a mutable slice to the mapped memory.
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [u8] {
		self.hview.as_mut_slice(self.size)
	}

	/// Returns a slice to the mapped memory.
	pub fn as_slice(&self) -> &[u8] {
		self.hview.as_slice(self.size)
	}

	/// Resizes the file, which will be remapped in memory.
	///
	/// **Note:** Since the mapping pointers will change, any existing slices
	/// must be recreated. The following functions must be called again:
	/// * [`as_mut_slice`](crate::FileMapped::as_mut_slice);
	/// * [`as_slice`](crate::FileMapped::as_slice).
	pub fn resize(&mut self, num_bytes: usize) -> SysResult<()> {
		self.hview.UnmapViewOfFile()?;
		self.hmap.CloseHandle()?;

		self.file.resize(num_bytes)?;

		self.map_in_memory()?;
		Ok(())
	}

	/// Returns the size of the file. This value is cached.
	#[must_use]
	pub const fn size(&self) -> usize {
		self.size
	}
}
