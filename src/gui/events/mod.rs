//! Structs which expose the event methods of windows and controls.

mod func_store;

mod button_events;
mod msg_events;

pub use button_events::ButtonEvents;
pub use msg_events::MsgEvents;
pub(crate) use msg_events::ProcessResult;
