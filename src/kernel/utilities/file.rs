use crate::co;
use crate::decl::*;
use crate::guard::*;
use crate::prelude::*;

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

impl std::fmt::Display for FileAccess {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			FileAccess::ExistingReadOnly => "Existing file, read-only",
			FileAccess::ExistingRW => "Existing file, read and write",
			FileAccess::OpenOrCreateRW => "Open existing file or create new file, read and write",
			FileAccess::CreateRW => "Create new file, read and write",
		})
	}
}

/// Manages an [`HFILE`](crate::HFILE) handle, which provides file read/write
/// and other operations. It is closed automatically when the object goes out of
/// scope.
///
/// This is an alternative to the standard [`std::fs::File`], with a possibly
/// faster implementation since it's Windows-only.
///
/// If you just want to read the file, consider memory-mapping it with
/// [`FileMapped`](crate::FileMapped), which tends to be faster.
///
/// # Examples
///
/// Reading the contents as a string:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let f = w::File::open(
///     "C:\\Temp\\foo.txt",
///     w::FileAccess::ExistingRW,
/// )?;
/// let raw_bytes = f.read_all()?;
/// let text = w::WString::parse(&raw_bytes)?.to_string();
/// # w::SysResult::Ok(())
/// ```
///
/// Erasing the file and writing a string:
///
/// ```no_run
/// use winsafe::{self as w, prelude::*};
///
/// let f = w::File::open(
///     "C:\\Temp\\foo.txt",
///     w::FileAccess::OpenOrCreateRW,
/// )?;
/// f.set_size(0)?; // truncate
/// f.write("My text".as_bytes())?;
/// # w::SysResult::Ok(())
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

	/// Truncates the file size to zero, then writes the bytes.
	pub fn erase_and_write(&self, data: &[u8]) -> SysResult<()> {
		self.set_size(0)?;
		self.write(data)
	}

	/// Returns the underlying file handle.
	#[must_use]
	pub fn hfile(&self) -> &HFILE {
		&*self.hfile
	}

	/// Returns the position of the file pointer by calling
	/// [`HFILE::SetFilePointerEx`](crate::prelude::kernel_Hfile::SetFilePointerEx).
	#[must_use]
	pub fn pointer_offset(&self) -> SysResult<u64> {
		self.hfile.SetFilePointerEx(0, co::FILE_STARTING_POINT::CURRENT) // https://stackoverflow.com/a/17707021/6923555
			.map(|off| off as _)
	}

	/// Returns the size of the file by calling
	/// [`HFILE::GetFileSizeEx`](crate::prelude::kernel_Hfile::GetFileSizeEx),
	/// allocates the `Vec` buffer, then reads all the file bytes by calling
	/// [`HFILE::ReadFile`](crate::prelude::kernel_Hfile::ReadFile).
	///
	/// Note that the API limits the reading up to 4 GB.
	#[must_use]
	pub fn read_all(&self) -> SysResult<Vec<u8>> {
		self.set_pointer_offset(0)?;
		let mut buf = vec![0x00; self.size()? as _];
		self.read_buffer(&mut buf)?;
		Ok(buf)
	}

	/// Calls [`HFILE::ReadFile`](crate::prelude::kernel_Hfile::ReadFile) to
	/// read at most `buffer.len()` bytes from the file, starting at the current
	/// file pointer offset. Returns how many bytes were actually read. The file
	/// pointer is then incremented by the number of bytes read.
	///
	/// Note that the API limits the reading up to 4 GB.
	pub fn read_buffer(&self, buffer: &mut [u8]) -> SysResult<u32> {
		self.hfile.ReadFile(buffer)
	}

	/// Sets the position of the file pointer by calling
	/// [`HFILE::SetFilePointerEx`](crate::prelude::kernel_Hfile::SetFilePointerEx).
	pub fn set_pointer_offset(&self, offset: u64) -> SysResult<()> {
		self.hfile.SetFilePointerEx(offset as _, co::FILE_STARTING_POINT::BEGIN)
			.map(|_| ())
	}

	/// Truncates or expands the file by calling
	/// [`HFILE::SetFilePointerEx`](crate::prelude::kernel_Hfile::SetFilePointerEx)
	/// and
	/// [`HFILE::SetEndOfFile`](crate::prelude::kernel_Hfile::SetEndOfFile),
	/// then sets the file pointer to the beginning of the file.
	///
	/// If the size is increased, the contents in the new area are undefined.
	///
	/// Note that sometimes the system will make the file larger than you
	/// specified, so don't bindly trust this method.
	pub fn set_size(&self, num_bytes: u64) -> SysResult<()> {
		self.set_pointer_offset(num_bytes)?;
		self.hfile.SetEndOfFile()?;
		self.set_pointer_offset(0)
	}

	/// Returns the size of the file by calling
	/// [`HFILE::GetFileSizeEx`](crate::prelude::kernel_Hfile::GetFileSizeEx).
	#[must_use]
	pub fn size(&self) -> SysResult<u64> {
		self.hfile.GetFileSizeEx()
	}

	/// Returns, in current time zone, 3 times of the file, respectively:
	/// 1. creation time;
	/// 2. last access time;
	/// 3. last write time.
	#[must_use]
	pub fn times(&self) -> SysResult<(SYSTEMTIME, SYSTEMTIME, SYSTEMTIME)> {
		let (ft_creation, ft_last_access, ft_last_write) = self.hfile.GetFileTime()?;

		let st_creation_utc = FileTimeToSystemTime(&ft_creation)?;
		let st_last_access_utc = FileTimeToSystemTime(&ft_last_access)?;
		let st_last_write_utc = FileTimeToSystemTime(&ft_last_write)?;

		let st_creation_local = SystemTimeToTzSpecificLocalTime(None, &st_creation_utc)?;
		let st_last_access_local = SystemTimeToTzSpecificLocalTime(None, &st_last_access_utc)?;
		let st_last_write_local = SystemTimeToTzSpecificLocalTime(None, &st_last_write_utc)?;

		Ok((st_creation_local, st_last_access_local, st_last_write_local))
	}

	/// Writes the bytes at the current file pointer by calling
	/// [`HFILE::WriteFile`](crate::prelude::kernel_Hfile::WriteFile).
	///
	/// This method will fail if the file was opened with
	/// [`FileAccess::ExistingReadOnly`](crate::FileAccess::ExistingReadOnly).
	pub fn write(&self, data: &[u8]) -> SysResult<()> {
		self.hfile.WriteFile(data)
			.map(|_| ())
	}
}
