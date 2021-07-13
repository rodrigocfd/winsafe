use crate::aliases::WinResult;
use crate::co;
use crate::handles::HFILE;
use crate::various::{MappedFile, MappedFileAccess, WString};

/// Keeps sections and key/value pairs of a `.ini` file, also doing parsing and
/// serialization of the data.
///
/// # Examples
///
/// ```rust,ignore
/// use winsafe::Ini;
///
/// let fini = Ini::parse_from_file("C:\\Temp\\my_file.ini").unwrap();
///
/// for section in fini.sections.iter() {
///     println!("Section: {}", section.name);
///     for entry of section.entries.iter() {
///         println!("Key: {}; Value: {}", entry.key, entry.val);
///     }
/// }
/// ```
pub struct Ini {
	/// All the sections of the file.
	pub sections: Vec<IniSection>,
}

/// A single section of an [`Ini`](crate::Ini).
pub struct IniSection {
	/// The name of this section.
	pub name: String,
	/// All key/value pairs of this section.
	pub entries: Vec<IniEntry>,
}

/// A single key/value pair of an [`IniSection`](crate::IniSection) of an
/// [`Ini`](crate::Ini).
pub struct IniEntry {
	/// Key of this entry.
	pub key: String,
	/// Value of this entry.
	pub val: String,
}

impl Ini {
	/// Parses an `Ini` from a string.
	pub fn parse_str(contents: &str) -> Ini {
		let mut sections = Vec::default();
		let mut cur_section = IniSection {
			name: "".to_owned(),
			entries: Vec::default(),
		};

		for line in contents.lines() {
			let line = line.trim();

			if line.starts_with('[') && line.ends_with(']') {
				if cur_section.name != "" || !cur_section.entries.is_empty() {
					sections.push(cur_section);
				}
				cur_section = IniSection {
					name: line[1..line.len() - 1].to_owned(),
					entries: Vec::default(),
				};
				continue;
			}

			if let Some(eq_idx) = line.find('=') {
				cur_section.entries.push(IniEntry {
					key: line[..eq_idx].to_owned(),
					val: line[eq_idx + 1..].to_owned(),
				});
			}
		}
		if cur_section.name != "" || !cur_section.entries.is_empty() {
			sections.push(cur_section);
		}

		Self { sections }
	}

	/// Parses an `Ini` from raw bytes.
	pub fn parse_bytes(bytes: &[u8]) -> WinResult<Ini> {
		Ok(
			Self::parse_str(&WString::parse_str(bytes)?.to_string()),
		)
	}

	/// Parses an `Ini` directly from a file.
	pub fn parse_from_file(ini_path: &str) -> WinResult<Ini> {
		let fin = MappedFile::open(ini_path, MappedFileAccess::Read)?;
		Self::parse_bytes(fin.as_slice())
	}

	/// Serializes the data to a string.
	pub fn serialize_to_str(&self) -> String {
		let mut tot_size = 0;
		for (idx, section) in self.sections.iter().enumerate() {
			tot_size += section.name.len() + 2 + 2;
			for entry in section.entries.iter() {
				tot_size += entry.key.len() + 1 + entry.val.len() + 2;
			}
			if idx < self.sections.len() - 1 {
				tot_size += 2;
			}
		}

		let mut buf = String::with_capacity(tot_size);
		for (idx, section) in self.sections.iter().enumerate() {
			buf.push('[');
			buf.push_str(&section.name);
			buf.push_str("]\r\n");

			for entry in section.entries.iter() {
				buf.push_str(&entry.key);
				buf.push('=');
				buf.push_str(&entry.val);
				buf.push_str("\r\n");
			}

			if idx < self.sections.len() - 1 {
				buf.push_str("\r\n");
			}
		}
		buf
	}

	/// Serializes the data to raw bytes.
	pub fn serialize_to_bytes(&self) -> Vec<u8> {
		self.serialize_to_str().into_bytes()
	}

	/// Serializes the data directly to a file.
	pub fn serialize_to_file(&self, ini_path: &str) -> WinResult<()> {
		let (fout, _) = HFILE::CreateFile(ini_path, co::GENERIC::WRITE,
			co::FILE_SHARE::NONE, None, co::DISPOSITION::CREATE_ALWAYS,
			co::FILE_ATTRIBUTE::NORMAL, None)?;

		fout.WriteFile(&self.serialize_to_bytes(), None)?;
		fout.CloseHandle()?;
		Ok(())
	}

	/// Returns a reference to the specified value, if any.
	pub fn value(&self, section: &str, key: &str) -> Option<&str> {
		self.sections.iter()
			.find(|s| s.name == section)
			.map(|s| s.entries.iter()
				.find(|e| e.key == key)
				.map(|e| e.val.as_ref())
			).flatten()
	}

	/// Returns a mutable reference to the specified value, if any.
	pub fn value_mut(&mut self, section: &str, key: &str) -> Option<&mut str> {
		self.sections.iter_mut()
			.find(|s| s.name == section)
			.map(|s| s.entries.iter_mut()
				.find(|e| e.key == key)
				.map(|e| e.val.as_mut())
			).flatten()
	}
}
