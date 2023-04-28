mod idailytrigger;
mod ieventtrigger;
mod iidletrigger;
mod ilogontrigger;
mod iregisteredtask;
mod itaskdefinition;
mod itaskfolder;
mod itaskservice;
mod itrigger;
mod itriggercollection;

pub mod decl {
	pub use super::idailytrigger::IDailyTrigger;
	pub use super::ieventtrigger::IEventTrigger;
	pub use super::iidletrigger::IIdleTrigger;
	pub use super::ilogontrigger::ILogonTrigger;
	pub use super::iregisteredtask::IRegisteredTask;
	pub use super::itaskdefinition::ITaskDefinition;
	pub use super::itaskfolder::ITaskFolder;
	pub use super::itaskservice::ITaskService;
	pub use super::itrigger::ITrigger;
	pub use super::itriggercollection::ITriggerCollection;
}

pub mod traits {
	pub use super::idailytrigger::taskschd_IDailyTrigger;
	pub use super::ieventtrigger::taskschd_IEventTrigger;
	pub use super::iidletrigger::taskschd_IIdleTrigger;
	pub use super::ilogontrigger::taskschd_ILogonTrigger;
	pub use super::iregisteredtask::taskschd_IRegisteredTask;
	pub use super::itaskdefinition::taskschd_ITaskDefinition;
	pub use super::itaskfolder::taskschd_ITaskFolder;
	pub use super::itaskservice::taskschd_ITaskService;
	pub use super::itrigger::taskschd_ITrigger;
	pub use super::itriggercollection::taskschd_ITriggerCollection;
}

pub mod vt {
	pub use super::idailytrigger::IDailyTriggerVT;
	pub use super::ieventtrigger::IEventTriggerVT;
	pub use super::iidletrigger::IIdleTriggerVT;
	pub use super::ilogontrigger::ILogonTriggerVT;
	pub use super::iregisteredtask::IRegisteredTaskVT;
	pub use super::itaskdefinition::ITaskDefinitionVT;
	pub use super::itaskfolder::ITaskFolderVT;
	pub use super::itaskservice::ITaskServiceVT;
	pub use super::itrigger::ITriggerVT;
	pub use super::itriggercollection::ITriggerCollectionVT;
}
