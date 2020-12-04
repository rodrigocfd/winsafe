//! Shell COM interfaces.

pub mod clsid;
pub mod iid;

mod itaskbarlist;
mod itaskbarlist2;

pub use itaskbarlist::*;
pub use itaskbarlist2::*;