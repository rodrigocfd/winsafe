use std::fs;
use std::path::Path;

fn main() {
	const RES: &str = "resources/resources.res";

	// Create a copy of .res file, and append .lib extension.
	// This is necessary because of a limitation in the toolchain.
	// https://github.com/rust-lang/rust/issues/81488

	let mut dotlib = String::from(RES);
	dotlib.push_str(".lib");

	if Path::new(RES).exists() {
		fs::copy(RES, &dotlib).unwrap();
	}

	println!("cargo:rustc-link-lib=dylib={}", RES);
}
