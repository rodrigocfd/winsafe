mod haccesstoken;
mod heventlog;
mod hkey;
mod hprocess;
mod hsc;
mod hservice;
mod hservicestatus;
mod hthread;
mod htransaction;

pub mod decl {
	pub use super::haccesstoken::HACCESSTOKEN;
	pub use super::heventlog::HEVENTLOG;
	pub use super::hkey::HKEY;
	pub use super::hsc::HSC;
	pub use super::hservice::HSERVICE;
	pub use super::hservicestatus::HSERVICESTATUS;
	pub use super::htransaction::HTRANSACTION;
}

pub mod traits {
	pub use super::haccesstoken::advapi_Haccesstoken;
	pub use super::heventlog::advapi_Heventlog;
	pub use super::hkey::advapi_Hkey;
	pub use super::hprocess::advapi_Hprocess;
	pub use super::hsc::advapi_Hsc;
	pub use super::hservice::advapi_Hservice;
	pub use super::hservicestatus::advapi_Hservicestatus;
	pub use super::hthread::advapi_Hthread;
	pub use super::htransaction::advapi_Htransaction;
}
