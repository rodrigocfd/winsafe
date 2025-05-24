mod hmodule;
mod hprocess;

pub mod traits {
	pub use super::hmodule::psapi_Hmodule;
	pub use super::hprocess::psapi_Hprocess;
}
