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

impl Stats {
	/// Returns the stats as the formatted output.
	pub fn format(&self) -> String {
		format!("{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}",
			format!("| Functions | {} |", Self::fmt_thou(self.ffis)),
			format!("| Structs | {} |", Self::fmt_thou(self.structs)),
			format!("| Constants | {} |", Self::fmt_thou(self.consts)),
			format!("| Window messages | {} |", Self::fmt_thou(self.wmsgs)),
			format!("| Handles | {} |", Self::fmt_thou(self.handles)),
			format!("| COM interfaces | {} |", Self::fmt_thou(self.com_interfaces)),
			format!("| COM methods | {} |", Self::fmt_thou(self.com_methods)),
		)
	}

	fn fmt_thou(n: usize) -> String {
		let thou = (n - (n % 1000)) / 1000;
		if thou > 0 {
			format!("{},{}", thou, n % 1000)
		} else {
			n.to_string()
		}
	}

	/// Reads all files in the target directory and processes all the stats.
	/// 
	/// The callback is called after processing each file, to give a progressive
	/// feedback of the whole operation.
	pub fn gather<F>(target: &str, callback: F) -> w::SysResult<Self>
		where F: Fn(usize),
	{
		let mut me = Self::default();

		w::path::dir_walk(target)
			.enumerate()
			.try_for_each(|(idx, path)| -> w::SysResult<_> {
				let path = path?;
				if w::path::has_extension(&path, &[".rs"]) {
					let contents = {
						let f = w::FileMapped::open(&path, w::FileAccess::ExistingReadOnly)?;
						w::WString::parse(f.as_slice())?.to_string()
					};
					me.count_ffis(&contents, &path);
					me.count_structs(&contents);
					me.count_consts(&contents);
					me.count_wmsgs(&contents);
					me.count_handles(&contents);
					me.count_com(&contents, &path);
					callback(idx); // pass the zero-based index of the file that has been processed
				}
				Ok(())
			})?;

		Ok(me)
	}

	fn count_ffis(&mut self, contents: &str, path: &str) {
		if let Some(file_name) = w::path::get_file_name(path) {
			if file_name != "ffi.rs" {
				return;
			}
		}

		let mut inside_block = false;
		for line in contents.lines() {
			if inside_block {
				if line.starts_with("}") {
					inside_block = false;
				} else {
					self.ffis += 1;
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
				self.structs += 1;
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
					if !line.starts_with("\t//") &&
						!line.starts_with("\t=>") {
						self.consts += 1;
					}
				}
			} else {
				if line.starts_with("const_values!") ||
					line.starts_with("const_bitflag!") ||
					line.starts_with("const_ordinary!") ||
					line.starts_with("const_wm!") ||
					line.starts_with("const_nm!") ||
					line.starts_with("const_cmd!") ||
					line.starts_with("const_ws!") ||
					line.starts_with("const_wsex!") {

					inside_block = true;
				}
			}
		}
	}

	fn count_wmsgs(&mut self, contents: &str) {
		for line in contents.lines() {
			if line.contains("/// Return type: ") {
				self.wmsgs += 1;
			}
		}
	}

	fn count_handles(&mut self, contents: &str) {
		for line in contents.lines() {
			if line.contains("impl_handle! { ") {
				self.handles += 1;
			}
		}
	}

	fn count_com(&mut self, contents: &str, path: &str) {
		if !path.contains("\\com_interfaces\\") {
			return;
		} else if let Some(file_name) = w::path::get_file_name(path) {
			if !file_name.starts_with('i') {
				return;
			}
		}

		let mut is_com_interface_file = false;
		let mut inside_block = false;

		for line in contents.lines() {
			if !is_com_interface_file && line.starts_with("com_interface! { ") {
				is_com_interface_file = true;
				self.com_interfaces += 1;
			} else if is_com_interface_file {
				if !inside_block && line.starts_with("pub trait ") {
					inside_block = true;
				} else if inside_block {
					if line.starts_with('}') {
						inside_block = false;
					} else if line.starts_with("\tfn ") {
						self.com_methods += 1;
					}
				}
			}
		}
	}
}