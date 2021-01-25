use std::path::{Path, PathBuf};

use clap::{App, Arg, ArgMatches};
use log::LevelFilter;

pub const CREATE_DICT: &str = "create-dict";
pub const MERGE_DICT: &str = "merge-dict";
pub const SWAP_DICT: &str = "swap-dict";
pub const TRANSLATE: &str = "translate";
pub const OUTPUT: &str = "output";
pub const VERBOSITY: &str = "verbose";

pub mod short {
	pub const CREATE_DICT: &str = "c";
	pub const MERGE_DICT: &str = "m";
	pub const SWAP_DICT: &str = "s";
	pub const TRANSLATE: &str = "t";
	pub const OUTPUT: &str = "o";
	pub const VERBOSITY: &str = "v";
}

pub struct Options {
	command: Command,
	verbosity: LevelFilter,
	output: Option<PathBuf>,
}

pub enum Command {
	CreateDict { src: PathBuf, dst: Option<PathBuf> },
	MergeDict { dict1: PathBuf, dict2: PathBuf },
	SwapDict { dict: PathBuf },
	Translate { src: PathBuf, dict: PathBuf },
	None,
}

impl Options {
	pub fn new() -> Self {
		let matches = matches();

		let command = {
			if let Some(mut values) = matches.values_of(CREATE_DICT) {
				let src = next_path(&mut values).unwrap();
				let dst = next_path(&mut values);
				Command::CreateDict { src, dst }
			} else if let Some(mut values) = matches.values_of(MERGE_DICT) {
				let dict1 = next_path(&mut values).unwrap();
				let dict2 = next_path(&mut values).unwrap();
				Command::MergeDict { dict1, dict2 }
			} else if let Some(mut values) = matches.values_of(SWAP_DICT) {
				let dict = next_path(&mut values).unwrap();
				Command::SwapDict { dict }
			} else if let Some(mut values) = matches.values_of(TRANSLATE) {
				let src = next_path(&mut values).unwrap();
				let dict = next_path(&mut values).unwrap();
				Command::Translate { src, dict }
			} else {
				Command::None
			}
		};
		let verbosity = match matches.occurrences_of(VERBOSITY) {
			//0 => LevelFilter::Error, // error
			0 => LevelFilter::Warn,  // error + warn
			1 => LevelFilter::Info,  // error + warn + info
			2 => LevelFilter::Debug, // error + warn + info + debug
			_ => LevelFilter::Trace, // error + warn + info + debug + trace
		};
		let output = matches.value_of(OUTPUT).map(|p| Path::new(p).to_path_buf());

		Options {
			command,
			verbosity,
			output,
		}
	}

	#[inline]
	pub fn verbosity(&self) -> LevelFilter {
		self.verbosity
	}

	#[inline]
	pub fn command(&self) -> &Command {
		&self.command
	}

	#[inline]
	pub fn output(&self) -> Option<&Path> {
		self.output.as_ref().map(|p| p.as_path())
	}
}

fn next_path(values: &mut clap::Values) -> Option<PathBuf> {
	values.next().map(|p| Path::new(p).to_path_buf())
}

fn matches<'a>() -> ArgMatches<'a> {
	App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg(
			Arg::with_name(CREATE_DICT)
				.long(CREATE_DICT)
				.short(short::CREATE_DICT)
				.value_names(&["SRC"])
				.min_values(1)
				.max_values(2)
				.required_unless_one(&[MERGE_DICT, SWAP_DICT, TRANSLATE])
				.conflicts_with_all(&[MERGE_DICT, SWAP_DICT, TRANSLATE])
				.help(
					"Create a new dictionary from a source file, and optionaly a destination file",
				),
		)
		.arg(
			Arg::with_name(MERGE_DICT)
				.long(MERGE_DICT)
				.short(short::MERGE_DICT)
				.value_names(&["DICT1", "DICT2"])
				.required_unless_one(&[CREATE_DICT, SWAP_DICT, TRANSLATE])
				.conflicts_with_all(&[CREATE_DICT, SWAP_DICT, TRANSLATE])
				.help("Merge two dictionaries"),
		)
		.arg(
			Arg::with_name(SWAP_DICT)
				.long(SWAP_DICT)
				.short(short::SWAP_DICT)
				.value_names(&["DICT"])
				.required_unless_one(&[CREATE_DICT, MERGE_DICT, TRANSLATE])
				.conflicts_with_all(&[CREATE_DICT, MERGE_DICT, TRANSLATE])
				.help("Swap the source and destination of a dictionary"),
		)
		.arg(
			Arg::with_name(TRANSLATE)
				.long(TRANSLATE)
				.short(short::TRANSLATE)
				.value_names(&["SRC", "DICT"])
				.required_unless_one(&[CREATE_DICT, MERGE_DICT, SWAP_DICT])
				.conflicts_with_all(&[CREATE_DICT, MERGE_DICT, SWAP_DICT])
				.help("Translate a source file with a dictionary"),
		)
		.arg(
			Arg::with_name(OUTPUT)
				.long(OUTPUT)
				.short(short::OUTPUT)
				.value_name("FILE")
				.help("Save the result in a file instead of writing in the the standard output"),
		)
		.arg(
			Arg::with_name(VERBOSITY)
				.long(VERBOSITY)
				.short(short::VERBOSITY)
				.multiple(true)
				.help(
					"Set the level of verbosity, the number of occurences inscreases the verbosity",
				),
		)
		.get_matches()
}
