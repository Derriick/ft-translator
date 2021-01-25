#![allow(unused_variables)]

use std::io;

use anyhow::{Context, Result};
use env_logger;
use log::error;
use translator::{csv_stream, dict::Dict};

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
		Command::CreateDict { src, dst: None } => Dict::from_file_src(src, get_entry)?,
		Command::CreateDict {
			src,
			dst: Some(dst),
		} => Dict::from_file_src_dst(src, dst, get_entry)?,
		Command::MergeDict { dict1, dict2 } => Dict::merge_file(dict1, dict2)?,
		Command::SwapDict { dict } => Dict::swap_file(dict)?,
		Command::Translate { src, dict } => todo!(),
		Command::None => todo!(),
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

fn get_entry(record: (String, String, String, String)) -> ((String, String, String), String) {
	((record.0, record.1, record.2), record.3)
}
