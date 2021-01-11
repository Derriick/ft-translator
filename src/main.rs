use std::io;

use anyhow::Result;
use clap::ArgMatches;
use env_logger;
use log::error;
use serde::{Deserialize, Serialize};

mod file;
mod options;

#[derive(Debug, Deserialize)]
struct Entry {
	comp_type: String,
	comp_name: String,
	ref_number: u32,
	text: Option<String>,
}

#[derive(Debug, Serialize)]
struct DictEntry {
	src: String,
	dst: String,
}

fn main() {
	let matches = options::matches();

	env_logger::builder()
		.filter_level(options::verbosity(&matches))
		.init();

	if let Err(err) = run(matches) {
		error!("{}", err);
	}
}

#[inline]
fn run(matches: ArgMatches) -> Result<()> {
	let result = {
		if let Some(mut values) = matches.values_of(options::CREATE_DICT) {
			let path_src = values.next().unwrap();
			let dict = match values.next() {
				Some(path_dst) => file::dict_from_src_dst(path_src, path_dst)?,
				None => file::dict_from_src(path_src)?,
			};
			let mut result = dict.into_iter().collect::<Vec<_>>();
			result.sort();
			result
		} else {
			todo!();
		}
	};

	let mut wtr = csv::WriterBuilder::new()
		.delimiter(b'\t')
		.has_headers(false)
		.from_writer(io::stdout());
	for record in result {
		let _ = wtr.serialize(record)?;
	}
	wtr.flush()?;

	Ok(())
}
