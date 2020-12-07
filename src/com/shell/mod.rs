//! Shell COM interfaces.

pub mod clsid;

mod itaskbarlist;
mod itaskbarlist2;
mod itaskbarlist3;

pub use itaskbarlist::*;
pub use itaskbarlist2::*;
pub use itaskbarlist3::*;