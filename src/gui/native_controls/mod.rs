mod base_ctrl;
mod button;
mod check_box;
mod combo_box;
mod combo_box_items;
mod date_time_picker;
mod edit;
mod header;
mod header_item;
mod header_items;
mod label;
mod list_box;
mod list_box_items;
mod list_view;
mod list_view_col;
mod list_view_cols;
mod list_view_item;
mod list_view_items;
mod month_calendar;
mod progress_bar;
mod radio_button;
mod radio_group;
mod status_bar;
mod status_bar_part;
mod status_bar_parts;
mod tab;
mod tab_item;
mod tab_items;
mod tab_page;
mod trackbar;
mod tree_view;
mod tree_view_item;
mod tree_view_items;
mod up_down;

pub(in crate::gui) mod privs {
	pub(in crate::gui) use super::base_ctrl::BaseCtrl;
	pub(in crate::gui) use super::tree_view_items::{TreeViewChildItemIter, TreeViewItemIter};
}

pub mod decl {
	pub use super::button::{Button, ButtonOpts};
	pub use super::check_box::{CheckBox, CheckBoxOpts};
	pub use super::combo_box::{ComboBox, ComboBoxOpts};
	pub use super::date_time_picker::{DateTimePicker, DateTimePickerOpts};
	pub use super::edit::{Edit, EditOpts};
	pub use super::header::{Header, HeaderOpts};
	pub use super::header_item::{HeaderArrow, HeaderItem, HeaderJustify};
	pub use super::label::{Label, LabelOpts};
	pub use super::list_box::{ListBox, ListBoxOpts};
	pub use super::list_view::{ListView, ListViewOpts};
	pub use super::list_view_col::ListViewCol;
	pub use super::list_view_item::ListViewItem;
	pub use super::month_calendar::{MonthCalendar, MonthCalendarOpts};
	pub use super::progress_bar::{ProgressBar, ProgressBarOpts};
	pub use super::radio_button::{RadioButton, RadioButtonOpts};
	pub use super::radio_group::RadioGroup;
	pub use super::status_bar::{SbPart, StatusBar};
	pub use super::status_bar_part::StatusBarPart;
	pub use super::tab::{Tab, TabOpts};
	pub use super::tab_item::TabItem;
	pub use super::tab_page::{TabPage, TabPageOpts};
	pub use super::trackbar::{Trackbar, TrackbarOpts};
	pub use super::tree_view::{TreeView, TreeViewOpts};
	pub use super::tree_view_item::TreeViewItem;
	pub use super::up_down::{UpDown, UpDownOpts};
}

pub mod collections {
	//! Objects which exposes methods to work upon individual items of certain
	//! native controls.

	pub use super::combo_box_items::ComboBoxItems;
	pub use super::header_items::HeaderItems;
	pub use super::list_box_items::ListBoxItems;
	pub use super::list_view_cols::ListViewCols;
	pub use super::list_view_items::ListViewItems;
	pub use super::status_bar_parts::StatusBarParts;
	pub use super::tab_items::TabItems;
	pub use super::tree_view_items::TreeViewItems;
}
