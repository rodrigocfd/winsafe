use winsafe::{self as w, co};

use crate::stats;

/// Replaces the stats lines directly in the README.md file.
pub fn ask_update_stats(
	hparent: &w::HWND,
	readme_md: &str,
	stats: &stats::Stats,
) -> w::AnyResult<()> {
	let (ret, _, _) = w::TaskDialogIndirect(&w::TASKDIALOGCONFIG {
		hwnd_parent: Some(hparent),
		common_buttons: co::TDCBF::CANCEL,
		main_icon: w::IconIdTd::Td(co::TD_ICON::WARNING),
		flags: co::TDF::ALLOW_DIALOG_CANCELLATION,
		window_title: Some("Update README.md"),
		content: Some("Do you want to update the README.md file?"),
		buttons: &[(co::DLGID::OK.into(), "&Update")],
		..Default::default()
	})?;
	if ret == co::DLGID::OK {
		write_readme(readme_md, stats)?;
	}
	Ok(())
}

fn write_readme(readme_md: &str, stats: &stats::Stats) -> w::AnyResult<()> {
	let (contents, num_bytes) = {
		let f = w::FileMapped::open(readme_md, w::FileAccess::ExistingReadOnly)?;
		(w::WString::parse(f.as_slice())?.to_string(), f.size())
	};

	let mut final_str = String::with_capacity(num_bytes as _); // will replace the README.md contents

	let lines = contents.lines().collect::<Vec<_>>(); // easier to work with a Vec than an iterator
	let idx_line_start = lines
		.iter()
		.position(|line| line.starts_with("| Functions | "))
		.unwrap();
	lines[0..idx_line_start].iter().for_each(|line| {
		final_str.push_str(line); // copy all lines before the block
		final_str.push_str("\r\n");
	});
	final_str.push_str(&stats.to_string()); // copy new block
	final_str.push_str("\r\n");
	lines[idx_line_start + 7..].iter().for_each(|line| {
		final_str.push_str(line); // copy all lines after the block
		final_str.push_str("\r\n");
	});

	let fout = w::File::open(readme_md, w::FileAccess::OpenOrCreateRW)?;
	fout.erase_and_write(&final_str.into_bytes())?;

	Ok(())
}
