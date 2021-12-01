use crate::aliases::WinResult;
use crate::co;
use crate::handles::{HFILEMAP, HFILEMAPVIEW};
use crate::handles::prelude::{Handle, HandleClose};
use crate::various::{File, FileAccess};

/// Manages a memory-mapped file, which can be read/written through slices. It
/// is closed automatically when the object goes out of scope.
pub struct FileMapped {
	access: FileAccess,
	file: File,
	hmap: HFILEMAP,
	hview: HFILEMAPVIEW,
	size: usize,
}

impl Drop for FileMapped {
	fn drop(&mut self) {
		if !self.hview.is_null() { self.hview.UnmapViewOfFile().ok(); } // ignore errors
		if !self.hmap.is_null() { self.hmap.CloseHandle().ok(); }
	}
}

impl FileMapped {
	/// Opens a file with the desired access, then map its contents in memory.
	pub fn open(
		file_path: &str, access: FileAccess) -> WinResult<FileMapped>
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

	fn map_in_memory(&mut self) -> WinResult<()> {
		self.hmap = self.file.hfile().CreateFileMapping(
			None,
			match self.access {
				FileAccess::ExistingReadOnly => co::PAGE::READONLY,
				FileAccess::ExistingReadWrite
					| FileAccess::OpenOrCreateReadWrite => co::PAGE::READWRITE,
			},
			None,
			None,
		)?;

		self.hview = self.hmap.MapViewOfFile(
			match self.access {
				FileAccess::ExistingReadOnly => co::FILE_MAP::READ,
				FileAccess::ExistingReadWrite
					| FileAccess::OpenOrCreateReadWrite => co::FILE_MAP::READ | co::FILE_MAP::WRITE,
			},
			0,
			None,
		)?;

		self.size = self.file.hfile().GetFileSizeEx()?; // cache
		Ok(())
	}

	/// Returns a mutable slice to the mapped memory.
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
	pub fn resize(&mut self, num_bytes: usize) -> WinResult<()> {
		self.hview.UnmapViewOfFile()?;
		self.hmap.CloseHandle()?;

		self.file.resize(num_bytes)?;

		self.map_in_memory()?;
		Ok(())
	}

	/// Returns the size of the file. This value is cached.
	pub const fn size(&self) -> usize {
		self.size
	}
}
