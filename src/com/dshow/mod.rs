//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! COM interfaces, structs and constants.
//!
//! To enable the DirectShow COM module, use:
//!
//! ```toml
//! [dependencies]
//! winsafe = { version = "0.0.5", features = ["dshow"] }
//! ```

pub mod co;
pub mod clsid;
pub mod guid;

#[macro_use] mod ienumfilters; // 2nd level interfaces
#[macro_use] mod ifilesinkfilter;
#[macro_use] mod ifiltergraph;
#[macro_use] mod imediaseeking;
#[macro_use] mod imfgetservice;
#[macro_use] mod imfvideodisplaycontrol;
#[macro_use] mod ipin;

#[macro_use] mod igraphbuilder; // 3rd level interfaces
#[macro_use] mod imediacontrol;
#[macro_use] mod imediafilter;

#[macro_use] mod ibasefilter; // 4th level interface

mod any_structs;

pub use any_structs::*;
pub use ibasefilter::IBaseFilter;
pub use ifilesinkfilter::IFileSinkFilter;
pub use ienumfilters::IEnumFilters;
pub use ifiltergraph::IFilterGraph;
pub use igraphbuilder::IGraphBuilder;
pub use imediacontrol::IMediaControl;
pub use imediafilter::IMediaFilter;
pub use imediaseeking::IMediaSeeking;
pub use imfgetservice::IMFGetService;
pub use imfvideodisplaycontrol::IMFVideoDisplayControl;
pub use ipin::IPin;

/// [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
/// COM virtual tables.
pub mod vt {
	pub use super::ibasefilter::IBaseFilterVT;
	pub use super::ifilesinkfilter::IFileSinkFilterVT;
	pub use super::ienumfilters::IEnumFiltersVT;
	pub use super::ifiltergraph::IFilterGraphVT;
	pub use super::igraphbuilder::IGraphBuilderVT;
	pub use super::imediacontrol::IMediaControlVT;
	pub use super::imediafilter::IMediaFilterVT;
	pub use super::imediaseeking::IMediaSeekingVT;
	pub use super::imfgetservice::IMFGetServiceVT;
	pub use super::imfvideodisplaycontrol::IMFVideoDisplayControlVT;
	pub use super::ipin::IPinVT;
}
