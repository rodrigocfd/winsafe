#![allow(non_snake_case)]

use crate::co;

/// [`USB_CONFIGURATION_DESCRIPTOR`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbspec/ns-usbspec-_usb_configuration_descriptor)
/// struct.
#[repr(C)]
pub struct USB_CONFIGURATION_DESCRIPTOR {
	bLength: u8,
	bDescriptorType: co::USB_DESCRIPTOR_TYPE,
	pub wTotalLength: u16,
	pub bNumInterfaces: u8,
	pub bConfigurationValue: u8,
	pub iConfiguration: u8,
	pub bmAttributes: u8,
	pub MaxPower: u8,
}

impl Default for USB_CONFIGURATION_DESCRIPTOR {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.bLength = std::mem::size_of::<Self>() as _;
		obj.bDescriptorType = co::USB_DESCRIPTOR_TYPE::CONFIGURATION;
		obj
	}
}

/// [`USB_DEVICE_DESCRIPTOR`](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usbspec/ns-usbspec-_usb_device_descriptor)
/// struct.
#[repr(C)]
pub struct USB_DEVICE_DESCRIPTOR {
	bLength: u8,
	bDescriptorType: co::USB_DESCRIPTOR_TYPE,
	pub bcdUSB: u16,
	pub bDeviceClass: u8,
	pub bDeviceSubClass: u8,
	pub bDeviceProtocol: u8,
	pub bMaxPacketSize0: u8,
	pub idVendor: u16,
	pub idProduct: u16,
	pub bcdDevice: u16,
	pub iManufacturer: u8,
	pub iProduct: u8,
	pub iSerialNumber: u8,
	pub bNumConfigurations: u8,
}

impl Default for USB_DEVICE_DESCRIPTOR {
	fn default() -> Self {
		let mut obj = unsafe { std::mem::zeroed::<Self>() };
		obj.bLength = std::mem::size_of::<Self>() as _;
		obj.bDescriptorType = co::USB_DESCRIPTOR_TYPE::DEVICE;
		obj
	}
}
