//! Raw bindings to comctl32.lib functions.

#[link(name = "comctl32")]
extern "system" {
	pub fn InitCommonControls();
}