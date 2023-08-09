use crate::decl::*;

/// High-level abstraction to load, manage and serialize sections and key/value
/// pairs of a `.ini` file.
///
/// # Examples
///
/// Printing all sections, keys and values:
///
/// ```no_run
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
/// ```no_run
/// use winsafe::prelude::*;
/// use winsafe::Ini;
///
/// let ini = Ini::parse_from_file("C:\\Temp\\foo.ini")?;
///
/// println!("{}", ini.value("the_section", "the_key").unwrap());
/// # Ok::<_, winsafe::co::ERROR>(())
/// ```
pub struct Ini {
	/// All the sections of the file. They can be modified at will.
	pub sections: Vec<IniSection>,
}

impl Ini {
	/// Parses an `Ini` from a string.
	#[must_use]
	pub fn parse_str(contents: &str) -> Self {
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
	pub fn parse_bytes(bytes: &[u8]) -> SysResult<Self> {
		Ok(
			Self::parse_str(&WString::parse(bytes)?.to_string()),
		)
	}

	/// Parses an `Ini` directly from a file with
	/// [`WString::parse`](crate::WString::parse). The file will be
	/// [mapped in memory](crate::FileMapped) during reading for maximum
	/// performance.
	#[must_use]
	pub fn parse_from_file(ini_path: &str) -> SysResult<Self> {
		let fin = FileMapped::open(ini_path, FileAccess::ExistingReadOnly)?;
		Self::parse_bytes(fin.as_slice())
	}

	/// Returns a reference to the [`IniSection`](crate::IniSection) with the
	/// given name, if any.
	///
	/// The search is case-insensitive.
	#[must_use]
	pub fn find_section(&self, name: &str) -> Option<&IniSection> {
		let name_uc = name.to_uppercase();
		self.sections.iter()
			.find(|sec| sec.name.to_uppercase() == name_uc)
			.map(|sec| sec)
	}

	/// Returns a mutable reference to the [`IniSection`](crate::IniSection)
	/// with the given name, if any.
	///
	/// The search is case-insensitive.
	#[must_use]
	pub fn find_section_mut(&mut self, name: &str) -> Option<&mut IniSection> {
		let name_uc = name.to_uppercase();
		self.sections.iter_mut()
			.find(|sec| sec.name.to_uppercase() == name_uc)
			.map(|sec| sec)
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
	/// [`String::into_bytes`](std::string::String::into_bytes).
	#[must_use]
	pub fn serialize_to_bytes(&self) -> Vec<u8> {
		self.serialize_to_str().into_bytes()
	}

	/// Serializes the data directly to a file with
	/// [`String::into_bytes`](std::string::String::into_bytes).
	pub fn serialize_to_file(&self, ini_path: &str) -> SysResult<()> {
		let fout = File::open(ini_path, FileAccess::OpenOrCreateRW)?;
		fout.erase_and_write(&self.serialize_to_bytes())?;
		Ok(())
	}

	/// If the section/key exists, changes its value, otherwise creates it.
	///
	/// # Examples
	///
	/// Changing or creating a value:
	///
	/// ```no_run
	/// use winsafe::prelude::*;
	/// use winsafe::Ini;
	///
	/// let ini_path = "C:\\Temp\\foo.ini";
	/// let mut ini = Ini::parse_from_file(ini_path)?;
	///
	/// ini.set_value("the_section", "the_key", "new_value");
	///
	/// ini.serialize_to_file(ini_path)?;
	/// # Ok::<_, winsafe::co::ERROR>(())
	/// ```
	pub fn set_value(&mut self, section: &str, key: &str, new_value: &str) {
		match self.find_section_mut(section) {
			Some(sec) => match sec.find_entry_mut(key) {
				Some(ent) => ent.val = new_value.to_owned(),
				None => sec.entries.push(IniEntry { // entry does not exist in section
					key: key.to_owned(),
					val: new_value.to_owned(),
				}),
			},
			None => self.sections.push(IniSection { // section does not exist
				name: section.to_owned(),
				entries: vec![IniEntry {
					key: key.to_owned(),
					val: new_value.to_owned(),
				}],
			}),
		}
	}

	/// Returns a reference to the specified value, if any.
	///
	/// The search is case-insensitive.
	#[must_use]
	pub fn value(&self, section: &str, key: &str) -> Option<&str> {
		self.find_section(section)
			.and_then(|sec| {
				sec.find_entry(key)
					.map(|ent| ent.val.as_ref())
			})
	}

	/// Returns a mutable reference to the specified value, if any.
	///
	/// The search is case-insensitive.
	///
	/// # Examples
	///
	/// Changing an existing value:
	///
	/// ```no_run
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
	pub fn value_mut(&mut self,
		section: &str, key: &str) -> Option<&mut String>
	{
		self.find_section_mut(section)
			.and_then(|sec| {
				sec.find_entry_mut(key)
					.map(|ent| &mut ent.val)
			})
	}
}

//------------------------------------------------------------------------------

/// A single section of an [`Ini`](crate::Ini).
pub struct IniSection {
	/// The name of this section.
	pub name: String,
	/// All key/value pairs of this section. They can be modified at will.
	pub entries: Vec<IniEntry>,
}

impl IniSection {
	/// Returns a reference to the [`IniEntry`](crate::IniEntry) with the given
	/// key, if any.
	///
	/// The search is case-insensitive.
	#[must_use]
	pub fn find_entry(&self, key: &str) -> Option<&IniEntry> {
		let key_uc = key.to_uppercase();
		self.entries.iter()
			.find(|ent| ent.key.to_uppercase() == key_uc)
			.map(|ent| ent)
	}

	/// Returns a mutable reference to the [`IniEntry`](crate::IniEntry) with
	/// the given key, if any.
	///
	/// The search is case-insensitive.
	#[must_use]
	pub fn find_entry_mut(&mut self, key: &str) -> Option<&mut IniEntry> {
		let key_uc = key.to_uppercase();
		self.entries.iter_mut()
			.find(|ent| ent.key.to_uppercase() == key_uc)
			.map(|ent| ent)
	}
}

//------------------------------------------------------------------------------

/// A single key/value pair of an [`IniSection`](crate::IniSection) of an
/// [`Ini`](crate::Ini).
pub struct IniEntry {
	/// Key of this entry.
	pub key: String,
	/// Value of this entry.
	pub val: String,
}
