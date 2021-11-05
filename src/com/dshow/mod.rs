//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! COM interfaces, structs and constants.
//!
//! To enable the DirectShow COM module, use:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.7", features = ["dshow"] }
//! ```

pub mod clsid;
pub mod co;
pub mod guid;

pub(in crate::com) mod any_structs;
pub(in crate::com) mod ibasefilter;
pub(in crate::com) mod ienumfilters;
pub(in crate::com) mod ifilesinkfilter;
pub(in crate::com) mod ifiltergraph;
pub(in crate::com) mod igraphbuilder;
pub(in crate::com) mod imediacontrol;
pub(in crate::com) mod imediafilter;
pub(in crate::com) mod imediaseeking;
pub(in crate::com) mod imfgetservice;
pub(in crate::com) mod imfvideodisplaycontrol;
pub(in crate::com) mod ipin;

pub use any_structs::*;
pub use ibasefilter::{IBaseFilter, IBaseFilterT};
pub use ienumfilters::{IEnumFilters, IEnumFiltersT};
pub use ifilesinkfilter::{IFileSinkFilter, IFileSinkFilterT};
pub use ifiltergraph::{IFilterGraph, IFilterGraphT};
pub use igraphbuilder::{IGraphBuilder, IGraphBuilderT};
pub use imediacontrol::{IMediaControl, IMediaControlT};
pub use imediafilter::{IMediaFilter, IMediaFilterT};
pub use imediaseeking::{IMediaSeeking, IMediaSeekingT};
pub use imfgetservice::{IMFGetService, IMFGetServiceT};
pub use imfvideodisplaycontrol::{IMFVideoDisplayControl, IMFVideoDisplayControlT};
pub use ipin::IPin;

pub(crate) mod prelude {
	pub use super::ibasefilter::IBaseFilterT;
	pub use super::ienumfilters::IEnumFiltersT;
	pub use super::ifilesinkfilter::IFileSinkFilterT;
	pub use super::ifiltergraph::IFilterGraphT;
	pub use super::igraphbuilder::IGraphBuilderT;
	pub use super::imediacontrol::IMediaControlT;
	pub use super::imediafilter::IMediaFilterT;
	pub use super::imediaseeking::IMediaSeekingT;
	pub use super::imfgetservice::IMFGetServiceT;
	pub use super::imfvideodisplaycontrol::IMFVideoDisplayControlT;
	pub use super::ipin::IPinT;
}

/// [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
/// COM virtual tables.
pub mod vt {
	pub use super::ibasefilter::IBaseFilterVT;
	pub use super::ienumfilters::IEnumFiltersVT;
	pub use super::ifilesinkfilter::IFileSinkFilterVT;
	pub use super::ifiltergraph::IFilterGraphVT;
	pub use super::igraphbuilder::IGraphBuilderVT;
	pub use super::imediacontrol::IMediaControlVT;
	pub use super::imediafilter::IMediaFilterVT;
	pub use super::imediaseeking::IMediaSeekingVT;
	pub use super::imfgetservice::IMFGetServiceVT;
	pub use super::imfvideodisplaycontrol::IMFVideoDisplayControlVT;
	pub use super::ipin::IPinVT;
}
