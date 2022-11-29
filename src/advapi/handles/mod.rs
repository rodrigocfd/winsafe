mod hkey;

pub mod decl {
	pub use super::hkey::HKEY;
}

pub mod guard {
	pub use super::hkey::HkeyGuard;
}

pub mod traits {
	pub use super::hkey::advapi_Hkey;
}
