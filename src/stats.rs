use winsafe as w;

#[derive(Default)]
pub struct Stats {
	pub ffis: usize,
	pub structs: usize,
	pub consts: usize,
	pub wmsgs: usize,
	pub handles: usize,
	pub com_interfaces: usize,
	pub com_methods: usize,
}

fn fmt_thousand(n: usize) -> String {
	let thou = (n - (n % 1000)) / 1000;
	if thou > 0 { format!("{},{:03}", thou, n % 1000) } else { n.to_string() }
}

impl std::fmt::Display for Stats {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}",
			format!("| Functions | {} |", fmt_thousand(self.ffis)),
			format!("| Structs | {} |", fmt_thousand(self.structs)),
			format!("| Constants | {} |", fmt_thousand(self.consts)),
			format!("| Window messages | {} |", fmt_thousand(self.wmsgs)),
			format!("| Handles | {} |", fmt_thousand(self.handles)),
			format!("| COM interfaces | {} |", fmt_thousand(self.com_interfaces)),
			format!("| COM methods | {} |", fmt_thousand(self.com_methods)),
		)
	}
}

/// Returns a list of the .rs paths to be scanned.
#[must_use]
pub fn rs_files_list(target: &str) -> w::SysResult<Vec<String>> {
	w::path::dir_list_recursive(target)
		.filter(|file_path| match file_path {
			Err(_) => true,
			Ok(file_path) => {
				w::path::has_extension(file_path, &["rs"]) && !file_path.ends_with("lib.rs")
			},
		})
		.collect::<w::SysResult<Vec<_>>>()
}

/// Reads all files in the target directory and processes all the stats.
///
/// The callback is called after processing each file, to give a progressive
/// feedback of the whole operation.
#[must_use]
pub fn gather<F>(files: &[impl AsRef<str>], callback: F) -> w::SysResult<Stats>
where
	F: Fn(usize),
{
	let mut stats = Stats::default();

	files.iter().enumerate().try_for_each(|(idx, rs_file)| {
		let contents = {
			let f = w::FileMapped::open(rs_file.as_ref(), w::FileAccess::ExistingReadOnly)?;
			w::WString::parse(f.as_slice())?.to_string()
		};
		stats.count_ffis(&contents, rs_file.as_ref());
		stats.count_structs(&contents);
		stats.count_consts(&contents);
		stats.count_wmsgs(&contents);
		stats.count_handles(&contents);
		stats.count_com(&contents, rs_file.as_ref());
		stats.count_com_impl(&contents, rs_file.as_ref());
		callback(idx); // pass the zero-based index of the file that has been processed

		Ok(())
	})?;

	Ok(stats)
}

impl Stats {
	fn count_ffis(&mut self, contents: &str, path: &str) {
		if let Some(file_name) = w::path::get_file_name(path) {
			if file_name != "ffi.rs" {
				return; // only in these files
			}
		}

		let mut inside_block = false;
		for line in contents.lines() {
			if inside_block {
				if line.starts_with("}") {
					inside_block = false;
				} else {
					self.ffis += 1; // each line inside FFI block is counted
				}
			} else {
				if line.starts_with("extern_sys!") {
					inside_block = true;
				}
			}
		}
	}

	fn count_structs(&mut self, contents: &str) {
		for line in contents.lines() {
			if line == "/// struct." {
				self.structs += 1; // simplest approach
			}
		}
	}

	fn count_consts(&mut self, contents: &str) {
		let mut inside_block = false;
		for line in contents.lines() {
			if inside_block {
				if line.starts_with("}") {
					inside_block = false;
				} else {
					if !line.trim().starts_with("//") // skip comments and separators
						&& !line.trim().starts_with("=>")
					{
						self.consts += 1;
					}
				}
			} else {
				if line.starts_with("const_basic_decl!")
					|| line.starts_with("const_ordinary!")
					|| line.starts_with("const_bitflag!")
					|| line.starts_with("const_wm!")
					|| line.starts_with("const_cmd!")
					|| line.starts_with("const_nm!")
					|| line.starts_with("const_ws!")
					|| line.starts_with("const_wsex!")
					|| line.starts_with("const_str!")
					|| line.starts_with("const_values_pub!")
					|| line.starts_with("const_guid_values!")
					|| line.starts_with("const_pkey_values!")
				{
					inside_block = true;
				}
			}
		}
	}

	fn count_wmsgs(&mut self, contents: &str) {
		for line in contents.lines() {
			if line.contains("/// Return type: ") {
				self.wmsgs += 1; // simplest approach
			}
		}
	}

	fn count_handles(&mut self, contents: &str) {
		for line in contents.lines() {
			if line.contains("handle! { ") {
				self.handles += 1; // simplest approach
			}
		}
	}

	fn count_com(&mut self, contents: &str, path: &str) {
		if !path.contains("\\com_interfaces\\") {
			return; // this folder must be present
		} else if let Some(file_name) = w::path::get_file_name(path) {
			if !file_name.starts_with('i') {
				return; // file must start with "i"
			}
		}

		let mut inside_block = false;

		for line in contents.lines() {
			if !inside_block && line.starts_with("com_interface! { ") {
				self.com_interfaces += 1;
				continue;
			}

			if !inside_block && line.starts_with("pub trait ") {
				inside_block = true;
				continue;
			}

			if inside_block {
				if line.starts_with("}") {
					return; // we're done
				} else if line.starts_with("\tfn ")
					|| line.starts_with("\tfn_com_noparm! { ")
					|| line.starts_with("\tfn_com_noparm_noret! { ")
					|| line.starts_with("\tfn_com_interface_get! { ")
					|| line.starts_with("\tfn_com_bstr_get! { ")
					|| line.starts_with("\tfn_com_bstr_set! { ")
				{
					self.com_methods += 1;
				}
			}
		}
	}

	fn count_com_impl(&mut self, contents: &str, path: &str) {
		if !path.contains("\\com_impls\\") {
			return; // this folder must be present
		} else if let Some(file_name) = w::path::get_file_name(path) {
			if !file_name.starts_with('i') {
				return; // file must start with "i"
			}
		}

		let mut inside_block = false;

		for line in contents.lines() {
			if !inside_block && line.starts_with("com_interface_userdef! { ") {
				self.com_interfaces += 1;
				continue;
			}

			if !inside_block && line.starts_with("impl ") {
				inside_block = true;
				continue;
			}

			if inside_block {
				if line.starts_with("}") {
					return; // we're done
				} else if line.starts_with("\tfn_com_userdef_event! { ") {
					self.com_methods += 1;
				}
			}
		}
	}
}
