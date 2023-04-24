mod itaskservice;

pub mod decl {
	pub use super::itaskservice::ITaskService;
}

pub mod traits {
	pub use super::itaskservice::taskschd_ITaskService;
}

pub mod vt {
	pub use super::itaskservice::ITaskServiceVT;
}
