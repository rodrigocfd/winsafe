mod imfattributes;
mod imfmediaeventgenerator;
mod imfmediasession;
mod imfpresentationdescriptor;
mod imftopology;
mod imftopologynode;

pub mod decl {
	pub use super::imfattributes::IMFAttributes;
	pub use super::imfmediaeventgenerator::IMFMediaEventGenerator;
	pub use super::imfmediasession::IMFMediaSession;
	pub use super::imfpresentationdescriptor::IMFPresentationDescriptor;
	pub use super::imftopology::IMFTopology;
	pub use super::imftopologynode::IMFTopologyNode;
}

pub mod traits {
	pub use super::imfattributes::mf_IMFAttributes;
	pub use super::imfmediaeventgenerator::mf_IMFMediaEventGenerator;
	pub use super::imfmediasession::mf_IMFMediaSession;
	pub use super::imfpresentationdescriptor::mf_IMFPresentationDescriptor;
	pub use super::imftopology::mf_IMFTopology;
	pub use super::imftopologynode::mf_IMFTopologyNode;
}

pub mod vt {
	pub use super::imfattributes::IMFAttributesVT;
	pub use super::imfmediaeventgenerator::IMFMediaEventGeneratorVT;
	pub use super::imfmediasession::IMFMediaSessionVT;
	pub use super::imfpresentationdescriptor::IMFPresentationDescriptorVT;
	pub use super::imftopology::IMFTopologyVT;
	pub use super::imftopologynode::IMFTopologyNodeVT;
}
