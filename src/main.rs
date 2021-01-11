use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use encoding_rs_io::DecodeReaderBytes;
use env_logger;
use log::{error, LevelFilter};
use serde::{Deserialize, Serialize};

use ft_translator::*;

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

mod key {
	pub const CREATE_DICT: &str = "create-dict";
	pub const MERGE_DICT: &str = "merge-dict";
	pub const SWAP_DICT: &str = "swap-dict";
	pub const TRANSLATE: &str = "translate";
	//pub const OUTPUT: &str = "output";
	pub const VERBOSITY: &str = "verbose";

	pub mod short {
		pub const CREATE_DICT: &str = "c";
		pub const MERGE_DICT: &str = "m";
		pub const SWAP_DICT: &str = "s";
		pub const TRANSLATE: &str = "t";
		//pub const OUTPUT: &str = "o";
		pub const VERBOSITY: &str = "v";
	}
}

fn main() {
	let matches = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(
			Arg::with_name(key::CREATE_DICT)
				.long(key::CREATE_DICT)
				.short(key::short::CREATE_DICT)
				.value_names(&["SRC"])
				.min_values(1)
				.max_values(2)
				.conflicts_with_all(&[key::MERGE_DICT, key::SWAP_DICT, key::TRANSLATE])
				.help(
					"Create a new dictionary from a source file, and optionaly a destination file",
				),
		)
		.arg(
			Arg::with_name(key::MERGE_DICT)
				.long(key::MERGE_DICT)
				.short(key::short::MERGE_DICT)
				.value_names(&["DICT1", "DICT2"])
				.conflicts_with_all(&[key::CREATE_DICT, key::SWAP_DICT, key::TRANSLATE])
				.help("Merge two dictionaries"),
		)
		.arg(
			Arg::with_name(key::SWAP_DICT)
				.long(key::SWAP_DICT)
				.short(key::short::SWAP_DICT)
				.value_names(&["DICT"])
				.conflicts_with_all(&[key::CREATE_DICT, key::MERGE_DICT, key::TRANSLATE])
				.help("Swap the source and destination of a dictionary"),
		)
		.arg(
			Arg::with_name(key::TRANSLATE)
				.long(key::TRANSLATE)
				.short(key::short::TRANSLATE)
				.value_names(&["SRC", "DICT"])
				.conflicts_with_all(&[key::CREATE_DICT, key::MERGE_DICT, key::SWAP_DICT])
				.help("Translate a source file with a dictionary"),
		)
		//.arg(
		//	Arg::with_name(key::OUTPUT)
		//		.long(key::OUTPUT)
		//		.short(key::short::OUTPUT)
		//		.value_name("FILE")
		//		.help("Save the result in a file instead of writing in the the standard output"),
		//)
		.arg(
			Arg::with_name(key::VERBOSITY)
				.long(key::VERBOSITY)
				.short(key::short::VERBOSITY)
				.multiple(true)
				.help(
					"Set the level of verbosity, the number of occurences inscreases the verbosity",
				),
		)
		.get_matches();

	env_logger::builder()
		.filter_level(verbosity(matches.occurrences_of(key::VERBOSITY)))
		.init();

	if let Err(err) = run(matches) {
		error!("{}", err);
	}
}

pub fn verbosity(level: u64) -> LevelFilter {
	match level {
		//0 => LevelFilter::Error, // error
		0 => LevelFilter::Warn,  // error + warn
		1 => LevelFilter::Info,  // error + warn + info
		2 => LevelFilter::Debug, // error + warn + info + debug
		_ => LevelFilter::Trace, // error + warn + info + debug + trace
	}
}

fn run(matches: ArgMatches) -> Result<()> {
	let result = if let Some(mut values) = matches.values_of(key::CREATE_DICT) {
		let path_src = values.next().unwrap();
		let result = if let Some(path_dst) = values.next() {
			dict_from_src_dst(path_src, path_dst)?
		} else {
			dict_from_src(path_src)?
		};
		let mut result = result
			.into_iter()
			.collect::<Vec<_>>();
			result.sort();
		result
	} else {
		todo!();
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

fn dict_from_src<P>(path: P) -> Result<Dict>
where
	P: AsRef<Path>,
{
	let src = read_file(path)?;
	Dict::from_src(&src)
}

fn dict_from_src_dst<P, Q>(path_src: P, path_dst: Q) -> Result<Dict>
where
	P: AsRef<Path>,
	Q: AsRef<Path>,
{
	let src = read_file(path_src)?;
	let dst = read_file(path_dst)?;
	Dict::from_src_dst(&src, &dst)
}

fn read_file<P>(path: P) -> Result<String>
where
	P: AsRef<Path>,
{
	let file = File::open(path)?;
	let mut decoder = DecodeReaderBytes::new(file);
	let mut transcoded = String::new();
	decoder.read_to_string(&mut transcoded)?;

	let content = transcoded
		.lines()
		.filter(|l| !l.starts_with('#'))
		.collect::<Vec<_>>()
		.join("\n");

	Ok(content)
}