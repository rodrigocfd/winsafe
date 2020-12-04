//! Component Object Model interfaces.

pub mod clsid;
pub mod iid;

mod itaskbarlist;
mod iunknown;

pub use itaskbarlist::*;
pub use iunknown::*;