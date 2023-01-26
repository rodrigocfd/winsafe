use winsafe as w;

pub struct Stats {
	pub ffis: usize,
	pub structs: usize,
	pub consts: usize,
}

impl Stats {
	fn new() -> Self {
		Self {
			ffis: 0,
			structs: 0,
			consts: 0,
		}
	}

	/// Returns the stats as the formatted output.
	pub fn format(&self) -> String {
		format!("{}\r\n{}\r\n{}",
			format!("| Functions | {} |", self.ffis),
			format!("| Structs | {} |", self.structs),
			format!("| Constants | {} |", self.consts),
		)
	}

	/// Reads all files in the target directory and processes all the stats, calling
	/// the callback to give feedback after processing each file.
	pub fn gather<F>(target: &str, callback: F) -> w::SysResult<Self>
		where F: Fn(usize),
	{
		let mut stats = Self::new();

		w::path::dir_walk(target)
			.enumerate()
			.try_for_each(|(idx, path)| -> w::SysResult<_> {
				let path = path?;
				if w::path::has_extension(&path, &[".rs"]) {
					let contents = {
						let f = w::FileMapped::open(&path, w::FileAccess::ExistingReadOnly)?;
						w::WString::parse(f.as_slice())?.to_string()
					};
					Self::count_ffis(&contents, &mut stats);
					Self::count_structs(&contents, &mut stats);
					Self::count_consts(&contents, &mut stats);
					callback(idx);
				}
				Ok(())
			})?;

		Ok(stats)
	}

	fn count_ffis(contents: &str, stats: &mut Stats) {
		let mut inside_block = false;
		for line in contents.lines() {
			if inside_block {
				if line.starts_with("}") {
					inside_block = false;
				} else {
					stats.ffis += 1;
				}
			} else {
				if line.starts_with("extern_sys!") {
					inside_block = true;
				}
			}
		}
	}

	fn count_structs(contents: &str, stats: &mut Stats) {
		for line in contents.lines() {

		}
	}

	fn count_consts(contents: &str, stats: &mut Stats) {
		for line in contents.lines() {

		}
	}
}
