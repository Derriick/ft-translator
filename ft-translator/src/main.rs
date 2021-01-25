#![allow(unused_variables)]

use std::io;

use anyhow::{Context, Result};
use env_logger;
use log::error;
use translator::{csv, dict::Dict};

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
		Command::CreateDict { src, dst: None } => {
			let src = csv::read_file(src)?;
			Dict::from_src(&src, get_entry)?
		}
		Command::CreateDict {
			src,
			dst: Some(dst),
		} => {
			let src = csv::read_file(src)?;
			let dst = csv::read_file(dst)?;
			Dict::from_src_dst(&src, &dst, get_entry)?
		}
		Command::MergeDict { dict1, dict2 } => {
			let dict1 = csv::read_file(dict1)?;
			let dict2 = csv::read_file(dict2)?;
			let dict1 = Dict::from_dict(&dict1)?;
			let dict2 = Dict::from_dict(&dict2)?;
			Dict::merge(dict1, dict2)
		}
		Command::SwapDict { dict } => {
			let dict = csv::read_file(dict)?;
			let dict = Dict::from_dict(&dict)?;
			Dict::swap(dict)
		}
		Command::Translate { src, dict } => todo!(),
		Command::None => todo!(),
	};

	let mut result = result.into_iter().collect::<Vec<_>>();
	result.sort();

	match options.output() {
		Some(path) => csv::write_file(result, path)
			.with_context(|| format!("Failed to write CSV records to file '{}'", path.display())),
		None => csv::write(result, io::stdout())
			.with_context(|| format!("Failed to write CSV records to STDOUT")),
	}
}

fn get_entry(record: (String, String, String, String)) -> ((String, String, String), String) {
	((record.0, record.1, record.2), record.3)
}
