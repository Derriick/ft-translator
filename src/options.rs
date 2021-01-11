use clap::{App, Arg, ArgMatches};
use log::LevelFilter;

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

#[inline]
pub fn matches<'a>() -> ArgMatches<'a> {
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
				.conflicts_with_all(&[CREATE_DICT, SWAP_DICT, TRANSLATE])
				.help("Merge two dictionaries"),
		)
		.arg(
			Arg::with_name(SWAP_DICT)
				.long(SWAP_DICT)
				.short(short::SWAP_DICT)
				.value_names(&["DICT"])
				.conflicts_with_all(&[CREATE_DICT, MERGE_DICT, TRANSLATE])
				.help("Swap the source and destination of a dictionary"),
		)
		.arg(
			Arg::with_name(TRANSLATE)
				.long(TRANSLATE)
				.short(short::TRANSLATE)
				.value_names(&["SRC", "DICT"])
				.conflicts_with_all(&[CREATE_DICT, MERGE_DICT, SWAP_DICT])
				.help("Translate a source file with a dictionary"),
		)
		//.arg(
		//	Arg::with_name(OUTPUT)
		//		.long(OUTPUT)
		//		.short(short::OUTPUT)
		//		.value_name("FILE")
		//		.help("Save the result in a file instead of writing in the the standard output"),
		//)
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

#[inline]
pub fn verbosity(matches: &ArgMatches) -> LevelFilter {
	match matches.occurrences_of(VERBOSITY) {
		//0 => LevelFilter::Error, // error
		0 => LevelFilter::Warn,  // error + warn
		1 => LevelFilter::Info,  // error + warn + info
		2 => LevelFilter::Debug, // error + warn + info + debug
		_ => LevelFilter::Trace, // error + warn + info + debug + trace
	}
}
