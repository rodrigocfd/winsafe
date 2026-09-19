#![allow(non_camel_case_types)]

use crate::macros::*;

const_ordinary! { USB_DESCRIPTOR_TYPE: u8;
	/// USB
	/// [descriptor type](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/usb/ns-usb-_urb_control_descriptor_request)
	/// (`u8`).
	///
	/// Originally has `USB` prefix and `DESCRIPTOR_TYPE` suffix.
	=>
	/// USB 1.1.
	DEVICE 0x01
	/// USB 1.1.
	CONFIGURATION 0x02
	/// USB 1.1.
	STRING 0x03
	/// USB 1.1.
	INTERFACE 0x04
	/// USB 1.1.
	ENDPOINT 0x05

	/// USB 2.0.
	DEVICE_QUALIFIER 0x06
	/// USB 2.0.
	OTHER_SPEED_CONFIGURATION 0x07
	/// USB 2.0.
	INTERFACE_POWER 0x08
	/// USB 2.0.
	EUSB2_ISOCH_ENDPOINT_COMPANION 0x12

	/// USB 3.0.
	OTG 0x09
	/// USB 3.0.
	DEBUG 0x0a
	/// USB 3.0.
	INTERFACE_ASSOCIATION 0x0b
	/// USB 3.0.
	BOS 0x0f
	/// USB 3.0.
	DEVICE_CAPABILITY 0x10
	/// USB 3.0.
	SUPERSPEED_ENDPOINT_COMPANION 0x30

	/// USB 3.1.
	SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION 0x31
}
