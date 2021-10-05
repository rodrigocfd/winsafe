use crate::aliases::WinResult;
use crate::co;
use crate::handles::HFILE;

/// Access types for [`File::open`](crate::File::open) and
/// [`FileMapped::open`](crate::FileMapped::open).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileAccess {
	/// Opens the file as read-only. Fails if the file doesn't exist.
	ExistingReadOnly,
	/// Opens the file as read/write. Fails if the file doesn't exist.
	ExistingReadWrite,
	/// Opens the file as read/write. If the file doesn't exist, it will be
	/// created.
	OpenOrCreateReadWrite,
}

//------------------------------------------------------------------------------

/// Manages an [`HFILE`](crate::HFILE) handle, which is closed automatically
/// when the object goes out of scope.
///
/// This is an alternative to the standard
/// [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html), with a
/// possibly faster implementation since it's Windows-only.
pub struct File {
	hfile: HFILE,
}

impl Drop for File {
	fn drop(&mut self) {
		self.hfile.CloseHandle().ok(); // ignore errors
	}
}

impl File {
	/// Opens a file with the desired access.
	pub fn open(file_path: &str, access: FileAccess) -> WinResult<File> {
		let (acc, share, disp) = match access {
			FileAccess::ExistingReadOnly =>  (
				co::GENERIC::READ,
				co::FILE_SHARE::READ,
				co::DISPOSITION::OPEN_EXISTING,
			),
			FileAccess::ExistingReadWrite => (
				co::GENERIC::READ | co::GENERIC::WRITE,
				co::FILE_SHARE::NoValue,
				co::DISPOSITION::OPEN_EXISTING,
			),
			FileAccess::OpenOrCreateReadWrite => (
				co::GENERIC::READ | co::GENERIC::WRITE,
				co::FILE_SHARE::NoValue,
				co::DISPOSITION::OPEN_ALWAYS,
			),
		};

		let (hfile, _) = HFILE::CreateFile(
			file_path, acc, share, None, disp, co::FILE_ATTRIBUTE::NORMAL, None)?;
		Ok(Self { hfile })
	}

	/// Erases the file content, then writes the new bytes.
	///
	/// The internal file pointer will be rewound to the beginning of the file.
	pub fn erase_and_write(&self, data: &[u8]) -> WinResult<()> {
		self.resize(data.len())?;
		self.write(data)?;
		self.rewind_pointer()
	}

	/// Returns the underlying file handle.
	pub const fn hfile(&self) -> HFILE {
		self.hfile
	}

	/// Returns the current offset of the internal pointer.
	pub fn pointer_offset(&self) -> WinResult<usize> {
		self.hfile.SetFilePointerEx(0, co::FILE_STARTING_POINT::CURRENT) // https://stackoverflow.com/a/17707021/6923555
			.map(|off| off as _)
	}

	/// Reads the given number of bytes from the file into a new `Vec`.
	///
	/// Note that the bytes will start being read from the current offset of the
	/// internal file pointer, which is then incremented by `num_bytes`.
	pub fn read(&self, num_bytes: usize) -> WinResult<Vec<u8>> {
		let mut buf = vec![0x00; num_bytes];
		self.hfile.ReadFile(&mut buf, num_bytes as _, None)?;
		Ok(buf)
	}

	/// Reads all the bytes from the file into a new `Vec`.
	///
	/// The internal file pointer will be rewound to the beginning of the file.
	pub fn read_all(&self) -> WinResult<Vec<u8>> {
		self.rewind_pointer()?;
		let data = self.read(self.size()?)?;
		self.rewind_pointer()?;
		Ok(data)
	}

	/// Truncates or expands the file, according to the new size. Zero will empty
	/// the file.
	pub fn resize(&self, num_bytes: usize) -> WinResult<()> {
		self.hfile.SetFilePointerEx(num_bytes as _, co::FILE_STARTING_POINT::BEGIN)?;
		self.hfile.SetEndOfFile()?;
		self.rewind_pointer()
	}

	/// Rewinds the internal file pointer to the beginning of the file.
	pub fn rewind_pointer(&self) -> WinResult<()> {
		self.hfile.SetFilePointerEx(0, co::FILE_STARTING_POINT::BEGIN)?;
		Ok(())
	}

	/// Returns the size of the file.
	pub fn size(&self) -> WinResult<usize> {
		self.hfile.GetFileSizeEx()
	}

	/// Writes the given bytes. The content will be written at the position
	/// currently pointed by the internal file pointer.
	pub fn write(&self, data: &[u8]) -> WinResult<()> {
		self.hfile.WriteFile(data, None)?;
		Ok(())
	}
}
