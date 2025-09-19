High-level GUI abstractions for user windows and native controls. They can be created programmatically or by loading resources from a `.res` file. These files can be created with a WYSIWYG [resource editor](https://en.wikipedia.org/wiki/Resource_(Windows)#Resource_software).

## Windows

There are 5 types of windows, which can host child controls:

* [`WindowControl`] – a custom, user-defined child control;
* [`WindowMain`];
* [`WindowMessageOnly`];
* [`WindowModal`];
* [`WindowModeless`].

You'll probably want to start your GUI application using the [`WindowMain`].

## Native controls

Native controls are hosted by windows, and receive various types of user input.

* [`Button`];
* [`CheckBox`];
* [`ComboBox`];
* [`DateTimePicker`];
* [`Edit`] (textbox);
* [`Header`];
* [`Label`];
* [`ListBox`];
* [`ListView`] (grid);
* [`MonthCalendar`];
* [`ProgressBar`];
* [`PropSheet`];
* [`RadioButton`];
* [`StatusBar`];
* [`Tab`];
* [`Trackbar`];
* [`TreeView`];
* [`UpDown`].
