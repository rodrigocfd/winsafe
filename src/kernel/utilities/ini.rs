use crate::kernel::decl::{File, FileAccess, FileMapped, SysResult, WString};

/// High-level abstraction to load, manage and serialize sections and key/value
/// pairs of a `.ini` file.
///
/// # Examples
///
/// Printing all sections, keys and values:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::Ini;
///
/// let ini = Ini::parse_from_file("C:\\Temp\\foo.ini")?;
///
/// for section in ini.sections.iter() {
///     println!("Section: {}", section.name);
///     for entry in section.entries.iter() {
///         println!("Key: {}; Value: {}", entry.key, entry.val);
///     }
/// }
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
///
/// Reading a value:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::Ini;
///
/// let ini = Ini::parse_from_file("C:\\Temp\\foo.ini")?;
///
/// println!("{}", ini.value("the_section", "the_key").unwrap());
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
///
/// Changing an existing value:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::Ini;
///
/// let ini_path = "C:\\Temp\\foo.ini";
/// let mut ini = Ini::parse_from_file(ini_path)?;
///
/// *ini.value_mut("the_section", "the_key").unwrap() = "new_value".to_owned();
///
/// ini.serialize_to_file(ini_path)?;
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub struct Ini {
	/// All the sections of the file. The values can be modified at will.
	pub sections: Vec<IniSection>,
}

/// A single section of an [`Ini`](crate::Ini).
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub struct IniSection {
	/// The name of this section.
	pub name: String,
	/// All key/value pairs of this section.
	pub entries: Vec<IniEntry>,
}

/// A single key/value pair of an [`IniSection`](crate::IniSection) of an
/// [`Ini`](crate::Ini).
#[cfg_attr(docsrs, doc(cfg(feature = "kernel")))]
pub struct IniEntry {
	/// Key of this entry.
	pub key: String,
	/// Value of this entry.
	pub val: String,
}

impl Ini {
	/// Parses an `Ini` from a string.
	#[must_use]
	pub fn parse_str(contents: &str) -> Ini {
		let mut sections = Vec::<IniSection>::default();
		let mut cur_section = IniSection {
			name: "".to_owned(),
			entries: Vec::<IniEntry>::default(),
		};

		for line in contents.lines() {
			let line = line.trim();

			if line.starts_with('[') && line.ends_with(']') {
				if cur_section.name != "" || !cur_section.entries.is_empty() {
					sections.push(cur_section);
				}
				cur_section = IniSection {
					name: line[1..line.len() - 1].to_owned(),
					entries: Vec::<IniEntry>::default(),
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

	/// Parses an `Ini` from raw bytes with
	/// [`WString::parse`](crate::WString::parse).
	#[must_use]
	pub fn parse_bytes(bytes: &[u8]) -> SysResult<Ini> {
		Ok(
			Self::parse_str(&WString::parse(bytes)?.to_string()),
		)
	}

	/// Parses an `Ini` directly from a file with
	/// [`WString::parse`](crate::WString::parse). The file will be
	/// [mapped in memory](crate::FileMapped) during reading for maximum
	/// performance.
	#[must_use]
	pub fn parse_from_file(ini_path: &str) -> SysResult<Ini> {
		let fin = FileMapped::open(ini_path, FileAccess::ExistingReadOnly)?;
		Self::parse_bytes(fin.as_slice())
	}

	/// Serializes the sections and entries to a string.
	#[must_use]
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

	/// Serializes the sections and entries to raw bytes with
	/// [`String::into_bytes`](https://doc.rust-lang.org/std/string/struct.String.html#method.into_bytes).
	#[must_use]
	pub fn serialize_to_bytes(&self) -> Vec<u8> {
		self.serialize_to_str().into_bytes()
	}

	/// Serializes the data directly to a file with
	/// [`String::into_bytes`](https://doc.rust-lang.org/std/string/struct.String.html#method.into_bytes).
	pub fn serialize_to_file(&self, ini_path: &str) -> SysResult<()> {
		let fout = File::open(ini_path, FileAccess::OpenOrCreateRW)?;
		fout.erase_and_write(&self.serialize_to_bytes())?;
		Ok(())
	}

	/// Returns a reference to the specified value, if any.
	#[must_use]
	pub fn value(&self, section: &str, key: &str) -> Option<&str> {
		self.sections.iter()
			.find(|s| s.name == section)
			.and_then(|s| {
				s.entries.iter()
					.find(|e| e.key == key)
					.map(|e| e.val.as_ref())
			})
	}

	/// Returns a mutable reference to the specified value, if any.
	///
	/// # Examples
	///
	/// Changing an existing value:
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::Ini;
	///
	/// let ini_path = "C:\\Temp\\foo.ini";
	/// let mut ini = Ini::parse_from_file(ini_path)?;
	///
	/// *ini.value_mut("the_section", "the_key").unwrap() = "new_value".to_owned();
	///
	/// ini.serialize_to_file(ini_path)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	#[must_use]
	pub fn value_mut(&mut self, section: &str, key: &str) -> Option<&mut String> {
		self.sections.iter_mut()
			.find(|s| s.name == section)
			.and_then(|s| {
				s.entries.iter_mut()
					.find(|e| e.key == key)
					.map(|e| &mut e.val)
			})
	}
}
