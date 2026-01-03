mod iaction;
mod iactioncollection;
mod iboottrigger;
mod icomhandleraction;
mod idailytrigger;
mod iemailaction;
mod ieventtrigger;
mod iexecaction;
mod iidletrigger;
mod ilogontrigger;
mod iprincipal;
mod iregisteredtask;
mod iregistrationinfo;
mod itaskdefinition;
mod itaskfolder;
mod itaskservice;
mod itasksettings;
mod itrigger;
mod itriggercollection;

pub mod decl {
	pub use super::iaction::IAction;
	pub use super::iactioncollection::IActionCollection;
	pub use super::iboottrigger::IBootTrigger;
	pub use super::icomhandleraction::IComHandlerAction;
	pub use super::idailytrigger::IDailyTrigger;
	pub use super::iemailaction::IEmailAction;
	pub use super::ieventtrigger::IEventTrigger;
	pub use super::iexecaction::IExecAction;
	pub use super::iidletrigger::IIdleTrigger;
	pub use super::ilogontrigger::ILogonTrigger;
	pub use super::iprincipal::IPrincipal;
	pub use super::iregisteredtask::IRegisteredTask;
	pub use super::iregistrationinfo::IRegistrationInfo;
	pub use super::itaskdefinition::ITaskDefinition;
	pub use super::itaskfolder::ITaskFolder;
	pub use super::itaskservice::ITaskService;
	pub use super::itasksettings::ITaskSettings;
	pub use super::itrigger::ITrigger;
	pub use super::itriggercollection::ITriggerCollection;
}

pub mod traits {
	pub use super::iaction::taskschd_IAction;
	pub use super::iactioncollection::taskschd_IActionCollection;
	pub use super::iboottrigger::taskschd_IBootTrigger;
	pub use super::icomhandleraction::taskschd_IComHandlerAction;
	pub use super::idailytrigger::taskschd_IDailyTrigger;
	pub use super::iemailaction::taskschd_IEmailAction;
	pub use super::ieventtrigger::taskschd_IEventTrigger;
	pub use super::iexecaction::taskschd_IExecAction;
	pub use super::iidletrigger::taskschd_IIdleTrigger;
	pub use super::ilogontrigger::taskschd_ILogonTrigger;
	pub use super::iprincipal::taskschd_IPrincipal;
	pub use super::iregisteredtask::taskschd_IRegisteredTask;
	pub use super::iregistrationinfo::taskschd_IRegistrationInfo;
	pub use super::itaskdefinition::taskschd_ITaskDefinition;
	pub use super::itaskfolder::taskschd_ITaskFolder;
	pub use super::itaskservice::taskschd_ITaskService;
	pub use super::itasksettings::taskschd_ITaskSettings;
	pub use super::itrigger::taskschd_ITrigger;
	pub use super::itriggercollection::taskschd_ITriggerCollection;
}
