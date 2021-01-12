use anyhow::Result;
use env_logger;
use log::error;
use serde::{Deserialize, Serialize};

mod file;
mod options;
use file::*;
use options::*;

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
		Command::CreateDict(path_src, None) => dict_from_src(path_src)?,
		Command::CreateDict(path_src, Some(path_dst)) => dict_from_src_dst(path_src, path_dst)?,
		_ => todo!(),
	};

	let mut result = result.into_iter().collect::<Vec<_>>();
	result.sort();

	match options.output() {
		Some(path) => write_csv_file(result, path),
		None => write_csv_stdout(result),
	}
}
