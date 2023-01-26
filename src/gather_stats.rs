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

pub fn process(target: &str) -> w::SysResult<String> {
	let mut stats = Stats::new();

	w::path::dir_walk(target).try_for_each(|path| -> w::SysResult<_> {
		let path = path?;
		if w::path::has_extension(&path, &[".rs"]) {
			count_ffis(&path, &mut stats);
			count_structs(&path, &mut stats);
			println!("[{}]", path);
		}
		Ok(())
	})?;

	Ok( "shadows disappear".to_owned() )
}

fn count_ffis(path: &str, stats: &mut Stats) {

}

fn count_structs(path: &str, stats: &mut Stats) {

}

fn count_consts(path: &str, stats: &mut Stats) {

}
