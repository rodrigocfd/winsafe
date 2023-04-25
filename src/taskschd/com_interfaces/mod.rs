mod iregisteredtask;
mod itaskdefinition;
mod itaskfolder;
mod itaskservice;
mod itriggercollection;

pub mod decl {
	pub use super::iregisteredtask::IRegisteredTask;
	pub use super::itaskdefinition::ITaskDefinition;
	pub use super::itaskfolder::ITaskFolder;
	pub use super::itaskservice::ITaskService;
	pub use super::itriggercollection::ITriggerCollection;
}

pub mod traits {
	pub use super::iregisteredtask::taskschd_IRegisteredTask;
	pub use super::itaskdefinition::taskschd_ITaskDefinition;
	pub use super::itaskfolder::taskschd_ITaskFolder;
	pub use super::itaskservice::taskschd_ITaskService;
	pub use super::itriggercollection::taskschd_ITriggerCollection;
}

pub mod vt {
	pub use super::iregisteredtask::IRegisteredTaskVT;
	pub use super::itaskdefinition::ITaskDefinitionVT;
	pub use super::itaskfolder::ITaskFolderVT;
	pub use super::itaskservice::ITaskServiceVT;
	pub use super::itriggercollection::ITriggerCollectionVT;
}
