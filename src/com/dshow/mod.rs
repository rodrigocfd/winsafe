//! [DirectShow](https://docs.microsoft.com/en-us/windows/win32/directshow/directshow)
//! COM interfaces, structs and constants.

pub mod co;
pub mod clsid;
pub mod guid;
pub mod vt;

#[macro_use] mod ienumfilters;
#[macro_use] mod ifilesinkfilter;
#[macro_use] mod ifiltergraph;
#[macro_use] mod imediacontrol;
#[macro_use] mod imediafilter;
#[macro_use] mod imediaseeking;
#[macro_use] mod imfgetservice;
#[macro_use] mod imfvideodisplaycontrol;
#[macro_use] mod ipin;

#[macro_use] mod ibasefilter;
#[macro_use] mod igraphbuilder;

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
