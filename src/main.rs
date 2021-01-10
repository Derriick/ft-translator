#[allow(dead_code)]
#[allow(unused)]
#[allow(unused_imports)]

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

use anyhow::{anyhow, Result};
use clap::{App, Arg, ArgMatches, Values};
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
		.arg(Arg::with_name(key::CREATE_DICT)
		     .long(key::CREATE_DICT)
		     .short(key::short::CREATE_DICT)
		     .value_names(&["SRC"])
		     .min_values(1)
		     .max_values(2)
		     .conflicts_with_all(&[key::MERGE_DICT, key::SWAP_DICT, key::TRANSLATE])
		     .help("Create a new dictionary from a source file, and optionaly a destination file"))
		.arg(Arg::with_name(key::MERGE_DICT)
		     .long(key::MERGE_DICT)
		     .short(key::short::MERGE_DICT)
		     .value_names(&["DICT1", "DICT2"])
		     .conflicts_with_all(&[key::CREATE_DICT, key::SWAP_DICT, key::TRANSLATE])
		     .help("Merge two dictionaries"))
		.arg(Arg::with_name(key::SWAP_DICT)
		     .long(key::SWAP_DICT)
		     .short(key::short::SWAP_DICT)
		     .value_names(&["DICT"])
		     .conflicts_with_all(&[key::CREATE_DICT, key::MERGE_DICT, key::TRANSLATE])
		     .help("Swap the source and destination of a dictionary"))
		.arg(Arg::with_name(key::TRANSLATE)
		     .long(key::TRANSLATE)
		     .short(key::short::TRANSLATE)
		     .value_names(&["SRC", "DICT"])
		     .conflicts_with_all(&[key::CREATE_DICT, key::MERGE_DICT, key::SWAP_DICT])
		     .help("Translate a source file with a dictionary"))
		//.arg(Arg::with_name(key::OUTPUT)
		//     .long(key::OUTPUT)
		//     .short(key::short::OUTPUT)
		//     .value_name("FILE")
		//     .help("Save the result in a file instead of writing in the the standard output"))
		.arg(Arg::with_name(key::VERBOSITY)
		     .long(key::VERBOSITY)
		     .short(key::short::VERBOSITY)
		     .multiple(true)
		     .help("Set the level of verbosity, the number of occurences inscreases the verbosity"))
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
	let _ = if let Some(mut values) = matches.values_of(key::CREATE_DICT) {
		let src = values.next().unwrap();
		if let Some(dst) = values.next() {
			Dict::from_src_dst(src, dst)
		} else {
			Dict::from_src(src)
		}
	} else {
		todo!();
	};

	let file = File::open("input.csv")?;
	let mut decoder = DecodeReaderBytes::new(file);
	let mut transcoded = String::new();
	decoder.read_to_string(&mut transcoded)?;

	let mut content = String::new();
	for line in transcoded.lines() {
		if !line.starts_with('#') {
			content.push_str(&line);
			content.push_str("\n");
		}
	}
	
	// Build the CSV reader and iterate over each record.
	let mut rdr = csv::ReaderBuilder::new()
		.delimiter(b'\t')
		.has_headers(false)
		.from_reader(content.as_bytes());

	let mut dict = HashSet::new();
	for result in rdr.deserialize() {
		// The iterator yields Result<StringRecord, Error>, so we check the
		// error here..
		let entry: Entry = result?;
		dict.insert(entry.text);
	}
	let mut dict = dict.into_iter().map(|text| text.unwrap_or(String::new())).collect::<Vec<_>>();
	dict.sort();

	let mut wtr = csv::WriterBuilder::new().delimiter(b'\t').has_headers(false).from_path("output.csv")?;
	for entry in dict {
		let _ = wtr.serialize(DictEntry {src: entry, dst: String::new()})?;
	}
	wtr.flush()?;

	Ok(())
}
