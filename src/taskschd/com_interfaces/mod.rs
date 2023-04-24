mod itaskdefinition;
mod itaskfolder;
mod itaskservice;

pub mod decl {
	pub use super::itaskdefinition::ITaskDefinition;
	pub use super::itaskfolder::ITaskFolder;
	pub use super::itaskservice::ITaskService;
}

pub mod traits {
	pub use super::itaskdefinition::taskschd_ITaskDefinition;
	pub use super::itaskfolder::taskschd_ITaskFolder;
	pub use super::itaskservice::taskschd_ITaskService;
}

pub mod vt {
	pub use super::itaskdefinition::ITaskDefinitionVT;
	pub use super::itaskfolder::ITaskFolderVT;
	pub use super::itaskservice::ITaskServiceVT;
}
