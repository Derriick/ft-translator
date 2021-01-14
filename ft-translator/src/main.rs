use std::io;

use anyhow::{Context, Result};
use env_logger;
use log::error;
use translator::csv_stream;
use translator::dict::Dict;

mod options;
use crate::options::*;

fn main() {
	let options = Options::new();

	env_logger::builder()
		.filter_level(options.verbosity())
		.init();

	if let Err(err) = run(options) {
		error!("{}", err);
	}
}

fn run(options: Options) -> Result<()> {
	let result = match options.command() {
		Command::CreateDict(path_src, None) => Dict::from_file_src(path_src, get_text)?,
		Command::CreateDict(path_src, Some(path_dst)) => {
			Dict::from_file_src_dst(path_src, path_dst, get_entry)?
		}
		_ => todo!(),
	};

	let mut result = result.into_iter().collect::<Vec<_>>();
	result.sort();

	match options.output() {
		Some(path) => csv_stream::write_file(result, path)
			.with_context(|| format!("Failed to write CSV records to file '{}'", path.display())),
		None => csv_stream::write(result, io::stdout())
			.with_context(|| format!("Failed to write CSV records to STDOUT")),
	}
}

fn get_text(record: (String, String, String, String)) -> String {
	record.3
}

fn get_entry(record: (String, String, String, String)) -> ((String, String, String), String) {
	((record.0, record.1, record.2), record.3)
}
