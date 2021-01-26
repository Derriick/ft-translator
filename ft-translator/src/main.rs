use anyhow::Result;
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
	match options.command() {
		Command::CreateDict { src, dst: None } => {
			let src = csv_stream::read_file(src)?;
			let result = Dict::from_src(&src, get_entry)?;
			csv_stream::write_collec(result, options.output())?
		}
		Command::CreateDict {
			src,
			dst: Some(dst),
		} => {
			let src = csv_stream::read_file(src)?;
			let dst = csv_stream::read_file(dst)?;
			let result = Dict::from_src_dst(&src, &dst, get_entry)?;
			csv_stream::write_collec(result, options.output())?
		}
		Command::MergeDict { dict1, dict2 } => {
			let dict1 = csv_stream::read_file(dict1)?;
			let dict2 = csv_stream::read_file(dict2)?;
			let dict1 = Dict::from_dict(&dict1)?;
			let dict2 = Dict::from_dict(&dict2)?;
			let result = dict1.merge(dict2);
			csv_stream::write_collec(result, options.output())?
		}
		Command::SwapDict { dict } => {
			let dict = csv_stream::read_file(dict)?;
			let dict = Dict::from_dict(&dict)?;
			let result = dict.swap();
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

fn get_entry(record: (String, String, String, String)) -> ((String, String, String), String) {
	((record.0, record.1, record.2), record.3)
}
