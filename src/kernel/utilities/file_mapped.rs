use crate::co;
use crate::kernel::decl::{File, FileAccess, HFILEMAP, HFILEMAPVIEW, SysResult};
use crate::kernel::guard::{HandleGuard, HfilemapviewGuard};
use crate::prelude::{
	Handle, kernel_Hfile, kernel_Hfilemap, kernel_Hfilemapview,
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
	hmap: HandleGuard<HFILEMAP>,
	hview: HfilemapviewGuard,
	size: usize,
}

impl FileMapped {
	/// Opens a file with the desired access, then map its contents in memory.
	#[must_use]
	pub fn open(
		file_path: &str, access: FileAccess) -> SysResult<FileMapped>
	{
		let file = File::open(file_path, access)?;
		let (hmap, hview) = Self::map_in_memory(&file, access)?;
		let size = file.hfile().GetFileSizeEx()?; // cache
		Ok(Self { access, file, hmap, hview, size })
	}

	#[must_use]
	fn map_in_memory(file: &File, access: FileAccess)
		-> SysResult<(HandleGuard<HFILEMAP>, HfilemapviewGuard)>
	{
		let hmap = file.hfile().CreateFileMapping(
			None,
			match access {
				FileAccess::ExistingReadOnly => co::PAGE::READONLY,
				FileAccess::ExistingRW
					| FileAccess::OpenOrCreateRW => co::PAGE::READWRITE,
			},
			None,
			None,
		)?;

		let hview = hmap.MapViewOfFile(
			match access {
				FileAccess::ExistingReadOnly => co::FILE_MAP::READ,
				FileAccess::ExistingRW
					| FileAccess::OpenOrCreateRW => co::FILE_MAP::READ | co::FILE_MAP::WRITE,
			},
			0,
			None,
		)?;

		Ok((hmap, hview))
	}

	/// Returns a mutable slice to the mapped memory.
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [u8] {
		self.hview.as_mut_slice(self.size)
	}

	/// Returns a slice to the mapped memory.
	#[must_use]
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
		self.hview = HfilemapviewGuard { handle: HFILEMAPVIEW::NULL }; // close mapping handles
		self.hmap = HandleGuard { handle: HFILEMAP::NULL };

		self.file.resize(num_bytes)?;
		let (hmap, hview) = Self::map_in_memory(&self.file, self.access)?;

		self.hmap = hmap;
		self.hview = hview;
		self.size = num_bytes;
		Ok(())
	}

	/// Returns the size of the file. This value is cached.
	#[must_use]
	pub const fn size(&self) -> usize {
		self.size
	}
}