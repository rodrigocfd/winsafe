const_type! { NM, i32,
	/// [`WM_NOTIFY`](crate::msg::WmNotify)
	/// notifications (`i32`) for:
	///
	/// * [common controls](https://docs.microsoft.com/en-us/windows/win32/controls/common-control-reference#notifications);
	/// * [button](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications);
	/// * [header](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-header-control-reference-notifications);
	/// * [ListView](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-notifications).
}

const_type_priv_values! { NM,
	FIRST, 0
	BCN_FIRST, -1250
	HDN_FIRST, -300
	LVN_FIRST, -100
	TVN_FIRST, -400
}

const_type_pub_values! { NM, // NM
	OUTOFMEMORY, Self::FIRST.0 - 1
	CLICK, Self::FIRST.0 - 2
	DBLCLK, Self::FIRST.0 - 3
	RETURN, Self::FIRST.0 - 4
	RCLICK, Self::FIRST.0 - 5
	RDBLCLK, Self::FIRST.0 - 6
	SETFOCUS, Self::FIRST.0 - 7
	KILLFOCUS, Self::FIRST.0 - 8
	CUSTOMDRAW, Self::FIRST.0 - 12
	HOVER, Self::FIRST.0 - 13
	NCHITTEST, Self::FIRST.0 - 14
	KEYDOWN, Self::FIRST.0 - 15
	RELEASEDCAPTURE, Self::FIRST.0 - 16
	SETCURSOR, Self::FIRST.0 - 17
	CHAR, Self::FIRST.0 - 18
	TOOLTIPSCREATED, Self::FIRST.0 - 19
	LDOWN, Self::FIRST.0 - 20
	RDOWN, Self::FIRST.0 - 21
	THEMECHANGED, Self::FIRST.0 - 22
}


const_type_pub_values! { NM, // BCN
	BCN_HOTITEMCHANGE, Self::BCN_FIRST.0 + 0x0001
	BCN_DROPDOWN, Self::BCN_FIRST.0 + 0x0002
}

const_type_pub_values! { NM, // HDM
	HDN_ITEMCHANGING, Self::HDN_FIRST.0 - 20
	HDN_ITEMCHANGED, Self::HDN_FIRST.0 - 21
	HDN_ITEMCLICK, Self::HDN_FIRST.0 - 22
	HDN_ITEMDBLCLICK, Self::HDN_FIRST.0 - 23
	HDN_DIVIDERDBLCLICK, Self::HDN_FIRST.0 - 25
	HDN_BEGINTRACK, Self::HDN_FIRST.0 - 26
	HDN_ENDTRACK, Self::HDN_FIRST.0 - 27
	HDN_TRACK, Self::HDN_FIRST.0 - 28
	HDN_GETDISPINFO, Self::HDN_FIRST.0 - 29
	HDN_BEGINDRAG, Self::HDN_FIRST.0 - 10
	HDN_ENDDRAG, Self::HDN_FIRST.0 - 11
	HDN_FILTERCHANGE, Self::HDN_FIRST.0 - 12
	HDN_FILTERBTNCLICK, Self::HDN_FIRST.0 - 13
	HDN_BEGINFILTEREDIT, Self::HDN_FIRST.0 - 14
	HDN_ENDFILTEREDIT, Self::HDN_FIRST.0 - 15
	HDN_ITEMSTATEICONCLICK, Self::HDN_FIRST.0 - 16
	HDN_ITEMKEYDOWN, Self::HDN_FIRST.0 - 17
	HDN_DROPDOWN, Self::HDN_FIRST.0 - 18
	HDN_OVERFLOWCLICK, Self::HDN_FIRST.0 - 19
}

const_type_pub_values! { NM, // LVN
	LVN_ITEMCHANGING, Self::LVN_FIRST.0 - 0
	LVN_ITEMCHANGED, Self::LVN_FIRST.0 - 1
	LVN_INSERTITEM, Self::LVN_FIRST.0 - 2
	LVN_DELETEITEM, Self::LVN_FIRST.0 - 3
	LVN_DELETEALLITEMS, Self::LVN_FIRST.0 - 4
	LVN_BEGINLABELEDIT, Self::LVN_FIRST.0 - 75
	LVN_ENDLABELEDIT, Self::LVN_FIRST.0 - 76
	LVN_COLUMNCLICK, Self::LVN_FIRST.0 - 8
	LVN_BEGINDRAG, Self::LVN_FIRST.0 - 9
	LVN_BEGINRDRAG, Self::LVN_FIRST.0 - 11
	LVN_ODCACHEHINT, Self::LVN_FIRST.0 - 13
	LVN_ODFINDITEM, Self::LVN_FIRST.0 - 79
	LVN_ITEMACTIVATE, Self::LVN_FIRST.0 - 14
	LVN_ODSTATECHANGED, Self::LVN_FIRST.0 - 15
	LVN_HOTTRACK, Self::LVN_FIRST.0 - 21
	LVN_GETDISPINFO, Self::LVN_FIRST.0 - 77
	LVN_SETDISPINFO, Self::LVN_FIRST.0 - 78
	LVN_KEYDOWN, Self::LVN_FIRST.0 - 55
	LVN_MARQUEEBEGIN, Self::LVN_FIRST.0 - 56
	LVN_GETINFOTIP, Self::LVN_FIRST.0 - 58
	LVN_INCREMENTALSEARCH, Self::LVN_FIRST.0 - 63
	LVN_COLUMNDROPDOWN, Self::LVN_FIRST.0 - 64
	LVN_COLUMNOVERFLOWCLICK, Self::LVN_FIRST.0 - 66
	LVN_BEGINSCROLL, Self::LVN_FIRST.0 - 80
	LVN_ENDSCROLL, Self::LVN_FIRST.0 - 81
	LVN_LINKCLICK, Self::LVN_FIRST.0 - 84
	LVN_GETEMPTYMARKUP, Self::LVN_FIRST.0 - 87
}

const_type_pub_values! { NM, // TVN
	TVN_SELCHANGING, Self::TVN_FIRST.0 - 50
	TVN_SELCHANGED, Self::TVN_FIRST.0 - 51
	TVN_GETDISPINFO, Self::TVN_FIRST.0 - 52
	TVN_SETDISPINFO, Self::TVN_FIRST.0 - 53
	TVN_ITEMEXPANDING, Self::TVN_FIRST.0 - 54
	TVN_ITEMEXPANDED, Self::TVN_FIRST.0 - 55
	TVN_BEGINDRAG, Self::TVN_FIRST.0 - 56
	TVN_BEGINRDRAG, Self::TVN_FIRST.0 - 57
	TVN_DELETEITEM, Self::TVN_FIRST.0 - 58
	TVN_BEGINLABELEDIT, Self::TVN_FIRST.0 - 59
	TVN_ENDLABELEDIT, Self::TVN_FIRST.0 - 60
	TVN_KEYDOWN, Self::TVN_FIRST.0 - 12
	TVN_GETINFOTIP, Self::TVN_FIRST.0 - 14
	TVN_SINGLEEXPAND, Self::TVN_FIRST.0 - 15
	TVN_ITEMCHANGING, Self::TVN_FIRST.0 - 17
	TVN_ITEMCHANGED, Self::TVN_FIRST.0 - 19
	TVN_ASYNCDRAW, Self::TVN_FIRST.0 - 20
}
