use std::hash::Hash;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use translator::{csv_stream, dict::Dict};

mod options;

use options::{Command, Options};

pub fn run<T, K>(get_entry: fn(T) -> (K, String)) -> Result<()>
where
	T: for<'de> Deserialize<'de>,
	K: Hash + Eq + Serialize,
{
	let options = Options::new();

	env_logger::builder()
		.filter_level(options.verbosity())
		.init();

	match options.command() {
		Command::CreateDict { src, dst: None } => {
			let src = csv_stream::read_file(src)?;
			let mut result: Vec<_> = Dict::from_src(&src, get_entry)?.into_iter().collect();
			result.sort();
			csv_stream::write_collec(result, options.output())?
		}
		Command::CreateDict {
			src,
			dst: Some(dst),
		} => {
			let src = csv_stream::read_file(src)?;
			let dst = csv_stream::read_file(dst)?;
			let mut result: Vec<_> = Dict::from_src_dst(&src, &dst, get_entry)?
				.into_iter()
				.collect();
			result.sort();
			csv_stream::write_collec(result, options.output())?
		}
		Command::MergeDict { dict1, dict2 } => {
			let dict1 = csv_stream::read_file(dict1)?;
			let dict2 = csv_stream::read_file(dict2)?;
			let dict1 = Dict::from_dict(&dict1)?;
			let dict2 = Dict::from_dict(&dict2)?;
			let mut result: Vec<_> = dict1.merge(dict2).into_iter().collect();
			result.sort();
			csv_stream::write_collec(result, options.output())?
		}
		Command::SwapDict { dict } => {
			let dict = csv_stream::read_file(dict)?;
			let dict = Dict::from_dict(&dict)?;
			let mut result: Vec<_> = dict.swap().into_iter().collect();
			result.sort();
			csv_stream::write_collec(result, options.output())?
		}
		Command::Translate { src, dict } => {
			let src = csv_stream::read_file(src)?;
			let dict = csv_stream::read_file(dict)?;
			let dict = Dict::from_dict(&dict)?;
			let result = dict.translate(&src, get_entry)?;
			csv_stream::write_collec(result, options.output())?
		}
		Command::None => todo!(),
	};
	Ok(())
}
