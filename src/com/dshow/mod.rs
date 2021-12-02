//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! COM interfaces, structs and constants.
//!
//! To enable the DirectShow COM module, use:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.8", features = ["dshow"] }
//! ```

pub mod clsid;
pub mod co;
pub mod guid;

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
pub(in crate::com) mod structs;

pub use ibasefilter::IBaseFilter;
pub use ienumfilters::IEnumFilters;
pub use ifilesinkfilter::IFileSinkFilter;
pub use ifiltergraph::IFilterGraph;
pub use igraphbuilder::IGraphBuilder;
pub use imediacontrol::IMediaControl;
pub use imediafilter::IMediaFilter;
pub use imediaseeking::IMediaSeeking;
pub use imfgetservice::IMFGetService;
pub use imfvideodisplaycontrol::IMFVideoDisplayControl;
pub use ipin::IPin;
pub use structs::*;

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
