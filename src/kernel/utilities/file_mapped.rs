use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::prelude::*;

/// Manages an [`HFILEMAP`](crate::HFILEMAP) handle, which provides
/// memory-mapped file operations, including read/write through slices. It is
/// closed automatically when the object goes out of scope.
///
/// # Examples
///
/// Usually the fastest way to read a file as string is by memory-mapping it,
/// like:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let f = w::FileMapped::open(
///     "C:\\Temp\\foo.txt",
///     w::FileAccess::ExistingReadOnly,
/// )?;
/// let raw_bytes = f.as_slice();
/// let text = w::WString::parse(raw_bytes)?.to_string();
/// # w::SysResult::Ok(())
/// ```
pub struct FileMapped {
	hview: UnmapViewOfFileGuard, // drop order is important
	_hmap: CloseHandleGuard<HFILEMAP>,
	file: File,
	size: u64,
}

impl FileMapped {
	/// Opens a file with the desired access, then map its contents in memory.
	#[must_use]
	pub fn open(file_path: &str, access: FileAccess) -> SysResult<Self> {
		let file = File::open(file_path, access)?;
		let hmap = file.hfile().CreateFileMapping(
			None,
			match access {
				FileAccess::ExistingReadOnly => co::PAGE::READONLY,
				FileAccess::ExistingRW | FileAccess::OpenOrCreateRW | FileAccess::CreateRW => {
					co::PAGE::READWRITE
				},
			},
			None,
			None,
		)?;
		let hview = hmap.MapViewOfFile(
			match access {
				FileAccess::ExistingReadOnly => co::FILE_MAP::READ,
				FileAccess::ExistingRW | FileAccess::OpenOrCreateRW | FileAccess::CreateRW => {
					co::FILE_MAP::READ | co::FILE_MAP::WRITE
				},
			},
			0,
			None,
		)?;
		let size = file.hfile().GetFileSizeEx()?; // cache
		Ok(Self { file, _hmap: hmap, hview, size })
	}

	/// Returns a mutable slice to the mapped memory.
	#[must_use]
	pub fn as_mut_slice(&mut self) -> &mut [u8] {
		self.hview.as_mut_slice(self.size as _)
	}

	/// Returns a slice to the mapped memory.
	#[must_use]
	pub fn as_slice(&self) -> &[u8] {
		self.hview.as_slice(self.size as _)
	}

	/// Returns the underlying file handle.
	#[must_use]
	pub fn hfile(&self) -> &HFILE {
		self.file.hfile()
	}

	/// Returns the size of the file.
	///
	/// This value is cached.
	#[must_use]
	pub const fn size(&self) -> u64 {
		self.size
	}

	/// Returns, in current time zone, 3 times of the file, respectively:
	/// 1. creation time;
	/// 2. last access time;
	/// 3. last write time.
	#[must_use]
	pub fn times(&self) -> SysResult<(SYSTEMTIME, SYSTEMTIME, SYSTEMTIME)> {
		self.file.times()
	}
}
