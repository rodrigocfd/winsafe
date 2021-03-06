//! Structs which expose the event methods of windows and controls.
//!
//! You cannot directly instantiate these objects, they are created internally
//! by the controls.

#[macro_use]
mod macros;

mod func_store;

mod events_bn_rg;
mod events_bn;
mod events_cbn;
mod events_dtn;
mod events_en;
mod events_lbn;
mod events_lvn;
mod events_mcn;
mod events_sbn;
mod events_stn;
mod events_wm;

pub use events_bn_rg::RadioGroupEvents;
pub use events_bn::ButtonEvents;
pub use events_cbn::ComboBoxEvents;
pub use events_dtn::DateTimePickerEvents;
pub use events_en::EditEvents;
pub use events_lbn::ListBoxEvents;
pub use events_lvn::ListViewEvents;
pub use events_mcn::MonthCalendarEvents;
pub use events_sbn::StatusBarEvents;
pub use events_stn::LabelEvents;
pub use events_wm::WindowEvents;
pub(crate) use events_wm::ProcessResult;
