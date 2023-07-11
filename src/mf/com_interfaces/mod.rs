mod imfattributes;
mod imfmediaeventgenerator;
mod imfmediasession;

pub mod decl {
	pub use super::imfattributes::IMFAttributes;
	pub use super::imfmediaeventgenerator::IMFMediaEventGenerator;
	pub use super::imfmediasession::IMFMediaSession;
}

pub mod traits {
	pub use super::imfattributes::mf_IMFAttributes;
	pub use super::imfmediaeventgenerator::mf_IMFMediaEventGenerator;
	pub use super::imfmediasession::mf_IMFMediaSession;
}

pub mod vt {
	pub use super::imfattributes::IMFAttributesVT;
	pub use super::imfmediaeventgenerator::IMFMediaEventGeneratorVT;
	pub use super::imfmediasession::IMFMediaSessionVT;
}
