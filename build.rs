fn main() {
	println!("cargo:rustc-link-lib=dylib:+verbatim=resources/resources.res");
}
