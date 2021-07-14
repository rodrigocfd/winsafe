use crate::aliases::WinResult;
use crate::co;
use crate::handles::{HFILE, HFILEMAP, HFILEMAPVIEW};

/// Access type for [`MappedFile::open`](crate::MappedFile::open).
pub enum MappedFileAccess {
	Read,
	ReadWrite,
}

/// Manages a memory-mapped file, which can be read/written through slices.
pub struct MappedFile {
	access: MappedFileAccess,
	hfile: HFILE,
	hmap: HFILEMAP,
	hview: HFILEMAPVIEW,
	size: usize,
}

impl Drop for MappedFile {
	fn drop(&mut self) {
		if !self.hview.is_null() { self.hview.UnmapViewOfFile().unwrap(); }
		if !self.hmap.is_null() { self.hmap.CloseHandle().unwrap(); }
		if !self.hfile.is_null() { self.hfile.CloseHandle().unwrap(); }
	}
}

impl MappedFile {
	/// Opens a file and maps it in memory according to the given permissions.
	pub fn open(
		file_path: &str, access: MappedFileAccess) -> WinResult<MappedFile>
	{
		let (hfile, _) = HFILE::CreateFile(
			file_path,
			match access {
				MappedFileAccess::Read => co::GENERIC::READ,
				MappedFileAccess::ReadWrite => co::GENERIC::READ | co::GENERIC::WRITE,
			},
			match access {
				MappedFileAccess::Read => co::FILE_SHARE::READ,
				MappedFileAccess::ReadWrite => co::FILE_SHARE::NoValue,
			},
			None,
			match access {
				MappedFileAccess::Read => co::DISPOSITION::OPEN_EXISTING,
				MappedFileAccess::ReadWrite => co::DISPOSITION::OPEN_ALWAYS
			},
			co::FILE_ATTRIBUTE::NORMAL,
			None,
		)?;

		let mut new_self = Self {
			access,
			hfile,
			hmap: HFILEMAP::NULL,
			hview: HFILEMAPVIEW::NULL,
			size: 0,
		};
		new_self.map_in_memory()?;
		Ok(new_self)
	}

	fn map_in_memory(&mut self) -> WinResult<()> {
		self.hmap = self.hfile.CreateFileMapping(
			None,
			match self.access {
				MappedFileAccess::Read => co::PAGE::READONLY,
				MappedFileAccess::ReadWrite => co::PAGE::READWRITE,
			},
			None,
			None,
		)?;

		self.hview = self.hmap.MapViewOfFile(
			match self.access {
				MappedFileAccess::Read => co::FILE_MAP::READ,
				MappedFileAccess::ReadWrite => co::FILE_MAP::READ | co::FILE_MAP::WRITE,
			},
			0,
			None,
		)?;

		self.size = self.hfile.GetFileSizeEx()?; // cache
		Ok(())
	}

	/// Returns the size of the file. This value is cached.
	pub fn size(&self) -> usize {
		self.size
	}

	/// Returns a slice to the mapped memory.
	pub fn as_slice(&self) -> &[u8] {
		self.hview.as_slice(self.size)
	}

	/// Returns a mutable slice to the mapped memory.
	pub fn as_mut_slice(&mut self) -> &mut [u8] {
		self.hview.as_mut_slice(self.size)
	}

	/// Resizes the file, which will be remapped in memory. All slices must be
	/// recreated.
	pub fn resize(&mut self, num_bytes: usize) -> WinResult<()> {
		self.hview.UnmapViewOfFile()?;
		self.hmap.CloseHandle()?;

		self.hfile.SetFilePointerEx(num_bytes as _, co::FILE_STARTING_POINT::BEGIN)?;
		self.hfile.SetEndOfFile()?;
		self.hfile.SetFilePointerEx(0, co::FILE_STARTING_POINT::BEGIN)?;

		self.map_in_memory()?;
		Ok(())
	}
}
