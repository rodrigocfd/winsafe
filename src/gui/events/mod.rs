//! Structs which expose the event methods of windows and controls.

#[macro_use]
mod macros;

mod func_store;

mod button_events;
mod combo_box_events;
mod edit_events;
mod label_events;
mod list_view_events;
mod msg_events;
mod radio_group_events;

pub use button_events::ButtonEvents;
pub use combo_box_events::ComboBoxEvents;
pub use edit_events::EditEvents;
pub use label_events::LabelEvents;
pub use list_view_events::ListViewEvents;
pub use msg_events::MsgEvents;
pub(crate) use msg_events::ProcessResult;
pub use radio_group_events::RadioGroupEvents;
