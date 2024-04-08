use std::cmp::Ordering;

use crate::gui::{*, spec::*};

pub(in crate::gui) extern "system" fn list_view_item_sort<F>(
	lparam1: isize,
	lparam2: isize,
	lparam_sort: isize,
) -> i32
	where F: FnMut(ListViewItem, ListViewItem) -> Ordering,
{
	let data = unsafe { &mut *(lparam_sort as *mut (&ListView, &mut F)) };
	let item1 = data.0.items().get(lparam1 as _);
	let item2 = data.0.items().get(lparam2 as _);
	data.1(item1, item2) as _
}
