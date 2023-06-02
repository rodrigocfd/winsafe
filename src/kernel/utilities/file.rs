use crate::co;
use crate::kernel::decl::{HFILE, SysResult};
use crate::kernel::guard::CloseHandleGuard;
use crate::prelude::kernel_Hfile;

/// Access types for [`File::open`](crate::File::open) and
/// [`FileMapped::open`](crate::FileMapped::open).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileAccess {
	/// Opens the file as read-only. Fails if the file doesn't exist.
	ExistingReadOnly,
	/// Opens the file as read/write. Fails if the file doesn't exist.
	ExistingRW,
	/// Opens the file as read/write. If the file doesn't exist, it will be
	/// created.
	OpenOrCreateRW,
	/// Creates a new file as read/write. Fails if the file already exists.
	CreateRW,
}

//------------------------------------------------------------------------------

/// Manages an [`HFILE`](crate::HFILE) handle, which provides file read/write
/// and other operations. It is closed automatically when the object goes out of
/// scope.
///
/// This is an alternative to the standard
/// [`std::fs::File`](https://doc.rust-lang.org/std/fs/struct.File.html), with a
/// possibly faster implementation since it's Windows-only.
///
/// If you just want to read the file, consider memory-mapping it with
/// [`FileMapped`](crate::FileMapped), which tends to be faster.
///
/// # Examples
///
/// Reading the contents as a string:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{File, FileAccess, WString};
///
/// let f = File::open("C:\\Temp\\foo.txt", FileAccess::ExistingRW)?;
/// let raw_bytes = f.read_all()?;
/// let text = WString::parse(&raw_bytes)?.to_string();
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
///
/// Writing a string:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{File, FileAccess};
///
/// let f = File::open("C:\\Temp\\foo.txt", FileAccess::OpenOrCreateRW)?;
/// f.erase_and_write("My text".as_bytes())?;
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
pub struct File {
	hfile: CloseHandleGuard<HFILE>,
}

impl File {
	/// Opens a file with the desired access.
	#[must_use]
	pub fn open(file_path: &str, access: FileAccess) -> SysResult<Self> {
		let (acc, share, disp) = match access {
			FileAccess::ExistingReadOnly => (
				co::GENERIC::READ,
				Some(co::FILE_SHARE::READ),
				co::DISPOSITION::OPEN_EXISTING,
			),
			FileAccess::ExistingRW => (
				co::GENERIC::READ | co::GENERIC::WRITE,
				None,
				co::DISPOSITION::OPEN_EXISTING,
			),
			FileAccess::OpenOrCreateRW => (
				co::GENERIC::READ | co::GENERIC::WRITE,
				None,
				co::DISPOSITION::OPEN_ALWAYS,
			),
			FileAccess::CreateRW => (
				co::GENERIC::READ | co::GENERIC::WRITE,
				None,
				co::DISPOSITION::CREATE_NEW,
			),
		};

		let (hfile, _) = HFILE::CreateFile(
			file_path, acc, share, None, disp,
			co::FILE_ATTRIBUTE::NORMAL, None, None, None)?;
		Ok(Self { hfile })
	}

	/// Erases the file content, then writes the new bytes.
	///
	/// The internal file pointer will be rewound to the beginning of the file.
	pub fn erase_and_write(&self, data: &[u8]) -> SysResult<()> {
		self.resize(data.len() as _)?;
		self.write(data)?;
		self.rewind_pointer()
	}

	/// Returns the underlying file handle.
	#[must_use]
	pub fn hfile(&self) -> &HFILE {
		&*self.hfile
	}

	/// Returns the current offset of the internal pointer.
	#[must_use]
	pub fn pointer_offset(&self) -> SysResult<u64> {
		self.hfile.SetFilePointerEx(0, co::FILE_STARTING_POINT::CURRENT) // https://stackoverflow.com/a/17707021/6923555
			.map(|off| off as _)
	}

	/// Reads bytes from the file.
	///
	/// Note that the bytes will start being read from the current offset of the
	/// internal file pointer, which is then incremented by the size of
	/// `buffer`.
	pub fn read(&self, buffer: &mut [u8]) -> SysResult<u64> {
		self.hfile.ReadFile(buffer, None)
			.map(|n| n as _)
	}

	/// Reads all the bytes from the file into a new `Vec`.
	///
	/// The internal file pointer will be rewound to the beginning of the file.
	#[must_use]
	pub fn read_all(&self) -> SysResult<Vec<u8>> {
		self.rewind_pointer()?;
		let mut data = vec![0u8; self.size()? as _];
		let bytes_read = self.read(&mut data)?;
		data.resize(bytes_read as _, 0);
		self.rewind_pointer()?;
		Ok(data)
	}

	/// Truncates or expands the file, according to the new size. Zero will empty
	/// the file.
	///
	/// The internal file pointer will be rewound to the beginning of the file.
	pub fn resize(&self, num_bytes: u64) -> SysResult<()> {
		self.hfile.SetFilePointerEx(num_bytes as _, co::FILE_STARTING_POINT::BEGIN)?;
		self.hfile.SetEndOfFile()?;
		self.rewind_pointer()
	}

	/// Rewinds the internal file pointer to the beginning of the file.
	pub fn rewind_pointer(&self) -> SysResult<()> {
		self.hfile.SetFilePointerEx(0, co::FILE_STARTING_POINT::BEGIN)?;
		Ok(())
	}

	/// Returns the size of the file.
	#[must_use]
	pub fn size(&self) -> SysResult<u64> {
		self.hfile.GetFileSizeEx()
	}

	/// Writes the given bytes. The content will be written at the position
	/// currently pointed by the internal file pointer.
	pub fn write(&self, data: &[u8]) -> SysResult<()> {
		self.hfile.WriteFile(data, None)?;
		Ok(())
	}
}
