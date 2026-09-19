use crate::co;
use crate::decl::*;

/// Variant parameter for:
///
/// * [`HUSB::GetDescriptor`](crate::HUSB::GetDescriptor)
pub enum UsbDescr {
	Device(USB_DEVICE_DESCRIPTOR),
	Configuration(UsbConfiguratorDescriptorGuard),
}

impl UsbDescr {
	/// Returns the correspondent [`co::USB_DESCRIPTOR_TYPE`](crate::co::USB_DESCRIPTOR_TYPE) flag.
	#[must_use]
	pub const fn flag(&self) -> co::USB_DESCRIPTOR_TYPE {
		use UsbDescr::*;
		match self {
			Device(_) => co::USB_DESCRIPTOR_TYPE::DEVICE,
			Configuration(_) => co::USB_DESCRIPTOR_TYPE::CONFIGURATION,
		}
	}
}
