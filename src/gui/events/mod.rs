mod base_ctrl_events;
mod base_wnd_events;
mod button_events;
mod combo_box_events;
mod date_time_picker_events;
mod edit_events;
mod header_events;
mod label_events;
mod list_box_events;
mod list_view_events;
mod month_calendar_events;
mod radio_group_events;
mod status_bar_events;
mod tab_events;
mod trackbar_events;
mod tree_view_events;
mod up_down_events;

pub(in crate::gui) mod privs {
	pub(in crate::gui) use super::base_ctrl_events::{BaseCtrlEvents, priv_ctrl_events};
	pub(in crate::gui) use super::base_wnd_events::BaseWndEvents;
}

pub mod traits {
	pub use super::base_wnd_events::{GuiEventsParent, GuiEventsWindow};
	pub use super::button_events::GuiEventsButton;
	pub use super::combo_box_events::GuiEventsComboBox;
	pub use super::date_time_picker_events::GuiEventsDateTimePicker;
	pub use super::edit_events::GuiEventsEdit;
	pub use super::header_events::GuiEventsHeader;
	pub use super::label_events::GuiEventsLabel;
	pub use super::list_box_events::GuiEventsListBox;
	pub use super::list_view_events::GuiEventsListView;
	pub use super::month_calendar_events::GuiEventsMonthCalendar;
	pub use super::radio_group_events::GuiEventsRadioGroup;
	pub use super::status_bar_events::GuiEventsStatusBar;
	pub use super::tab_events::GuiEventsTab;
	pub use super::trackbar_events::GuiEventsTrackbar;
	pub use super::tree_view_events::GuiEventsTreeView;
	pub use super::up_down_events::GuiEventsUpDown;
}
