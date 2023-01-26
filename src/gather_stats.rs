use winsafe as w;

struct Stats {
	ffis: usize,
	structs: usize,
	consts: usize,
}
impl Stats {
	fn new() -> Self {
		Self {
			ffis: 0,
			structs: 0,
			consts: 0,
		}
	}
}

/// Reads all files in the target directory and processes all the stats, calling
/// the callback to give feedback after processing each file.
pub fn process<F>(target: &str, callback: F) -> w::SysResult<String>
	where F: Fn(usize),
{
	let mut stats = Stats::new();

	w::path::dir_walk(target)
		.enumerate()
		.try_for_each(|(idx, path)| -> w::SysResult<_> {
			let path = path?;
			if w::path::has_extension(&path, &[".rs"]) {
				let contents = {
					let f = w::FileMapped::open(&path, w::FileAccess::ExistingReadOnly)?;
					w::WString::parse(f.as_slice())?.to_string()
				};
				count_ffis(&contents, &mut stats);
				count_structs(&contents, &mut stats);
				count_consts(&contents, &mut stats);
				callback(idx);
			}
			Ok(())
		})?;

	Ok( "shadows disappear".to_owned() )
}

fn count_ffis(contents: &str, stats: &mut Stats) {
	for line in contents.lines() {

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
