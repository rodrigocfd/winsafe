const_bitflag! { CC: u32;
	/// [`CHOOSECOLOR`](crate::CHOOSECOLOR) `Flags` (`u32`).
	=>
	=>
	/// Causes the dialog box to use the color specified in the `rgbResult`
	/// member as the initial color selection.
	RGBINIT 0x0000_0001
	/// Causes the dialog box to display the additional controls that allow the
	/// user to create custom colors. If this flag is not set the user must
	/// click the Define Custom Color button to display the custom color
	/// controls.
	FULLOPEN 0x0000_0002
	/// Disables the Define Custom Color button.
	PREVENTFULLOPEN 0x0000_0004
	/// Causes the dialog box to display the Help button. The `hwndOwner` member
	/// must specify the window to receive the `HELPMSGSTRING` registered
	/// messages that the dialog box sends when the user clicks the Help button.
	SHOWHELP 0x0000_0008
	/// Enables the hook procedure specified in the `lpfnHook` member of this
	/// structure. This flag is used only to initialize the dialog box.
	ENABLEHOOK 0x0000_0010
	/// The `hInstance` and `lpTemplateName` members specify a dialog box
	/// template to use in place of the default template. This flag is used only
	/// to initialize the dialog box.
	ENABLETEMPLATE 0x0000_0020
	/// The `hInstance` member identifies a data block that contains a preloaded
	/// dialog box template. The system ignores the `lpTemplateName` member if
	/// this flag is specified. This flag is used only to initialize the dialog
	/// box.
	ENABLETEMPLATEHANDLE 0x0000_0040
	/// Causes the dialog box to display only solid colors in the set of basic
	/// colors.
	SOLIDCOLOR 0x0000_0080
	/// Causes the dialog box to display all available colors in the set of
	/// basic colors.
	ANYCOLOR 0x0000_0100
}
