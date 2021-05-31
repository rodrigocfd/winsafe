use std::any::Any;

use crate::gui::base::Base;
use crate::gui::native_controls::{
	Button,
	CheckBox,
	ComboBox,
	DateTimePicker,
	Edit,
	Label,
	ListBox,
	ListView,
	MonthCalendar,
	ProgressBar,
	RadioButton,
	StatusBar,
	Trackbar,
};
use crate::gui::{WindowControl, WindowMain, WindowModal};
use crate::handles::HWND;

/// Trait to any window which can host child controls.
pub trait Parent {
	/// Returns a reference to the `Any` trait, allowing downcasting.
	fn as_any(&self) -> &dyn Any;
}

/// Trait to any child control.
pub trait Child {
	/// Returns a reference to the `Any` trait, allowing downcasting.
	fn as_any(&self) -> &dyn Any;
}

pub(crate) fn baseref_from_parent(parent: &dyn Parent) -> &Base {
	if let Some(w) = parent.as_any().downcast_ref::<WindowMain>() {
		w.base_ref()
	} else if let Some(w) = parent.as_any().downcast_ref::<WindowModal>() {
		w.base_ref()
	} else if let Some(w) = parent.as_any().downcast_ref::<WindowControl>() {
		w.base_ref()
	} else {
		panic!("Unknown Parent downcasting, something really bad happened.")
	}
}

pub(crate) fn hwndref_from_child(child: &dyn Child) -> &HWND {
	if let Some(c) = child.as_any().downcast_ref::<WindowControl>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<Button>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<CheckBox>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<ComboBox>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<DateTimePicker>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<Edit>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<Label>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<ListBox>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<ListView>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<MonthCalendar>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<ProgressBar>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<RadioButton>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<StatusBar>() {
		c.base_ref().hwnd_ref()
	} else if let Some(c) = child.as_any().downcast_ref::<Trackbar>() {
		c.base_ref().hwnd_ref()
	} else {
		panic!("Unknown Child downcasting, something really bad happened.")
	}
}
